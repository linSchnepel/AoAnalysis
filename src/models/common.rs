/// A Unix timestamp in milliseconds (as produced by JavaScript's Date.now()).
///
/// Stored as i64 to handle the full range. All timestamps in the source data
/// are millisecond-precision, so division by 1000 is required before comparing
/// to second-precision values.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UnixMillis(pub i64);

impl UnixMillis {
    pub fn as_millis(&self) -> i64 {
        self.0
    }

    pub fn as_seconds(&self) -> i64 {
        self.0 / 1000
    }
}

/// A single entry in a read history pairing a human-readable date label
/// with its Unix millisecond timestamp.
///
/// This type is the result of zipping the parallel `history` and `historyUNIX`
/// arrays from the source JSON. After construction, the two arrays no longer
/// exist separately; the pairing is structurally enforced.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HistoryEntry {
    /// Human-readable label.
    /// - In `_CLEAN.jsonl`: "04 Apr 2012" format
    /// - In `_HISTORIES.jsonl`: "2026-04-11" (ISO 8601) format
    /// Both are stored as-is after parsing; use `unix` for any date arithmetic.
    pub label: String,
    pub unix: UnixMillis,
}

/// Zip two parallel arrays (labels + unix timestamps) into a Vec<HistoryEntry>.
/// Returns an error string if the arrays have different lengths.
pub fn zip_history(
    labels: Vec<String>,
    unix_values: Vec<i64>,
) -> Result<Vec<HistoryEntry>, String> {
    if labels.len() != unix_values.len() {
        return Err(format!(
            "history/historyUNIX length mismatch: {} labels vs {} timestamps",
            labels.len(),
            unix_values.len()
        ));
    }
    Ok(labels
        .into_iter()
        .zip(unix_values)
        .map(|(label, unix)| HistoryEntry {
            label,
            unix: UnixMillis(unix),
        })
        .collect())
}