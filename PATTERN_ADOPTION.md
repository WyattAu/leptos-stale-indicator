# Pattern adoption

| Pattern | Status | Evidence |
|---|---|---|
| P21 | adopted | src/freshness.rs:40 (per-source timestamps plus a caller-supplied threshold; no global TTL) |
| T13 | adopted | src/freshness.rs:40 (pure freshness computation), src/freshness.rs:61 (derived Memo and badge rendering as the shell) |
