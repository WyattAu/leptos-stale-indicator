//! StaleIndicator Leptos component.

use leptos::prelude::*;
use std::collections::HashMap;

use crate::freshness::{compute_freshness, DataFreshness};

/// A data freshness indicator component.
///
/// Polls timestamps every 10 seconds and renders a status badge.
///
/// # Props
///
/// - `timestamps`: Signal<HashMap<String, f64>> — map of source name -> last fetch time
/// - `threshold`: f64 — max age in seconds before stale (default: 300.0 = 5 min)
/// - `class`: Option<String> — additional CSS class
#[component]
pub fn StaleIndicator(
    timestamps: Signal<HashMap<String, f64>>,
    #[prop(optional)] threshold: Option<f64>,
    #[prop(optional)] class: Option<String>,
) -> impl IntoView {
    let threshold = threshold.unwrap_or(300.0);

    // Poll every 10 seconds
    let (freshness, set_freshness) = signal(DataFreshness::Live);

    #[cfg(feature = "hydrate")]
    {
        // Initial computation
        let ts = timestamps.get();
        set_freshness.set(compute_freshness(&ts, threshold));

        // Periodic refresh
        leptos::task::spawn_local(async move {
            loop {
                gloo_timers::future::TimeoutFuture::new(10_000).await;
                let ts = timestamps.get();
                set_freshness.set(compute_freshness(&ts, threshold));
            }
        });
    }

    let css_class = move || {
        let base = freshness.get().css_class();
        match &class {
            Some(c) => format!("{} {}", base, c),
            None => base.to_string(),
        }
    };

    let text = move || freshness.get().display_text();

    view! {
        <span class=css_class>
            {text}
        </span>
    }
}
