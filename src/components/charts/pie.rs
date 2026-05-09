use leptos::prelude::*;

/// A single pie segment.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PieSlice {
    pub label: String,
    pub value: f64,
}

/// Compute SVG arc path for a pie segment.
/// Angles in radians, measured clockwise from 12 o'clock.
fn arc_path(cx: f64, cy: f64, r: f64, start_angle: f64, end_angle: f64) -> String {
    let x1 = cx + r * start_angle.sin();
    let y1 = cy - r * start_angle.cos();
    let x2 = cx + r * end_angle.sin();
    let y2 = cy - r * end_angle.cos();
    let large_arc = if end_angle - start_angle > std::f64::consts::PI { 1 } else { 0 };
    format!(
        "M {cx} {cy} L {x1} {y1} A {r} {r} 0 {large_arc} 1 {x2} {y2} Z",
    )
}

/// Static pie chart for completion breakdown.
/// TODO(explore): hover highlight, click-to-filter.
#[component]
pub fn PieChart(
    data: Vec<PieSlice>,
    #[prop(default = "Completion Breakdown".to_string())]
    title: String,
) -> impl IntoView {
    let view_box_w = 400.0_f64;
    let view_box_h = 400.0_f64;
    let cx = view_box_w / 2.0;
    let cy = view_box_h / 2.0;
    let r = 150.0_f64;

    if data.is_empty() {
        return view! { <svg viewBox=format!("0 0 {} {}", view_box_w, view_box_h)><text x="50%" y="50%" text-anchor="middle">"No data"</text></svg> }.into_any();
    }

    let total: f64 = data.iter().map(|s| s.value).sum();

    let mut angle = 0.0_f64;
    let slices: Vec<_> = data.into_iter().enumerate().map(|(i, slice)| {
        let sweep = (slice.value / total) * 2.0 * std::f64::consts::PI;
        let start = angle;
        let end = angle + sweep;
        angle = end;

        // Label position: midpoint of arc, pushed out past radius
        let mid_angle = start + sweep / 2.0;
        let label_r = r + 20.0;
        let label_x = cx + label_r * mid_angle.sin();
        let label_y = cy - label_r * mid_angle.cos();

        let path = arc_path(cx, cy, r, start, end);
        (i, path, label_x, label_y, slice.label)
    }).collect();

    view! {
        <svg
            viewBox=format!("0 0 {} {}", view_box_w, view_box_h)
            preserveAspectRatio="xMidYMid meet"
            class="chart pie-chart"
            role="img"
            aria-label=title.clone()
        >
            {slices.into_iter().map(|(i, path, lx, ly, label)| view! {
                <g class="pie-slice">
                    <path d=path class=format!("slice slice-{}", i) />
                    <text x=lx y=ly text-anchor="middle" class="slice-label">
                        {label}
                    </text>
                </g>
            }).collect_view()}
        </svg>
    }.into_any()
}