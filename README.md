# paceghost

Your usage window is a game clock, your best previous run is the ghost, and your own fleet keeps stealing your record.

## What this is

Racing games solved motivation decades ago. You do not race strangers. You race the ghost of your own best lap. Paceghost brings the ghost to agent work. Every task run records its splits, the phases timed as they complete, and your best comparable run drives beside you as a translucent pace line. Ahead by four minutes. Behind by two. The twist is that the ghost does not stay yours. Background forks from your own fleet run similar tasks while you are in meetings, and when one of them beats the record, it becomes the ghost. Tomorrow you race your own fleet's best work.

## Status

Seeded in October 2026. Setup has landed and the build has not started. Part of the Spectacle line in the 39 repo storefront perfection programme.

## Programme status

- Closed perfection rounds, 0 of 100. A round counts only when a full cycle of work, audit, review, red team, and approval has closed. Seeding and setup are not rounds.
- Contract, SETUP-1 to SETUP-4 met by the setup slice. The first slice obligations PG-1 to PG-4 stand in stub form in CONTRACT.md and are not yet met.
- The first slice is the window clock plus a ghost replayed from recorded runs stored locally, one past window raced end to end.
- Local gate, python3 scripts/codepoint_scan.py and python3 -m pytest tests -q. CI runs the same commands.

## Licence

MIT. See LICENSE.
