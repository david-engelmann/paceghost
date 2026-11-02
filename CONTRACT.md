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
  and written for this repo. Nothing real is ever committed. Check, a
  manual sanitization review of every added fixture and example, with
  its result recorded in the gate report.

## First slice obligations, stub form

The full Contract for the first slice is the first build act of this repo. These stubs state the obligations the first slice will be held to, in the form they will be extended.

- PG-1. A recorded run is a JSON object holding a task label, a window length in seconds, and an ordered list of splits, each split holding a phase name and the elapsed seconds at which that phase completed. A loader reads a recorded run from disk and names every malformed field instead of raising. Test, tests/test_setup.py in setup form and the loader tests in the first slice.
- PG-2. The ghost for a task is the recorded run of that task with the lowest total elapsed time, and ties break toward the earliest recorded run. Test, the first slice tests.
- PG-3. A race compares a live run against the ghost split by split and reports, at every split, whether the live run is ahead or behind and by how many seconds. Test, the first slice tests.
- PG-4. When any recorded run beats the standing ghost for its task, that run becomes the ghost, whoever ran it, and the change of ghost is reported with both totals. Test, the first slice tests.

## First slice obligations, full form

This section extends the stub section above, which stays standing as the record of the form these obligations started in. The full form below is the text slice 2 is held to. Where the two disagree, the full form governs.

### The recorded run shape

A recorded run is one JSON object with the fields named here. The field task is a non empty string. The field window_seconds is a positive number. The field runner is a non empty string naming who or what ran the run. The field recorded_at is a string holding an ISO calendar date. The field splits is a non empty list of split objects, and each split object holds a phase field with a non empty string and an elapsed_seconds field with a number. Elapsed values run strictly increasing down the list, the first elapsed value sits at or after zero, and no elapsed value is negative. The total of a run is the elapsed_seconds of its final split, and the loader reports that total as the total_seconds field of the run it returns. A file may carry its own total_seconds field, and when it does, that value must be a number and must equal the final split. A JSON true or false is not a number anywhere in this contract. A number must be finite, so a NaN or an infinity read from a file is a defect and never a value. Fields the contract does not name are ignored. Test, tests/test_runs.py.

### PG-1 full form, the loader names every malformed field and never raises

The loader entry points are load_run_text and load_run_file in the paceghost.runs module. Each returns a pair of the run and the list of defects it found. A clean input returns the run and an empty defect list. A malformed input returns no run and a defect list naming every malformed field the loader could reach, and neither entry point ever raises on malformed content. A file that cannot be read returns the single defect named file cannot be read. The defect vocabulary is fixed, and a test pins each phrase. Text that is not JSON at all draws the defect named input is not valid JSON. JSON that parses to something other than an object draws the defect named input is not a JSON object. An absent field draws missing field and the field name. A field of the wrong kind draws field and the field name and has the wrong type. An empty task or runner draws field and the field name and is empty. A window that is zero or below zero draws field window_seconds must be positive. A recorded_at string that is not an ISO calendar date draws field recorded_at is not an ISO date. A number that is not finite draws field and the field name and is not a finite number. An empty splits list draws field splits is empty. A split that is not an object draws split and its index and is not an object, and split indexes count from zero. A split missing a field draws split and its index and is missing field and the field name. A split field of the wrong kind draws split and its index and field and the field name and has the wrong type. An empty split phase draws split and its index and field phase is empty. A negative elapsed value draws split and its index and field elapsed_seconds is negative. An elapsed value that fails to beat the previous valid elapsed value draws split and its index and field elapsed_seconds is not greater than the previous split. A carried total_seconds that disagrees with the final split draws field total_seconds does not equal the final split. One malformed input can draw several defects in one pass, and the loader keeps naming fields after the first defect it finds. Test, tests/test_runs.py.

### PG-2 full form, the ghost is the lowest total with the earliest date on a tie

The select_ghost function in the paceghost.runs module takes the valid runs of one task and returns the ghost. The ghost is the run with the lowest total. When two or more runs share the lowest total, the ghost is the one with the earliest recorded_at date. When the totals and the dates both tie, the ghost is the run that came first in the input order. An empty input returns no ghost. Test, tests/test_runs.py.

### PG-3 full form, a race reports every split and the finish

