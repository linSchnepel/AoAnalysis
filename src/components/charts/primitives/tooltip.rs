// TODO: hover tooltip (Explore)
// stub now, implement for Explore

// TODO(explore): Interactive tooltip component.
//
// Will show x/y values on hover for line/bar charts, segment info for pie,
// and percentile values for the percentile strip chart.
//
// Design notes for when this is implemented:
// - Takes a `Signal<Option<TooltipData>>` driven by pointer events on the chart
// - Rendered as HTML overlay (not SVG) so it can overflow chart bounds
// - Position clamped to viewport edges
// - `TooltipData` will be an enum covering each chart type's needs