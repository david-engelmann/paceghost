# Paceghost FAQ

## Does Paceghost run tasks, timers, or background jobs?

No. `paceghost` is a pure offline reader over recorded JSON files (`AGENTS.md`). Fleet scheduling belongs to `thirdshift`, while `paceghost` computes exact pacing projections, stolen-record progressions, and statistical summaries over the resulting recordings.

## Why is there no standalone `FORMATS.md` at the repository root?

Under contract amendment `FIX-PG-13` (`CONTRACT.md`), `paceghost` consolidates its normative version 1 format specifications inside `CONTRACT.md` and `README.md` (`## Input files`), with copy-pasteable examples for all six input shapes provided in [GUIDE.md](GUIDE.md#input-file-schemas).

## How does the pacing projection formula work?

At any checkpoint read with `elapsed_seconds > 0` and window `W = window_seconds`, `paceghost` projects full-window consumption linearly as the exact rational `(usage * W) / elapsed_seconds`. On `board`, a read at `elapsed_seconds == 0` prints `projects no figure`, whereas `fieldday`, `ladder`, `rematch`, and `meet` skip zero-elapsed reads, and `ladder` additionally ignores reads where `elapsed_seconds` exceeds the evaluated window cutoff.

## Why do `mean`, `median`, `midhinge`, and `midrange` print identical numbers on small fixtures?

Every distribution reading computes over the **chaser field behind the record holder** (`m = active - 1` competitors). When `1` competitor holds a standing (`m = 0`), the mark prints `and no other standing`. When `2` or `3` competitors hold standings (`m = 1` or `2` chasers), arithmetic mean, median, midhinge, and midrange over `1` or `2` numbers are algebraically identical regardless of spacing. Once `4` or more competitors hold skewed standings (`m >= 3` chasers, as in the six-competitor example in [GUIDE.md](GUIDE.md#six-competitor-mark-example-separating-every-summary-statistic)), the statistics separate cleanly.

## What do the exit codes mean?

- **`0`** Valid input loaded and report printed cleanly.
- **`1`** Malformed input JSON refused with `invalid <kind>` and `defect ...` lines on `stdout` (or closed output stream write failure).
- **`2`** Unknown subcommand, unrecognized flags, or wrong argument count on `stderr`.
