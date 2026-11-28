# Changelog

All notable product and packaging changes to `paceghost` stand recorded in this file.

## Unreleased

### Added

- Prebuilt five-target release workflow (`release.yml`) bundling shell completions, JSON Schemas, and manual pages into Linux, macOS, and Windows archives alongside a `SHA256SUMS` verification installer (`install.sh`).
- Crate packaging and `cargo-binstall` metadata in `Cargo.toml`, excluding internal gate binaries and handbook build sources so published `.crate` archives stay compact, plus `deny.toml` license and advisory checks.
- Draft 2020-12 JSON Schemas under `schemas/`, static tab-completion scripts under `completions/` (`bash`, `zsh`, `fish`, `PowerShell`), offline `man/paceghost.1` manual page, and root composite GitHub Action (`action.yml`).
- Searchable `mdBook` operator handbook under `book/` paired with GitHub Pages CI (`docs.yml`), clean-room installer verification (`clean-install.yml`), and three-OS matrix validation (`cross-platform.yml`).
- Documentation and visual media suite under `docs/` and `docs/media/` covering the operator guide, five-tool pipeline cookbook, pipeline architecture, Spectacle Line comparison, measured Apple Silicon release benchmarks, social preview cards, animated CLI showcase, and pinned-compatible storefront README draft.

## 0.1.0

### Added

- Exact arbitrary precision rational pacing projections computing full window token burn rates from recorded run checkpoints without floating point drift.
- Single run display, standing ghost selector, two run split race, per model season summary, live pace board, field day, window ladder, and two day rematch commands.
- Nineteen multi day meet readings covering chronological record progressions, competitor tables, reigns, days, forks, marks, standings, traces, margins, spread, middle, tails, mean, median, quartiles, range, midhinge, midrange, and width.
- Whole input validation reporting every reachable JSON schema defect before any report line prints.
- Deterministic report rendering with terminal width aware help formatting and full line break fencing.
