# paceghost

<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="docs/media/paceghost-lockup-dark.svg">
    <source media="(prefers-color-scheme: light)" srcset="docs/media/paceghost-lockup-light.svg">
    <img alt="paceghost" src="docs/media/paceghost-lockup-light.svg" width="360">
  </picture>
</p>

<p align="center">
  <a href="docs/README.md">Docs</a> ·
  <a href="docs/GUIDE.md">Guide</a> ·
  <a href="docs/ARCHITECTURE.md">Architecture</a> ·
  <a href="docs/COMPARISON.md">Comparison</a> ·
  <a href="docs/PERFORMANCE.md">Performance</a> ·
  <a href="docs/FAQ.md">FAQ</a> ·
  <a href="schemas/">Schemas</a>
</p>

Paceghost is a command line tool that reads recorded task runs and field days from local files and prints reports over them.

A recorded run names a task and a runner and carries a usage window, a total time, and timed splits for its phases. A field day is a set of recorded runs inside one window, and a meet is three or more field days read as one record progression. The tool runs nothing and schedules nothing. Every report is computed from recordings that already exist on disk, and the same input files always print the same reports.

[![ci](https://github.com/david-engelmann/paceghost/actions/workflows/ci.yml/badge.svg)](https://github.com/david-engelmann/paceghost/actions/workflows/ci.yml)

<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="docs/media/paceghost-hero-dark.svg">
    <source media="(prefers-color-scheme: light)" srcset="docs/media/paceghost-hero-light.svg">
    <img alt="paceghost hero overview" src="docs/media/paceghost-hero-light.svg" width="880">
  </picture>
</p>

```text
$ paceghost reign examples/meet.json
meet reign, 3 days
spell, operator, first mark at 300 seconds on the first day, last mark at 400 seconds on the first day, 2 marks
spell, dawn fork, first mark at 600 seconds on the first day, last mark at 600 seconds on the first day, 1 marks
spell, operator, first mark at 700 seconds on the first day, last mark at 700 seconds on the first day, 1 marks
spell, night fork, first mark at 900 seconds on the first day, last mark at 300 seconds on the second day, 5 marks
spell, dawn fork, first mark at 400 seconds on the second day, last mark at 600 seconds on the third day, 7 marks
reign counts, marks walked 16, spells 5, holders 3, longest spell 7 marks held by dawn fork
```

<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="docs/media/spectacle-arc-dark.svg">
    <source media="(prefers-color-scheme: light)" srcset="docs/media/spectacle-arc-light.svg">
    <img alt="Spectacle Line ecosystem map" src="docs/media/spectacle-arc-light.svg" width="880">
  </picture>
</p>

## Install

Install a prebuilt release binary on Linux or macOS with SHA-256 verification, install via `cargo binstall`, or build from source on Rust `1.82` or newer.

```sh
curl -fsSL https://raw.githubusercontent.com/david-engelmann/paceghost/main/install.sh | sh
```

```sh
cargo binstall paceghost
```

```sh
git clone https://github.com/david-engelmann/paceghost.git
cd paceghost
cargo build --release
```

The build leaves the binary at target/release/paceghost. To install the command on your PATH, run cargo install --path . in the clone.

## Testing

The local gate is cargo test, cargo fmt --check, and cargo clippy --all-targets -- -D warnings, run from the repo root. CI runs the same commands.

One pin in the suite, the closed stream pin in rust/tests/test_cli_widths.rs, opens /dev/full and expects the write to fail. The pin stands skipped on a machine where that path is absent or is not a character device, and it names the state it found in its output when it skips.

## Quickstart

Run the reign command over the example meet that stands in the repo.

```text
$ paceghost reign examples/meet.json
meet reign, 3 days
spell, operator, first mark at 300 seconds on the first day, last mark at 400 seconds on the first day, 2 marks
spell, dawn fork, first mark at 600 seconds on the first day, last mark at 600 seconds on the first day, 1 marks
spell, operator, first mark at 700 seconds on the first day, last mark at 700 seconds on the first day, 1 marks
spell, night fork, first mark at 900 seconds on the first day, last mark at 300 seconds on the second day, 5 marks
spell, dawn fork, first mark at 400 seconds on the second day, last mark at 600 seconds on the third day, 7 marks
reign counts, marks walked 16, spells 5, holders 3, longest spell 7 marks held by dawn fork
```

## Input files

Paceghost reads six kinds of input file, and CONTRACT.md specifies each one. Every form stands at version 1. Fields the contract does not name are ignored. A new required field, a changed refusal, or a changed defect phrase raises the version of a form, and the defect vocabulary is part of the versioned surface, so a changed defect phrase is a version change and never a silent one. The forms are the recorded run of PG-1 with the Round 2 run extension, the field of PG-5, the field day of the Round 3 section of CONTRACT.md, the ladder of the Round 4 section, the rematch of the Round 5 section, and the meet of the Round 6 section.

A recorded run file holds one run, as the shipped example at examples/operator-run.json shows whole.

```json
{
  "task": "draft the weekly summary",
  "window_seconds": 1800,
  "runner": "operator",
  "recorded_at": "2026-01-04",
  "total_seconds": 1500,
  "splits": [
    {"phase": "plan", "elapsed_seconds": 300},
    {"phase": "draft", "elapsed_seconds": 900},
    {"phase": "polish", "elapsed_seconds": 1500}
  ]
}
```


<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="docs/media/paceghost-architecture-dark.svg">
    <source media="(prefers-color-scheme: light)" srcset="docs/media/paceghost-architecture-light.svg">
    <img alt="paceghost architecture" src="docs/media/paceghost-architecture-light.svg" width="880">
  </picture>
</p>

## Usage

A file that does not match the recorded shape is refused. The command prints the defects the loader found, naming each malformed field, and exits with status one. Every command below runs over the synthetic example files that stand in the examples directory, and the output under each command is the output the command prints.

### show

The show command prints one recorded run.

```text
$ paceghost show examples/operator-run.json
task draft the weekly summary
runner operator
recorded_at 2026-01-04
window_seconds 1800
total_seconds 1500
splits 3
split plan at 300 seconds
split draft at 900 seconds
split polish at 1500 seconds
window, this run finished inside its window with 300 seconds to spare
```

### ghost

The ghost command prints the standing ghost for a task, the recorded run with the lowest total, over the run files named.

```text
$ paceghost ghost examples/operator-run.json examples/night-fork-run.json
ghost for task draft the weekly summary
runner night fork
recorded_at 2026-01-05
total_seconds 1320
```

### race

The race command prints one recorded run against a ghost run, split by split and at the finish.

```text
$ paceghost race examples/operator-run.json examples/night-fork-run.json
plan ahead by 60 seconds
draft ahead by 120 seconds
polish ahead by 180 seconds
window, the live run finished inside its window with 480 seconds to spare
finish ahead by 180 seconds
```

### season

The season command prints a report over a field of recorded runs grouped by model and read against a ghost record.

```text
$ paceghost season examples/field.json examples/ghost-run.json
season for task draft the weekly summary
model birch
runs raced 2
inside window 2
inside budget 1
total usage 2170
mean usage 1085
head to head against the ghost, wins 2 of 2
model cedar
runs raced 2
inside window 2
inside budget 2
total usage 2300
mean usage 1150
head to head against the ghost, wins 1 of 2
model atlas
runs raced 3
inside window 2
inside budget 2
total usage 3290
mean usage 1096.666667
head to head against the ghost, wins 1 of 3
```

### board

The board command prints the board for one recorded run that carries timed reads, read against a ghost record.

```text
$ paceghost board examples/board-run.json examples/ghost-run.json
read 0 at 0 seconds, usage 0, projects no figure
read 1 at 300 seconds, usage 210, projection 1260, past the ghost usage, past the budget
read 2 at 700 seconds, usage 430, projection 1105.714286, inside the ghost usage, inside the budget
read 3 at 1200 seconds, usage 760, projection 1140, inside the ghost usage, inside the budget
finish, actual usage 1090 against a last projection of 1140, inside the ghost usage, inside the budget, the board called it
```

### fieldday

The fieldday command prints the record progression of the recorded runs of one field day inside one window.

```text
$ paceghost fieldday examples/field-day.json
field day, window 1800 seconds, 4 runs, operator operator
record at 300 seconds, operator at a projection of 1260
record at 600 seconds, dawn fork at a projection of 1200
record at 700 seconds, operator at a projection of 1105.714286
record at 900 seconds, night fork at a projection of 940
record at 1500 seconds, night fork at a projection of 840
finish, record held by night fork at a projection of 840
record stolen from operator at 900 seconds by night fork, margin 300 at the close
```

### ladder

The ladder command prints one field day read through a list of windows.

```text
$ paceghost ladder examples/ladder.json
field day, window 600 seconds, 4 runs, operator operator
record at 300 seconds, operator at a projection of 420
record at 600 seconds, dawn fork at a projection of 400
finish, record held by dawn fork at a projection of 400
record stolen from operator at 600 seconds by dawn fork, margin 20 at the close
field day, window 900 seconds, 4 runs, operator operator
record at 300 seconds, operator at a projection of 630
record at 600 seconds, dawn fork at a projection of 600
record at 700 seconds, operator at a projection of 552.857143
record at 900 seconds, night fork at a projection of 470
finish, record held by night fork at a projection of 470
record stolen from operator at 900 seconds by night fork, margin 82.857143 at the close
field day, window 1800 seconds, 4 runs, operator operator
record at 300 seconds, operator at a projection of 1260
record at 600 seconds, dawn fork at a projection of 1200
record at 700 seconds, operator at a projection of 1105.714286
record at 900 seconds, night fork at a projection of 940
record at 1500 seconds, night fork at a projection of 840
finish, record held by night fork at a projection of 840
record stolen from operator at 900 seconds by night fork, margin 300 at the close
ladder, the closing holder differs from the first window at 900 seconds, 1800 seconds
```

### rematch

The rematch command prints two field days as one record progression.

```text
$ paceghost rematch examples/rematch.json
rematch, first day window 1800 seconds, 4 runs, second day window 1800 seconds, 3 runs
record at 300 seconds on the first day, operator at a projection of 1260
record at 600 seconds on the first day, dawn fork at a projection of 1200
record at 700 seconds on the first day, operator at a projection of 1105.714286
record at 900 seconds on the first day, night fork at a projection of 940
record at 1500 seconds on the first day, night fork at a projection of 840
record at 400 seconds on the second day, dawn fork at a projection of 675
finish, record held by dawn fork at a projection of 675
record changed hands on the second day at 400 seconds, dawn fork took the record from night fork
```

### meet

The meet command prints three or more field days as one record progression.

```text
$ paceghost meet examples/meet.json
meet, 3 days
record at 300 seconds on the first day, operator at a projection of 1260
record at 600 seconds on the first day, dawn fork at a projection of 1200
record at 700 seconds on the first day, operator at a projection of 1105.714286
record at 900 seconds on the first day, night fork at a projection of 940
record at 1500 seconds on the first day, night fork at a projection of 840
close of the first day, record held by night fork at a projection of 840
record at 400 seconds on the second day, dawn fork at a projection of 675
close of the second day, record held by dawn fork at a projection of 675
record at 450 seconds on the third day, dawn fork at a projection of 560
close of the third day, record held by dawn fork at a projection of 560
finish, record held by dawn fork at a projection of 560
record set on the third day at 450 seconds
days read, 3
```

### table

The table command prints the meet read competitor by competitor, naming the lowest standing projection of each competitor, the count of marks each competitor held the record after, and the standing projection at the close.

```text
$ paceghost table examples/meet.json
meet table, 3 days
competitor operator, lowest standing projection 750, set on the third day at 600 seconds, record held after 3 marks, standing projection 750 at the close
competitor night fork, lowest standing projection 720, set on the third day at 500 seconds, record held after 5 marks, standing projection 720 at the close
competitor dawn fork, lowest standing projection 560, set on the third day at 450 seconds, record held after 8 marks, standing projection 560 at the close
competitor quiet fork, no standing projection at any mark, record held after 0 marks, no standing projection at the close
table counts, competitors 4, record holders 3, marks walked 16
```

### reign

The reign command prints the meet read spell by spell, naming each maximal run of consecutive marks one competitor held the record after.

```text
$ paceghost reign examples/meet.json
meet reign, 3 days
spell, operator, first mark at 300 seconds on the first day, last mark at 400 seconds on the first day, 2 marks
spell, dawn fork, first mark at 600 seconds on the first day, last mark at 600 seconds on the first day, 1 marks
spell, operator, first mark at 700 seconds on the first day, last mark at 700 seconds on the first day, 1 marks
spell, night fork, first mark at 900 seconds on the first day, last mark at 300 seconds on the second day, 5 marks
spell, dawn fork, first mark at 400 seconds on the second day, last mark at 600 seconds on the third day, 7 marks
reign counts, marks walked 16, spells 5, holders 3, longest spell 7 marks held by dawn fork
```

### days

The days command prints the meet read day by day, naming the marks and the handovers of each day and the record that stood at its close.

```text
$ paceghost days examples/meet.json
meet days, 3 days
day, the first day, marks 8, handovers 3, record at the close held by night fork at a projection of 840
day, the second day, marks 5, handovers 1, record at the close held by dawn fork at a projection of 675
day, the third day, marks 3, handovers 0, record at the close held by dawn fork at a projection of 560
days counts, days 3, marks walked 16, handovers 4
```

### forks

The forks command prints the meet read competitor by competitor, naming every standing each competitor set and the record spells it held and the figures of its close.

```text
$ paceghost forks examples/meet.json
meet forks, 3 days, 4 competitors
competitor operator
standing, at a projection of 1260, set at 300 seconds on the first day
standing, at a projection of 1105.714286, set at 700 seconds on the first day
standing, at a projection of 1140, set at 1200 seconds on the first day
standing, at a projection of 1140, set at 300 seconds on the second day
standing, at a projection of 900, set at 800 seconds on the second day
standing, at a projection of 750, set at 600 seconds on the third day
spell, first mark at 300 seconds on the first day, last mark at 400 seconds on the first day, 2 marks
spell, first mark at 700 seconds on the first day, last mark at 700 seconds on the first day, 1 marks
close, days stood in 3, marks stood at 16, spells held 2, standing projection 750 at the close
competitor night fork
standing, at a projection of 1350, set at 400 seconds on the first day
standing, at a projection of 940, set at 900 seconds on the first day
standing, at a projection of 840, set at 1500 seconds on the first day
standing, at a projection of 1152, set at 500 seconds on the second day
standing, at a projection of 1008, set at 1000 seconds on the second day
standing, at a projection of 720, set at 500 seconds on the third day
spell, first mark at 900 seconds on the first day, last mark at 300 seconds on the second day, 5 marks
close, days stood in 3, marks stood at 15, spells held 1, standing projection 720 at the close
competitor dawn fork
standing, at a projection of 1200, set at 600 seconds on the first day
standing, at a projection of 998.181818, set at 1100 seconds on the first day
standing, at a projection of 675, set at 400 seconds on the second day
standing, at a projection of 560, set at 450 seconds on the third day
spell, first mark at 600 seconds on the first day, last mark at 600 seconds on the first day, 1 marks
spell, first mark at 400 seconds on the second day, last mark at 600 seconds on the third day, 7 marks
close, days stood in 3, marks stood at 14, spells held 2, standing projection 560 at the close
competitor quiet fork
close, days stood in 1, marks stood at 0, spells held 0, no standing projection at the close
forks counts, competitors 4, marks stood at 45, spells 5
```

### marks

The marks command prints the meet read mark by mark, naming the place of each mark in the whole walk, the holder after it, the record figure after it, and the standing the mark took against the state that stood before it.

```text
$ paceghost marks examples/meet.json
meet marks, 3 days
day, the first day, marks 8
mark 1, at 300 seconds on the first day, operator at a projection of 1260, set the standing record
mark 2, at 400 seconds on the first day, operator at a projection of 1260, stood below the standing record of 1260
mark 3, at 600 seconds on the first day, dawn fork at a projection of 1200, took the record from operator, displacing a projection of 1260
mark 4, at 700 seconds on the first day, operator at a projection of 1105.714286, took the record from dawn fork, displacing a projection of 1200
mark 5, at 900 seconds on the first day, night fork at a projection of 940, took the record from operator, displacing a projection of 1105.714286
mark 6, at 1100 seconds on the first day, night fork at a projection of 940, stood below the standing record of 940
mark 7, at 1200 seconds on the first day, night fork at a projection of 940, stood below the standing record of 940
mark 8, at 1500 seconds on the first day, night fork at a projection of 840, extended its own record, displacing a projection of 940
day, the second day, marks 5
mark 9, at 300 seconds on the second day, night fork at a projection of 840, stood below the standing record of 840
mark 10, at 400 seconds on the second day, dawn fork at a projection of 675, took the record from night fork, displacing a projection of 840
mark 11, at 500 seconds on the second day, dawn fork at a projection of 675, stood below the standing record of 675
mark 12, at 800 seconds on the second day, dawn fork at a projection of 675, stood below the standing record of 675
mark 13, at 1000 seconds on the second day, dawn fork at a projection of 675, stood below the standing record of 675
day, the third day, marks 3
mark 14, at 450 seconds on the third day, dawn fork at a projection of 560, extended its own record, displacing a projection of 675
mark 15, at 500 seconds on the third day, dawn fork at a projection of 560, stood below the standing record of 560
mark 16, at 600 seconds on the third day, dawn fork at a projection of 560, stood below the standing record of 560
marks counts, marks walked 16, record marks 7, days 3
```

### standings

The standings command prints the meet read at the close of each day, naming every competitor's standing at every day's close and closing each section with the closing line of the meet report for that day.

```text
$ paceghost standings examples/meet.json
meet standings, 3 days, 4 competitors
day, the first day, standings 3
competitor operator, standing projection 1140 at the close
competitor night fork, standing projection 840 at the close
competitor dawn fork, standing projection 998.181818 at the close
competitor quiet fork, no standing projection at the close
close of the first day, record held by night fork at a projection of 840
day, the second day, standings 3
competitor operator, standing projection 900 at the close
competitor night fork, standing projection 1008 at the close
competitor dawn fork, standing projection 675 at the close
competitor quiet fork, no standing projection at the close
close of the second day, record held by dawn fork at a projection of 675
day, the third day, standings 3
competitor operator, standing projection 750 at the close
competitor night fork, standing projection 720 at the close
competitor dawn fork, standing projection 560 at the close
competitor quiet fork, no standing projection at the close
close of the third day, record held by dawn fork at a projection of 560
standings counts, days 3, competitors 4, standings held 9
```

### traces

The traces command prints the meet read competitor by competitor at the close of each day, naming every competitor's standing at the close of each day in one section of its own and the closes at which it held a standing.

```text
$ paceghost traces examples/meet.json
meet traces, 3 days, 4 competitors
competitor operator
the first day, standing projection 1140 at the close
the second day, standing projection 900 at the close
the third day, standing projection 750 at the close
close, standings held at 3 closes, standing projection 750 at the close
competitor night fork
the first day, standing projection 840 at the close
the second day, standing projection 1008 at the close
the third day, standing projection 720 at the close
close, standings held at 3 closes, standing projection 720 at the close
competitor dawn fork
the first day, standing projection 998.181818 at the close
the second day, standing projection 675 at the close
the third day, standing projection 560 at the close
close, standings held at 3 closes, standing projection 560 at the close
competitor quiet fork
the first day, no standing projection at the close
the second day, no standing projection at the close
the third day, no standing projection at the close
close, standings held at 0 closes, no standing projection at the close
traces counts, competitors 4, days 3, standings held 9
```

### margins

The margins command prints the meet read mark by mark, naming the holder after each mark, the record figure after it, the nearest standing held after it by a competitor other than its holder, and the margin by which the record stands ahead of that nearest standing.

```text
$ paceghost margins examples/meet.json
meet margins, 3 days
day, the first day, marks 8
mark 1, at 300 seconds on the first day, operator at a projection of 1260, and no other standing
mark 2, at 400 seconds on the first day, operator at a projection of 1260, nearest standing 1350 held by night fork, margin 90
mark 3, at 600 seconds on the first day, dawn fork at a projection of 1200, nearest standing 1260 held by operator, margin 60
mark 4, at 700 seconds on the first day, operator at a projection of 1105.714286, nearest standing 1200 held by dawn fork, margin 94.285714
mark 5, at 900 seconds on the first day, night fork at a projection of 940, nearest standing 1105.714286 held by operator, margin 165.714286
mark 6, at 1100 seconds on the first day, night fork at a projection of 940, nearest standing 998.181818 held by dawn fork, margin 58.181818
mark 7, at 1200 seconds on the first day, night fork at a projection of 940, nearest standing 998.181818 held by dawn fork, margin 58.181818
mark 8, at 1500 seconds on the first day, night fork at a projection of 840, nearest standing 998.181818 held by dawn fork, margin 158.181818
day, the second day, marks 5
mark 9, at 300 seconds on the second day, night fork at a projection of 840, nearest standing 998.181818 held by dawn fork, margin 158.181818
mark 10, at 400 seconds on the second day, dawn fork at a projection of 675, nearest standing 840 held by night fork, margin 165
mark 11, at 500 seconds on the second day, dawn fork at a projection of 675, nearest standing 1140 held by operator, margin 465
mark 12, at 800 seconds on the second day, dawn fork at a projection of 675, nearest standing 900 held by operator, margin 225
mark 13, at 1000 seconds on the second day, dawn fork at a projection of 675, nearest standing 900 held by operator, margin 225
day, the third day, marks 3
mark 14, at 450 seconds on the third day, dawn fork at a projection of 560, nearest standing 900 held by operator, margin 340
mark 15, at 500 seconds on the third day, dawn fork at a projection of 560, nearest standing 720 held by night fork, margin 160
mark 16, at 600 seconds on the third day, dawn fork at a projection of 560, nearest standing 720 held by night fork, margin 160
margins counts, marks walked 16, marks with another standing 15, days 3, closest margin 58.181818 at 1100 seconds on the first day
```

### spread

The spread command prints the meet read mark by mark, naming the holder after each mark, the record figure after it, the count of competitors holding a standing after it, the farthest standing held after it by a competitor other than its holder, and the spread by which that farthest standing stands behind the record.

```text
$ paceghost spread examples/meet.json
meet spread, 3 days
day, the first day, marks 8
mark 1, at 300 seconds on the first day, operator at a projection of 1260, field of 1, and no other standing
mark 2, at 400 seconds on the first day, operator at a projection of 1260, field of 2, farthest standing 1350 held by night fork, spread 90
mark 3, at 600 seconds on the first day, dawn fork at a projection of 1200, field of 3, farthest standing 1350 held by night fork, spread 150
mark 4, at 700 seconds on the first day, operator at a projection of 1105.714286, field of 3, farthest standing 1350 held by night fork, spread 244.285714
mark 5, at 900 seconds on the first day, night fork at a projection of 940, field of 3, farthest standing 1200 held by dawn fork, spread 260
mark 6, at 1100 seconds on the first day, night fork at a projection of 940, field of 3, farthest standing 1105.714286 held by operator, spread 165.714286
mark 7, at 1200 seconds on the first day, night fork at a projection of 940, field of 3, farthest standing 1140 held by operator, spread 200
mark 8, at 1500 seconds on the first day, night fork at a projection of 840, field of 3, farthest standing 1140 held by operator, spread 300
day, the second day, marks 5
mark 9, at 300 seconds on the second day, night fork at a projection of 840, field of 3, farthest standing 1140 held by operator, spread 300
mark 10, at 400 seconds on the second day, dawn fork at a projection of 675, field of 3, farthest standing 1140 held by operator, spread 465
mark 11, at 500 seconds on the second day, dawn fork at a projection of 675, field of 3, farthest standing 1152 held by night fork, spread 477
mark 12, at 800 seconds on the second day, dawn fork at a projection of 675, field of 3, farthest standing 1152 held by night fork, spread 477
mark 13, at 1000 seconds on the second day, dawn fork at a projection of 675, field of 3, farthest standing 1008 held by night fork, spread 333
day, the third day, marks 3
mark 14, at 450 seconds on the third day, dawn fork at a projection of 560, field of 3, farthest standing 1008 held by night fork, spread 448
mark 15, at 500 seconds on the third day, dawn fork at a projection of 560, field of 3, farthest standing 900 held by operator, spread 340
mark 16, at 600 seconds on the third day, dawn fork at a projection of 560, field of 3, farthest standing 750 held by operator, spread 190
spread counts, marks walked 16, marks with another standing 15, days 3, fullest field 3 at 600 seconds on the first day
```

### middle

The middle command prints the meet read mark by mark, naming the holder after each mark, the record figure after it, the middle standing held after it by a competitor other than its holder, and the middle margin by which the record stands ahead of that middle standing.

```text
$ paceghost middle examples/meet.json
meet middle, 3 days
day, the first day, marks 8
mark 1, at 300 seconds on the first day, operator at a projection of 1260, and no other standing
mark 2, at 400 seconds on the first day, operator at a projection of 1260, middle standing 1350 held by night fork, middle margin 90
mark 3, at 600 seconds on the first day, dawn fork at a projection of 1200, middle standing 1260 held by operator, middle margin 60
mark 4, at 700 seconds on the first day, operator at a projection of 1105.714286, middle standing 1200 held by dawn fork, middle margin 94.285714
mark 5, at 900 seconds on the first day, night fork at a projection of 940, middle standing 1105.714286 held by operator, middle margin 165.714286
mark 6, at 1100 seconds on the first day, night fork at a projection of 940, middle standing 998.181818 held by dawn fork, middle margin 58.181818
mark 7, at 1200 seconds on the first day, night fork at a projection of 940, middle standing 998.181818 held by dawn fork, middle margin 58.181818
mark 8, at 1500 seconds on the first day, night fork at a projection of 840, middle standing 998.181818 held by dawn fork, middle margin 158.181818
day, the second day, marks 5
mark 9, at 300 seconds on the second day, night fork at a projection of 840, middle standing 998.181818 held by dawn fork, middle margin 158.181818
mark 10, at 400 seconds on the second day, dawn fork at a projection of 675, middle standing 840 held by night fork, middle margin 165
mark 11, at 500 seconds on the second day, dawn fork at a projection of 675, middle standing 1140 held by operator, middle margin 465
mark 12, at 800 seconds on the second day, dawn fork at a projection of 675, middle standing 900 held by operator, middle margin 225
mark 13, at 1000 seconds on the second day, dawn fork at a projection of 675, middle standing 900 held by operator, middle margin 225
day, the third day, marks 3
mark 14, at 450 seconds on the third day, dawn fork at a projection of 560, middle standing 900 held by operator, middle margin 340
mark 15, at 500 seconds on the third day, dawn fork at a projection of 560, middle standing 720 held by night fork, middle margin 160
mark 16, at 600 seconds on the third day, dawn fork at a projection of 560, middle standing 720 held by night fork, middle margin 160
middle counts, marks walked 16, marks with another standing 15, days 3, closest middle margin 58.181818 at 1100 seconds on the first day
```

### tails

The tails command prints the meet read mark by mark, naming the holder after each mark, the record figure after it, the lower tail standing and the upper tail standing held after it by the field behind its holder, and the lower tail margin and the upper tail margin by which the record stands ahead of each tail standing.

```text
$ paceghost tails examples/meet.json
meet tails, 3 days
day, the first day, marks 8
mark 1, at 300 seconds on the first day, operator at a projection of 1260, and no other standing
mark 2, at 400 seconds on the first day, operator at a projection of 1260, lower tail standing 1350, lower tail margin 90, upper tail standing 1350, upper tail margin 90
mark 3, at 600 seconds on the first day, dawn fork at a projection of 1200, lower tail standing 1260, lower tail margin 60, upper tail standing 1350, upper tail margin 150
mark 4, at 700 seconds on the first day, operator at a projection of 1105.714286, lower tail standing 1200, lower tail margin 94.285714, upper tail standing 1350, upper tail margin 244.285714
mark 5, at 900 seconds on the first day, night fork at a projection of 940, lower tail standing 1105.714286, lower tail margin 165.714286, upper tail standing 1200, upper tail margin 260
mark 6, at 1100 seconds on the first day, night fork at a projection of 940, lower tail standing 998.181818, lower tail margin 58.181818, upper tail standing 1105.714286, upper tail margin 165.714286
mark 7, at 1200 seconds on the first day, night fork at a projection of 940, lower tail standing 998.181818, lower tail margin 58.181818, upper tail standing 1140, upper tail margin 200
mark 8, at 1500 seconds on the first day, night fork at a projection of 840, lower tail standing 998.181818, lower tail margin 158.181818, upper tail standing 1140, upper tail margin 300
day, the second day, marks 5
mark 9, at 300 seconds on the second day, night fork at a projection of 840, lower tail standing 998.181818, lower tail margin 158.181818, upper tail standing 1140, upper tail margin 300
mark 10, at 400 seconds on the second day, dawn fork at a projection of 675, lower tail standing 840, lower tail margin 165, upper tail standing 1140, upper tail margin 465
mark 11, at 500 seconds on the second day, dawn fork at a projection of 675, lower tail standing 1140, lower tail margin 465, upper tail standing 1152, upper tail margin 477
mark 12, at 800 seconds on the second day, dawn fork at a projection of 675, lower tail standing 900, lower tail margin 225, upper tail standing 1152, upper tail margin 477
mark 13, at 1000 seconds on the second day, dawn fork at a projection of 675, lower tail standing 900, lower tail margin 225, upper tail standing 1008, upper tail margin 333
day, the third day, marks 3
mark 14, at 450 seconds on the third day, dawn fork at a projection of 560, lower tail standing 900, lower tail margin 340, upper tail standing 1008, upper tail margin 448
mark 15, at 500 seconds on the third day, dawn fork at a projection of 560, lower tail standing 720, lower tail margin 160, upper tail standing 900, upper tail margin 340
mark 16, at 600 seconds on the third day, dawn fork at a projection of 560, lower tail standing 720, lower tail margin 160, upper tail standing 750, upper tail margin 190
tails counts, marks walked 16, marks with another standing 15, days 3, closest lower tail margin 58.181818 at 1100 seconds on the first day, closest upper tail margin 90 at 400 seconds on the first day
```

### mean

The mean command prints the meet read mark by mark, naming the holder after each mark, the record figure after it, the mean standing held after it by the field behind its holder, and the mean margin by which the record stands ahead of the mean standing.

```text
$ paceghost mean examples/meet.json
meet mean, 3 days
day, the first day, marks 8
mark 1, at 300 seconds on the first day, operator at a projection of 1260, and no other standing
mark 2, at 400 seconds on the first day, operator at a projection of 1260, mean standing 1350, mean margin 90
mark 3, at 600 seconds on the first day, dawn fork at a projection of 1200, mean standing 1305, mean margin 105
mark 4, at 700 seconds on the first day, operator at a projection of 1105.714286, mean standing 1275, mean margin 169.285714
mark 5, at 900 seconds on the first day, night fork at a projection of 940, mean standing 1152.857143, mean margin 212.857143
mark 6, at 1100 seconds on the first day, night fork at a projection of 940, mean standing 1051.948052, mean margin 111.948052
mark 7, at 1200 seconds on the first day, night fork at a projection of 940, mean standing 1069.090909, mean margin 129.090909
mark 8, at 1500 seconds on the first day, night fork at a projection of 840, mean standing 1069.090909, mean margin 229.090909
day, the second day, marks 5
mark 9, at 300 seconds on the second day, night fork at a projection of 840, mean standing 1069.090909, mean margin 229.090909
mark 10, at 400 seconds on the second day, dawn fork at a projection of 675, mean standing 990, mean margin 315
mark 11, at 500 seconds on the second day, dawn fork at a projection of 675, mean standing 1146, mean margin 471
mark 12, at 800 seconds on the second day, dawn fork at a projection of 675, mean standing 1026, mean margin 351
mark 13, at 1000 seconds on the second day, dawn fork at a projection of 675, mean standing 954, mean margin 279
day, the third day, marks 3
mark 14, at 450 seconds on the third day, dawn fork at a projection of 560, mean standing 954, mean margin 394
mark 15, at 500 seconds on the third day, dawn fork at a projection of 560, mean standing 810, mean margin 250
mark 16, at 600 seconds on the third day, dawn fork at a projection of 560, mean standing 735, mean margin 175
mean counts, marks walked 16, marks with another standing 15, days 3, closest mean margin 90 at 400 seconds on the first day
```

### median

The median command prints the meet read mark by mark, naming the holder after each mark, the record figure after it, the median standing held after it by the field behind its holder, and the median margin by which the record stands ahead of the median standing.

```text
$ paceghost median examples/meet.json
meet median, 3 days
day, the first day, marks 8
mark 1, at 300 seconds on the first day, operator at a projection of 1260, and no other standing
mark 2, at 400 seconds on the first day, operator at a projection of 1260, median standing 1350, median margin 90
mark 3, at 600 seconds on the first day, dawn fork at a projection of 1200, median standing 1305, median margin 105
mark 4, at 700 seconds on the first day, operator at a projection of 1105.714286, median standing 1275, median margin 169.285714
mark 5, at 900 seconds on the first day, night fork at a projection of 940, median standing 1152.857143, median margin 212.857143
mark 6, at 1100 seconds on the first day, night fork at a projection of 940, median standing 1051.948052, median margin 111.948052
mark 7, at 1200 seconds on the first day, night fork at a projection of 940, median standing 1069.090909, median margin 129.090909
mark 8, at 1500 seconds on the first day, night fork at a projection of 840, median standing 1069.090909, median margin 229.090909
day, the second day, marks 5
mark 9, at 300 seconds on the second day, night fork at a projection of 840, median standing 1069.090909, median margin 229.090909
mark 10, at 400 seconds on the second day, dawn fork at a projection of 675, median standing 990, median margin 315
mark 11, at 500 seconds on the second day, dawn fork at a projection of 675, median standing 1146, median margin 471
mark 12, at 800 seconds on the second day, dawn fork at a projection of 675, median standing 1026, median margin 351
mark 13, at 1000 seconds on the second day, dawn fork at a projection of 675, median standing 954, median margin 279
day, the third day, marks 3
mark 14, at 450 seconds on the third day, dawn fork at a projection of 560, median standing 954, median margin 394
mark 15, at 500 seconds on the third day, dawn fork at a projection of 560, median standing 810, median margin 250
mark 16, at 600 seconds on the third day, dawn fork at a projection of 560, median standing 735, median margin 175
median counts, marks walked 16, marks with another standing 15, days 3, closest median margin 90 at 400 seconds on the first day
```

### quartiles

The quartiles command prints the meet read mark by mark, naming the holder after each mark, the record figure after it, the lower quartile standing and the upper quartile standing held after it by the field behind its holder, and the lower quartile margin and the upper quartile margin by which the record stands ahead of each.

```text
$ paceghost quartiles examples/meet.json
meet quartiles, 3 days
day, the first day, marks 8
mark 1, at 300 seconds on the first day, operator at a projection of 1260, and no other standing
mark 2, at 400 seconds on the first day, operator at a projection of 1260, lower quartile standing 1350, lower quartile margin 90, upper quartile standing 1350, upper quartile margin 90
mark 3, at 600 seconds on the first day, dawn fork at a projection of 1200, lower quartile standing 1260, lower quartile margin 60, upper quartile standing 1350, upper quartile margin 150
mark 4, at 700 seconds on the first day, operator at a projection of 1105.714286, lower quartile standing 1200, lower quartile margin 94.285714, upper quartile standing 1350, upper quartile margin 244.285714
mark 5, at 900 seconds on the first day, night fork at a projection of 940, lower quartile standing 1105.714286, lower quartile margin 165.714286, upper quartile standing 1200, upper quartile margin 260
mark 6, at 1100 seconds on the first day, night fork at a projection of 940, lower quartile standing 998.181818, lower quartile margin 58.181818, upper quartile standing 1105.714286, upper quartile margin 165.714286
mark 7, at 1200 seconds on the first day, night fork at a projection of 940, lower quartile standing 998.181818, lower quartile margin 58.181818, upper quartile standing 1140, upper quartile margin 200
mark 8, at 1500 seconds on the first day, night fork at a projection of 840, lower quartile standing 998.181818, lower quartile margin 158.181818, upper quartile standing 1140, upper quartile margin 300
day, the second day, marks 5
mark 9, at 300 seconds on the second day, night fork at a projection of 840, lower quartile standing 998.181818, lower quartile margin 158.181818, upper quartile standing 1140, upper quartile margin 300
mark 10, at 400 seconds on the second day, dawn fork at a projection of 675, lower quartile standing 840, lower quartile margin 165, upper quartile standing 1140, upper quartile margin 465
mark 11, at 500 seconds on the second day, dawn fork at a projection of 675, lower quartile standing 1140, lower quartile margin 465, upper quartile standing 1152, upper quartile margin 477
mark 12, at 800 seconds on the second day, dawn fork at a projection of 675, lower quartile standing 900, lower quartile margin 225, upper quartile standing 1152, upper quartile margin 477
mark 13, at 1000 seconds on the second day, dawn fork at a projection of 675, lower quartile standing 900, lower quartile margin 225, upper quartile standing 1008, upper quartile margin 333
day, the third day, marks 3
mark 14, at 450 seconds on the third day, dawn fork at a projection of 560, lower quartile standing 900, lower quartile margin 340, upper quartile standing 1008, upper quartile margin 448
mark 15, at 500 seconds on the third day, dawn fork at a projection of 560, lower quartile standing 720, lower quartile margin 160, upper quartile standing 900, upper quartile margin 340
mark 16, at 600 seconds on the third day, dawn fork at a projection of 560, lower quartile standing 720, lower quartile margin 160, upper quartile standing 750, upper quartile margin 190
quartiles counts, marks walked 16, marks with another standing 15, days 3, closest lower quartile margin 58.181818 at 1100 seconds on the first day, closest upper quartile margin 90 at 400 seconds on the first day
```

### range

The range command prints the meet read mark by mark, naming the holder after each mark, the record figure after it, and the interquartile range held after it by the field behind its holder, the upper quartile standing less the lower quartile standing of that field.

```text
$ paceghost range examples/meet.json
meet range, 3 days
day, the first day, marks 8
mark 1, at 300 seconds on the first day, operator at a projection of 1260, and no other standing
mark 2, at 400 seconds on the first day, operator at a projection of 1260, interquartile range 0
mark 3, at 600 seconds on the first day, dawn fork at a projection of 1200, interquartile range 90
mark 4, at 700 seconds on the first day, operator at a projection of 1105.714286, interquartile range 150
mark 5, at 900 seconds on the first day, night fork at a projection of 940, interquartile range 94.285714
mark 6, at 1100 seconds on the first day, night fork at a projection of 940, interquartile range 107.532468
mark 7, at 1200 seconds on the first day, night fork at a projection of 940, interquartile range 141.818182
mark 8, at 1500 seconds on the first day, night fork at a projection of 840, interquartile range 141.818182
day, the second day, marks 5
mark 9, at 300 seconds on the second day, night fork at a projection of 840, interquartile range 141.818182
mark 10, at 400 seconds on the second day, dawn fork at a projection of 675, interquartile range 300
mark 11, at 500 seconds on the second day, dawn fork at a projection of 675, interquartile range 12
mark 12, at 800 seconds on the second day, dawn fork at a projection of 675, interquartile range 252
mark 13, at 1000 seconds on the second day, dawn fork at a projection of 675, interquartile range 108
day, the third day, marks 3
mark 14, at 450 seconds on the third day, dawn fork at a projection of 560, interquartile range 108
mark 15, at 500 seconds on the third day, dawn fork at a projection of 560, interquartile range 180
mark 16, at 600 seconds on the third day, dawn fork at a projection of 560, interquartile range 30
range counts, marks walked 16, marks with another standing 15, days 3, narrowest interquartile range 0 at 400 seconds on the first day
```

### midhinge

The midhinge command prints the meet read mark by mark, naming the holder after each mark, the record figure after it, and the midhinge standing held after it by the field behind its holder, the mean of the lower quartile standing and the upper quartile standing of that field.

```text
$ paceghost midhinge examples/meet.json
meet midhinge, 3 days
day, the first day, marks 8
mark 1, at 300 seconds on the first day, operator at a projection of 1260, and no other standing
mark 2, at 400 seconds on the first day, operator at a projection of 1260, midhinge standing 1350, midhinge margin 90
mark 3, at 600 seconds on the first day, dawn fork at a projection of 1200, midhinge standing 1305, midhinge margin 105
mark 4, at 700 seconds on the first day, operator at a projection of 1105.714286, midhinge standing 1275, midhinge margin 169.285714
mark 5, at 900 seconds on the first day, night fork at a projection of 940, midhinge standing 1152.857143, midhinge margin 212.857143
mark 6, at 1100 seconds on the first day, night fork at a projection of 940, midhinge standing 1051.948052, midhinge margin 111.948052
mark 7, at 1200 seconds on the first day, night fork at a projection of 940, midhinge standing 1069.090909, midhinge margin 129.090909
mark 8, at 1500 seconds on the first day, night fork at a projection of 840, midhinge standing 1069.090909, midhinge margin 229.090909
day, the second day, marks 5
mark 9, at 300 seconds on the second day, night fork at a projection of 840, midhinge standing 1069.090909, midhinge margin 229.090909
mark 10, at 400 seconds on the second day, dawn fork at a projection of 675, midhinge standing 990, midhinge margin 315
mark 11, at 500 seconds on the second day, dawn fork at a projection of 675, midhinge standing 1146, midhinge margin 471
mark 12, at 800 seconds on the second day, dawn fork at a projection of 675, midhinge standing 1026, midhinge margin 351
mark 13, at 1000 seconds on the second day, dawn fork at a projection of 675, midhinge standing 954, midhinge margin 279
day, the third day, marks 3
mark 14, at 450 seconds on the third day, dawn fork at a projection of 560, midhinge standing 954, midhinge margin 394
mark 15, at 500 seconds on the third day, dawn fork at a projection of 560, midhinge standing 810, midhinge margin 250
mark 16, at 600 seconds on the third day, dawn fork at a projection of 560, midhinge standing 735, midhinge margin 175
midhinge counts, marks walked 16, marks with another standing 15, days 3, closest midhinge margin 90 at 400 seconds on the first day
```

### midrange

The midrange command prints the meet read mark by mark, naming the holder after each mark, the record figure after it, and the midrange standing held after it by the field behind its holder, the mean of the nearest standing and the farthest standing of that field.

```text
$ paceghost midrange examples/meet.json
meet midrange, 3 days
day, the first day, marks 8
mark 1, at 300 seconds on the first day, operator at a projection of 1260, and no other standing
mark 2, at 400 seconds on the first day, operator at a projection of 1260, midrange standing 1350, midrange margin 90
mark 3, at 600 seconds on the first day, dawn fork at a projection of 1200, midrange standing 1305, midrange margin 105
mark 4, at 700 seconds on the first day, operator at a projection of 1105.714286, midrange standing 1275, midrange margin 169.285714
mark 5, at 900 seconds on the first day, night fork at a projection of 940, midrange standing 1152.857143, midrange margin 212.857143
mark 6, at 1100 seconds on the first day, night fork at a projection of 940, midrange standing 1051.948052, midrange margin 111.948052
mark 7, at 1200 seconds on the first day, night fork at a projection of 940, midrange standing 1069.090909, midrange margin 129.090909
mark 8, at 1500 seconds on the first day, night fork at a projection of 840, midrange standing 1069.090909, midrange margin 229.090909
day, the second day, marks 5
mark 9, at 300 seconds on the second day, night fork at a projection of 840, midrange standing 1069.090909, midrange margin 229.090909
mark 10, at 400 seconds on the second day, dawn fork at a projection of 675, midrange standing 990, midrange margin 315
mark 11, at 500 seconds on the second day, dawn fork at a projection of 675, midrange standing 1146, midrange margin 471
mark 12, at 800 seconds on the second day, dawn fork at a projection of 675, midrange standing 1026, midrange margin 351
mark 13, at 1000 seconds on the second day, dawn fork at a projection of 675, midrange standing 954, midrange margin 279
day, the third day, marks 3
mark 14, at 450 seconds on the third day, dawn fork at a projection of 560, midrange standing 954, midrange margin 394
mark 15, at 500 seconds on the third day, dawn fork at a projection of 560, midrange standing 810, midrange margin 250
mark 16, at 600 seconds on the third day, dawn fork at a projection of 560, midrange standing 735, midrange margin 175
midrange counts, marks walked 16, marks with another standing 15, days 3, closest midrange margin 90 at 400 seconds on the first day
```

### width

The width command prints the meet read mark by mark, naming the holder after each mark, the record figure after it, and the field width held after it by the field behind its holder, the farthest standing less the nearest standing of that field.

```text
$ paceghost width examples/meet.json
meet width, 3 days
day, the first day, marks 8
mark 1, at 300 seconds on the first day, operator at a projection of 1260, and no other standing
mark 2, at 400 seconds on the first day, operator at a projection of 1260, field width 0
mark 3, at 600 seconds on the first day, dawn fork at a projection of 1200, field width 90
mark 4, at 700 seconds on the first day, operator at a projection of 1105.714286, field width 150
mark 5, at 900 seconds on the first day, night fork at a projection of 940, field width 94.285714
mark 6, at 1100 seconds on the first day, night fork at a projection of 940, field width 107.532468
mark 7, at 1200 seconds on the first day, night fork at a projection of 940, field width 141.818182
mark 8, at 1500 seconds on the first day, night fork at a projection of 840, field width 141.818182
day, the second day, marks 5
mark 9, at 300 seconds on the second day, night fork at a projection of 840, field width 141.818182
mark 10, at 400 seconds on the second day, dawn fork at a projection of 675, field width 300
mark 11, at 500 seconds on the second day, dawn fork at a projection of 675, field width 12
mark 12, at 800 seconds on the second day, dawn fork at a projection of 675, field width 252
mark 13, at 1000 seconds on the second day, dawn fork at a projection of 675, field width 108
day, the third day, marks 3
mark 14, at 450 seconds on the third day, dawn fork at a projection of 560, field width 108
mark 15, at 500 seconds on the third day, dawn fork at a projection of 560, field width 180
mark 16, at 600 seconds on the third day, dawn fork at a projection of 560, field width 30
width counts, marks walked 16, marks with another standing 15, days 3, narrowest field width 0 at 400 seconds on the first day
```

## Status

Seeded in October 2026. Setup has landed, and Rounds 1 to 33 have closed, with all slices merged on main. Part of the Spectacle line in the 39 repo storefront perfection programme.

## Programme status

- Closed perfection rounds, 33 of 100. A round counts only when a full cycle of work, audit, review, red team, and approval has closed. Seeding and setup are not rounds.
- Contract, SETUP-1 to SETUP-4 and PG-1 to PG-8 met. PG-1 to PG-4 stand in full form in CONTRACT.md with the Round 1 repair amendments, and Round 1 closed on the merged slice 2 after three reviews, a repair pass, and an approving re-review. PG-5 and PG-6 stand in full form with the Round 2 repair amendment.
- Contract, PG-7 and PG-8 met. They stand in full form in CONTRACT.md, and Round 3 closed on the merged slice 4 after three reviews, a repair pass, and an approving re-review.
- Contract, PG-9 and PG-10 met. They stand in full form in CONTRACT.md with the Round 4 repair amendments, and Round 4 closed on the merged slice 5 after three reviews, a repair pass, and an approving re-review.
- Contract, PG-11 and PG-12 met. They stand in full form in CONTRACT.md with the Round 5 repair amendment, and Round 5 closed on the merged slice 6 after three reviews, a repair pass, and an approving re-review.
- Contract, PG-13 and PG-14 met. They stand in full form in CONTRACT.md, and Round 6 closed on the merged slice 7 after three reviews, a repair pass, and an approving re-review.
- Contract, PG-15 and PG-16 met. They stand in full form in CONTRACT.md, and Round 7 closed on the merged slice 8 after three reviews approving with no findings.
- Contract, PG-17 and PG-18 met. They stand in full form in CONTRACT.md, and Round 8 closed on the merged slice 9 after three reviews, no repair pass, and the red team condition discharged on executed probes.
- Contract, PG-19 and PG-20 met. They stand in full form in CONTRACT.md, and Round 9 closed on the merged slice 10 after three reviews, no repair pass, and the red team condition discharged on executed probes.
- Contract, PG-21 and PG-22 met. They stand in full form in CONTRACT.md under the Round 10 obligations with the FIX-PG-11 repair amendment, and Round 10 closed on the merged slice 11 after three reviews approving with no findings, with the red team's one observation on the field day report carried to a later round as a fencing candidate.
- Contract, PG-23 and PG-24 met. They stand in full form in CONTRACT.md under the Round 11 obligations with the FIX-PG-12 repair amendment, and Round 11 closed on the merged slice 12 after three reviews, one standards Minor on a test comment repaired, with the red team's one observation on the task, phase, model, and recorded_at fields carried to a later round as a fencing candidate.
- Contract, PG-25 and PG-26 met. They stand in full form in CONTRACT.md under the Round 12 obligations with the FIX-PG-13 and FIX-PG-14 repair amendments, and Round 12 closed on the merged slice 13 after three reviews, one standards Minor on a test comment repaired.
- Contract, PG-27 and PG-28 met. They stand in full form in CONTRACT.md under the Round 13 obligations with the FIX-PG-15 repair amendment, and Round 13 closed on the merged slice 14 after three reviews approving with no findings, with one stable toolchain lint repair on the traces tests at the landing.
- Contract, PG-29 and PG-30 met. They stand in full form in CONTRACT.md under the Round 14 obligations, and Round 14 closed on the merged slice 15 after three reviews approving with findings, with the red team's one Minor recorded as a known envelope limit of the shared seconds formatter.
- Contract, PG-31 and PG-32 met. They stand in full form in CONTRACT.md under the Round 15 obligations, and Round 15 closed on the merged slice 16 after three reviews approving with findings, with the one Minor standing against the design record and not the slice.
- Contract, PG-33 and PG-34 met. They stand in full form in CONTRACT.md under the Round 16 obligations, and Round 16 closed on the merged slice 17 after three reviews, with the standards Minor repaired in the slice and the red team's one Minor standing against the evidence record and not the slice.
- Contract, PG-35 and PG-36 met. They stand in full form in CONTRACT.md under the Round 17 obligations, and Round 17 closed on the merged slice 18 after three reviews, with the M-6 evidence item closed by execution and its record corrected in the slice, the refuted pin 2 header claim restated in the slice by a disclosed repair commit, and the red team's one Major standing against the build record and not the slice.
- Contract, PG-37 and PG-38 met. They stand in full form in CONTRACT.md under the Round 18 obligations, and Round 18 closed on the merged slice 19 after three reviews, with the header postures restated to their executed form and the design plan's README pins landed in the slice.
- Contract, PG-39 and PG-40 met. They stand in full form in CONTRACT.md under the Round 19 obligations, and Round 19 closed on the merged slice 20 after three reviews approving with findings, with the pin headers restated to their executed postures and the generated median pin's record comparison repaired in the slice, and the red team's local execution set carried as named debt to the Round 20 design.
- Contract, PG-41 and PG-42 met. They stand in full form in CONTRACT.md under the Round 20 obligations, and Round 20 closed on the merged slice 21 after three reviews, with the red team approving on recomputation of both quartiles goldens byte for byte, the fifth field's fixture restated under the loader rule its design had not modelled, the note's two missing commands pins carried into the slice, and the assertions that passed under their own negation restated to assertions over the binary's own output.
- Contract, PG-43 and PG-44 met. They stand in full form in CONTRACT.md under the Round 21 obligations, and Round 21 closed on the merged slice 22 after three reviews, with the equality condition of PG-44 restated by the FIX-PG-17 repair amendment and the exact subtraction pin added in the same repair, and both range goldens recomputed byte for byte by the red team.
- Contract, PG-45 and PG-46 met. They stand in full form in CONTRACT.md under the Round 22 obligations, and Round 22 closed on the merged slice 23 after three reviews, with the execution record of PG-45 and the probe executions of PG-46 standing executed in the red team's report and in the second shape run on the merged tree, and with the middle reading's holder-alone pin passing under R-6 against the design record's computed class, its executed posture standing in the red team record pin by pin, and with the closed-stream pins of the second shape run skipped under their stated posture at run time, the hosted gate standing as their deciding read, and with the standards review approving with findings at two Observations and the spec review approving with findings at one Minor standing against the design record and the red team approving with findings at one Minor against the design record and three Observations, and with CodeRabbit's one Major answered by the executions and its one Minor dismissed with reason on the record.
- Slice 2 has landed. It carries the recorded run loader that names every malformed field, the ghost rule, the race report, the ghost change report, and the show, ghost, and race commands, with two synthetic example runs and a golden race report under examples.
- The Round 2 slice has landed. It carries the season report over a recorded field of runs grouped by model and the live board over recorded timed reads, with the season and board commands, a synthetic example field, a synthetic ghost record carrying a usage figure, a synthetic example run carrying timed reads, and golden renderings under examples. PG-5 and PG-6 stand in full form in CONTRACT.md with the Round 2 repair amendment, and Round 2 closed on the merged slice after three reviews, a repair pass, a micro repair, and re-reviews approving the final head. The board reads a finished recording, and watching a real run while it runs remains future work.
- Slice 4 has landed. It carries the field day loader that names every malformed field, the stolen record report that walks the record progression of one window across a field of recorded runs, and the fieldday command, with a synthetic example field day and its golden report under examples.
- Slice 5 has landed. It carries the ladder loader that names every malformed field in a ladder file, the ladder report that reads one field day as it stood at each window in a list and closes with a summary line over the closing holders, and the ladder command, with a synthetic example ladder and its golden report under examples.
- Slice 6 has landed. It carries the rematch loader that names every malformed field in a rematch file, the rematch report that reads two field days as one record progression and closes by naming where the record changed hands between the days, and the rematch command, with two synthetic example rematches and their golden reports under examples.
- Slice 7 has landed. It carries the meet loader that names every malformed field in a meet file, the meet report that reads three or more field days as one record progression and names the closing holder of each day and the day the standing record was set, and the meet command, with a synthetic example meet and its golden report under examples.
- Slice 8 has landed. It carries the meet table that reads a meet competitor by competitor over the one record progression of the meet report, naming the lowest standing projection of each competitor and the day and mark at which it was set, the count of marks each competitor held the record after, and their standing projection at the close, and the table command, with the golden meet table of the standing example meet under examples.
- Slice 9 has landed. It carries the reign reading that reads a meet spell by spell over the one record progression of the meet report, naming each maximal run of consecutive marks one competitor held the record after with the marks it spans and its length, and the count of marks and spells and the longest spell at the close, and the reign command, with the golden reign reading of the standing example meet under examples.
- Slice 10 has landed. It carries the days reading that reads a meet day by day over the one record progression of the meet report, naming the marks and handovers of each day and the record that stood at its close, and the days command, with the golden days reading of the standing example meet under examples, and the README brought to the fleet README standard, its lede, install, quickstart, and usage sections standing with captured output.
- Slice 11 has landed. It carries the forks reading that reads a meet competitor by competitor over the one record progression of the meet report, naming every standing each competitor set and the record spells it held and the figures of its close, and the forks command, with the golden forks reading of the standing example meet under examples, and the label fencing closure of the meet readings under FIX-PG-11, and the README opening paragraph repaired of its lede restatement.
- Slice 12 has landed. It carries the marks reading that reads a meet mark by mark over the one record progression of the meet report, naming the place of each mark in the whole meet and the competitor who held the record after it and the standing the mark took against the state that stood before it, and the marks command, with the golden marks readings of the standing example meet and of a constructed meet under examples, and the label fencing closure of the field day family and the run readings under FIX-PG-12.
- Slice 13 has landed. It carries the standings reading that reads a meet at the close of each day over the one record progression of the meet report, naming every competitor's standing at every day's close and closing each section with the closing line of the meet report for that day, and the standings command, with the golden standings readings of the standing example meet and of a constructed meet under examples, and the file carried fencing closure of the loaders and the summary under FIX-PG-14, and the input forms named at version 1 with the guarded closed stream pin under FIX-PG-13, and the seeded generated fixture family pinned by the closing relations the contract states.
- Slice 14 has landed. It carries the traces reading that reads a meet competitor by competitor at the close of each day over the one record progression of the meet report, naming every competitor's standing at the close of each day in one section of its own and the closes at which it held a standing, and the traces command, with the golden traces readings of the standing example meet and of a constructed meet under examples, and the summary guard order pins completed under FIX-PG-15, and the seeded generated fixture family extended to the traces reading.
- Slice 15 has landed. It carries the margins reading that reads a meet mark by mark over the one record progression of the meet report, naming the holder after each mark, the record figure after it, the nearest standing held after it by a competitor other than its holder, and the margin by which the record stands ahead of that nearest standing, and the margins command, with the golden margins readings of the standing example meet and of a constructed meet under examples, and the multi day walk extended in its own pass to record the nearest standing after each mark, and the seeded generated fixture family extended to the margins reading.
- Slice 16 has landed. It carries the spread reading that reads a meet mark by mark over the one record progression of the meet report, naming the holder after each mark, the record figure after it, the count of competitors holding a standing after it, the farthest standing held after it by a competitor other than its holder, and the spread by which that farthest standing stands behind the record, and the spread command, with the golden spread readings of the standing example meet and of a constructed meet under examples, and the multi day walk extended in its own pass to record the farthest standing and the field after each mark, and the seeded generated fixture family extended to the spread reading.
- Slice 17 has landed. It carries the middle reading that reads a meet mark by mark over the one record progression of the meet report, naming the holder after each mark, the record figure after it, the middle standing held after it by a competitor other than its holder, and the middle margin by which the record stands ahead of that middle standing, and the middle command, with the golden middle readings of the standing example meet and of a constructed meet under examples, and the multi day walk extended in its own pass to record the middle standing after each mark, and the seeded generated fixture family extended to the middle reading.
- Slice 18 has landed. It carries the tails reading that reads a meet mark by mark over the one record progression of the meet report, naming the holder after each mark, the record figure after it, the lower tail standing and the upper tail standing held after it by the field behind its holder, and the lower tail margin and the upper tail margin by which the record stands ahead of each tail standing, and the tails command, with the golden tails readings of the standing example meet and of a constructed meet under examples, and the multi day walk extended in its own pass to record the lower tail standing and the upper tail standing after each mark, and the seeded generated fixture family extended to the tails reading.
- Slice 19 has landed. It carries the mean reading that reads a meet mark by mark over the one record progression of the meet report, naming the holder after each mark, the record figure after it, the mean standing held after it by the field behind its holder, and the mean margin by which the record stands ahead of the mean standing, and the mean command, with the golden mean readings of the standing example meet and of a constructed meet under examples, and the multi day walk extended in its own pass to record the mean standing after each mark, and the seeded generated fixture family extended to the mean reading.
- Slice 20 has landed. It carries the median reading that reads a meet mark by mark over the one record progression of the meet report, naming the holder after each mark, the record figure after it, the median standing held after it by the field behind its holder, and the median margin by which the record stands ahead of the median standing, and the median command, with the golden median readings of the standing example meet and of a constructed meet under examples, and the multi day walk extended in its own pass to record the median standing after each mark, and the seeded generated fixture family extended to the median reading.
- Slice 21 has landed. It carries the quartiles reading that reads a meet mark by mark over the one record progression of the meet report, naming the holder after each mark, the record figure after it, the lower quartile standing and the upper quartile standing held after it by the field behind its holder, and the lower quartile margin and the upper quartile margin by which the record stands ahead of each, and the quartiles command, with the golden quartiles readings of the standing example meet and of a constructed meet under examples, and the multi day walk extended in its own pass to record the lower quartile standing and the upper quartile standing after each mark, and the seeded generated fixture family extended to the quartiles reading, and the record prefix repair across the five standing generated pins, and the README Usage sections for the marks, standings, traces, and margins readings and the standing form guard over the Usage block.
- Slice 22 has landed. It carries the range reading that reads a meet mark by mark over the one record progression of the meet report, naming the holder after each mark, the record figure after it, and the interquartile range held after it by the field behind its holder, the upper quartile standing less the lower quartile standing of that field, and the range command, with the golden range readings of the standing example meet and of a constructed meet under examples, and the multi day walk extended in its own pass to record the interquartile range after each mark, and the seeded generated fixture family extended to the range reading, and the README range section, and the README badge set.
- Slice 23 has landed. It carries the execution record of the one unexecuted mutation of the Round 21 family and of the census of the opening tree in both shapes, and the probe recompositions of PG-46 over the three probes of the Round 19 record that stood unexecuted as recorded, with no new reading and no new command.
- Contract, PG-47 and PG-48 met. They stand in full form in CONTRACT.md under the Round 23 obligations, and Round 23 closed on the merged slice 24 after three reviews, with FIX-PG-18 spent in the slice on the restatement of PG-47's two first-separation clauses, and with the spec review approving with findings at four Observations standing against records and the standards review approving with findings at one Minor cured in the slice by a comment-only repair and three Observations and the red team approving with findings at four Observations standing against records, its independent transcription reproducing all eight goldens byte for byte and its mutation executions matching the stated splits, and with CodeRabbit's two inline comments standing valid and cured in the slice under FIX-PG-18 and the pin 5 edge case, and with the one erratum the round leaves priced standing as a named records item and not as met work, PG-48's generated paragraph compressing the day-close counts against the family's own computation, its cure standing owed through the amendment mechanism at a later round.
- Contract, PG-49 and PG-50 met. They stand in full form in CONTRACT.md under the Round 24 obligations, and Round 24 closed on the merged slice 25 after three reviews, with FIX-PG-19 spent in the slice on the cure charged at the Round 23 closure, PG-48's generated paragraph standing compressed against the family's own computation, and with FIX-PG-20 spent in the slice on the repairs the round's reviews charged, and with the spec review approving with findings at one Major cured in the slice and one Observation and the standards review approving with findings at one Minor and the red team approving with findings at one Major cured in the slice and one Minor, and with CodeRabbit returning one inline Minor on the slice, posted after the three reviews had read none, standing valid and cured in the slice under FIX-PG-20, and with the charged FIX-PG-19 cure landing as designed and then standing itself miscounted, the design's section 3 account standing true as written and the amendment's conversion carrying one at seven from the first closes alone and fifty-eight from the later closes alone into a by-count partition, the true partition counted on three agreeing instruments at 2 closes at a count of two, 6 at four, 18 at five, 20 at six, 14 at seven, and 7 at nine, with fifty-nine standing at counts of five and more, and with FIX-PG-20 restating the distribution truly in both contract places, and with PG-50's second equality clause standing false in both directions as shipped, pin 1's first mark standing against its necessity and pin 5's fixture standing against its sufficiency, and its restatement to the spread-equals-margin form standing under the same amendment, pin 10 asserting the true condition from its birth and its comment restated to name the parting, and with the build record's two partings standing corrected by the repair report, its pin 10 account's attribution clause and its section 5 repeating the false distribution pair as verified, and with the design note's section 4 carrying the same false pair, standing as a records item since amendments do not reach design notes, and with the Round 22 design record's manifest figure of 73 against the counted 71 standing as a records reconciliation and not as cured work, and with the two stated S postures standing confirmed by execution, pin 5 passing under S-1 whole and the generated pin failing under S-4 at seed 1's tie.
- Contract, PG-51 and PG-52 met. They stand in full form in CONTRACT.md under the Round 25 obligations, and Round 25 closed on the merged slice 26 after three reviews, with the spec review approving with findings at two Observations standing against the design record, and the standards review approving with findings at one Minor and two Observations, and the red team approving with no findings, and with CodeRabbit returning no inline comments on the slice, and with the standards review's Minor standing cured in the slice by the repair commit 85f22890, the cli.rs module doc count restated at twenty seven, comment-only and strip-proven, the census unmoved, and with the design record's seventh section naming two standing command surface pins against the counted three, all three standing restated in the build commit as the landing falsified them, and with pin 8's tie meet standing recomposed in the shipped fixture, the meet as transcribed standing unloadable under the loader's rising-usage rule, every figure the design states standing identical in the shipped lines, and its S-4 discrimination standing proven end to end by the red team on the recomposed fixture itself, and with pin 5's presence in the S-1 and S-6 sets standing on its second edge under the design's own structural rule, its five fixture marks holding under both, and with the day close example pin's split standing as the shape-forced split the execution ran, failing under S-1, S-5, and S-6 and passing under S-2, S-3, and S-4, the design stating it only by reference to the reading pin, and with S-2 and S-3 standing at identical failing sets as honest discrimination stated in the open, the two partners parting from each other by figure inside the round's own fixtures and each standing caught independently at nine pins, a separating fixture standing priced for a later round, and with the three records items standing as records only and not as cured work, the Round 24 design note's fourth section carrying the false distribution pair as written, one at seven and fifty eight at five and more against the counted fourteen at seven and fifty nine at five and more, its contract places standing cured inside Round 24 by FIX-PG-20, and the Round 23 design note's fifth section carrying its compression as written, and the Round 22 design record's manifest figure of 73 standing against the counted 71 as a records reconciliation, and with FIX-PG-21 standing free and unspent through the round.
- Contract, PG-53 and PG-54 met. They stand in full form in CONTRACT.md under the Round 26 obligations, and Round 26, the separation round, closed on the merged slice 27 after three reviews, with the spec review approving with findings at two Observations, and the standards review approving with findings at two Minors and four Observations, with both Minors standing cured in the slice by the repair commit bbb5310a, and the red team approving with no findings on its completion lane's report, its first seating standing incomplete on the write fault and discharged by execution, and with CodeRabbit returning three inline comments on the slice, two standing valid and cured in the repair, the qualifier half of the standards review's first Minor and the quartile standings of its second, and one standing not valid, the exact comparison comment, dismissed on the family's standing print grain tolerance standing at one hundred and thirty one lines of the base's generated file alone, and with the separation standing executed whole, under S-2 the failing set standing at ten with pins 17 and 18 passing, and under S-3 at exactly those ten plus pins 17 and 18, the sets parting at the separating meet's pins and nowhere else, and with the impossibility standing proved at the desk and swept by execution, one thousand five hundred and fourteen meets and three thousand and twenty eight figured marks at no counterexample, the claim's antecedent standing exercised at eight hundred and fifty three marks, and with the two cures standing proven in both directions, pin 16 failing at eight hundred and forty two partings counted against seven hundred and fifty six under disturbances at the tails seat and at the range seat where the unrepaired pin passed whole, and pin 18 failing at its lower quartile assertion under the compensating movement where the unrepaired pin passed, and with the build's fresh extraction re-run standing discharged under the bounded fault limb, the form gate on the identical tree and the hosted gate standing as its verdicts of record, and the repair's own fresh extraction gate standing executed whole at 959 and 0, and with the design note's section 0 base tree string standing for the errata queue, the base commit's tree standing at 7842fa27, and with the three records items of the design note's second section standing carried forward named with no cure charged, the Round 24 design note's fourth section carrying the false distribution pair as written, one at seven and fifty eight at five and more against the counted fourteen at seven and fifty nine at five and more, its contract places standing cured inside Round 24 by FIX-PG-20, and the Round 23 design note's fifth section carrying its compression as written, and the Round 22 design record's manifest figure of 73 standing against the counted 71 as a records reconciliation, and with FIX-PG-21 standing free and unspent through the round.
- Contract, PG-55 and PG-56 met. They stand in full form in CONTRACT.md under the Round 27 obligations, and Round 27, the width execution record round, closed on the merged slice 28 after three reviews, the slice standing as pull request 59 merged at a8668aea with main's tree standing verified equal to the repaired slice tree 535193a7 and the census standing at 961, with the spec review approving with findings at three Observations, each standing against the design record and none charged to the slice, and the standards review approving with findings at one Minor and three Observations, the Minor standing charged on the round twenty seven contract pin asserting its headings alone over a round whose whole substance stands in its contract body, and standing cured in the slice by the repair commit 4ad6eef0 extending the pin with ten content assertions over the four PG-55 totals, the posture sentence, and the five W sentences, the cure standing proven in both directions by execution, the repaired pin failing at the falsified S-1 sentence and at the falsified W-2 count where the unrepaired pin passed the whole setup target, and the repair's fresh extraction gate standing executed whole at 961 and 0, and the red team approving with no findings, and with CodeRabbit returning two inline comments on the slice, both standing valid as the standards review's Minor at its two seats and cured in the repair, its LanguageTool sub-notes standing dismissed on the contract's standing spelled out number form, and with the execution record standing confirmed by independent execution whole, under S-1 the suite standing at nine hundred and thirty nine passed and twenty failed, under S-5 at nine hundred and forty passed and nineteen failed, under S-6 at eight hundred and eleven passed and one hundred and forty eight failed standing at one hundred and twenty three distinct names, and under S-4 at nine hundred and fifty five passed and four failed, pins 17 and 18 falling under S-1, S-5, and S-6 and passing under S-4 exactly as the width file's header states, and with the hunt standing executed whole, five fresh edits falling and none passing whole, and the red team's own edit RT-1 falling at ten range family pins as it predicted before its run, and with the tests step standing proven comment only by mechanical stripping and the base and head binaries standing in agreement on two hundred and seventy drives at no parting, and with the design note's fifth section naming a round seven count pin and a Round 27 in flight heading the base's setup file does not carry, the construction standing on the family's executed form and the note's place counts standing exact as occurrence counts, and with the design report's section figure of 7,635 characters standing against the contract side figure of 7,651, the sixteen character difference standing fully accounted and every paragraph standing byte identical between the two spans, and with the standards review's whole family whole obligation cure standing as a priced later round question in the grandjury species, and with the three records items of the Round 26 note's second section standing carried forward named with no cure charged, the Round 24 design note's fourth section carrying the false distribution pair as written, one at seven and fifty eight at five and more against the counted fourteen at seven and fifty nine at five and more, its contract places standing cured inside Round 24 by FIX-PG-20, and the Round 23 design note's fifth section carrying its compression as written, and the Round 22 design record's manifest figure of 73 standing against the counted 71 as a records reconciliation, and with the Round 26 design note's section 0 base tree string standing for the errata queue, 0b9e4eea against the true base tree 7842fa27, and with FIX-PG-21 standing free and unspent through the round.
- Contract, PG-57 and PG-58 met. They stand in full form in CONTRACT.md under the Round 28 obligations, PG-57 standing as the whole obligation assertion cure over the contract pins of Rounds 22 through 26 and PG-58 as the hunt record of the field selections beside it, the hunt standing executed whole and no edit passing whole anywhere. The slice standing as pull request 61 merged at f6154458 with main's tree standing verified equal to the repaired slice tree ca0445e8 and the census standing at 963. The spec review approving with findings at one Observation, standing against the design record and none charged to the slice, that PG-58's opening sentence in the design note's fourth section states the hunt's four edits in executed form while its own later sentences stand them computed and carried, the item riding the errata queue. The standards review approving with findings at one Minor and one Observation, the Minor standing charged on the five restated pins carrying no local justification comment naming PG-57 where their alterations land, and standing cured in the slice by the repair commit d5ea2dd9 adding the one comment at each of the five seats in the file's own double slash form naming PG-57 and that the block asserts the standing subsection text whole, the cure standing proven in both directions, and the Observation standing against no text in the slice, the heading count block repeating at six seats in the standing family form and priced forward only. CodeRabbit returning one inline comment on the slice, standing valid as the standards review's Minor at its seat and cured in the same repair. The red team approving with findings at two Observations and nothing charged to the slice, the first standing against the standing record, that the Round 27 contract pin asserts PG-55 and PG-56 by named sentences only with nine sentences of PG-55 and six of PG-56 standing under no literal, priced forward for a later round, and the second naming the helper subsection_span as the cure's one shared component, priced in the open and guarded by its own count assertion. The hunt standing executed whole as first executions, X-1 at the design desk at nine hundred and fifty passed and eleven failed, X-2 at nine hundred and thirty eight passed and twenty three failed, X-3 at nine hundred and forty two passed and nineteen failed at eighteen distinct pins, and X-4 at nine hundred and forty six passed and fifteen failed, every failure an assertion failure and every set standing inside its computed expectation on the single channel ground its edit moves one figure alone. The cure's tightness standing verified by the red team's computation, the twelve span literals standing equal to the spans computed from the head contract under the pins' own semantics, the ten cuts standing re-derived in both directions over every contract reading pin, passing whole at the base and falling on the form at the pin of their own round alone, and the shared span break attempts falling at every seat whose span they move and at no other. The form family standing carried at this closure on the write fault at the attempts the closure report records, the form gate, the ten cuts, the directions d1, d2, r1, rad1, and f57, the candidate legs A and B, and the fresh extraction gate standing unexecuted here as they stood at the build and the red team, their computed sets standing confirmed pin by pin at the red team's desk over every pin that reads the contract text, and the hosted gate on the closure head standing as the verdict of record in their place. The three records items of the Round 26 note's second section standing carried forward named with no cure charged, the Round 24 design note's fourth section carrying the false distribution pair as written, one at seven and fifty eight at five and more against the counted fourteen at seven and fifty nine at five and more, its contract places standing cured inside Round 24 by FIX-PG-20, and the Round 23 design note's fifth section carrying its day close compression as written, and the Round 22 design record's manifest figure of 73 standing against the counted 71 as a records reconciliation. The Round 26 design note's section 0 base tree string standing for the errata queue, 0b9e4eea against the true base tree 7842fa27. The Round 27 design record's items standing carried the same way, its note's fifth section naming a round seven count pin and a Round 27 in flight heading the base's setup file does not carry, and its design report's section figure of 7,635 standing against the contract side figure of 7,651. And FIX-PG-21 standing free and unspent through the round.
- Contract, PG-59 and PG-60 met, the two standing in full form in CONTRACT.md under the Round 29 obligations, PG-59 standing as the whole obligation assertion cure over the contract pin of Round 27 and PG-60 as the cut record of the Round 27 cohort beside it, its cuts standing executed whole at their computed seats under the warm up of Round 30, no cut passing whole on the form anywhere, the round standing closed under the rule of PG-60 on that execution. The slice standing as pull request 63 merged at 7a37a2c1 with main's tree standing verified equal to the repaired slice tree d9464174 and the census standing at 965. The spec review approving with findings at one Minor and two Observations, the Minor standing charged to the slice as the wording of the Round 28 hunt ledger carried over Round 29's own family in the standing and slice texts, and standing cured in the slice by the repair commit 4a9a7da3 restating the clause to carried unexecuted at exactly the four seats, two in this file and two in the setup file, the third seat in each file standing as the Round 28 line of PG-58, true of that round and not moved, and the two Observations standing against the design record and the build record with nothing further charged to the slice. The standards review approving with findings at one Minor, two Observations, and one further Minor, the Minor standing charged to the slice as the same wording at its seats and cured in the same repair, the two Observations standing against the design record, its fifth section carrying a false clause over the family it narrates and its seventh section phrasing the construction in five commits where the slice stands in three, and the further Minor standing against the build record, its eighth section's stage tree figures standing nowhere on the remote where the commits' own trees stand read in their place, every blob level claim of the build standing verified whole. The red team approving with findings at one Minor and two Observations, the Minor standing as the same wording at its seats and cured in the same repair, the first Observation standing charged to the slice at no action, that a subsection planted in the Round 27 section outside the two spans passes the restated pin whole, priced in the open, and the second Observation standing against the build record. CodeRabbit returning one inline comment on the slice, standing valid as the same Minor at its seat and cured in the same repair. The cure's tightness standing verified at the red team's desk, the two span literals standing byte-equal to the spans computed from the head contract at 3,479 characters for PG-55 and 2,681 for PG-56, the eight cuts standing re-derived in both directions name for name, and the candidate legs standing read on their whole evaluable surface. The warm up family standing at this closure with one leg executed and the rest carried, the Round 28 form gate standing executed on expectation at the closure's third seating, the whole suite on the Round 28 form tree at 963 passed and 0 failed with the format check and clippy each at exit 0 and no fault signature in its log, and the Round 29 trees standing extracted whole and verified at that seating after ten extraction runs across the closure's three seatings, the base tree and the form tree each at 232 of 232 files matching its remote listing blob for blob, the base tree's last four files standing landed by direct write after their writes refused across twenty one passes of the extractor, the remaining legs standing carried on the write fault, the rate limit stops the closure report records, and the runtime's kills of the hunt's processes mid compile on the contended box, the form gate of Round 29 and its base gate, the ten cuts of Round 28, the directions and the relocation and rad1 and f57 of that round, the eight cuts and directions of Round 29 on its base tree and the ten legs on its form tree including rad1 and f59, and the candidate legs A and B of both rounds standing unexecuted here as they stood at the build and the red team, their computed sets standing confirmed at the red team's desk, and the hosted gate on the closure head standing as the verdict of record in their place. The records items standing carried forward named with no cure charged, the Round 24 design note's fourth section carrying the false distribution pair as written, one at seven and fifty eight at five and more against the counted fourteen at seven and fifty nine at five and more, its contract places standing cured inside Round 24 by FIX-PG-20, the Round 23 design note's fifth section carrying its day close compression as written, and the Round 22 design record's manifest figure of 73 standing against the counted 71 as a records reconciliation. The Round 26 design note's section 0 base tree string standing for the errata queue, 0b9e4eea against the true base tree 7842fa27. The Round 27 design record's items standing carried the same way, its note's fifth section namings and its design report's section figure of 7,635 standing against the contract side figure of 7,651. The Round 28 design note's fourth section carrying its PG-58 opening sentence as written. The Round 28 red team's literal counts standing corrected on the record at eight for PG-55 and twelve for PG-56, the extraction standing at five content literals in each body, ten in the two bodies beside the three heading assertions, its sentence counts and its finding standing confirmed whole. And FIX-PG-21 standing free and unspent through the round.
- Contract, PG-61 and PG-62 met, the two standing in full form in CONTRACT.md under the Round 30 obligations, PG-61 standing as the section census cure over the obligations pins of Rounds 22 through 29 and PG-62 as the plant record of the guarded sections beside it, its record standing executed whole at its computed seats under the warm up of Round 30 and the completion execution of PG-64 in Round 31, no plant passing whole on the form inside the guarded sections anywhere, the round standing closed under the rule of PG-62 on that execution. The slice standing as pull request 68 merged at 8aa9462b with main's tree standing verified equal to the repaired slice tree 623cb7ec and the census standing at 967. The spec review approving the slice whole at no finding against it, one Minor and two Observations standing against the design record, the Minor standing as the closed count carried one round early at the slice's status step against the standing form the Round 29 record states, the defect it names standing cured in the slice by the repair the standards review charged, and the two Observations standing as the span figure of PG-61 and the assembled form total each standing one character high in the design desk's assembled copy against the note's printed section the contract step rightly took, and the usage sentence of the note's status section standing in no file at the base or at the head and carrying no seat in the family's status form, with nothing standing against the build record. The standards review approving with findings at one Minor, three Observations, and one further Observation, the Minor standing charged to the slice as the same count at its seats, the README's count line and twenty seven places in the setup file asserting the advanced count of closed rounds while Round 30 stood open, and standing cured in the slice by the repair commit a7ba5f44 restoring the standing count at the count line, at the twenty six carried assertions, and at the born status pin's assertion, the standing and slice lines standing untouched and the census unmoved, the three Observations standing against the design record, its status sections carrying the root wording of the count carry, its span figures standing one character high as the spec review records, and its construction section charging a comment line its helpers section nowhere prints, and the further Observation standing against the build record, its completion seating's sections standing out of numeric order in the banked report with its content standing complete and its cross references reading true. The red team approving with findings at no finding against the slice, two Minors standing against the design record, the span figures as the other axes record them and the Round 29 note's carried family restatement naming the cuts of Round 28 at the status pin of Round 22 and leaving the following rounds' pins unqualified where the Round 28 record prices the obligations pins, and two Majors and one Observation standing against the build record, the warm up's banked Round 28 form tree standing poisoned at its setup file, zeroed by the write fault's silent form after the Round 29 closure ran it whole, the warm up machinery as banked standing unable to complete a single falling leg, its seat table naming status pins for thirteen of the Round 28 family's legs against their records and its failing set reader leaving the separator standing on every failing name it records, and the banked staging transport pricing absence as refusal at twenty seconds a file, the tax that left the build's own warm up standing in its first leg's staging. CodeRabbit returning two inline comments on the slice, the first standing valid as the same Minor at the count line's seat and cured in the same repair, and the second, that the comment above the Round 27 pin should retain its PG-59 attribution beside the PG-61 one, standing dismissed, the comment naming the obligation whose restatement the pin currently stands under in the family's standing single line form, as at every seat at the base. The cure's tightness standing verified at the red team's desk, PG-61's figures standing confirmed by independent derivation at every priced seat, the sixteen whole form spans and the censuses of the eight guarded sections standing re-derived seat by seat, and the census assertions standing enumerated whole over the head tree at exactly their nine charged seats and nowhere else. The warm up family standing at this closure with the carried families of Rounds 28 and 29 standing executed whole on expectation at the red team's hunt and the plant record of this round standing executed in its base direction whole and in nine of its twenty form legs, the Round 28 family standing executed at the hunt's corrected tables on the repaired form tree, its form gate at 963 passed and 0 failed, its ten cuts each at the obligations pin of its own round at 962 passed and 1 failed, its directions at the Round 25 pin, its relocation at the pins of Rounds 25 and 26 together, rad1 passing whole, f57 at the Round 28 obligations pin, and its candidate legs at 961 passed and none failed and at 960 passed and 3 failed on the pins its record prices, the Round 29 family standing executed beside it, its base gate at 963 passed and 0 failed and its eight base legs on their computed splits, its form gate standing executed on its clean re-run at 965 passed and 0 failed after one fault straddled run standing as no evidence for its set, its eight form legs each at the Round 27 obligations pin, rad1 passing whole at 965, f59 at the Round 29 obligations pin, and its candidate legs at 963 passed and none failed and at 962 passed and 3 failed, the plant record of this round standing executed in its base direction, the eight plants at the ends of the sections of Rounds 22 through 29 and the cut of one sentence of the Round 29 surface paragraph each passing the whole suite at 965 passed and none failed, and in its form direction at the nine plants at the ends of the sections of Rounds 22 through 30, each falling at its own section's obligations pin alone at 966 passed and 1 failed, its remaining legs standing carried on the write fault at this closure, the plant before the first obligation of the Round 29 section standing fault straddled once with no test executed and no evidence read off it, and the plant between the second obligation and the surface, the plant inside the span of PG-60, the removal of the Round 29 fixture subsection, the rename of the Round 28 surface subheading, the relocation of the Round 28 fixture subsection into the Round 29 section, the promotion of the Round 29 fixture subsection, the plants at the ends of the sections of Rounds 21 and 12, the surface cut on the form, the control cut of PG-60, and the candidate legs A and B standing unexecuted with the hunt standing in flight on them in their charged order under the red team's driver on the shared ledger, its probe cycles standing against the shut window at this closure's construction, no leg of any family standing fallen anywhere but its computed seat at any execution in the record, and the hosted gate on the closure head standing as the verdict of record in their place. The records items standing carried forward named with no cure charged, the Round 29 design note's fifth section carrying its part executed clause over the cuts of PG-60 where nothing stood executed, the root wording of the Minor its reviews charged and its slice cured, its seventh section speaking of five commits where the slice as built stood in three, the Round 29 build report's three stage tree figures standing nowhere on the remote against the true stage trees, the Round 28 red team's literal counts of eight and twelve standing against the extracted five content literals in each body, the Round 30 design record's span figure of PG-61 at 2,841 characters and its form total at 433,775 standing against the measured 2,840 and 433,774, and the Round 29 note's carried family restatement standing as the red team prices it in its review. And FIX-PG-21 standing free and unspent through the round.
- Contract, PG-63 and PG-64 met, the two standing in full form in CONTRACT.md under the Round 31 obligations, PG-63 standing as the earned met forms of the standing lines of Rounds 29 and 30 and PG-64 as the completion record of the plant family of PG-62 beside it, its relocation leg standing executed at its corrected construction at its priced seat, no leg of the plant family standing carried, the round standing closed under the rule of PG-64 on that execution.
- Contract, PG-65 and PG-66 met, the two standing in full form in CONTRACT.md under the Round 32 obligations, PG-65 standing as the paragraph text extension over the guarded sections and PG-66 as the extension record of the guarded sections' texts beside it, its sixty five legs standing executed whole at their computed seats under the red team's execution after the slice landed, no cut of a guarded section's standing texts passing whole on the form anywhere, the round standing closed under the rule of PG-66 on that execution. The slice standing as pull request 76 merged at 8123f189 with main's tree standing verified equal to the slice tree a0c67f95 and the census standing at 971. The spec review approving the slice whole at no finding against it, one Minor standing against the design record, the note and its report stating the base commit at a hash that stands nowhere on the remote with two parents where the remote's main stands at the true base commit single parented on the Round 31 closure, the tree the note charges standing byte identical to the tree that main carries, with nothing standing against the build record. The standards review approving with findings at one Observation, the Observation standing priced in the open and not charged, the helper section_span repeating the opening shape of the standing helper subsection_span, the shared head standing as the seat PG-65's own text prices and the two helpers parting at the seat the extension exists for, one Minor standing against the design record at the same base commit seat the spec review records, with nothing standing against the build record. The red team approving the slice whole at no finding against it across its three seatings, its verdict standing seated at the third, the sixty five legs standing executed whole at their computed seats as the opening movement states, three Minors standing against the design record, the base commit's stated identity at the seat the other axes record, the thirty texts standing measured at 24,096 characters under the join measure at the base against the note's printed 24,097 with the fixture paragraph of Round 27 at 9,591 against its printed 9,592, the whole parting standing at the Round 31 fixture span at the file's end, and candidate leg B standing computed at 968 passed and 3 failed against its executed 967 passed and 4 failed, the fourth fall standing at the obligations pin of Round 31, the design's own priced position seat standing omitted from the computation, and one Minor standing against the build record, its fourth section claiming the join measure's exactness in sum at the base against its own fifth section's measure of 338 characters at that seat. CodeRabbit returning a summary comment on the slice stating no actionable comments, no inline comment standing on the pull request and no finding standing from it. The cure's tightness standing verified at the red team's desk, the thirty texts standing re-derived over the head contract seat by seat under the helper's join measure, the spans of the guarded sections standing re-derived under both measures, the discrimination standing re-summed at 65 legs with no dud, and the census standing recounted at 971 over the head tree. PG-66's paragraph standing restated to its executed form at this closure under the swap its own text states, the swap standing named in the closure's report, its whole span literal standing restated beside it in the born obligations pin. The two executions the verdict lane left carried standing carried again at this closure on the write fault, the whole run of the Round 27 fixture leg standing attempted four times at this seating in its own fresh target directories, every attempt standing refused at the build on the fault's signatures with no test standing executed and no assertion standing fallen, and its staging standing refused whole across this seating's copy passes, the hosted gate on the closure head standing as the verdict of record in their place. The records items standing carried forward named with no cure charged, the design note's eighth section carrying the setup obligations preamble standing read at 13 sentences under the design desk's grain against the 22 the Round 30 note states for that section's body, and the design record's and the build record's Minors standing as their reviews record them for the errata queue. And FIX-PG-21 standing free and unspent through the round.
- Contract, PG-67 and PG-68 met, the two standing in full form in CONTRACT.md under the Round 33 obligations, PG-67 standing as the guarded form cure over the sections of Rounds 13 through 21 and PG-68 as the cure record of those sections beside it, its one hundred and two legs standing executed in four of their seats at this closure and carried in the remaining ninety eight under the write fault, the carrying standing charged forward from this close exactly as the evidence leaves it, the round standing closed on the counted fault evidence with the hosted gate as the verdict of record for every carried leg. The slice standing as pull request 80 merged at a76ecf92 with main's tree standing verified equal to the repaired slice tree 8abe17cd and the census standing at 973. The spec review approving the slice whole at no finding against it, one Minor standing against the design record, the guarded cohort figure standing at twenty in the note's first and third sections against the twenty one the contract's own precedent seats fix, and one Minor standing against the build record, the warm up's carrying standing honestly named at every place it touches and discharged on the build report's counted fault evidence with the hosted gate as verdict of record. The standards review approving at no finding against the slice, the design record, or the build record, its reading of the CodeRabbit comment standing as its one dispute, the comment's proposed figure standing at no place in the slice's texts at its seat. The red team approving with one Major standing against the slice and the design record at the same seat, PG-67's closing sentence and the born pin's matching literal stating the cohort at twenty sections where PG-63 states ten at Round 31's close and PG-65 eleven at Round 32's close in the identical sentence form, the stating round's own section standing inside the count at both seats, so the figure stands checkable false from the contract's own text alone, the Major standing cured in the slice by the repair commit c7ab2b83 restating the one word in the contract and in the pin's literal together, and the cure standing re-verified entire at no residual finding, the figure standing at twenty one in both texts at the repaired head. CodeRabbit returning one inline comment on the slice, standing valid as the same figure at its seat and cured in the same repair, its style notes standing as no defect under the family's standing prose form and dismissed silently. The cure's tightness standing verified at the red team's desk, the witness form standing verified pin by pin over the nine restated pins and the born pin, the enumeration's figures standing re-derived whole, the eighteen obligations at 433 sentences and the twenty seven standing texts at 162 sentences and 37,070 characters, the position seat standing at its exact measure, and the discrimination standing re-derived by an independent mirror over the slice's own pins at one hundred and eight evaluations with no deviation. PG-68's paragraph standing in its carried form whole at this closure, no swap standing made, its text and its whole span literal standing as the slice left them, the execution and the swap standing charged forward beside the legs themselves. The executions standing run at this closure in the design record's order, the fault free whole run of the Round 27 fixture leg standing executed at its computed seat of 970 passed and one failed after its carrying across two closures, the fresh extraction gate standing at 971 passed and none failed over the base tree, the base gate and the form gate standing whole at 971 passed and none failed and at 973 passed and none failed, the cuts of PG-27, of PG-28, and of the preamble in the Round 13 section standing executed whole on the base at 971 passed and none failed each, and the cut of PG-27 in the Round 13 section standing executed on the form at 972 passed and one failed at the obligations pin of Round 13 alone, the remaining ninety eight legs of the record standing carried named at their counted attempts in the closure's report beside the two candidate legs and the fresh extraction gate over the slice head standing carried at five counted attempts each, no leg standing fallen anywhere but its computed seat at any execution of this closure and the stopping rule of PG-68 standing unfired. The records items standing carried forward named with no cure charged, the design note's first and third sections and the design report's fourth section carrying the cohort figure of twenty as written, the shipped seats standing cured by the repair, and the Round 32 design record's and build record's items standing in the errata queue as their reviews record them. And FIX-PG-21 standing free and unspent through the round.
- Slice 24 has landed. It carries the midhinge reading that reads a meet mark by mark over the one record progression of the meet report, naming the holder after each mark, the record figure after it, the midhinge standing held after it by the field behind its holder, the mean of the lower quartile standing and the upper quartile standing of that field, and the midhinge margin by which the record stands ahead of that standing, and the midhinge command, with the golden midhinge readings of the standing example meet and of a constructed meet under examples, and the multi day walk extended in its own pass to record the midhinge standing after each mark, and the seeded generated fixture family extended to the midhinge reading, and the README midhinge section.
- Slice 25 has landed. It carries the midrange reading that reads a meet mark by mark over the one record progression of the meet report, naming the holder after each mark, the record figure after it, the midrange standing held after it by the field behind its holder, the mean of the nearest standing and the farthest standing of that field, and the midrange margin by which the record stands ahead of that standing, and the midrange command, with the golden midrange readings of the standing example meet and of a constructed meet under examples, and the multi day walk extended in its own pass to record the midrange standing after each mark, and the seeded generated fixture family extended to the midrange reading, and the README midrange section.
- Slice 26 has landed. It carries the width reading that reads a meet mark by mark over the one record progression of the meet report, naming the holder after each mark, the record figure after it, and the field width held after it by the field behind its holder, the farthest standing less the nearest standing of that field, and the width command, with the golden width readings of the standing example meet and of a constructed meet under examples, and the multi day walk extended in its own pass to record the field width after each mark, and the seeded generated fixture family extended to the width reading, and the README width section.
- Slice 27 has landed. It carries the separating meet of PG-53, a hand built meet whose third mark holds the five figure field that separates the width reading's two partners, the range standing at the field width at every mark of the meet and the difference of the tails standing below it at the third, and the coincidence pin of PG-54 over the generated fixture family, with no reading and no command added.
- Slice 28 has landed. It carries the execution record of PG-55, the four mutations of the Round 25 family standing executed against the width file's pins with the header's stated splits for the separating meet pin and the separation relation pin confirmed whole and no parting standing, and the hunt record of PG-56 over the width reading's neighbourhood, five fresh edits falling and none passing whole, with no reading and no command added.
- Slice 29 has landed. It carries the whole obligation assertion cure of PG-57, the contract pins of Rounds 22 through 26 standing restated to assert the whole text of their two obligations each, and the hunt record of PG-58 over the field selections standing part executed and part carried under the write fault, with no reading and no command added.
- Slice 30 has landed. It carries the whole obligation assertion cure of PG-59, the contract pin of Round 27 standing restated to assert the whole text of its two obligations, and the cut record of PG-60 over the Round 27 cohort standing carried unexecuted under the write fault, with no reading and no command added.
- Slice 31 has landed. It carries the section census cure over the obligations pins of Rounds 22 through 29, each pin asserting the standing census of its round's section beside every assertion it already carried, and the plant record of the guarded sections standing carried under the write fault, with no reading and no command added.
- Slice 32 has landed. It carries the earned met forms of PG-63, the standing lines of Rounds 29 and 30 standing restated to the standings their executions earned, and the completion record of PG-64 over the plant family of PG-62 standing executed whole at its corrected construction, with no reading and no command added.
- Slice 33 has landed. It carries the paragraph text extension of PG-65, the obligations pins of Rounds 22 through 31 standing restated to assert their sections' standing texts whole, and the extension record of PG-66 over the guarded sections' texts standing executed whole at its computed seats, with no reading and no command added.
- Slice 34 has landed. It carries the guarded form cure of PG-67, the obligations pins of Rounds 13 through 21 standing restated to assert their sections whole, and the cure record of PG-68 over those sections standing executed in four of its one hundred and two legs and carried in the rest, with no reading and no command added.
- The Rust port has landed. It carries the behavior the Python package carried, checked against the same goldens, and the Python tree stands removed under PORT-10.
- Local gate, cargo test, cargo fmt --check, and cargo clippy --all-targets -- -D warnings. CI runs the same commands.

## Licence

MIT. See LICENSE.
