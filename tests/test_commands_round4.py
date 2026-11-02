"""Tests for the Round 4 command line surface, the ladder command."""

import json

from paceghost.cli import main


def ladder_day(**overrides):
    day = {
        "window_seconds": 1800,
        "operator": "operator",
        "runs": [
            {
                "runner": "operator",
                "usage": 900,
                "reads": [{"elapsed_seconds": 300, "usage": 210}],
            },
            {
                "runner": "fork",
                "usage": 950,
                "reads": [{"elapsed_seconds": 600, "usage": 200}],
            },
        ],
    }
    day.update(overrides)
    return day


def write_ladder(tmp_path, windows=(300, 600), day=None, name="ladder.json"):
    path = tmp_path / name
    ladder = {
        "day": ladder_day() if day is None else day,
        "windows": list(windows),
    }
    path.write_text(json.dumps(ladder), encoding="utf-8")
    return str(path)


def test_ladder_prints_the_report(capsys, tmp_path):
    path = write_ladder(tmp_path)
    assert main(["ladder", path]) == 0
    assert capsys.readouterr().out.splitlines() == [
        "field day, window 300 seconds, 2 runs, operator operator",
        "record at 300 seconds, operator at a projection of 210",
        "finish, record held by operator at a projection of 210",
        "record kept by operator to the close",
        "field day, window 600 seconds, 2 runs, operator operator",
        "record at 300 seconds, operator at a projection of 420",
        "record at 600 seconds, fork at a projection of 200",
        "finish, record held by fork at a projection of 200",
        "record stolen from operator at 600 seconds by fork, "
        "margin 220 at the close",
        "ladder, the closing holder differs from the first window "
        "at 600 seconds",
    ]


def test_ladder_prints_the_defects_of_a_malformed_ladder(capsys, tmp_path):
    path = write_ladder(tmp_path, windows=[])
    assert main(["ladder", path]) == 1
    assert capsys.readouterr().out.splitlines() == [
        "invalid ladder",
        "defect field windows is empty",
    ]


def test_ladder_prints_the_day_defects_of_a_malformed_day(capsys, tmp_path):
    path = write_ladder(tmp_path, day=ladder_day(runs=[]))
    assert main(["ladder", path]) == 1
    assert capsys.readouterr().out.splitlines() == [
        "invalid ladder",
        "defect day field runs is empty",
    ]


def test_ladder_prints_the_defect_of_an_unreadable_file(capsys, tmp_path):
    assert main(["ladder", str(tmp_path / "missing.json")]) == 1
    assert capsys.readouterr().out.splitlines() == [
        "invalid ladder",
        "defect file cannot be read",
    ]


def test_a_forged_label_inside_the_day_prints_no_report(capsys, tmp_path):
    day = ladder_day(
        runs=[
            {"runner": "operator", "usage": 900},
            {"runner": "fork\nfinish, no record was set", "usage": 950},
        ]
    )
    path = write_ladder(tmp_path, day=day)
    assert main(["ladder", path]) == 1
    out = capsys.readouterr().out
    assert out.splitlines() == [
        "invalid ladder",
        "defect day run 1 field runner holds a line break",
    ]
    assert "field day, window" not in out
