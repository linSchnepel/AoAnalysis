use leptos::prelude::*;
use crate::components::charts::primitives::LinearScale;

/// Pre-computed percentile values for one metric.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PercentileRow {
    pub label: String,
    pub p20: f64,
    pub p50: f64,
    pub p70: f64,
    pub p90: f64,
    pub p99: f64,
}

/// Horizontal percentile strip chart.
/// One row per metric (kudos, hits, words, bookmarks).
/// Shows distribution shape via percentile markers on a shared scale.
/// TODO(explore): hover to show exact values, log scale toggle.
#[component]
pub fn PercentileChart(
    data: Vec<PercentileRow>,
    #[prop(default = "Distribution".to_string())]
    title: String,
) -> impl IntoView {
    let view_box_w = 800.0_f64;
    let row_h = 60.0_f64;
    let margin_left = 100.0_f64;
    let margin_right = 20.0_f64;
    let margin_top = 20.0_f64;
    let margin_bottom = 30.0_f64;
    let plot_w = view_box_w - margin_left - margin_right;
    let view_box_h = margin_top + margin_bottom + row_h * data.len() as f64;

    if data.is_empty() {
        return view! { <svg viewBox=format!("0 0 {} {}", view_box_w, 200.0)><text x="50%" y="50%" text-anchor="middle">"No data"</text></svg> }.into_any();
    }

    // Shared scale across all rows: 0 to global p99 max
    let global_max = data.iter()
        .map(|r| r.p99)
        .fold(0.0_f64, f64::max);
    let scale = LinearScale::new(0.0, global_max * 1.05, margin_left, margin_left + plot_w);

    view! {
        <svg
            viewBox=format!("0 0 {} {}", view_box_w, view_box_h)
            preserveAspectRatio="xMidYMid meet"
            class="chart percentile-chart"
            role="img"
            aria-label=title.clone()
        >
            {data.into_iter().enumerate().map(|(i, row)| {
                let y_center = margin_top + row_h * i as f64 + row_h / 2.0;
                let track_y1 = y_center - 2.0;
                let track_y2 = y_center + 2.0;

                let x_p20 = scale.map(row.p20);
                let x_p50 = scale.map(row.p50);
                let x_p70 = scale.map(row.p70);
                let x_p90 = scale.map(row.p90);
                let x_p99 = scale.map(row.p99);

                view! {
                    <g class="percentile-row">
                        // Row label
                        <text
                            x=margin_left - 10.0
                            y=y_center
                            text-anchor="end"
                            dominant-baseline="middle"
                            class="row-label"
                        >
                            {row.label}
                        </text>
                        // Track line from 0 to p99
                        <rect
                            x=scale.map(0.0)
                            y=track_y1
                            width=x_p99 - scale.map(0.0)
                            height=track_y2 - track_y1
                            class="percentile-track"
                        />
                        // IQR-style filled band p20→p90
                        <rect
                            x=x_p20
                            y=y_center - 8.0
                            width=x_p90 - x_p20
                            height=16.0
                            class="percentile-band"
                        />
                        // Median marker at p50
                        <line
                            x1=x_p50 y1=y_center - 12.0
                            x2=x_p50 y2=y_center + 12.0
                            class="percentile-median"
                        />
                        // p70 marker
                        <line
                            x1=x_p70 y1=y_center - 8.0
                            x2=x_p70 y2=y_center + 8.0
                            class="percentile-p70"
                        />
                        // p99 dot
                        <circle cx=x_p99 cy=y_center r=4.0 class="percentile-outlier" />
                    </g>
                }
            }).collect_view()}
        </svg>
    }.into_any()
}