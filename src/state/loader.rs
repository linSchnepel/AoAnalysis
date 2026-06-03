/// Data loading. Server only.
///
/// Scans `assets/data/` for JSONL files, groups them by fandom name
/// (the prefix before `_CLEAN`, `_HISTORIES`, etc.), parses each file, and
/// merges entries into their corresponding Listings.
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;

use tracing::{error, info, warn};
use chrono::{Datelike, TimeZone, Utc};

use crate::models::{History, Listing, Category};
use crate::state::{AppState, FandomData, SharedState, ListingIndex, AppConfig, FandomStats};
use crate::models::ChapterCount;

const DATA_DIR: &str = "assets/data";

const SUFFIX_CLEAN: &str = "_CLEAN";
const SUFFIX_HISTORIES: &str = "_HISTORIES";
// TODO: const SUFFIX_COMMENTS: &str = "_COMMENTS";

/// Scan the data directory, parse all JSONL files, merge, and
/// return a fully populated `SharedState`.
///
/// Parse errors on individual lines are logged and skipped; a file that
/// entirely fails to open is also logged and skipped.
pub async fn load_app_state() -> SharedState {
    info!("Loading data from {DATA_DIR}");

    let dir = Path::new(DATA_DIR);
    if !dir.exists() {
        warn!("Data directory {DATA_DIR} does not exist — starting with empty state");
        return Arc::new(AppState::default());
    }

    // Discover files and group by fandom name.
    let grouped = discover_files(dir);

    let mut fandoms: HashMap<String, FandomData> = HashMap::new();

    for (fandom_name, file_group) in grouped {
        info!("Loading fandom: {fandom_name}");

        // Parse listings
        let listings = match &file_group.clean {
            Some(path) => parse_jsonl_lines::<Listing>(path),
            None => {
                warn!("Fandom '{fandom_name}' has no _CLEAN file — skipping");
                continue;
            }
        };

        // Parse histories
        let mut histories: Vec<History> = file_group
            .histories
            .as_ref()
            .map(|path| parse_jsonl_lines::<History>(path))
            .unwrap_or_default();

        // TODO: parse comments, kudos, etc

        // Merge into listings
        let listings = merge_histories(listings, &mut histories, &fandom_name);

        info!(
            "  {} listings loaded ({} with inline history, {} leftover from _HISTORIES)",
            listings.len(),
            listings.iter().filter(|l| l.history.is_some()).count(),
            histories.len(),
        );

        // Warn about any records that didn't match a Listing
        for unmatched in &histories {
            warn!(
                "  History id '{}' has no matching listing in '{fandom_name}'",
                unmatched.id
            );
        }

        fandoms.insert(fandom_name, FandomData { listings, index: ListingIndex::default(), stats: FandomStats::default() });
    }

    // Load config
    let config = AppConfig::load().unwrap_or_else(|e| {
        warn!("Failed to load config: {e} — no featured fandom set");
        AppConfig { featured_fandom: String::new() }
    });

    // Build per-fandom index and stats
    for fandom_data in fandoms.values_mut() {
        for listing in &fandom_data.listings {
            fandom_data.index.add(listing);
        }
        fandom_data.stats = compute_stats(&fandom_data.listings);
    }

    info!(
        "Data loading complete. {} fandom(s), {} total listings",
        fandoms.len(),
        fandoms.values().map(|f| f.listings.len()).sum::<usize>()
    );

    let mut index = ListingIndex::default();
    for fandom_data in fandoms.values() {
        for listing in &fandom_data.listings {
            index.add(listing);
        }
    }

    info!(
        "Index built: {} tags, {} authors, {} series",
        index.tags.len(),
        index.authors.len(),
        index.series.len(),
    );

    Arc::new(AppState { fandoms, featured_fandom: config.featured_fandom  })
}

