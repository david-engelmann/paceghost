<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="media/paceghost-lockup-dark.svg">
    <source media="(prefers-color-scheme: light)" srcset="media/paceghost-lockup-light.svg">
    <img alt="paceghost" src="media/paceghost-lockup-light.svg" width="380">
  </picture>
</p>

# Paceghost Documentation

Offline agent window-pacing projections, stolen-record progressions, and exact rational meet statistics.

## Documentation Map

| Document | What it covers |
| --- | --- |
| [GUIDE.md](GUIDE.md) | Installation, 60-second walkthrough, categorized command matrix, and multi-agent pipeline recipes |
| [ARCHITECTURE.md](ARCHITECTURE.md) | Four-stage processing pipeline, module map, exact arithmetic guarantees, and input fencing |
| [FAQ.md](FAQ.md) | Scope boundaries against sibling Spectacle tools, exit codes, determinism, and common troubleshooting |
| [GUIDE.md#input-file-schemas](GUIDE.md#input-file-schemas) | All six version 1 JSON input shapes (`run`, `field`, `fieldday`, `ladder`, `rematch`, and `meet`) |
| [../CONTRACT.md](../CONTRACT.md) | Normative behavioural specification and numbered round obligations |
| [../CHANGELOG.md](../CHANGELOG.md) | Release history and packaging milestones |

## Design Guarantees

- **Pure offline execution.** `paceghost` makes zero network calls, schedules no work, and mutates none of the recorded run or meet files handed to it.
- **Exact rational projections and quantiles.** Full-window burn projections (`usage * window_seconds / elapsed_seconds`), tie-breaks, quartiles, midhinges, and midranges compute over `BigRational` values before formatting.
- **Whole-input defect reporting.** Any defect in any nested split, checkpoint read, or field day refuses the entire file with a complete defect list before printing a single report line.
- **Terminal injection fencing.** Task, phase, model, date, and printed runner labels are fenced against embedded newlines across loaders and report entry points.
