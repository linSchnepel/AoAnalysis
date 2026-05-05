/// Data loading. Server only.
///
/// Scans `assets/data/` for JSONL files, groups them by fandom name
/// (the prefix before `_CLEAN`, `_HISTORIES`, etc.), parses each file, and
/// merges entries into their corresponding Listings.
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;

use tracing::{error, info, warn};

use crate::models::{History, Listing};
use crate::state::{AppState, FandomData, SharedState};

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

        fandoms.insert(fandom_name, FandomData { listings });
    }

    info!(
        "Data loading complete. {} fandom(s), {} total listings",
        fandoms.len(),
        fandoms.values().map(|f| f.listings.len()).sum::<usize>()
    );

    Arc::new(AppState { fandoms })
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
        // Unknown suffix → silently ignore
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