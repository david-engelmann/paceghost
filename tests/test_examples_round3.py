"""Pins for the synthetic Round 3 example under examples.

The example field day holds four runs in one window, three of them
carrying timed reads and one carrying none, and the operator run
loses the record to the night fork partway through the window. The
stored golden report has to stay byte identical to what the code
renders today, and to what the fieldday command prints.
"""

import pathlib

from paceghost.cli import main
from paceghost.fieldday import field_day_report, load_field_day_file

EXAMPLES = pathlib.Path(__file__).resolve().parent.parent / "examples"


def test_the_example_field_day_loads_clean_with_four_runs():
    day, defects = load_field_day_file(EXAMPLES / "field-day.json")
    assert defects == []
    assert day["window_seconds"] == 1800
    assert day["operator"] == "operator"
    assert [run["runner"] for run in day["runs"]] == [
        "operator",
        "night fork",
        "dawn fork",
        "quiet fork",
    ]


def test_the_example_carries_a_run_with_no_timed_reads():
    day, defects = load_field_day_file(EXAMPLES / "field-day.json")
    assert defects == []
    quiet = day["runs"][3]
    assert quiet["runner"] == "quiet fork"
    assert "reads" not in quiet


def test_the_golden_field_day_report_is_byte_identical():
    day, defects = load_field_day_file(EXAMPLES / "field-day.json")
    assert defects == []
    rendered = ("\n".join(field_day_report(day)) + "\n").encode()
    golden = (EXAMPLES / "field-day-golden.txt").read_bytes()
    assert rendered == golden


def test_the_fieldday_command_prints_the_golden_report(capsys):
    path = str(EXAMPLES / "field-day.json")
    assert main(["fieldday", path]) == 0
    golden = (EXAMPLES / "field-day-golden.txt").read_text(encoding="utf-8")
    assert capsys.readouterr().out == golden


def test_the_example_operator_loses_the_record_partway_through():
    day, defects = load_field_day_file(EXAMPLES / "field-day.json")
    assert defects == []
    lines = field_day_report(day)
    assert lines[1] == "record at 300 seconds, operator at a projection of 1260"
    assert lines[-1] == (
        "record stolen from operator at 900 seconds by night fork, "
        "margin 300 at the close"
    )
