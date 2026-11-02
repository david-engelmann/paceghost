# paceghost

Your usage window is a game clock, your best previous run is the ghost, and your own fleet keeps stealing your record.

## What this is

Racing games solved motivation decades ago. You do not race strangers. You race the ghost of your own best lap. Paceghost brings the ghost to agent work. Every task run records its splits, the phases timed as they complete, and your best comparable run drives beside you as a translucent pace line. Ahead by four minutes. Behind by two. The twist is that the ghost does not stay yours. Background forks from your own fleet run similar tasks while you are in meetings, and when one of them beats the record, it becomes the ghost. Tomorrow you race your own fleet's best work.

## Status

Seeded in October 2026. Setup has landed, and Rounds 1 and 2 have closed, with both slices merged on main. Part of the Spectacle line in the 39 repo storefront perfection programme.

## Programme status

- Closed perfection rounds, 2 of 100. A round counts only when a full cycle of work, audit, review, red team, and approval has closed. Seeding and setup are not rounds.
- Contract, SETUP-1 to SETUP-4 and PG-1 to PG-6 met. PG-1 to PG-4 stand in full form in CONTRACT.md with the Round 1 repair amendments, and Round 1 closed on the merged slice 2 after three reviews, a repair pass, and an approving re-review. PG-5 and PG-6 stand in full form with the Round 2 repair amendment.
- Slice 2 has landed. It carries the recorded run loader that names every malformed field, the ghost rule, the race report, the ghost change report, and the show, ghost, and race commands, with two synthetic example runs and a golden race report under examples.
- The Round 2 slice has landed. It carries the season report over a recorded field of runs grouped by model and the live board over recorded timed reads, with the season and board commands, a synthetic example field, a synthetic ghost record carrying a usage figure, a synthetic example run carrying timed reads, and golden renderings under examples. PG-5 and PG-6 stand in full form in CONTRACT.md with the Round 2 repair amendment, and Round 2 closed on the merged slice after three reviews, a repair pass, a micro repair, and re-reviews approving the final head. The board reads a finished recording, and watching a real run while it runs remains future work.
- Local gate, python3 scripts/codepoint_scan.py and python3 -m pytest tests -q. CI runs the same commands.

## Licence

MIT. See LICENSE.
