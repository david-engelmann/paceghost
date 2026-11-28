# Paceghost User Guide

Paceghost reads recorded task runs, timed checkpoint splits, and multi-competitor field days from local JSON files and prints twenty-seven deterministic pacing and statistical reports over them.

## Installation

### Prebuilt binary (Linux and macOS)

```sh
curl -fsSL https://raw.githubusercontent.com/david-engelmann/paceghost/main/install.sh | sh
export PATH="$HOME/.local/bin:$PATH"
```

Pass `--to <DIR>` to install somewhere other than `$HOME/.local/bin`, and ensure the destination directory stands on `PATH`.

### Cargo Binstall

```sh
cargo binstall paceghost
```

### Build from source

Note that `paceghost` requires **Rust `1.82` or newer** (CI runs on `1.91.1`).

```sh
git clone https://github.com/david-engelmann/paceghost.git
cd paceghost
cargo install --path . --bin paceghost --locked
```

## 60-Second Walkthrough

From the root of a `paceghost` repository checkout (`git clone https://github.com/david-engelmann/paceghost.git && cd paceghost`), inspect a single recorded run, find the fastest standing ghost, race a live run split-by-split against an earlier ghost (`ghost_file` first, `live_file` second), project mid-run token burn against a budget board, and analyze a 3-day multi-runner meet.

```sh
paceghost show examples/operator-run.json
paceghost ghost examples/operator-run.json examples/night-fork-run.json
paceghost race examples/operator-run.json examples/night-fork-run.json
paceghost board examples/board-run.json examples/ghost-run.json
paceghost fieldday examples/field-day.json
paceghost meet examples/meet.json
paceghost table examples/meet.json
paceghost quartiles examples/meet.json
```

## Input File Schemas

Every input shape stands at version 1 (`FIX-PG-13`). Unknown extra fields are ignored, and any malformed required field refuses the whole file with `invalid <kind>` and `defect ...` lines on `stdout` (exit status `1`).

### 1. Recorded run (`show`, `ghost`, `race`, `board`, `season`)

```json
{
  "task": "draft the weekly summary",
  "window_seconds": 1800,
  "runner": "operator",
  "recorded_at": "2026-01-11",
  "total_seconds": 1400,
  "usage": 1090,
  "budget": 1200,
  "model": "atlas",
  "splits": [
    {"phase": "plan", "elapsed_seconds": 320},
    {"phase": "draft", "elapsed_seconds": 880},
    {"phase": "polish", "elapsed_seconds": 1400}
  ],
  "reads": [
    {"elapsed_seconds": 300, "usage": 210},
    {"elapsed_seconds": 700, "usage": 430},
    {"elapsed_seconds": 1200, "usage": 760}
  ]
}
```

- `splits` must list non-empty phase names in strictly rising `elapsed_seconds`. `total_seconds` may be omitted when derived from the final split (`PG-1`), or stated equal to it.
- Basic `show`, `ghost`, and `race` runs do not require `usage`, `budget`, `model`, or `reads`, except that any run carrying `reads` must also carry both `usage` and `budget` (`PG-6`), and `season` requires `model`, `usage`, and `budget` on every field run (`PG-5`).

### 2. Season field (`season <field.json> <ghost-run.json>`)

An object `{"runs": [ ... ]}` holding one or more recorded run objects sharing the ghost run's `task`, each carrying `model`, `usage`, and `budget`.

### 3. Field day (`fieldday <field-day.json>`)

```json
{
  "window_seconds": 1800,
  "operator": "operator",
  "runs": [
    {
      "runner": "operator",
      "usage": 1090,
      "reads": [
        {"elapsed_seconds": 300, "usage": 210},
        {"elapsed_seconds": 1200, "usage": 760}
      ]
    },
    {
      "runner": "night fork",
      "usage": 830,
      "reads": [
        {"elapsed_seconds": 900, "usage": 470}
      ]
    }
  ]
}
```

At each read with `elapsed_seconds > 0`, the projected full-window burn is `usage * window_seconds / elapsed_seconds` (reads at `elapsed_seconds == 0` are skipped in field days, ladders, rematches, and meets). Lower projections beat higher projections. When tied at the minimum projection, the incumbent record holder keeps the record, and only when the incumbent does not stand at that minimum do earliest standing mark and earlier appearance order under `PG-7` and `PG-13` break ties. `record stolen from <operator>` fires whenever a competitor other than `<operator>` holds the closing record after `<operator>` once held it.

### 4. Window ladder (`ladder <ladder.json>`)

An object `{"day": <field-day-object>, "windows": [600, 900, 1800]}` evaluating the same field day in list order across distinct positive whole-number window cutoffs (ignoring reads with `elapsed_seconds > window` at each cutoff).

### 5. Two-day rematch (`rematch <rematch.json>`)

An object `{"first": <field-day-object>, "second": <field-day-object>}` walking two consecutive field days as one continuous record progression.

### 6. Multi-day meet (`meet <meet.json>` and 18 sibling commands)

An object `{"days": [<field-day-1>, <field-day-2>, <field-day-3>, ...]}` holding between `3` and `10` field days (`first` through `tenth` in the ordinal table of `PG-13`).

## Six-Competitor Mark Example Separating Every Summary Statistic

In `paceghost`, every mark-level distribution reading (`mean`, `median`, `quartiles`, `range`, `midhinge`, `midrange`, `width`, `middle`, and `tails`) measures **the field behind the record holder** (`m = active - 1` chasers, excluding the holder `h` under `PG-35` through `PG-52`). With only two or three active competitors (`m = 1` or `2` chasers), symmetric location figures coincide. At a mark where `6` active competitors hold projections `[100, 120, 150, 210, 400, 900]` (record holder `h = 100` and `m = 5` sorted chasers `[120, 150, 210, 400, 900]`), `middle` and `median` both select the odd middle chaser `c_3 = 210` (whereas for `m = 2` chasers `middle` picks `c_1` and `median` averages `(c_1 + c_2) / 2`) while `mean`, `median`, `quartiles`, `range`, `midhinge`, `midrange`, `margins`, `spread`, `width`, and `tails` all separate cleanly.

