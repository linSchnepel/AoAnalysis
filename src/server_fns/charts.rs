use leptos::prelude::*;

use crate::components::charts::{
    bar::BarDatum, line::LinePoint, percentile::PercentileRow, pie::PieSlice,
};

/// TODO: move
/// All chart data for the featured fandom homepage.
/// This is the only thing that crosses the wire — never `FandomStats` itself.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HomepageChartDto {
    pub fandom_name: String,
    pub line: Vec<LinePoint>,
    pub pie: Vec<PieSlice>,
    pub bar: Vec<BarDatum>,
    pub percentile: Vec<PercentileRow>,
}

// the fn body compiles for both targets even though it only runs on the server.
#[server]
pub async fn get_homepage_charts() -> Result<HomepageChartDto, ServerFnError> {
    #[cfg(feature = "ssr")] use chrono::{Datelike, TimeZone, Utc};

    use crate::state::{FandomStats, SharedState};

    let state = use_context::<SharedState>()
        .ok_or_else(|| ServerFnError::new("Missing app state"))?;

    let (fandom_name, stats) = state
        .featured()
        .map(|f| (state.featured_fandom.clone(), &f.stats))
        .ok_or_else(|| ServerFnError::new("Featured fandom not found"))?;

    const MONTHS: &[&str] = &[
        "Jan","Feb","Mar","Apr","May","Jun",
        "Jul","Aug","Sep","Oct","Nov","Dec",
    ];

    // --- Line chart: creations by year ---
    let line = stats
        .creations_by_month
        .iter()
        .map(|&(year, month, count)| LinePoint {
            x: (year as f64) * 12.0 + (month as f64 - 1.0),
            y: count as f64,
            label:
                if month == 1 {
                    format!("{} {}", MONTHS[(month - 1) as usize], year)
                } else {
                    MONTHS[(month - 1) as usize].to_string()
                },
        })
        .collect();

    // --- Pie chart: completion breakdown ---
    let (one_shots, complete_multi, incomplete_multi) = stats.completion_breakdown;
    let pie = vec![
        PieSlice { label: "One Shots".into(),           value: one_shots as f64 },
        PieSlice { label: "Complete".into(),             value: complete_multi as f64 },
        PieSlice { label: "Work in Progress".into(),     value: incomplete_multi as f64 },
    ];

    // --- Bar chart: category counts ---
    let mut bar: Vec<BarDatum> = stats
        .category_counts
        .iter()
        .map(|(cat, &count)| BarDatum {
            label: cat.to_string(),
            value: count as f64,
        })
        .collect();
    bar.sort_by(|a, b| b.value.partial_cmp(&a.value).unwrap_or(std::cmp::Ordering::Equal));

    // --- Percentile chart ---
    let pcts = &[20.0, 50.0, 70.0, 90.0, 99.0];
    let percentile = vec![
        make_row("Kudos",      &stats.kudos_sorted,     pcts),
        make_row("Hits",       &stats.hits_sorted,      pcts),
        make_row("Words",      &stats.words_sorted,     pcts),
        make_row("Bookmarks",  &stats.bookmarks_sorted, pcts),
    ];

    Ok(HomepageChartDto { fandom_name, line, pie, bar, percentile })
}

fn make_row(label: &str, sorted: &[u32], pcts: &[f64]) -> PercentileRow {
    use crate::state::FandomStats;
    let v = FandomStats::percentiles(sorted, pcts);
    PercentileRow {
        label: label.into(),
        p20: v[0] as f64,
        p50: v[1] as f64,
        p70: v[2] as f64,
        p90: v[3] as f64,
        p99: v[4] as f64,
    }
}