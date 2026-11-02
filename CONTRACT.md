# paceghost Contract

This Contract is the executable spec for paceghost. Every obligation is checked by a named test or by a named check. The Contract is reported as met or not met in the README programme status.

## Setup obligations

- SETUP-1. The repo carries AGENTS.md with the canonical template verbatim
  above its marked line and the repo annex below it. Test,
  tests/test_setup.py.
- SETUP-2. The README programme status states the closed round count out
  of 100 and reports this Contract as met or not met. Test,
  tests/test_setup.py.
- SETUP-3. The codepoint scan at scripts/codepoint_scan.py counts U+2014
  and U+2013 in every Markdown and text file in the repo and requires
  zero of each. The scan runs in the local gate and in CI.
- SETUP-4. Every fixture and example committed to this repo is synthetic
  and written for this repo. Nothing real is ever committed.

## First slice obligations, stub form

The full Contract for the first slice is the first build act of this repo. These stubs state the obligations the first slice will be held to, in the form they will be extended.

- PG-1. A recorded run is a JSON object holding a task label, a window length in seconds, and an ordered list of splits, each split holding a phase name and the elapsed seconds at which that phase completed. A loader reads a recorded run from disk and names every malformed field instead of raising. Test, tests/test_setup.py in setup form and the loader tests in the first slice.
- PG-2. The ghost for a task is the recorded run of that task with the lowest total elapsed time, and ties break toward the earliest recorded run. Test, the first slice tests.
- PG-3. A race compares a live run against the ghost split by split and reports, at every split, whether the live run is ahead or behind and by how many seconds. Test, the first slice tests.
- PG-4. When any recorded run beats the standing ghost for its task, that run becomes the ghost, whoever ran it, and the change of ghost is reported with both totals. Test, the first slice tests.
