# paceghost

## What this is

Paceghost is a command line tool that reads recorded task runs from local files and prints reports about them. A recorded run names a task and a runner and carries a usage window, a total time, and timed splits for its phases. The tool runs nothing and schedules nothing. Every report is computed from recordings that already exist on disk, and the same input files always print the same reports.

A file that does not match the recorded shape is refused. The command prints the defects the loader found, naming each malformed field, and exits with status one.

The show command prints one recorded run. The ghost command prints the standing ghost for a task, the recorded run with the lowest total. The race command prints one recorded run against a ghost run, split by split and at the finish. The season command prints a report over a field of recorded runs grouped by model and read against a ghost record, and the board command prints the board for one recorded run that carries timed reads. The fieldday command prints the record progression of the recorded runs of one field day inside one window, and the ladder command prints one field day read through a list of windows. The rematch command prints two field days as one record progression, and the meet command prints three or more field days as one record progression. The table command prints the meet read competitor by competitor, naming the lowest standing projection of each competitor, the count of marks each competitor held the record after, and the standing projection at the close.

## Status

Seeded in October 2026. Setup has landed, and Rounds 1 to 8 have closed, with all slices merged on main. Part of the Spectacle line in the 39 repo storefront perfection programme.

## Programme status

- Closed perfection rounds, 8 of 100. A round counts only when a full cycle of work, audit, review, red team, and approval has closed. Seeding and setup are not rounds.
- Contract, SETUP-1 to SETUP-4 and PG-1 to PG-8 met. PG-1 to PG-4 stand in full form in CONTRACT.md with the Round 1 repair amendments, and Round 1 closed on the merged slice 2 after three reviews, a repair pass, and an approving re-review. PG-5 and PG-6 stand in full form with the Round 2 repair amendment.
- Contract, PG-7 and PG-8 met. They stand in full form in CONTRACT.md, and Round 3 closed on the merged slice 4 after three reviews, a repair pass, and an approving re-review.
- Contract, PG-9 and PG-10 met. They stand in full form in CONTRACT.md with the Round 4 repair amendments, and Round 4 closed on the merged slice 5 after three reviews, a repair pass, and an approving re-review.
- Contract, PG-11 and PG-12 met. They stand in full form in CONTRACT.md with the Round 5 repair amendment, and Round 5 closed on the merged slice 6 after three reviews, a repair pass, and an approving re-review.
- Contract, PG-13 and PG-14 met. They stand in full form in CONTRACT.md, and Round 6 closed on the merged slice 7 after three reviews, a repair pass, and an approving re-review.
- Contract, PG-15 and PG-16 met. They stand in full form in CONTRACT.md, and Round 7 closed on the merged slice 8 after three reviews approving with no findings.
- Contract, PG-17 and PG-18 met. They stand in full form in CONTRACT.md, and Round 8 closed on the merged slice 9 after three reviews, no repair pass, and the red team condition discharged on executed probes.
- Slice 2 has landed. It carries the recorded run loader that names every malformed field, the ghost rule, the race report, the ghost change report, and the show, ghost, and race commands, with two synthetic example runs and a golden race report under examples.
- The Round 2 slice has landed. It carries the season report over a recorded field of runs grouped by model and the live board over recorded timed reads, with the season and board commands, a synthetic example field, a synthetic ghost record carrying a usage figure, a synthetic example run carrying timed reads, and golden renderings under examples. PG-5 and PG-6 stand in full form in CONTRACT.md with the Round 2 repair amendment, and Round 2 closed on the merged slice after three reviews, a repair pass, a micro repair, and re-reviews approving the final head. The board reads a finished recording, and watching a real run while it runs remains future work.
- Slice 4 has landed. It carries the field day loader that names every malformed field, the stolen record report that walks the record progression of one window across a field of recorded runs, and the fieldday command, with a synthetic example field day and its golden report under examples.
- Slice 5 has landed. It carries the ladder loader that names every malformed field in a ladder file, the ladder report that reads one field day as it stood at each window in a list and closes with a summary line over the closing holders, and the ladder command, with a synthetic example ladder and its golden report under examples.
- Slice 6 has landed. It carries the rematch loader that names every malformed field in a rematch file, the rematch report that reads two field days as one record progression and closes by naming where the record changed hands between the days, and the rematch command, with two synthetic example rematches and their golden reports under examples.
- Slice 7 has landed. It carries the meet loader that names every malformed field in a meet file, the meet report that reads three or more field days as one record progression and names the closing holder of each day and the day the standing record was set, and the meet command, with a synthetic example meet and its golden report under examples.
- Slice 8 has landed. It carries the meet table that reads a meet competitor by competitor over the one record progression of the meet report, naming the lowest standing projection of each competitor and the day and mark at which it was set, the count of marks each competitor held the record after, and their standing projection at the close, and the table command, with the golden meet table of the standing example meet under examples.
- Slice 9 has landed. It carries the reign reading that reads a meet spell by spell over the one record progression of the meet report, naming each maximal run of consecutive marks one competitor held the record after with the marks it spans and its length, and the count of marks and spells and the longest spell at the close, and the reign command, with the golden reign reading of the standing example meet under examples.
- The Rust port has landed. It carries the behavior the Python package carried, checked against the same goldens, and the Python tree stands removed under PORT-10.
- Local gate, cargo test, cargo fmt --check, and cargo clippy --all-targets -- -D warnings. CI runs the same commands.

## Licence

MIT. See LICENSE.
