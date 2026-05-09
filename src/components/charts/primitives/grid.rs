// grid lines
// depends on scale — render grid lines at tick positions

use leptos::prelude::*;
use super::scale::LinearScale;

/// Horizontal grid lines across the chart area, aligned to y-axis ticks.
#[component]
pub fn YGridLines(
    scale: LinearScale,
    /// Left edge of chart area (where lines start).
    x_min: f64,
    /// Right edge of chart area (where lines end).
    x_max: f64,
    #[prop(default = 6)]
    tick_count: usize,
) -> impl IntoView {
    let ticks = scale.ticks(tick_count);
    view! {
        <g class="grid y-grid">
            {ticks.into_iter().map(|t| {
                let y = scale.map(t);
                view! {
                    <line x1=x_min y1=y x2=x_max y2=y class="grid-line"/>
                }
            }).collect_view()}
        </g>
    }
}

/// Vertical grid lines across the chart area, aligned to x-axis ticks.
#[component]
pub fn XGridLines(
    scale: LinearScale,
    y_min: f64,
    y_max: f64,
    #[prop(default = 6)]
    tick_count: usize,
) -> impl IntoView {
    let ticks = scale.ticks(tick_count);
    view! {
        <g class="grid x-grid">
            {ticks.into_iter().map(|t| {
                let x = scale.map(t);
                view! {
                    <line x1=x y1=y_min x2=x y2=y_max class="grid-line"/>
                }
            }).collect_view()}
        </g>
    }
}