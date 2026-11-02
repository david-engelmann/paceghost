"""Pins for the command line surface in CONTRACT.md.

Each command gets a clean run with its exact printed lines and a
malformed run with its exit status and named defects, so a command
that started swallowing defects or printing extra lines would fail.
"""

import json

from paceghost.cli import main


def write_run(tmp_path, name, data):
    path = tmp_path / name
    path.write_text(json.dumps(data), encoding="utf-8")
    return str(path)


def run_data(runner, recorded_at, splits, task="draft the weekly summary"):
    return {
        "task": task,
        "window_seconds": 3600,
        "runner": runner,
        "recorded_at": recorded_at,
        "splits": [
            {"phase": phase, "elapsed_seconds": elapsed}
            for phase, elapsed in splits
        ],
    }


def test_show_prints_the_run_summary(tmp_path, capsys):
    path = write_run(
        tmp_path,
        "run.json",
        run_data("operator", "2026-01-04", [("plan", 300), ("draft", 900)]),
    )
    assert main(["show", path]) == 0
    assert capsys.readouterr().out.splitlines() == [
        "task draft the weekly summary",
        "runner operator",
        "recorded_at 2026-01-04",
        "window_seconds 3600",
        "total_seconds 900",
        "splits 2",
        "split plan at 300 seconds",
        "split draft at 900 seconds",
        "window, this run finished inside its window "
        "with 2700 seconds to spare",
    ]


def test_show_prints_defects_and_exits_one_on_a_malformed_run(tmp_path, capsys):
    path = write_run(tmp_path, "bad.json", {"task": 5, "splits": []})
    assert main(["show", path]) == 1
    out = capsys.readouterr().out
    assert "invalid recorded run" in out
    assert "defect field task has the wrong type" in out
    assert "defect field splits is empty" in out


def test_show_reports_an_unreadable_file(tmp_path, capsys):
    assert main(["show", str(tmp_path / "missing.json")]) == 1
    assert "defect file cannot be read" in capsys.readouterr().out


def test_ghost_prints_the_faster_run_as_the_ghost(tmp_path, capsys):
    slow = write_run(
        tmp_path,
        "slow.json",
        run_data(
            "operator",
            "2026-01-04",
            [("plan", 300), ("draft", 900), ("polish", 1500)],
        ),
    )
    fast = write_run(
        tmp_path,
        "fast.json",
        run_data(
            "night fork",
            "2026-01-05",
            [("plan", 240), ("draft", 780), ("polish", 1320)],
        ),
    )
    assert main(["ghost", slow, fast]) == 0
    assert capsys.readouterr().out.splitlines() == [
        "ghost for task draft the weekly summary",
        "runner night fork",
        "recorded_at 2026-01-05",
        "total_seconds 1320",
    ]


def test_ghost_rejects_runs_of_more_than_one_task(tmp_path, capsys):
    one = write_run(
        tmp_path,
        "one.json",
        run_data("operator", "2026-01-04", [("plan", 300)]),
    )
    two = write_run(
        tmp_path,
        "two.json",
        run_data(
            "operator", "2026-01-04", [("plan", 200)], task="another task"
        ),
    )
    assert main(["ghost", one, two]) == 1
    assert "runs do not share one task" in capsys.readouterr().out


def test_ghost_rejects_a_malformed_file(tmp_path, capsys):
    good = write_run(
        tmp_path,
        "good.json",
        run_data("operator", "2026-01-04", [("plan", 300)]),
    )
    bad = write_run(tmp_path, "bad.json", {"task": "x"})
    assert main(["ghost", good, bad]) == 1
    assert "defect missing field runner" in capsys.readouterr().out


def test_race_prints_the_race_report(tmp_path, capsys):
    ghost = write_run(
        tmp_path,
        "ghost.json",
        run_data(
            "operator",
            "2026-01-04",
            [("plan", 300), ("draft", 900), ("polish", 1500)],
        ),
    )
    live = write_run(
        tmp_path,
        "live.json",
        run_data(
            "night fork",
            "2026-01-05",
            [("plan", 240), ("draft", 780), ("polish", 1320)],
        ),
    )
    assert main(["race", ghost, live]) == 0
    assert capsys.readouterr().out.splitlines() == [
        "plan ahead by 60 seconds",
        "draft ahead by 120 seconds",
        "polish ahead by 180 seconds",
        "window, the live run finished inside its window "
        "with 2280 seconds to spare",
        "finish ahead by 180 seconds",
    ]


def test_race_rejects_a_malformed_input(tmp_path, capsys):
    ghost = write_run(
        tmp_path,
        "ghost.json",
        run_data("operator", "2026-01-04", [("plan", 300)]),
    )
    bad = write_run(tmp_path, "bad.json", {"splits": "no"})
    assert main(["race", ghost, bad]) == 1
    assert "invalid recorded run" in capsys.readouterr().out


def test_show_prints_defects_for_a_task_with_an_unpaired_surrogate(
    tmp_path, capsys
):
    data = run_data("operator", "2026-01-04", [("plan", 300)])
    data["task"] = "\ud800x"
    path = write_run(tmp_path, "surrogate.json", data)
    assert main(["show", path]) == 1
    out = capsys.readouterr().out
    assert "invalid recorded run" in out
    assert "defect field task holds an unpaired surrogate" in out


def test_race_rejects_runs_of_different_tasks(tmp_path, capsys):
    ghost = write_run(
        tmp_path,
        "ghost.json",
        run_data("operator", "2026-01-04", [("plan", 300)]),
    )
    live = write_run(
        tmp_path,
        "live.json",
        run_data(
            "operator", "2026-01-05", [("plan", 200)], task="another task"
        ),
    )
    assert main(["race", ghost, live]) == 1
    assert "runs do not share one task" in capsys.readouterr().out
