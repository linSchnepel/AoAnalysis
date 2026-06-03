use serde::{Deserialize, Deserializer, Serialize};

use super::common::{zip_history, HistoryEntry, UnixMillis};

// ---------------------------------------------------------------------------
// Enums
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Rating {
    #[serde(rename = "General Audiences")]
    GeneralAudiences,
    #[serde(rename = "Teen And Up Audiences")]
    TeenAndUp,
    #[serde(rename = "Mature")]
    Mature,
    #[serde(rename = "Explicit")]
    Explicit,
    #[serde(rename = "Not Rated")]
    NotRated,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum Warning {
    #[serde(rename = "No Archive Warnings Apply")]
    NoWarnings,
    #[serde(rename = "Creator Chose Not To Use Archive Warnings")]
    CreatorChoseNotToWarn,
    #[serde(rename = "Graphic Depictions Of Violence")]
    GraphicViolence,
    #[serde(rename = "Major Character Death")]
    MajorCharacterDeath,
    #[serde(rename = "Underage Sex")]
    UnderlageSex,
    #[serde(rename = "Rape/Non-Con")]
    RapeNonCon,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum Category {
    #[serde(rename = "Gen")]
    Gen,
    #[serde(rename = "F/M")]
    FM,
    #[serde(rename = "M/M")]
    MM,
    #[serde(rename = "F/F")]
    FF,
    #[serde(rename = "Multi")]
    Multi,
    #[serde(rename = "Other")]
    Other,
    #[serde(rename = "No category")]
    NoCategory,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Completion {
    #[serde(rename = "Complete Work")]
    CompleteWork,
    #[serde(rename = "Work in Progress")]
    WorkInProgress,
}

// ---------------------------------------------------------------------------
// Stats
// ---------------------------------------------------------------------------

/// The "Chapters" field in stats arrives as a string like "3/5" or "3/?".
/// We parse it into a structured type so callers never need to split strings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChapterCount {
    pub published: u32,
    /// None means the total is unknown ("?").
    pub total: Option<u32>,
}

impl ChapterCount {
    fn parse(s: &str) -> Option<Self> {
        let mut parts = s.splitn(2, '/');
        let published = parts.next()?.trim().parse::<u32>().ok()?;
        let total = match parts.next()?.trim() {
            "?" => None,
            n => Some(n.parse::<u32>().ok()?),
        };
        Some(ChapterCount { published, total })
    }
}

/// Stats as they appear in the raw JSON — all values may be null.
/// This is a private intermediate type used only during deserialization.
#[derive(Debug, Deserialize)]
struct RawStats {
    #[serde(rename = "Language")]
    language: Option<String>,
    #[serde(rename = "Words")]
    words: Option<u32>,
    #[serde(rename = "Chapters")]
    chapters: Option<String>,
    #[serde(rename = "Kudos")]
    kudos: Option<u32>,
    #[serde(rename = "Bookmarks")]
    bookmarks: Option<u32>,
    #[serde(rename = "Hits")]
    hits: Option<u32>,
    #[serde(rename = "Comments")]
    comments: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stats {
    pub language: Option<String>,
    pub words: Option<u32>,
    pub chapters: Option<ChapterCount>,
    pub kudos: Option<u32>,
    pub bookmarks: Option<u32>,
    pub hits: Option<u32>,
    pub comments: Option<u32>,
}

impl From<RawStats> for Stats {
    fn from(raw: RawStats) -> Self {
        Stats {
            language: raw.language,
            words: raw.words,
            chapters: raw.chapters.as_deref().and_then(ChapterCount::parse),
            kudos: raw.kudos,
            bookmarks: raw.bookmarks,
            hits: raw.hits,
            comments: raw.comments,
        }
    }
}

// ---------------------------------------------------------------------------
// Listing
// ---------------------------------------------------------------------------

/// Raw shape of a listing as it appears in `_CLEAN.jsonl`.
/// Private. Used only for deserialization; callers work with `Listing`.
#[derive(Debug, Deserialize)]
struct RawListing {
    id: String,
    title: String,
    rating: Rating,
    warnings: Vec<Warning>,
    category: Vec<Category>,
    tags: Vec<String>,
    summary: Option<String>,
    series: Vec<String>,
    fandoms: Vec<String>,
    archetypes: Vec<String>,
    completion: Completion,
    authors: Vec<String>,
    restricted: bool,
    stats: RawStats,
    update: String,
    #[serde(rename = "updateUNIX")]
    update_unix: i64,
    /// Parallel array labels. Null for multi-chapter works.
    history: Option<Vec<String>>,
    /// Parallel array timestamps. Null for multi-chapter works.
    #[serde(rename = "historyUNIX")]
    history_unix: Option<Vec<i64>>,
}

/// A fully parsed AO3 listing (work metadata).
///
/// `history` is `None` for multi-chapter works; their read history lives in
/// the corresponding `_HISTORIES.jsonl` file and is merged in at load time.
#[derive(Debug, Clone, Serialize)]
pub struct Listing {
    pub id: String,
    pub title: String,
    pub rating: Rating,
    pub warnings: Vec<Warning>,
    pub category: Vec<Category>,
    pub tags: Vec<String>,
    pub summary: Option<String>,
    pub series: Vec<String>,
    pub fandoms: Vec<String>,
    pub archetypes: Vec<String>,
    pub completion: Completion,
    pub authors: Vec<String>,
    pub restricted: bool,
    pub stats: Stats,
    /// Human-readable update date, e.g. "04 Apr 2012".
    pub update: String,
    pub update_unix: UnixMillis,
    /// Read history entries. `None` if this is a multi-chapter work whose
    /// history has not yet been merged in from `_HISTORIES.jsonl`.
    pub history: Option<Vec<HistoryEntry>>,
}

impl<'de> Deserialize<'de> for Listing {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let raw = RawListing::deserialize(deserializer)?;

        // Zip the parallel history arrays, if present.
        let history = match (raw.history, raw.history_unix) {
            (Some(labels), Some(unix_vals)) => {
                let entries = zip_history(labels, unix_vals).map_err(serde::de::Error::custom)?;
                Some(entries)
            }
            // Both null → multi-chapter work, history comes from _HISTORIES
            (None, None) => None,
            // Mismatched nullity is malformed data
            _ => {
                return Err(serde::de::Error::custom(
                    "history and historyUNIX must both be present or both be null",
                ))
            }
        };

        Ok(Listing {
            id: raw.id,
            title: raw.title,
            rating: raw.rating,
            warnings: raw.warnings,
            category: raw.category,
            tags: raw.tags,
            summary: raw.summary,
            series: raw.series,
            fandoms: raw.fandoms,
            archetypes: raw.archetypes,
            completion: raw.completion,
            authors: raw.authors,
            restricted: raw.restricted,
            stats: raw.stats.into(),
            update: raw.update,
            update_unix: UnixMillis(raw.update_unix),
            history,
        })
    }
}