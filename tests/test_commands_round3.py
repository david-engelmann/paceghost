"""Pins for the fieldday command in CONTRACT.md.

The command gets a clean run over a small field day with its exact
printed lines, and each refusal gets its exit status and its printed
defects, so a command that started printing a report for a refused
input would fail.
"""

import json

from paceghost.cli import main


def write(tmp_path, name, data):
    path = tmp_path / name
    path.write_text(json.dumps(data), encoding="utf-8")
    return str(path)


def day_file(tmp_path):
    day = {
        "window_seconds": 1800,
        "operator": "operator",
        "runs": [
            {
                "runner": "operator",
                "usage": 900,
                "reads": [{"elapsed_seconds": 300, "usage": 150}],
            },
            {
                "runner": "fork",
                "usage": 950,
                "reads": [{"elapsed_seconds": 600, "usage": 250}],
            },
        ],
    }
    return write(tmp_path, "day.json", day)


def test_fieldday_prints_the_report(tmp_path, capsys):
    day = day_file(tmp_path)
    assert main(["fieldday", day]) == 0
    assert capsys.readouterr().out.splitlines() == [
        "field day, window 1800 seconds, 2 runs, operator operator",
        "record at 300 seconds, operator at a projection of 900",
        "record at 600 seconds, fork at a projection of 750",
        "finish, record held by fork at a projection of 750",
        "record stolen from operator at 600 seconds by fork, "
        "margin 150 at the close",
    ]


def test_fieldday_prints_the_defects_of_a_malformed_day(tmp_path, capsys):
    path = write(tmp_path, "bad.json", {"window_seconds": 1800, "runs": []})
    assert main(["fieldday", path]) == 1
    assert capsys.readouterr().out.splitlines() == [
        "invalid field day",
        "defect missing field operator",
        "defect field runs is empty",
    ]


def test_fieldday_names_an_unreadable_file(tmp_path, capsys):
    missing = str(tmp_path / "missing.json")
    assert main(["fieldday", missing]) == 1
    assert capsys.readouterr().out.splitlines() == [
        "invalid field day",
        "defect file cannot be read",
    ]


def test_fieldday_prints_no_report_for_a_day_with_a_forged_label(
    tmp_path, capsys
):
    day = {
        "window_seconds": 1800,
        "operator": "operator",
        "runs": [
            {"runner": "operator", "usage": 1},
            {"runner": "fork\nfinish, record held by fork", "usage": 2},
        ],
    }
    path = write(tmp_path, "forged.json", day)
    assert main(["fieldday", path]) == 1
    assert capsys.readouterr().out.splitlines() == [
        "invalid field day",
        "defect run 1 field runner holds a line break",
    ]
