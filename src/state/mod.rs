#[cfg(feature = "ssr")]
mod loader;

#[cfg(feature = "ssr")]
pub use loader::load_app_state;

use std::collections::HashMap;
use std::sync::Arc;

use crate::models::Listing;

/// All data for a single fandom, loaded once at startup.
#[derive(Debug, Default)]
pub struct FandomData {
    pub listings: Vec<Listing>,
    // TODO: comments: Vec<Comment>,
}

impl FandomData {
    pub fn listing_count(&self) -> usize {
        self.listings.len()
    }
}

/// Global application state. Loaded once at server startup, never mutated.
#[derive(Debug, Default)]
pub struct AppState {
    /// from "fandom_CLEAN.jsonl".
    pub fandoms: HashMap<String, FandomData>,
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
}

/// Convenience alias used throughout the app.
pub type SharedState = Arc<AppState>;