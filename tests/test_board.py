"""Pins for PG-6 in CONTRACT.md and the Round 2 run extension.

The loader names every malformed extension field, a recorded run
without reads loads exactly as Round 1 defined it, and the board
projections are computed by hand here so a board that extrapolated,
compared, or judged differently would fail.
"""

import json

from paceghost.runs import load_run_text
from paceghost.season import board_report


def run_data(**extra):
    data = {
        "task": "t",
        "window_seconds": 100,
        "runner": "someone",
        "recorded_at": "2026-01-04",
        "splits": [{"phase": "only", "elapsed_seconds": 90}],
    }
    data.update(extra)
    return data


def board_run(reads, usage=30, budget=50, window=100):
    return run_data(
        usage=usage, budget=budget, window_seconds=window, reads=reads
    )


def load(data):
    return load_run_text(json.dumps(data))


def ghost(usage=40):
    return {"task": "t", "usage": usage}


def test_a_round_1_record_without_reads_loads_exactly_as_before():
    run, defects = load(run_data())
    assert defects == []
    assert set(run) == {
        "task",
        "window_seconds",
        "runner",
        "recorded_at",
        "splits",
        "total_seconds",
    }
    assert run["total_seconds"] == 90


def test_a_run_with_usage_and_budget_but_no_reads_loads_clean():
    run, defects = load(run_data(usage=12, budget=20))
    assert defects == []
    assert run["usage"] == 12
    assert run["budget"] == 20
    assert "reads" not in run


def test_usage_of_the_wrong_type_is_named():
    run, defects = load(run_data(usage="many"))
    assert run is None
    assert defects == ["field usage has the wrong type"]


def test_usage_that_is_not_finite_is_named():
    run, defects = load_run_text('{"task": "t", "window_seconds": 100, '
                         '"runner": "r", "recorded_at": "2026-01-04", '
                         '"splits": [{"phase": "a", "elapsed_seconds": 1}], '
                         '"usage": NaN}')
    assert run is None
    assert defects == ["field usage is not a finite number"]


def test_negative_usage_is_named():
    run, defects = load(run_data(usage=-1))
    assert run is None
    assert defects == ["field usage is negative"]


def test_budget_of_the_wrong_type_is_named():
    run, defects = load(run_data(budget="some"))
    assert run is None
    assert defects == ["field budget has the wrong type"]


def test_budget_that_is_not_finite_is_named():
    run, defects = load_run_text('{"task": "t", "window_seconds": 100, '
                         '"runner": "r", "recorded_at": "2026-01-04", '
                         '"splits": [{"phase": "a", "elapsed_seconds": 1}], '
                         '"budget": Infinity}')
    assert run is None
    assert defects == ["field budget is not a finite number"]


def test_a_budget_of_zero_is_named():
    run, defects = load(run_data(budget=0))
    assert run is None
    assert defects == ["field budget must be positive"]


def test_reads_of_the_wrong_type_is_named():
    run, defects = load(run_data(usage=1, budget=2, reads={}))
    assert run is None
    assert defects == ["field reads has the wrong type"]


def test_an_empty_reads_list_is_named():
    run, defects = load(run_data(usage=1, budget=2, reads=[]))
    assert run is None
    assert defects == ["field reads is empty"]


def test_a_read_that_is_not_an_object_is_named():
    run, defects = load(board_run([7]))
    assert run is None
    assert defects == ["read 0 is not an object"]


def test_a_read_missing_a_field_is_named():
    run, defects = load(board_run([{"elapsed_seconds": 5}]))
    assert run is None
    assert defects == ["read 0 is missing field usage"]


def test_a_read_field_of_the_wrong_type_is_named():
    run, defects = load(board_run([{"elapsed_seconds": "5", "usage": 1}]))
    assert run is None
    assert defects == ["read 0 field elapsed_seconds has the wrong type"]


def test_a_read_elapsed_that_is_not_finite_is_named():
    run, defects = load_run_text('{"task": "t", "window_seconds": 100, '
                         '"runner": "r", "recorded_at": "2026-01-04", '
                         '"splits": [{"phase": "a", "elapsed_seconds": 1}], '
                         '"usage": 1, "budget": 2, '
                         '"reads": [{"elapsed_seconds": NaN, "usage": 1}]}')
    assert run is None
    assert defects == ["read 0 field elapsed_seconds is not a finite number"]


def test_a_negative_read_elapsed_is_named():
    run, defects = load(board_run([{"elapsed_seconds": -5, "usage": 1}]))
    assert run is None
    assert defects == ["read 0 field elapsed_seconds is negative"]


def test_a_read_elapsed_that_fails_to_increase_is_named():
    reads = [
        {"elapsed_seconds": 50, "usage": 1},
        {"elapsed_seconds": 50, "usage": 2},
    ]
    run, defects = load(board_run(reads))
    assert run is None
    assert defects == [
        "read 1 field elapsed_seconds is not greater than the previous read"
    ]


def test_a_read_usage_that_is_not_finite_is_named():
    run, defects = load_run_text('{"task": "t", "window_seconds": 100, '
                         '"runner": "r", "recorded_at": "2026-01-04", '
                         '"splits": [{"phase": "a", "elapsed_seconds": 1}], '
                         '"usage": 1, "budget": 2, '
                         '"reads": [{"elapsed_seconds": 5, "usage": NaN}]}')
    assert run is None
    assert defects == ["read 0 field usage is not a finite number"]


