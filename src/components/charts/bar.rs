use leptos::prelude::*;
use crate::components::charts::primitives::{LinearScale, YAxis, YGridLines};

/// A single bar: a label and its value.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BarDatum {
    pub label: String,
    pub value: f64,
}

/// Static horizontal bar chart for category breakdown.
/// TODO(explore): accept Signal<Vec<BarDatum>>, add hover state.
#[component]
pub fn BarChart(
    data: Vec<BarDatum>,
    #[prop(default = "Category Breakdown".to_string())]
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
        return view! { <svg viewBox=format!("0 0 {} {}", view_box_w, view_box_h)><text x="50%" y="50%" text-anchor="middle">"No data"</text></svg> }.into_any();
    }

    let y_max = data.iter().map(|d| d.value).fold(0.0_f64, f64::max);
    let y_scale = LinearScale::new(0.0, y_max * 1.1, margin_top + plot_h, margin_top);

    let bar_count = data.len();
    let bar_gap = 0.2_f64; // proportion of bar slot used for gap
    let slot_w = plot_w / bar_count as f64;
    let bar_w = slot_w * (1.0 - bar_gap);

    view! {
        <svg
            viewBox=format!("0 0 {} {}", view_box_w, view_box_h)
            preserveAspectRatio="xMidYMid meet"
            class="chart bar-chart"
            role="img"
            aria-label=title.clone()
        >
            <YGridLines scale=y_scale x_min=margin_left x_max=margin_left + plot_w tick_count=5 />
            <YAxis scale=y_scale x=margin_left tick_count=5 />
            {data.into_iter().enumerate().map(|(i, d)| {
                let x = margin_left + i as f64 * slot_w + (slot_w * bar_gap / 2.0);
                let bar_top = y_scale.map(d.value);
                let bar_bottom = y_scale.map(0.0);
                let bar_h = (bar_bottom - bar_top).max(0.0);
                let label_y = margin_top + plot_h + 20.0;
                view! {
                    <g class="bar-group">
                        <rect
                            x=x
                            y=bar_top
                            width=bar_w
                            height=bar_h
                            class="bar"
                        />
                        <text
                            x=x + bar_w / 2.0
                            y=label_y
                            text-anchor="middle"
                            class="bar-label"
                        >
                            {d.label}
                        </text>
                    </g>
                }
            }).collect_view()}
        </svg>
    }.into_any()
}