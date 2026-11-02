"""Pins for the season and board commands in CONTRACT.md.

Each command gets a clean run with its exact printed lines and each
refusal with its exit status and its single line, so a command that
started printing a report for a refused input would fail.
"""

import json

from paceghost.cli import main


def write(tmp_path, name, data):
    path = tmp_path / name
    path.write_text(json.dumps(data), encoding="utf-8")
    return str(path)


def run_data(total, usage=None, budget=None, model=None, task="t", **extra):
    data = {
        "task": task,
        "window_seconds": 200,
        "runner": "someone",
        "recorded_at": "2026-01-04",
        "splits": [{"phase": "only", "elapsed_seconds": total}],
    }
    if usage is not None:
        data["usage"] = usage
    if budget is not None:
        data["budget"] = budget
    if model is not None:
        data["model"] = model
    data.update(extra)
    return data


def field_file(tmp_path):
    runs = [
        run_data(100, usage=40, budget=50, model="peak"),
        run_data(120, usage=20, budget=50, model="vale"),
    ]
    return write(tmp_path, "field.json", {"runs": runs})


def ghost_file(tmp_path, usage=45, task="t"):
    data = run_data(180, task=task)
    if usage is not None:
        data["usage"] = usage
    return write(tmp_path, "ghost.json", data)


def test_season_prints_the_report(tmp_path, capsys):
    field = field_file(tmp_path)
    ghost = ghost_file(tmp_path)
    assert main(["season", field, ghost]) == 0
    assert capsys.readouterr().out.splitlines() == [
        "season for task t",
        "model vale",
        "runs raced 1",
        "inside window 1",
        "inside budget 1",
        "total usage 20",
        "mean usage 20",
        "head to head against the ghost, wins 1 of 1",
        "model peak",
        "runs raced 1",
        "inside window 1",
        "inside budget 1",
        "total usage 40",
        "mean usage 40",
        "head to head against the ghost, wins 1 of 1",
    ]


def test_season_on_a_malformed_field_prints_its_defects(tmp_path, capsys):
    field = write(tmp_path, "field.json", {"runs": [{"task": "t"}]})
    ghost = ghost_file(tmp_path)
    assert main(["season", field, ghost]) == 1
    lines = capsys.readouterr().out.splitlines()
    assert lines[0] == "invalid field"
    assert "defect run 0 missing field splits" in lines


def test_season_on_a_ghost_without_usage_is_refused(tmp_path, capsys):
    field = field_file(tmp_path)
    ghost = ghost_file(tmp_path, usage=None)
    assert main(["season", field, ghost]) == 1
    assert capsys.readouterr().out.splitlines() == [
        "ghost record carries no usage"
    ]


def test_season_on_a_task_mismatch_is_refused(tmp_path, capsys):
    field = field_file(tmp_path)
    ghost = ghost_file(tmp_path, task="other")
    assert main(["season", field, ghost]) == 1
    assert capsys.readouterr().out.splitlines() == [
        "runs do not share one task"
    ]


def board_file(tmp_path):
    data = run_data(
        90,
        usage=30,
        budget=50,
        window_seconds=100,
        reads=[
            {"elapsed_seconds": 0, "usage": 0},
            {"elapsed_seconds": 50, "usage": 10},
        ],
    )
    return write(tmp_path, "run.json", data)


def test_board_prints_the_board(tmp_path, capsys):
    run = board_file(tmp_path)
    ghost = ghost_file(tmp_path, usage=40)
    assert main(["board", run, ghost]) == 0
    assert capsys.readouterr().out.splitlines() == [
        "read 0 at 0 seconds, usage 0, projects no figure",
        "read 1 at 50 seconds, usage 10, projection 20, "
        "inside the ghost usage, inside the budget",
        "finish, actual usage 30 against a last projection of 20, "
        "inside the ghost usage, inside the budget, the board called it",
    ]


def test_board_on_a_run_without_reads_is_refused(tmp_path, capsys):
    run = write(tmp_path, "run.json", run_data(90, usage=30, budget=50))
    ghost = ghost_file(tmp_path, usage=40)
    assert main(["board", run, ghost]) == 1
    assert capsys.readouterr().out.splitlines() == [
        "run carries no timed reads"
    ]


def test_board_on_a_ghost_without_usage_is_refused(tmp_path, capsys):
    run = board_file(tmp_path)
    ghost = ghost_file(tmp_path, usage=None)
    assert main(["board", run, ghost]) == 1
    assert capsys.readouterr().out.splitlines() == [
        "ghost record carries no usage"
    ]


def test_board_on_a_task_mismatch_is_refused(tmp_path, capsys):
    run = board_file(tmp_path)
    ghost = ghost_file(tmp_path, usage=40, task="other")
    assert main(["board", run, ghost]) == 1
    assert capsys.readouterr().out.splitlines() == [
        "runs do not share one task"
    ]


def test_board_on_a_malformed_run_prints_its_defects(tmp_path, capsys):
    run = write(tmp_path, "run.json", {"task": "t"})
    ghost = ghost_file(tmp_path, usage=40)
    assert main(["board", run, ghost]) == 1
    lines = capsys.readouterr().out.splitlines()
    assert lines[0] == "invalid recorded run"
    assert "defect missing field splits" in lines
