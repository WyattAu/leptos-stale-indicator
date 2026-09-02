# leptos-stale-indicator

Data freshness indicator for Leptos — shows LIVE/DATA STALE status based on fetch timestamps.

## Features

- Displays "LIVE" or "DATA STALE · Xm ago" badge
- Polls every 10 seconds automatically
- Configurable stale threshold (default: 5 minutes)
- CSS class hooks for custom styling
- Supports SSR and hydration

## Installation

```bash
cargo add leptos-stale-indicator
```

## Quick Start

```rust
use leptos::prelude::*;
use leptos_stale_indicator::StaleIndicator;
use std::collections::HashMap;

#[component]
fn Dashboard(timestamps: Signal<HashMap<String, f64>>) -> impl IntoView {
    view! {
        <StaleIndicator timestamps=timestamps/>
    }
}
```

## API Reference

### StaleIndicator

| Prop | Type | Description |
|------|------|-------------|
| `timestamps` | `Signal<HashMap<String, f64>>` | Map of source name → last fetch time (seconds since epoch) |
| `threshold` | `f64` | Max age in seconds before stale (default: `300.0`) |
| `class` | `Option<String>` | Additional CSS class |

### DataFreshness

```rust
pub enum DataFreshness {
    Live,
    Stale { minutes: u32 },
}

impl DataFreshness {
    pub fn display_text(&self) -> String;  // "LIVE" or "DATA STALE · Xm ago"
    pub fn css_class(&self) -> &str;       // "status-dot status-dot--live" or "status-dot status-dot--stale"
}
```

### use_data_freshness

```rust
use leptos_stale_indicator::use_data_freshness;

let freshness: Signal<DataFreshness> = use_data_freshness(timestamps, 300.0);
```

## CSS Classes

The component applies these classes for styling:

```css
.status-dot {
  display: inline-flex;
  align-items: center;
  padding: 2px 8px;
  border-radius: 4px;
  font-size: 12px;
  font-weight: 600;
}

.status-dot--live {
  background: rgba(34, 197, 94, 0.15);
  color: #22c55e;
}

.status-dot--stale {
  background: rgba(234, 179, 8, 0.15);
  color: #eab308;
}
```

## Features

```toml
[dependencies]
leptos-stale-indicator = { version = "0.1", features = ["hydrate"] }
```

- `hydrate` (default) — Client-side hydration support
- `ssr` — Server-side rendering support

## License

MIT
