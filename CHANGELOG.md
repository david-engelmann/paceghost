# Changelog

All notable product and packaging changes to `paceghost` stand recorded in this file.

## Unreleased

### Added

- Prebuilt release workflow covering Linux, macOS, and Windows archives alongside a `SHA256SUMS` verification installer (`install.sh`) for Linux and macOS.
- Crate packaging and `cargo-binstall` metadata in `Cargo.toml`, excluding the internal `codepoint_scan` gate binary from published crates, plus `deny.toml` license and advisory checks.
- Documentation suite under `docs/` covering the operator guide, pipeline architecture, multi-agent orchestration recipes, and frequently asked questions, paired with dark and light vector diagrams under `docs/media/`.

## 0.1.0

### Added

- Exact arbitrary precision rational pacing projections computing full window token burn rates from recorded run checkpoints without floating point drift.
- Single run display, standing ghost selector, two run split race, per model season summary, live pace board, field day, window ladder, and two day rematch commands.
- Nineteen multi day meet readings covering chronological record progressions, competitor tables, reigns, days, forks, marks, standings, traces, margins, spread, middle, tails, mean, median, quartiles, range, midhinge, midrange, and width.
- Whole input validation reporting every reachable JSON schema defect before any report line prints.
- Deterministic report rendering with terminal width aware help formatting and full line break fencing.
