use serde::Deserialize;

use super::common::{zip_history, HistoryEntry};

/// Raw shape of a record in `_HISTORIES.jsonl`.
/// Private. Used only for deserialization.
#[derive(Debug, Deserialize)]
struct RawHistory {
    id: String,
    /// Parallel array — ISO 8601 date labels, e.g. "2026-04-11".
    history: Vec<String>,
    /// Parallel array — Unix millisecond timestamps.
    #[serde(rename = "historyUNIX")]
    history_unix: Vec<i64>,
}

/// Parsed history for a multi-chapter work.
///
/// This type is a temporary deserialization target. After all `_HISTORIES.jsonl`
/// records are loaded, each `History` is merged into its corresponding `Listing`
/// by matching on `id`, and the `Vec<History>` is discarded.
#[derive(Debug)]
pub struct History {
    pub id: String,
    pub entries: Vec<HistoryEntry>,
}

impl<'de> serde::Deserialize<'de> for History {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let raw = RawHistory::deserialize(deserializer)?;

        let entries =
            zip_history(raw.history, raw.history_unix).map_err(serde::de::Error::custom)?;

        Ok(History {
            id: raw.id,
            entries,
        })
    }
}