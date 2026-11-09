# paceghost

Paceghost is a command line tool that reads recorded task runs and field days from local files and prints reports over them.

A recorded run names a task and a runner and carries a usage window, a total time, and timed splits for its phases. A field day is a set of recorded runs inside one window, and a meet is three or more field days read as one record progression. The tool runs nothing and schedules nothing. Every report is computed from recordings that already exist on disk, and the same input files always print the same reports.

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

## Install

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

## Status

Seeded in October 2026. Setup has landed, and Rounds 1 to 14 have closed, with all slices merged on main. Part of the Spectacle line in the 39 repo storefront perfection programme.

## Programme status

- Closed perfection rounds, 14 of 100. A round counts only when a full cycle of work, audit, review, red team, and approval has closed. Seeding and setup are not rounds.
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
- The Rust port has landed. It carries the behavior the Python package carried, checked against the same goldens, and the Python tree stands removed under PORT-10.
- Local gate, cargo test, cargo fmt --check, and cargo clippy --all-targets -- -D warnings. CI runs the same commands.

## Licence

MIT. See LICENSE.
