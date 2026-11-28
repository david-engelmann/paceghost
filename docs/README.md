<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="media/paceghost-lockup-dark.svg">
    <source media="(prefers-color-scheme: light)" srcset="media/paceghost-lockup-light.svg">
    <img alt="paceghost" src="media/paceghost-lockup-light.svg" width="380">
  </picture>
</p>

# Paceghost Documentation

Offline agent window-pacing projections, stolen-record progressions, and exact rational meet statistics.

<p align="center">
  <img alt="paceghost terminal demo" src="media/money-shot.gif" width="840">
</p>

## Documentation Map

| Document | What it covers |
| --- | --- |
| [GUIDE.md](GUIDE.md) | Installation, 60-second walkthrough, categorized command matrix, and multi-agent pipeline recipes |
| [COOKBOOK.md](COOKBOOK.md) | Five-stage closed-loop Spectacle pipeline (`myriad` -> `grandjury` -> `deadheat` -> `stickler` -> `paceghost`), reusable GitHub Action, and `man` page |
| [ARCHITECTURE.md](ARCHITECTURE.md) | Four-stage processing pipeline, module map, exact arithmetic guarantees, and input fencing |
| [COMPARISON.md](COMPARISON.md) | Spectacle Line 5-tool boundary matrix and comparison against cloud eval SaaS and scripts |
| [PERFORMANCE.md](PERFORMANCE.md) | Measured release binary size, sub-100ms benchmark latencies, and parser memory bounds |
| [FAQ.md](FAQ.md) | Scope boundaries, exit codes (`0`, `1`, `2`), determinism, and troubleshooting |
| [GUIDE.md#input-file-schemas](GUIDE.md#input-file-schemas) | All six version 1 JSON input shapes (`run`, `field`, `fieldday`, `ladder`, `rematch`, and `meet`) |
| [../schemas/](../schemas/) | Draft 2020-12 JSON Schema definitions (`*.schema.json`) for IDE autocomplete and CI validation |
| [../completions/](../completions/) | Ready-to-source `bash`, `zsh`, `fish`, and `PowerShell` tab-completion scripts |
| [../man/paceghost.1](../man/paceghost.1) | Offline `man(1)` manual page covering synopsis, commands, exit codes, and examples |
| [../action.yml](../action.yml) | Root composite GitHub Action for installing and running checks in CI workflows |
| [../CONTRACT.md](../CONTRACT.md) | Normative behavioural specification and numbered round obligations |
| [../CHANGELOG.md](../CHANGELOG.md) | Release history and packaging milestones |

## Design Guarantees

- **Pure offline execution.** `paceghost` makes zero network calls, schedules no work, and mutates none of the recorded run or meet files handed to it.
- **Exact rational projections and quantiles.** Full-window burn projections (`usage * window_seconds / elapsed_seconds`), tie-breaks, quartiles, midhinges, and midranges compute over `BigRational` values before formatting.
- **Whole-input defect reporting.** Any defect in any nested split, checkpoint read, or field day refuses the entire file with a complete defect list before printing a single report line.
- **Single-line output fencing.** Task, phase, model, date, and printed runner labels reject unpaired UTF-16 surrogates and line breaks (`\n` and `\r`) across loaders and report entry points, though untrusted text displayed in a terminal should still pass through a pager or sanitizer.

## The Spectacle Line

<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="media/spectacle-showcase-dark.svg">
    <source media="(prefers-color-scheme: light)" srcset="media/spectacle-showcase-light.svg">
    <img alt="The Spectacle Line five-repository portfolio overview" src="media/spectacle-showcase-light.svg" width="880">
  </picture>
</p>
