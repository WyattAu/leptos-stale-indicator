//! Data freshness computation.

use std::collections::HashMap;
use leptos::prelude::*;

/// The freshness state of fetched data.
#[derive(Clone, Debug, PartialEq)]
pub enum DataFreshness {
    /// All data is fresh (max age < threshold).
    Live,
    /// Some data is stale (max age >= threshold).
    Stale { minutes: u32 },
}

impl DataFreshness {
    /// Display text for the freshness badge.
    pub fn display_text(&self) -> String {
        match self {
            DataFreshness::Live => "LIVE".to_string(),
            DataFreshness::Stale { minutes } => format!("DATA STALE \u{00b7} {}m ago", minutes),
        }
    }

    /// CSS class for the freshness badge.
    pub fn css_class(&self) -> &'static str {
        match self {
            DataFreshness::Live => "status-dot status-dot--live",
            DataFreshness::Stale { .. } => "status-dot status-dot--stale",
        }
    }
}

/// Compute the freshness of data given timestamps and a threshold.
///
/// - `timestamps`: Map of source name -> last fetch time (seconds since epoch)
/// - `threshold`: Maximum age in seconds before data is considered stale (default: 300 = 5 min)
pub fn compute_freshness(timestamps: &HashMap<String, f64>, threshold: f64) -> DataFreshness {
    let now = js_sys::Date::now() / 1000.0;

    let max_age = timestamps
        .values()
        .map(|ts| now - ts)
        .fold(0.0f64, f64::max);

    if max_age >= threshold {
        let minutes = (max_age / 60.0).floor() as u32;
        DataFreshness::Stale { minutes }
    } else {
        DataFreshness::Live
    }
}

/// Create a derived signal that computes data freshness.
pub fn use_data_freshness(
    timestamps: Signal<HashMap<String, f64>>,
    threshold: f64,
) -> Signal<DataFreshness> {
    Memo::new(move |_| {
        compute_freshness(&timestamps.get(), threshold)
    })
    .into()
}