| Command | Exact `paceghost` definition at the mark (`h = 100`, chasers `c = [120, 150, 210, 400, 900]`) | Standing / figure printed |
| --- | --- | --- |
| `margins` | Nearest chaser less record holder (`c_1 - h = 120 - 100`) | `margin 20` |
| `spread` | Farthest chaser less record holder (`c_5 - h = 900 - 100`) | `spread 800` |
| `middle` | Single chaser at 1-based rank `(m + 1) div 2` (`c_3` for `m = 5`, and the lower chaser `c_1` for `m = 2`, distinguishing it from `median`, `PG-35`) | `middle standing 210` |
| `tails` | Arithmetic means of lowest `ceil(m / 2)` chasers `[120, 150, 210]` and highest `ceil(m / 2)` chasers `[210, 400, 900]` (`PG-37`) | `lower tail standing 160`, `upper tail standing 503.333333` |
| `width` | Span across the chaser pack behind the holder (`c_5 - c_1 = 900 - 120`, `PG-51`) | `field width 780` |
| `mean` | Arithmetic mean of the `5` chasers `(120 + 150 + 210 + 400 + 900) / 5` (`PG-39`) | `mean standing 356`, `mean margin 256` |
| `median` | Middle value of the `5` chasers (`c_3`, `PG-41`) | `median standing 210`, `median margin 110` |
| `quartiles` | Medians of lower half `[120, 150, 210]` (`Q1`) and upper half `[210, 400, 900]` (`Q3`, `PG-45`) | `lower quartile standing 150`, `upper quartile standing 400` |
| `range` | Interquartile range of the chaser pack (`Q3 - Q1 = 400 - 150`, `PG-43`) | `interquartile range 250` |
| `midhinge` | Midpoint of the chaser quartiles `(Q1 + Q3) / 2 = (150 + 400) / 2` (`PG-47`) | `midhinge standing 275`, `midhinge margin 175` |
| `midrange` | Midpoint of nearest and farthest chasers `(c_1 + c_5) / 2 = (120 + 900) / 2` (`PG-49`) | `midrange standing 510`, `midrange margin 410` |

## Command Reference Matrix (27 Commands)

### Run, ghost, and field-day commands (8 commands)

| Command | Arguments | Purpose |
| --- | --- | --- |
| `show` | `<run.json>` | Print task, runner, splits, and window spare/overrun summary for one run |
| `ghost` | `<run.json> [<run2.json> ...]` | Identify the fastest finishing run ("standing ghost") across runs of one task |
| `race` | `<ghost-run.json> <live-run.json>` | Compare `<live-run.json>` split-by-split and at the finish against `<ghost-run.json>` |
| `board` | `<run.json> <ghost-run.json>` | Project full-window usage at every timed read against both ghost usage and budget |
| `season` | `<field.json> <ghost-run.json>` | Aggregate window compliance, budget compliance, mean usage, and ghost win rate per model |
| `fieldday` | `<field-day.json>` | Walk timed reads in one window and report record progressions and stolen records |
| `ladder` | `<ladder.json>` | Replay one field day across multiple window lengths to see how horizon length changes the holder |
| `rematch` | `<rematch.json>` | Walk two field days sequentially and report whether day two held or broke day one's record |

### Multi-day meet commands (19 commands, all take `<meet.json>`)

| Category | Commands | What they report |
| --- | --- | --- |
| Record progression | `meet`, `table`, `reign`, `days`, `forks`, `marks` | Chronological record walk (`meet`), per-competitor lows/closes (`table`), contiguous leadership spells (`reign`), per-day handovers (`days`), per-competitor event history (`forks`), and mark-by-mark outcomes (`marks`) |
| Day-close matrices | `standings`, `traces` | Competitor projections at each day's close grouped by day (`standings`) or transposed by competitor (`traces`) |
| Chaser-field geometry | `margins`, `spread`, `middle`, `tails`, `width` | Holder-to-nearest margin (`margins`), holder-to-farthest span (`spread`), single chaser at sorted position `(m + 1) div 2` (`middle`), arithmetic means of the lowest and highest `ceil(m / 2)` chasers (`tails`), and chaser-pack span (`width`) |
| Chaser-field statistics | `mean`, `median`, `quartiles`, `range`, `midhinge`, `midrange` | Exact rational location and dispersion figures (`range` reporting the interquartile range `Q3 - Q1`) across the chaser field behind the holder at each mark |

## Multi-Agent Fleet Pacing Recipe (`thirdshift` Companion)

When running autonomous worker fleets inside fixed rate-limit or credit windows (where `thirdshift` schedules the work and `paceghost` scores the recorded telemetry), wire `paceghost` into your post-checkpoint hook.

1. **Emit checkpoint reads.** Have each worker append `(elapsed_seconds, usage)` token or credit checkpoints periodically into its run JSON alongside `usage` and `budget`.
2. **Audit window burn against a baseline ghost.** Run `paceghost board current-run.json baseline-ghost.json` to see at every checkpoint whether linear burn projects inside or past the window budget (`the board called it`).
3. **Compare worker variants across multi-day meets.** Bundle 3 to 10 consecutive fleet field days into `meet.json` and run `paceghost table`, `paceghost traces`, `paceghost quartiles`, and `paceghost range` to compare chaser-field dispersion across model or prompt revisions without float drift.
