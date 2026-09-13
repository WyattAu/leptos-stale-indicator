//! # leptos-stale-indicator
//!
//! Data freshness indicator for Leptos.
//!
//! Shows "LIVE" or "DATA STALE · Xm ago" based on fetch timestamps.
//! Polls every 10 seconds and renders a status badge.
//!
//! ## Usage
//!
//! ```rust,no_run
//! use leptos::prelude::*;
//! use leptos_stale_indicator::StaleIndicator;
//!
//! #[component]
//! fn Dashboard(timestamps: Signal<std::collections::HashMap<String, f64>>) -> impl IntoView {
//!     view! {
//!         <StaleIndicator timestamps=timestamps/>
//!     }
//! }
//! ```

#![deny(missing_docs)]

mod freshness;
mod indicator;

pub use freshness::{use_data_freshness, DataFreshness};
pub use indicator::StaleIndicator;