def test_a_negative_read_usage_is_named():
    run, defects = load(board_run([{"elapsed_seconds": 5, "usage": -1}]))
    assert run is None
    assert defects == ["read 0 field usage is negative"]


def test_a_read_usage_that_decreases_is_named():
    reads = [
        {"elapsed_seconds": 5, "usage": 10},
        {"elapsed_seconds": 9, "usage": 4},
    ]
    run, defects = load(board_run(reads))
    assert run is None
    assert defects == ["read 1 field usage is less than the previous read"]


def test_reads_without_usage_and_budget_are_named():
    data = run_data(reads=[{"elapsed_seconds": 5, "usage": 1}])
    run, defects = load(data)
    assert run is None
    assert defects == ["missing field usage", "missing field budget"]


def test_reads_with_usage_but_no_budget_names_only_the_budget():
    data = run_data(usage=9, reads=[{"elapsed_seconds": 5, "usage": 1}])
    run, defects = load(data)
    assert run is None
    assert defects == ["missing field budget"]


def test_the_board_lines_are_the_hand_computed_ones():
    reads = [
        {"elapsed_seconds": 0, "usage": 0},
        {"elapsed_seconds": 50, "usage": 10},
        {"elapsed_seconds": 150, "usage": 45},
    ]
    run, defects = load(board_run(reads))
    assert defects == []
    assert board_report(run, ghost()) == [
        "read 0 at 0 seconds, usage 0, projects no figure",
        "read 1 at 50 seconds, usage 10, projection 20, "
        "inside the ghost usage, inside the budget",
        "read 2 at 150 seconds, usage 45, projection 30, "
        "inside the ghost usage, inside the budget",
        "finish, actual usage 30 against a last projection of 30, "
        "inside the ghost usage, inside the budget, the board called it",
    ]


def test_a_projection_past_both_limits_says_so():
    reads = [{"elapsed_seconds": 10, "usage": 30}]
    run, defects = load(board_run(reads))
    assert defects == []
    lines = board_report(run, ghost())
    assert lines[0] == (
        "read 0 at 10 seconds, usage 30, projection 300, "
        "past the ghost usage, past the budget"
    )


def test_a_projection_level_with_the_ghost_usage_says_so():
    reads = [{"elapsed_seconds": 25, "usage": 10}]
    run, defects = load(board_run(reads, usage=40))
    assert defects == []
    lines = board_report(run, ghost())
    assert lines[0] == (
        "read 0 at 25 seconds, usage 10, projection 40, "
        "level with the ghost usage, inside the budget"
    )
    assert lines[-1] == (
        "finish, actual usage 40 against a last projection of 40, "
        "level with the ghost usage, inside the budget, the board called it"
    )


def test_the_board_missed_when_the_projection_verdicts_differ():
    reads = [{"elapsed_seconds": 50, "usage": 30}]
    run, defects = load(board_run(reads))
    assert defects == []
    lines = board_report(run, ghost())
    assert lines[-1] == (
        "finish, actual usage 30 against a last projection of 60, "
        "inside the ghost usage, inside the budget, the board missed it"
    )


def test_a_board_of_only_a_zero_read_makes_no_projection():
    reads = [{"elapsed_seconds": 0, "usage": 0}]
    run, defects = load(board_run(reads))
    assert defects == []
    assert board_report(run, ghost()) == [
        "read 0 at 0 seconds, usage 0, projects no figure",
        "finish, actual usage 30, no projection was made, "
        "inside the ghost usage, inside the budget",
    ]


def test_a_tiny_projection_renders_at_twelve_places_in_plain_notation():
    reads = [{"elapsed_seconds": 30000000, "usage": 1}]
    run, defects = load(board_run(reads, usage=1, window=1))
    assert defects == []
    lines = board_report(run, ghost())
    assert lines[0] == (
        "read 0 at 30000000 seconds, usage 1, projection 0.000000033333, "
        "inside the ghost usage, inside the budget"
    )


def test_a_projection_of_exactly_half_the_smallest_unit_rounds_up():
    reads = [{"elapsed_seconds": 2000000, "usage": 1}]
    run, defects = load(board_run(reads, usage=1, window=1))
    assert defects == []
    lines = board_report(run, ghost())
    assert lines[0] == (
        "read 0 at 2000000 seconds, usage 1, projection 0.000001, "
        "inside the ghost usage, inside the budget"
    )


def test_an_exact_half_projection_rounds_up():
    reads = [{"elapsed_seconds": 2000000, "usage": 4000001}]
    run, defects = load(board_run(reads, window=1))
    assert defects == []
    lines = board_report(run, ghost())
    assert lines[0] == (
        "read 0 at 2000000 seconds, usage 4000001, projection 2.000001, "
        "inside the ghost usage, inside the budget"
    )


def test_the_board_is_byte_deterministic():
    reads = [
        {"elapsed_seconds": 0, "usage": 0},
        {"elapsed_seconds": 40, "usage": 7},
    ]
    run, defects = load(board_run(reads))
    assert defects == []
    assert board_report(run, ghost()) == board_report(run, ghost())
