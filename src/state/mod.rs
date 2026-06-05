// All of these should be gated because only DTOs will cross the wire to the client
// This is the main application state, loaded once at server startup and never mutated.

#[cfg(feature = "ssr")]
mod loader;

#[cfg(feature = "ssr")]
mod config;

#[cfg(feature = "ssr")]
pub use loader::load_app_state;

#[cfg(feature = "ssr")]
pub use config::AppConfig;

use std::collections::HashMap;
use std::sync::Arc;

use crate::models::{Listing, Warning, Category};

/// All data for a single fandom, loaded once at startup.
#[derive(Debug, Default)]
pub struct FandomData {
    pub listings: Vec<Listing>,
    pub index: ListingIndex,
    pub stats: FandomStats, // main page stats
    // TODO: pub dataframe: polars::frame::DataFrame,
    // TODO: comments: Vec<Comment>,
}

impl FandomData {
    pub fn listing_count(&self) -> usize {
        self.listings.len()
    }
}

/// Pre-computed statistics for a fandom, derived at load time.
/// Never sent to the client directly. Exposed only via ChartDto through server fns.
#[derive(Debug, Default)]
pub struct FandomStats {
    /// Works created per (year, month): (year, month, count). Month is 1-indexed.
    pub creations_by_month: Vec<(i32, u32, u32)>,
    /// Completion breakdown: (one_shots, complete_multi, incomplete_multi)
    pub completion_breakdown: (u32, u32, u32),
    /// Count of works per category (works can have multiple)
    pub category_counts: HashMap<Category, u32>,
    /// Sorted kudos values for percentile computation
    pub kudos_sorted: Vec<u32>,
    /// Sorted hit values for percentile computation
    pub hits_sorted: Vec<u32>,
    /// Sorted word counts for percentile computation
    pub words_sorted: Vec<u32>,
    /// Sorted bookmark counts for percentile computation
    pub bookmarks_sorted: Vec<u32>,

    pub total_listings: u32,
    pub unique_tags: u32,
    pub unique_authors: u32,
    pub unique_series: u32,
    pub total_words: u64,
    pub total_hits: u64,
    pub total_kudos: u64,
    pub total_bookmarks: u64,
    pub total_comments: u64,
}

impl FandomStats {
    pub fn percentiles(sorted: &[u32], pcts: &[f64]) -> Vec<u32> {
        if sorted.is_empty() {
            return vec![0; pcts.len()];
        }
        pcts.iter().map(|&p| {
            let idx = ((p / 100.0) * (sorted.len() - 1) as f64).round() as usize;
            sorted[idx.min(sorted.len() - 1)]
        }).collect()
    }
}

// This is for better performance when filtering by tags, fandoms, archetypes, etc
#[derive(Debug, Default)]
pub struct ListingIndex {
    pub tags:       HashMap<String, Vec<String>>,
    pub fandoms:    HashMap<String, Vec<String>>,
    pub archetypes: HashMap<String, Vec<String>>,
    pub authors:    HashMap<String, Vec<String>>,
    pub series:     HashMap<String, Vec<String>>,
    pub warnings:   HashMap<Warning, Vec<String>>,
    pub category:   HashMap<Category, Vec<String>>,
}

impl ListingIndex {
    fn insert_str(map: &mut HashMap<String, Vec<String>>, key: &str, id: &str) {
        map.entry(key.to_owned()).or_default().push(id.to_owned());
    }

    pub fn add(&mut self, listing: &Listing) {
        let id = &listing.id;
        for t in &listing.tags       { Self::insert_str(&mut self.tags,       t, id); }
        for f in &listing.fandoms    { Self::insert_str(&mut self.fandoms,    f, id); }
        for a in &listing.archetypes { Self::insert_str(&mut self.archetypes, a, id); }
        for a in &listing.authors    { Self::insert_str(&mut self.authors,    a, id); }
        for s in &listing.series     { Self::insert_str(&mut self.series,     s, id); }
        for w in &listing.warnings   { self.warnings.entry(w.clone()).or_default().push(id.clone()); }
        for c in &listing.category   { self.category.entry(c.clone()).or_default().push(id.clone()); }
    }
}

/// Global application state. Loaded once at server startup, never mutated.
#[derive(Debug, Default)]
pub struct AppState {
    pub fandoms: HashMap<String, FandomData>,
    pub featured_fandom: String,
}

impl AppState {
    pub fn total_listings(&self) -> usize {
        self.fandoms.values().map(|f| f.listing_count()).sum()
    }

    pub fn fandom_names(&self) -> Vec<&str> {
        let mut names: Vec<&str> = self.fandoms.keys().map(String::as_str).collect();
        names.sort();
        names
    }

    pub fn featured(&self) -> Option<&FandomData> {
        self.fandoms.get(&self.featured_fandom)
    }
}

/// Convenience alias used throughout the app.
pub type SharedState = Arc<AppState>;