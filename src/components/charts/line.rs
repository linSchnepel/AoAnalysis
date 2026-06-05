use leptos::prelude::*;
use crate::components::charts::primitives::{LinearScale, YGridLines};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LinePoint {
    pub x: f64,
    pub y: f64,
    pub label: String,
}

#[component]
pub fn LineChart(
    data: Vec<LinePoint>,

    #[prop(default = "Work Creation History".to_string())]
    title: String,

    #[prop(default = "preview graph".to_string())]
    subtitle: String,

    #[prop(default = "amount of works created".to_string())]
    y_title: String,

    #[prop(default = "months".to_string())]
    x_title: String,
    /// If true, only label Jan/Apr/Jul/Oct. Minor ticks for other months.
    #[prop(default = false)]
    quarterly: bool,
) -> impl IntoView {
    let view_box_w = 900.0_f64;
    let view_box_h = 500.0_f64;
    let margin_top    = 60.0_f64;  // room for title + subtitle
    let margin_right  = 60.0_f64;
    let margin_bottom = 80.0_f64;  // room for rotated labels + x title
    let margin_left   = 80.0_f64;  // room for y axis title + labels
    let plot_w = view_box_w - margin_left - margin_right;
    let plot_h = view_box_h - margin_top  - margin_bottom;

    if data.is_empty() {
        return view! {
            <svg viewBox=format!("0 0 {} {}", view_box_w, view_box_h)>
                <text x="50%" y="50%" text-anchor="middle">"No data"</text>
            </svg>
        }.into_any();
    }

    let x_min = data.iter().map(|p| p.x).fold(f64::INFINITY,     f64::min);
    let x_max = data.iter().map(|p| p.x).fold(f64::NEG_INFINITY, f64::max);
    let y_max = data.iter().map(|p| p.y).fold(0.0_f64,           f64::max);
    let y_min = 0.0_f64;

    let x_scale = LinearScale::new(x_min, x_max, margin_left, margin_left + plot_w);
    let y_scale = LinearScale::new(y_min, y_max * 1.1, margin_top + plot_h, margin_top);

    // Replace data.iter() if want just the highest point
    //let max_point = data.iter().max_by(|a, b| a.y.partial_cmp(&b.y).unwrap());

    let points_str: String = data.iter()
        .map(|p| format!("{},{}", x_scale.map(p.x), y_scale.map(p.y)))
        .collect::<Vec<_>>()
        .join(" ");

    // Y axis: major ticks + 3 minor ticks between each
    let major_ticks = y_scale.ticks(6);
    let minor_tick_count = 3;

    // X axis: which points get labels vs minor ticks
    // quarterly months: Jan=1, Apr=4, Jul=7, Oct=10
    let quarterly_months = [1u32, 4, 7, 10];

    let axis_y = margin_top + plot_h;

    let aria_label = title.clone();

    let median_y = {
        let mut ys: Vec<f64> = data.iter().map(|p| p.y).collect();
        ys.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let mid = ys.len() / 2;
        if ys.len() % 2 == 0 {
            (ys[mid - 1] + ys[mid]) / 2.0
        } else {
            ys[mid]
        }
    };

    view! {
        <svg
            viewBox=format!("0 0 {} {}", view_box_w, view_box_h)
            preserveAspectRatio="xMidYMid meet"
            class="chart line-chart"
            role="img"
            aria-label=aria_label
        >
            // --- Titles ---
            <text
                x=0
                y=15
                text-anchor="left"
                class="chart-title" // TODO: header 4
            >{title}</text>
            <text
                x=0
                y=35.0
                text-anchor="left"
                class="chart-subtitle"
            >{subtitle}</text>

            // --- Y axis title (rotated) ---
            <text
                x=0.0
                y=0.0
                text-anchor="middle"
                class="axis-title"
                transform=format!(
                    "translate({}, {}) rotate(-90)",
                    16.0,
                    margin_top + plot_h / 2.0
                )
            >{y_title}</text>

            // --- X axis title ---
            <text
                x=margin_left + plot_w / 2.0
                y=view_box_h - 8.0
                text-anchor="middle"
                class="axis-title"
            >{x_title}</text>

            // --- Grid lines (major y only) ---
            <YGridLines
                scale=y_scale
                x_min=margin_left
                x_max=margin_left + plot_w
                tick_count=6
            />

            // --- Y axis with major + minor ticks ---
            <line
                x1=margin_left y1=margin_top
                x2=margin_left y2=axis_y
                class="axis-line"
            />
            {major_ticks.iter().enumerate().map(|(i, &t)| {
                let y = y_scale.map(t);
                let label = format!("{:.0}", t);

                // Minor ticks between this major tick and the next
                let minor_marks = if i + 1 < major_ticks.len() {
                    let next_t = major_ticks[i + 1];
                    let step = (next_t - t) / (minor_tick_count + 1) as f64;
                    (1..=minor_tick_count).map(|k| {
                        let my = y_scale.map(t + step * k as f64);
                        view! {
                            <line
                                x1=margin_left - 3.0 y1=my
                                x2=margin_left         y2=my
                                class="tick-mark-minor"
                            />
                        }
                    }).collect_view()
                } else {
                    vec![].into_iter().collect_view()
                };

                view! {
                    <g class="tick">
                        <line
                            x1=margin_left - 6.0 y1=y
                            x2=margin_left         y2=y
                            class="tick-mark"
                        />
                        <text
                            x=margin_left - 10.0
                            y=y
                            text-anchor="end"
                            dominant-baseline="middle"
                            class="tick-label"
                        >{label}</text>
                        {minor_marks}
                    </g>
                }
            }).collect_view()}

            // --- X axis baseline ---
            <line
                x1=margin_left y1=axis_y
                x2=margin_left + plot_w y2=axis_y
                class="axis-line"
            />

            // --- X ticks and labels ---
            {data.iter().map(|p| {
                // Recover month from x = year*12 + (month-1)
                let month_idx = (p.x % 12.0).round() as u32 + 1;
                let x = x_scale.map(p.x);

                // Determine if this point gets a label or just a minor tick
                let is_labeled = if quarterly {
                    quarterly_months.contains(&month_idx)
                } else {
                    true
                };

                if is_labeled {
                    view! {
                        <g class="tick">
                            <line
                                x1=x y1=axis_y
                                x2=x y2=axis_y + 6.0
                                class="tick-mark"
                            />
                            <text
                                x=x
                                y=axis_y + 10.0
                                text-anchor="end"
                                class="tick-label"
                                transform=format!("rotate(-75, {}, {})", x, axis_y + 10.0)
                            >{p.label.clone()}</text>
                        </g>
                    }.into_any()
                } else {
                    view! {
                        <line
                            x1=x y1=axis_y
                            x2=x y2=axis_y + 3.0
                            class="tick-mark-minor"
                        />
                    }.into_any()
                }
            }).collect_view()}

            // --- Vertical tick marks at each data point ---
            {data.iter().map(|p| {
                let x = x_scale.map(p.x);
                let y = y_scale.map(p.y);
                view! {
                    <line
                        x1=x y1=y - 10.0
                        x2=x y2=y + 10.0
                        class="point-tick"
                    />
                }
            }).collect_view()}

            // --- Median fill (transparent green below median line) ---
            <rect
                x=margin_left
                y=y_scale.map(median_y)
                width=plot_w
                height=y_scale.map(0.0) - y_scale.map(median_y)
                class="median-fill"
            />

            // --- Median line ---
            <line
                x1=margin_left
                y1=y_scale.map(median_y)
                x2=margin_left + plot_w
                y2=y_scale.map(median_y)
                class="median-line"
            />

            // --- Median label ---
            <text
                x=margin_left + plot_w + 4.0
                y=y_scale.map(median_y)
                dominant-baseline="middle"
                class="median-label"
            >{format!("median {:.0}", median_y)}</text>

            // --- Line path ---
            <polyline
                points=points_str
                fill="none"
                class="line-path"
            />

            // --- High/low labels ---
            {data.iter().map(|p| {
                let x = x_scale.map(p.x);
                let y = y_scale.map(p.y);
                view! {
                    <text
                        x=x y=y - 30.0
                        text-anchor="middle"
                        class="point-label point-label-high"
                    >{format!("{:.0}", p.y)}</text>
                }
            }).collect_view()}

        </svg>
    }.into_any()
}