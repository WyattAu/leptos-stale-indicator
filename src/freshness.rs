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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn live_display_text() {
        assert_eq!(DataFreshness::Live.display_text(), "LIVE");
    }

    #[test]
    fn stale_display_text() {
        let stale = DataFreshness::Stale { minutes: 5 };
        assert_eq!(stale.display_text(), "DATA STALE \u{00b7} 5m ago");
    }

    #[test]
    fn stale_zero_minutes() {
        let stale = DataFreshness::Stale { minutes: 0 };
        assert_eq!(stale.display_text(), "DATA STALE \u{00b7} 0m ago");
    }

    #[test]
    fn stale_large_minutes() {
        let stale = DataFreshness::Stale { minutes: 1440 };
        assert_eq!(stale.display_text(), "DATA STALE \u{00b7} 1440m ago");
    }

    #[test]
    fn live_css_class() {
        assert_eq!(
            DataFreshness::Live.css_class(),
            "status-dot status-dot--live"
        );
    }

    #[test]
    fn stale_css_class() {
        let stale = DataFreshness::Stale { minutes: 10 };
        assert_eq!(
            stale.css_class(),
            "status-dot status-dot--stale"
        );
    }

    #[test]
    fn live_eq_live() {
        assert_eq!(DataFreshness::Live, DataFreshness::Live);
    }

    #[test]
    fn stale_eq_stale() {
        assert_eq!(
            DataFreshness::Stale { minutes: 5 },
            DataFreshness::Stale { minutes: 5 }
        );
    }

    #[test]
    fn stale_ne_live() {
        assert_ne!(
            DataFreshness::Stale { minutes: 5 },
            DataFreshness::Live
        );
    }

    #[test]
    fn stale_different_minutes_ne() {
        assert_ne!(
            DataFreshness::Stale { minutes: 5 },
            DataFreshness::Stale { minutes: 10 }
        );
    }

}
