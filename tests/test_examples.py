"""Pins for the synthetic examples under examples.

The two example runs are the slice demo. The faster night fork run has
to steal the ghost from the operator run, and the stored race report
has to stay byte identical to the report the code renders today.
"""

import pathlib

from paceghost.runs import ghost_change, load_run_file, race_report, select_ghost

EXAMPLES = pathlib.Path(__file__).resolve().parent.parent / "examples"


def load_example(name):
    run, defects = load_run_file(EXAMPLES / name)
    assert defects == []
    return run


def test_both_example_runs_load_clean():
    operator = load_example("operator-run.json")
    night_fork = load_example("night-fork-run.json")
    assert operator["runner"] == "operator"
    assert operator["total_seconds"] == 1500
    assert night_fork["runner"] == "night fork"
    assert night_fork["total_seconds"] == 1320
    assert operator["task"] == night_fork["task"]


def test_the_example_night_fork_steals_the_ghost():
    operator = load_example("operator-run.json")
    night_fork = load_example("night-fork-run.json")
    assert select_ghost([operator, night_fork]) is night_fork
    steals, line = ghost_change(operator, night_fork)
    assert steals is True
    assert line == (
        "ghost changes from operator at 1500 seconds "
        "to night fork at 1320 seconds"
    )


def test_the_golden_race_report_is_byte_identical():
    operator = load_example("operator-run.json")
    night_fork = load_example("night-fork-run.json")
    rendered = ("\n".join(race_report(operator, night_fork)) + "\n").encode()
    golden = (EXAMPLES / "race-golden.txt").read_bytes()
    assert rendered == golden
