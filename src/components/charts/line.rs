use leptos::prelude::*;
use crate::components::charts::primitives::{LinearScale, YAxis, YGridLines};

/// A single (x, y) point on the line chart.
/// x is the year as f64 for scale mapping.
/// label is pre-formatted server-side — no date logic on the client.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LinePoint {
    pub x: f64,   // pixel-ready: just the year as f64 for scale mapping
    pub y: f64,
    pub label: String, // e.g. "2019"
}

/// Static line chart for works created over time.
/// Data is pre-bucketed by year on the server.
/// TODO: accept Signal<Vec<LinePoint>> for reactive updates.
#[component]
pub fn LineChart(
    data: Vec<LinePoint>,
    #[prop(default = "Works Over Time".to_string())]
    title: String,
) -> impl IntoView {
    let view_box_w = 800.0_f64;
    let view_box_h = 400.0_f64;
    let margin_top = 20.0_f64;
    let margin_right = 20.0_f64;
    let margin_bottom = 50.0_f64;
    let margin_left = 60.0_f64;
    let plot_w = view_box_w - margin_left - margin_right;
    let plot_h = view_box_h - margin_top - margin_bottom;

    if data.is_empty() {
        return view! {
            <svg viewBox=format!("0 0 {} {}", view_box_w, view_box_h)>
                <text x="50%" y="50%" text-anchor="middle">"No data"</text>
            </svg>
        }.into_any();
    }

    let x_min = data.iter().map(|p| p.x).fold(f64::INFINITY, f64::min);
    let x_max = data.iter().map(|p| p.x).fold(f64::NEG_INFINITY, f64::max);
    let y_max = data.iter().map(|p| p.y).fold(0.0_f64, f64::max);

    let x_scale = LinearScale::new(x_min, x_max, margin_left, margin_left + plot_w);
    let y_scale = LinearScale::new(0.0, y_max * 1.1, margin_top + plot_h, margin_top);

    let points_str: String = data.iter()
        .map(|p| format!("{},{}", x_scale.map(p.x), y_scale.map(p.y)))
        .collect::<Vec<_>>()
        .join(" ");

    view! {
        <svg
            viewBox=format!("0 0 {} {}", view_box_w, view_box_h)
            preserveAspectRatio="xMidYMid meet"
            class="chart line-chart"
            role="img"
            aria-label=title
        >
            <YGridLines scale=y_scale x_min=margin_left x_max=margin_left + plot_w tick_count=5 />
            <YAxis scale=y_scale x=margin_left tick_count=5 />

            // X axis baseline
            <line
                x1=margin_left
                y1=margin_top + plot_h
                x2=margin_left + plot_w
                y2=margin_top + plot_h
                class="axis-line"
            />

            // X tick labels — pre-formatted server-side, no date logic here
            {data.iter().map(|p| {
                let x = x_scale.map(p.x);
                let y = margin_top + plot_h;
                view! {
                    <g class="tick">
                        <line x1=x y1=y x2=x y2=y + 6.0 class="tick-mark"/>
                        <text x=x y=y + 18.0 text-anchor="middle" class="tick-label">
                            {p.label.clone()}
                        </text>
                    </g>
                }
            }).collect_view()}

            <polyline points=points_str fill="none" class="line-path"/>
        </svg>
    }.into_any()
}