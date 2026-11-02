"""Pins for the synthetic Round 2 examples under examples.

The example field holds seven runs across three models against the
example ghost record, and the stored season report and board have
to stay byte identical to what the code renders today.
"""

import pathlib

from paceghost.runs import load_run_file
from paceghost.season import board_report, load_field_file, season_report

EXAMPLES = pathlib.Path(__file__).resolve().parent.parent / "examples"


def load_ghost():
    ghost, defects = load_run_file(EXAMPLES / "ghost-run.json")
    assert defects == []
    return ghost


def test_the_example_field_loads_clean_across_three_models():
    runs, defects = load_field_file(EXAMPLES / "field.json")
    assert defects == []
    assert len(runs) == 7
    assert {run["model"] for run in runs} == {"atlas", "birch", "cedar"}


def test_the_example_ghost_carries_a_usage_figure():
    ghost = load_ghost()
    assert ghost["usage"] == 1180
    assert ghost["total_seconds"] == 1320


def test_the_golden_season_report_is_byte_identical():
    runs, defects = load_field_file(EXAMPLES / "field.json")
    assert defects == []
    rendered = ("\n".join(season_report(runs, load_ghost())) + "\n").encode()
    golden = (EXAMPLES / "season-golden.txt").read_bytes()
    assert rendered == golden


def test_the_example_board_run_loads_clean_with_a_zero_read():
    run, defects = load_run_file(EXAMPLES / "board-run.json")
    assert defects == []
    assert len(run["reads"]) == 4
    assert run["reads"][0]["elapsed_seconds"] == 0


def test_the_golden_board_is_byte_identical():
    run, defects = load_run_file(EXAMPLES / "board-run.json")
    assert defects == []
    rendered = ("\n".join(board_report(run, load_ghost())) + "\n").encode()
    golden = (EXAMPLES / "board-golden.txt").read_bytes()
    assert rendered == golden
