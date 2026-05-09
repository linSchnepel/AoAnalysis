// linear/time scale: domain → pixel mapping
// most critical. maps a data domain [min, max] to a pixel range [0, width].
// Everything else depends on it. Needs to handle both f64 linear and i64 timestamp (Unix) scales.

// the foundation everything else depends on
/**
 The primitives layer needs to:

Be purely computational on the server-irrelevant parts (scale, tick generation)
Produce SVG via Leptos view! macros
Work with fixed viewBox now, responsive later
Be generic enough for line, bar, pie, and percentile charts to all compose from them
 */

/// Maps a data domain [min, max] to a pixel range [0, width].
/// Two variants: linear (f64) and timestamp (unix ms → f64).
/// Clone + Copy so chart components can pass it around freely.

#[derive(Debug, Clone, Copy)]
pub struct LinearScale {
    pub domain_min: f64,
    pub domain_max: f64,
    pub range_min: f64,
    pub range_max: f64,
}

impl LinearScale {
    pub fn new(domain_min: f64, domain_max: f64, range_min: f64, range_max: f64) -> Self {
        Self { domain_min, domain_max, range_min, range_max }
    }

    /// Map a domain value to a pixel position.
    pub fn map(&self, value: f64) -> f64 {
        if (self.domain_max - self.domain_min).abs() < f64::EPSILON {
            return self.range_min;
        }
        let t = (value - self.domain_min) / (self.domain_max - self.domain_min);
        self.range_min + t * (self.range_max - self.range_min)
    }

    /// Generate n "nice" tick values across the domain.
    pub fn ticks(&self, n: usize) -> Vec<f64> {
        if n == 0 { return vec![]; }
        let step = nice_step(self.domain_min, self.domain_max, n);
        let start = (self.domain_min / step).ceil() * step;
        let mut ticks = vec![];
        let mut v = start;
        while v <= self.domain_max + f64::EPSILON {
            ticks.push(v);
            v += step;
        }
        ticks
    }
}

/// Round a raw step to a "nice" human-readable value (1, 2, 5, 10, ...).
fn nice_step(min: f64, max: f64, n: usize) -> f64 {
    let raw = (max - min) / n as f64;
    let mag = raw.log10().floor();
    let pow = 10f64.powf(mag);
    let frac = raw / pow;
    let nice = if frac < 1.5 { 1.0 }
        else if frac < 3.0 { 2.0 }
        else if frac < 7.0 { 5.0 }
        else { 10.0 };
    nice * pow
}

/// A scale specifically for Unix millisecond timestamps → pixel position.
/// Internally delegates to LinearScale; tick generation is calendar-aware.

/// The scale computation itself happens server-side when building chart DTOs;
/// only the pixel-mapped f64 values cross the wire to components.
// This can't be "ssr" because the mod.rs needs to show it
#[derive(Debug, Clone, Copy)]
pub struct TimeScale {
    inner: LinearScale,
}

#[cfg(feature = "ssr")]
impl TimeScale {
    pub fn new(min_unix_ms: i64, max_unix_ms: i64, range_min: f64, range_max: f64) -> Self {
        Self {
            inner: LinearScale::new(
                min_unix_ms as f64,
                max_unix_ms as f64,
                range_min,
                range_max,
            ),
        }
    }

    pub fn map(&self, unix_ms: i64) -> f64 {
        self.inner.map(unix_ms as f64)
    }

    /// Generate year-boundary ticks as unix ms values.
    /// TODO(explore): finer granularity (month/day) based on zoom level.
    pub fn year_ticks(&self) -> Vec<i64> {
        // chrono needs the clock or wasmbind feature consideration
        // But since TimeScale is SSR-only, it's fine as-is.
        use chrono::{Datelike, TimeZone, Utc};
        let start = Utc.timestamp_millis_opt(self.inner.domain_min as i64)
            .single()
            .map(|dt| dt.year())
            .unwrap_or(2000);
        let end = Utc.timestamp_millis_opt(self.inner.domain_max as i64)
            .single()
            .map(|dt| dt.year())
            .unwrap_or(2030);
        (start..=end)
            .filter_map(|y| {
                Utc.with_ymd_and_hms(y, 1, 1, 0, 0, 0)
                    .single()
                    .map(|dt| dt.timestamp_millis())
            })
            .collect()
    }
}