#[cfg(feature = "ssr")]
fn compute_stats(listings: &[Listing]) -> FandomStats {
    let mut creations_by_year: HashMap<i32, u32> = HashMap::new();
    let mut one_shots = 0u32;
    let mut complete_multi = 0u32;
    let mut incomplete_multi = 0u32;
    let mut category_counts: HashMap<Category, u32> = HashMap::new();
    let mut kudos: Vec<u32> = Vec::with_capacity(listings.len());
    let mut hits: Vec<u32> = Vec::with_capacity(listings.len());
    let mut words: Vec<u32> = Vec::with_capacity(listings.len());
    let mut bookmarks: Vec<u32> = Vec::with_capacity(listings.len());

    for listing in listings {
        // Creation date: first historyUNIX entry, or update_unix
        let created_unix = listing.history
            .as_ref()
            .and_then(|h| h.first())
            .map(|e| e.unix.as_millis())
            .unwrap_or(listing.update_unix.as_millis());

        // Unix ms → year
        let year = chrono::DateTime::from_timestamp_millis(created_unix)
            .map(|dt| dt.year())
            .unwrap_or(0);
        *creations_by_year.entry(year).or_default() += 1;

        // Completion breakdown
        let is_one_shot = matches!(
            &listing.stats.chapters,
            Some(ChapterCount { published: 1, total: Some(1) })
        );
        let is_complete = listing.completion == crate::models::Completion::CompleteWork;
        match (is_one_shot, is_complete) {
            (true, _)      => one_shots += 1,
            (false, true)  => complete_multi += 1,
            (false, false) => incomplete_multi += 1,
        }

        // Category counts
        for cat in &listing.category {
            *category_counts.entry(cat.clone()).or_default() += 1;
        }

        // Stats for percentiles
        kudos.push(listing.stats.kudos.unwrap_or(0) as u32);
        hits.push(listing.stats.hits.unwrap_or(0) as u32);
        words.push(listing.stats.words.unwrap_or(0) as u32);
        bookmarks.push(listing.stats.bookmarks.unwrap_or(0) as u32);

        for t in &listing.tags    { unique_tags.insert(t.as_str()); }
        for a in &listing.authors { unique_authors.insert(a.as_str()); }
        for s in &listing.series  { unique_series.insert(s.as_str()); }

        total_words     += listing.stats.words.unwrap_or(0) as u64;
        total_hits      += listing.stats.hits.unwrap_or(0) as u64;
        total_kudos     += listing.stats.kudos.unwrap_or(0) as u64;
        total_bookmarks += listing.stats.bookmarks.unwrap_or(0) as u64;
        total_comments  += listing.stats.comments.unwrap_or(0) as u64;
    }

    // Sort for percentile queries
    kudos.sort_unstable();
    hits.sort_unstable();
    words.sort_unstable();
    bookmarks.sort_unstable();

    // Flatten and sort creations by year
    let mut creations_by_year: Vec<(i32, u32)> = creations_by_year.into_iter().collect();
    creations_by_year.sort_unstable_by_key(|(year, _)| *year);

    FandomStats {
        creations_by_year,
        completion_breakdown: (one_shots, complete_multi, incomplete_multi),
        category_counts,
        kudos_sorted: kudos,
        hits_sorted: hits,
        words_sorted: words,
        bookmarks_sorted: bookmarks,
        
        total_listings:  listings.len() as u32,
        unique_tags:     unique_tags.len() as u32,
        unique_authors:  unique_authors.len() as u32,
        unique_series:   unique_series.len() as u32,
        total_words:     total_words,
        total_hits:      total_hits,
        total_kudos:     total_kudos,
        total_bookmarks: total_bookmarks,
        total_comments:  total_comments,
    }
}

// ---------------------------------------------------------------------------
// File discovery
// ---------------------------------------------------------------------------

struct FileGroup {
    clean: Option<std::path::PathBuf>,
    histories: Option<std::path::PathBuf>,
}

fn discover_files(dir: &Path) -> HashMap<String, FileGroup> {
    let mut groups: HashMap<String, FileGroup> = HashMap::new();

    let entries = match std::fs::read_dir(dir) {
        Ok(e) => e,
        Err(e) => {
            error!("Failed to read data directory: {e}");
            return groups;
        }
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("jsonl") {
            continue;
        }

        let stem = match path.file_stem().and_then(|s| s.to_str()) {
            Some(s) => s.to_owned(),
            None => continue,
        };

        if let Some(fandom) = stem.strip_suffix(SUFFIX_CLEAN) {
            groups
                .entry(fandom.to_owned())
                .or_insert_with(|| FileGroup { clean: None, histories: None })
                .clean = Some(path);
        } else if let Some(fandom) = stem.strip_suffix(SUFFIX_HISTORIES) {
            groups
                .entry(fandom.to_owned())
                .or_insert_with(|| FileGroup { clean: None, histories: None })
                .histories = Some(path);
        }
        // Unknown suffix will ignore
    }

    groups
}

// ---------------------------------------------------------------------------
// JSONL parsing
// ---------------------------------------------------------------------------

/// Parse a JSONL file line by line. Malformed lines are logged and skipped.
fn parse_jsonl_lines<T>(path: &Path) -> Vec<T>
where
    T: serde::de::DeserializeOwned,
{
    let content = match std::fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => {
            error!("Failed to read {}: {e}", path.display());
            return Vec::new();
        }
    };

    let mut results = Vec::new();
    for (line_num, line) in content.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        match serde_json::from_str::<T>(line) {
            Ok(item) => results.push(item),
            Err(e) => {
                error!(
                    "{}:{} parse error: {e}",
                    path.display(),
                    line_num + 1
                );
            }
        }
    }

    results
}

// ---------------------------------------------------------------------------
// History merging
// ---------------------------------------------------------------------------

/// Merge `histories` into the matching listings by `id`.
///
/// Matched histories are removed from the `histories` Vec so that any
/// remaining entries can be reported as unmatched by the caller.
fn merge_histories(
    mut listings: Vec<Listing>,
    histories: &mut Vec<History>,
    fandom_name: &str,
) -> Vec<Listing> {
    if histories.is_empty() {
        return listings;
    }

    // Build index: id → position in histories vec.
    let id_to_pos: HashMap<String, usize> = histories
        .iter()
        .enumerate()
        .map(|(i, h)| (h.id.clone(), i))
        .collect();

    let total_histories = id_to_pos.len();
    let mut matched_positions: Vec<usize> = Vec::new();

    for listing in &mut listings {
        if listing.history.is_some() {
            // Already has inline history — skip.
            continue;
        }
        if let Some(&pos) = id_to_pos.get(&listing.id) {
            listing.history = Some(histories[pos].entries.clone());
            matched_positions.push(pos);
        }
    }

    // Remove matched histories (in reverse order to preserve indices).
    matched_positions.sort_unstable();
    matched_positions.dedup();
    for pos in matched_positions.into_iter().rev() {
        histories.swap_remove(pos);
    }

    info!("  Merged {} history records into listings for '{fandom_name}'", total_histories - histories.len());

    listings
}