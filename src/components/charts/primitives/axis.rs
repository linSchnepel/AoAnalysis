// axis lines, tick marks, tick labels
// depends on scale — given a scale, generate N nice ticks, render labels and marks

use leptos::prelude::*;
use super::scale::LinearScale;

/// A rendered horizontal (bottom) axis.
#[component]
pub fn XAxis(
    scale: LinearScale,
    /// Pixel y-position of the axis line.
    y: f64,
    /// Number of major ticks.
    #[prop(default = 6)]
    tick_count: usize,
    /// Format a tick value to a label string.
    #[prop(default = |v: f64| format!("{:.0}", v))]
    format: fn(f64) -> String,
) -> impl IntoView {
    let ticks = scale.ticks(tick_count);
    view! {
        <g class="x-axis">
            // Axis baseline
            <line
                x1=scale.range_min
                y1=y
                x2=scale.range_max
                y2=y
                class="axis-line"
            />
            {ticks.into_iter().map(|t| {
                let x = scale.map(t);
                let label = format(t);
                view! {
                    <g class="tick">
                        // Major tick mark
                        <line x1=x y1=y x2=x y2=y + 6.0 class="tick-mark"/>
                        <text x=x y=y + 18.0 class="tick-label" text-anchor="middle">
                            {label}
                        </text>
                    </g>
                }
            }).collect_view()}
        </g>
    }
}

/// A rendered vertical (left) axis.
#[component]
pub fn YAxis(
    scale: LinearScale,
    /// Pixel x-position of the axis line.
    x: f64,
    #[prop(default = 6)]
    tick_count: usize,
    #[prop(default = |v: f64| format!("{:.0}", v))]
    format: fn(f64) -> String,
) -> impl IntoView {
    let ticks = scale.ticks(tick_count);
    view! {
        <g class="y-axis">
            <line
                x1=x
                y1=scale.range_min
                x2=x
                y2=scale.range_max
                class="axis-line"
            />
            {ticks.into_iter().map(|t| {
                let y = scale.map(t);
                let label = format(t);
                view! {
                    <g class="tick">
                        <line x1=x - 6.0 y1=y x2=x y2=y class="tick-mark"/>
                        <text x=x - 10.0 y=y class="tick-label" text-anchor="end" dominant-baseline="middle">
                            {label}
                        </text>
                    </g>
                }
            }).collect_view()}
        </g>
    }
}