The race_report function in the paceghost.runs module takes a ghost run and a live run and returns one line per split plus a final line, and the report is a pure function of the two runs. Splits pair by position. A paired line names the phase and reports the live run as ahead by the exact difference in seconds when the live elapsed value is the lower one, behind by the exact difference when it is the higher one, and level when the two values are equal. When the two phase names at one position differ, the line for that position names both phases and reports no difference for that split. When the two runs carry different counts of splits, only the shared positions pair, and one extra line reports that the split counts differ and gives both counts, placed before the final line. The final line reports the finish as ahead, behind, or level, with the exact difference in seconds taken from the two totals. Test, tests/test_runs.py, and the golden race report held in tests/test_examples.py.

### PG-4 full form, a faster run steals the ghost and the change is reported with both totals

The ghost_change function in the paceghost.runs module takes the standing ghost and a candidate run and returns whether the candidate takes the ghost, together with one report line. The candidate takes the ghost exactly when it would be picked as the ghost from the pair under the PG-2 rule, so a lower total always steals, an equal total steals only with an earlier recorded_at date, and a higher total never steals. The report line names both runners and both totals, whether the ghost changed hands or stayed. Test, tests/test_runs.py.

### The command line surface

The paceghost.cli module provides a main function and a console script named paceghost with three commands. The show command takes one file and prints a summary of the run it holds, or prints its defects and exits with status one when the run is malformed. The ghost command takes one or more files holding runs of one task and prints the standing ghost, and it exits with status one when any file is malformed or the files hold runs of more than one task. The race command takes a ghost file and a live file and prints the race report, and it exits with status one when either file is malformed or the two runs hold different tasks. Every printed report is deterministic, so the same inputs always print the same bytes. Test, tests/test_cli.py.

## Round 1 repair amendments

This section records the repairs the Round 1 reviews required. Each amendment names the obligation it amends and the finding it closes. Every earlier text stays standing, and where an amendment and an earlier text disagree, the amendment governs.

### FIX-PG-1, amending PG-1 in full form, closing red team finding F1 and spec finding M-1

A number too large to convert to a finite float, such as an integer literal with hundreds of digits, counts as not a finite number under the recorded run shape, and it draws the not a finite number defect for its field at the window, at a split elapsed value, and at a carried total alike. The promise that neither loader entry point ever raises also covers input nested too deeply for the JSON parser to read, and such input draws the defect named input is not valid JSON. Test, tests/test_runs.py.

### FIX-PG-2, amending PG-1 in full form, closing red team finding F3

A text field holding an unpaired surrogate is malformed, because such a string cannot be printed or written as UTF-8 text. A task or a runner holding an unpaired surrogate draws field and the field name and holds an unpaired surrogate. A split phase holding an unpaired surrogate draws split and the split index and field phase holds an unpaired surrogate. Test, tests/test_runs.py.

### FIX-PG-3, amending PG-3 in full form and the command line surface, closing spec finding M-2

The window clock now carries behaviour. The race report carries one window line for the live run, standing after the split lines and after the split counts line when one is present, and before the finish line. When the live total is at or under the live window, the window line says the live run finished inside its window and names the seconds to spare. When the live total is over the live window, the window line says the live run ran past its window and names the overrun in seconds. The summary printed by the show command carries the same window line for its run as its last line. Test, tests/test_runs.py and tests/test_cli.py.

### FIX-PG-4, amending the recorded run shape, closing red team finding F5 and spec finding m-3

The recorded_at field accepts only the extended calendar form, four digits for the year, a dash, two digits for the month, a dash, and two digits for the day, and the date it names must be a real date. The compact form without dashes and the ISO week form are not calendar dates under this contract, and each draws the defect named field recorded_at is not an ISO date. Test, tests/test_runs.py.

### FIX-PG-5, amending PG-3 in full form, closing red team finding F6 and spec finding m-1

A difference in the race report renders exactly at up to six decimal places. A nonzero difference smaller than that rendering precision renders at full precision instead, so a line that reports the live run ahead or behind never prints its difference as zero. Test, tests/test_runs.py.

### FIX-PG-6, amending SETUP-3, closing red team finding F4 and spec finding m-4

The scan counts U+2014 and U+2013 in every file whose suffix is .md, .txt, .py, .json, .toml, .yml, or .yaml, and in every extensionless file, and it skips the files inside the ignored directories the scan names. A test runs the scan against a planted tree and pins a counted file and a skipped file. Test, tests/test_scan.py. The action pins in the CI workflow and the setuptools floor and licence restatement in pyproject.toml, which landed on this branch in the step 0 repair commit, are recorded here as step 0 repairs to the setup gate, and no PG obligation rests on them.
