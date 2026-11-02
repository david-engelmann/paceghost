"""Pins for PG-1 to PG-4 in the full form section of CONTRACT.md.

Each malformed shape the contract names gets a test that fails if the
loader stops naming that shape, and each rule gets a test whose numbers
would change under the opposite rule.
"""

import json

from paceghost.runs import (
    ghost_change,
    load_run_file,
    load_run_text,
    race_report,
    select_ghost,
)


def valid_data():
    return {
        "task": "draft the weekly summary",
        "window_seconds": 3600,
        "runner": "operator",
        "recorded_at": "2026-01-04",
        "splits": [
            {"phase": "plan", "elapsed_seconds": 300},
            {"phase": "draft", "elapsed_seconds": 900},
        ],
    }


def load(data):
    return load_run_text(json.dumps(data))


def make_run(runner, recorded_at, splits, task="draft the weekly summary"):
    return {
        "task": task,
        "window_seconds": 3600,
        "runner": runner,
        "recorded_at": recorded_at,
        "splits": [
            {"phase": phase, "elapsed_seconds": elapsed}
            for phase, elapsed in splits
        ],
        "total_seconds": splits[-1][1],
    }


# PG-1, clean inputs


def test_valid_run_loads_with_total_from_final_split():
    run, defects = load(valid_data())
    assert defects == []
    assert run["task"] == "draft the weekly summary"
    assert run["runner"] == "operator"
    assert run["window_seconds"] == 3600
    assert run["recorded_at"] == "2026-01-04"
    assert run["total_seconds"] == 900
    assert set(run) == {
        "task",
        "window_seconds",
        "runner",
        "recorded_at",
        "splits",
        "total_seconds",
    }


def test_valid_run_with_matching_carried_total_loads():
    data = valid_data()
    data["total_seconds"] = 900
    run, defects = load(data)
    assert defects == []
    assert run["total_seconds"] == 900


def test_extra_fields_are_ignored():
    data = valid_data()
    data["notes"] = "written after the run"
    run, defects = load(data)
    assert defects == []
    assert "notes" not in run


def test_first_split_at_zero_is_valid():
    data = valid_data()
    data["splits"][0]["elapsed_seconds"] = 0
    run, defects = load(data)
    assert defects == []
    assert run["splits"][0]["elapsed_seconds"] == 0


# PG-1, malformed inputs the loader must name


def test_non_json_input_is_named():
    run, defects = load_run_text("this is not json")
    assert run is None
    assert defects == ["input is not valid JSON"]


def test_non_object_json_is_named():
    for text in ("[1, 2]", '"just a string"', "42"):
        run, defects = load_run_text(text)
        assert run is None
        assert defects == ["input is not a JSON object"]


def test_each_missing_top_level_field_is_named():
    for field in ("task", "runner", "window_seconds", "recorded_at", "splits"):
        data = valid_data()
        del data[field]
        run, defects = load(data)
        assert run is None
        assert f"missing field {field}" in defects


def test_task_wrong_type_and_empty_are_named():
    data = valid_data()
    data["task"] = 5
    run, defects = load(data)
    assert run is None
    assert "field task has the wrong type" in defects
    data = valid_data()
    data["task"] = ""
    run, defects = load(data)
    assert run is None
    assert "field task is empty" in defects


def test_runner_wrong_type_and_empty_are_named():
    data = valid_data()
    data["runner"] = ["operator"]
    run, defects = load(data)
    assert run is None
    assert "field runner has the wrong type" in defects
    data = valid_data()
    data["runner"] = ""
    run, defects = load(data)
    assert run is None
    assert "field runner is empty" in defects


def test_window_wrong_type_is_named():
    data = valid_data()
    data["window_seconds"] = "3600"
    run, defects = load(data)
    assert run is None
    assert "field window_seconds has the wrong type" in defects


def test_window_boolean_is_not_a_number():
    data = valid_data()
    data["window_seconds"] = True
    run, defects = load(data)
    assert run is None
    assert "field window_seconds has the wrong type" in defects


def test_window_must_be_positive():
    for value in (0, -5):
        data = valid_data()
        data["window_seconds"] = value
        run, defects = load(data)
        assert run is None
        assert "field window_seconds must be positive" in defects


def test_window_nan_is_not_a_finite_number():
    text = (
        '{"task": "t", "window_seconds": NaN, "runner": "r", '
        '"recorded_at": "2026-01-04", '
        '"splits": [{"phase": "plan", "elapsed_seconds": 3}]}'
    )
    run, defects = load_run_text(text)
    assert run is None
    assert "field window_seconds is not a finite number" in defects


def test_recorded_at_wrong_type_is_named():
    data = valid_data()
    data["recorded_at"] = 20260104
    run, defects = load(data)
    assert run is None
    assert "field recorded_at has the wrong type" in defects


def test_recorded_at_must_be_an_iso_date():
    for value in ("last Tuesday", "2026-13-40", "04/01/2026"):
        data = valid_data()
        data["recorded_at"] = value
        run, defects = load(data)
        assert run is None
        assert "field recorded_at is not an ISO date" in defects


def test_splits_wrong_type_and_empty_are_named():
    data = valid_data()
    data["splits"] = {"phase": "plan"}
    run, defects = load(data)
    assert run is None
    assert "field splits has the wrong type" in defects
    data = valid_data()
    data["splits"] = []
    run, defects = load(data)
    assert run is None
    assert "field splits is empty" in defects


def test_split_that_is_not_an_object_is_named():
    data = valid_data()
    data["splits"] = [42]
    run, defects = load(data)
    assert run is None
    assert "split 0 is not an object" in defects


def test_split_missing_fields_are_named():
    data = valid_data()
    data["splits"] = [{"elapsed_seconds": 300}]
    run, defects = load(data)
    assert run is None
    assert "split 0 is missing field phase" in defects
    data = valid_data()
    data["splits"] = [{"phase": "plan"}]
    run, defects = load(data)
    assert run is None
    assert "split 0 is missing field elapsed_seconds" in defects


def test_split_phase_wrong_type_and_empty_are_named():
    data = valid_data()
    data["splits"] = [{"phase": 7, "elapsed_seconds": 300}]
    run, defects = load(data)
    assert run is None
    assert "split 0 field phase has the wrong type" in defects
    data = valid_data()
    data["splits"] = [{"phase": "", "elapsed_seconds": 300}]
    run, defects = load(data)
    assert run is None
    assert "split 0 field phase is empty" in defects


def test_split_elapsed_wrong_type_and_boolean_are_named():
    for value in ("300", True):
        data = valid_data()
        data["splits"] = [{"phase": "plan", "elapsed_seconds": value}]
        run, defects = load(data)
        assert run is None
        assert "split 0 field elapsed_seconds has the wrong type" in defects


def test_split_elapsed_negative_is_named():
    data = valid_data()
    data["splits"] = [{"phase": "plan", "elapsed_seconds": -1}]
    run, defects = load(data)
    assert run is None
    assert "split 0 field elapsed_seconds is negative" in defects


def test_split_elapsed_must_increase():
    for second in (300, 200):
        data = valid_data()
        data["splits"] = [
            {"phase": "plan", "elapsed_seconds": 300},
            {"phase": "draft", "elapsed_seconds": second},
        ]
        run, defects = load(data)
        assert run is None
        assert (
            "split 1 field elapsed_seconds is not greater than the "
            "previous split"
        ) in defects


def test_carried_total_mismatch_is_named():
    data = valid_data()
    data["total_seconds"] = 901
    run, defects = load(data)
    assert run is None
    assert "field total_seconds does not equal the final split" in defects


def test_carried_total_wrong_type_is_named():
    data = valid_data()
    data["total_seconds"] = "900"
    run, defects = load(data)
    assert run is None
    assert "field total_seconds has the wrong type" in defects


def test_several_defects_are_named_in_one_pass():
    run, defects = load({"task": 5, "splits": []})
    assert run is None
    assert "field task has the wrong type" in defects
    assert "missing field runner" in defects
    assert "missing field window_seconds" in defects
    assert "missing field recorded_at" in defects
    assert "field splits is empty" in defects
    assert len(defects) >= 5


def test_loader_never_raises_on_garbage():
    for text in ("", "{", "null", '{"splits": "x"}', '{"a": 1} trailing'):
        run, defects = load_run_text(text)
        assert run is None
        assert defects


def test_load_run_file_reports_an_unreadable_file(tmp_path):
    run, defects = load_run_file(tmp_path / "no such run.json")
    assert run is None
    assert defects == ["file cannot be read"]


def test_load_run_file_round_trip(tmp_path):
    path = tmp_path / "run.json"
    path.write_text(json.dumps(valid_data()), encoding="utf-8")
    run, defects = load_run_file(path)
    assert defects == []
    assert run["total_seconds"] == 900


# PG-2, the ghost rule


def test_ghost_is_the_lowest_total():
    slow = make_run("operator", "2026-01-04", [("plan", 300), ("draft", 900)])
    fast = make_run("night fork", "2026-01-05", [("plan", 200), ("draft", 700)])
    mid = make_run("weekend fork", "2026-01-06", [("plan", 250), ("draft", 800)])
    assert select_ghost([slow, fast, mid]) is fast


def test_ghost_tie_breaks_toward_the_earliest_recorded_run():
    later = make_run("operator", "2026-01-06", [("plan", 200), ("draft", 700)])
    earlier = make_run("night fork", "2026-01-03", [("plan", 240), ("draft", 700)])
    assert select_ghost([later, earlier]) is earlier


def test_ghost_full_tie_keeps_the_input_order():
    first = make_run("operator", "2026-01-04", [("plan", 200), ("draft", 700)])
    second = make_run("night fork", "2026-01-04", [("plan", 260), ("draft", 700)])
    assert select_ghost([first, second]) is first


def test_select_ghost_with_no_runs_returns_none():
    assert select_ghost([]) is None


# PG-3, the race report


def test_race_reports_ahead_behind_and_the_finish_with_exact_seconds():
    ghost = make_run(
        "operator",
        "2026-01-04",
        [("plan", 300), ("draft", 900), ("polish", 1500)],
    )
    live = make_run(
        "operator",
        "2026-01-05",
        [("plan", 240), ("draft", 960), ("polish", 1440)],
    )
    assert race_report(ghost, live) == [
        "plan ahead by 60 seconds",
        "draft behind by 60 seconds",
        "polish ahead by 60 seconds",
        "window, the live run finished inside its window "
        "with 2160 seconds to spare",
        "finish ahead by 60 seconds",
    ]


def test_race_reports_level_splits_and_a_level_finish():
    splits = [("plan", 300), ("draft", 900)]
    ghost = make_run("operator", "2026-01-04", splits)
    live = make_run("night fork", "2026-01-05", splits)
    assert race_report(ghost, live) == [
        "plan level at 300 seconds",
        "draft level at 900 seconds",
        "window, the live run finished inside its window "
        "with 2700 seconds to spare",
        "finish level at 900 seconds",
    ]


def test_race_reports_a_finish_behind():
    ghost = make_run("operator", "2026-01-04", [("plan", 300), ("draft", 900)])
    live = make_run("operator", "2026-01-05", [("plan", 310), ("draft", 990)])
    assert race_report(ghost, live)[-1] == "finish behind by 90 seconds"


def test_race_names_a_phase_mismatch_without_a_difference():
    ghost = make_run(
        "operator", "2026-01-04", [("plan", 300), ("draft", 900)]
    )
    live = make_run(
        "operator", "2026-01-05", [("plan", 240), ("write", 800)]
    )
    lines = race_report(ghost, live)
    assert lines[0] == "plan ahead by 60 seconds"
    assert lines[1] == "split 1 phase mismatch, ghost draft and live write"
    assert lines[-1] == "finish ahead by 100 seconds"


def test_race_reports_different_split_counts_before_the_finish():
    ghost = make_run(
        "operator",
        "2026-01-04",
        [("plan", 300), ("draft", 900), ("polish", 1500)],
    )
    live = make_run(
        "operator",
        "2026-01-05",
        [("plan", 240), ("draft", 800), ("polish", 1300), ("send", 1400)],
    )
    lines = race_report(ghost, live)
    assert lines[-3] == "split counts differ, ghost 3 and live 4"
    assert lines[-2] == (
        "window, the live run finished inside its window "
        "with 2200 seconds to spare"
    )
    assert lines[-1] == "finish ahead by 100 seconds"


def test_race_renders_fractional_seconds_exactly():
    ghost = make_run("operator", "2026-01-04", [("plan", 100.5)])
    live = make_run("operator", "2026-01-05", [("plan", 88)])
    assert race_report(ghost, live) == [
        "plan ahead by 12.5 seconds",
        "window, the live run finished inside its window "
        "with 3512 seconds to spare",
        "finish ahead by 12.5 seconds",
    ]


# PG-4, the ghost changing hands


def test_a_faster_candidate_steals_the_ghost_with_both_totals():
    old = make_run(
        "operator", "2026-01-04", [("plan", 300), ("polish", 1500)]
    )
    candidate = make_run(
        "night fork", "2026-01-05", [("plan", 240), ("polish", 1320)]
    )
    steals, line = ghost_change(old, candidate)
    assert steals is True
    assert line == (
        "ghost changes from operator at 1500 seconds "
        "to night fork at 1320 seconds"
    )


def test_a_slower_candidate_leaves_the_ghost_with_both_totals():
    old = make_run(
        "operator", "2026-01-04", [("plan", 300), ("polish", 1500)]
    )
    candidate = make_run(
        "night fork", "2026-01-05", [("plan", 400), ("polish", 1800)]
    )
    steals, line = ghost_change(old, candidate)
    assert steals is False
    assert line == (
        "ghost stays with operator at 1500 seconds "
        "against night fork at 1800 seconds"
    )


def test_an_equal_total_with_an_earlier_date_steals():
    old = make_run("operator", "2026-01-06", [("plan", 300), ("polish", 1500)])
    candidate = make_run(
        "night fork", "2026-01-02", [("plan", 200), ("polish", 1500)]
    )
    steals, line = ghost_change(old, candidate)
    assert steals is True
    assert line == (
        "ghost changes from operator at 1500 seconds "
        "to night fork at 1500 seconds"
    )


def test_an_equal_total_with_a_later_or_same_date_does_not_steal():
    old = make_run("operator", "2026-01-04", [("plan", 300), ("polish", 1500)])
    for date in ("2026-01-04", "2026-01-09"):
        candidate = make_run(
            "night fork", date, [("plan", 200), ("polish", 1500)]
        )
        steals, line = ghost_change(old, candidate)
        assert steals is False
        assert line == (
            "ghost stays with operator at 1500 seconds "
            "against night fork at 1500 seconds"
        )


# Round 1 repair pins, FIX-PG-1 to FIX-PG-5
#
# Each pin below names the finding it closes. The Major pins were
# validated against the pre fix code, where each one fails, and the
# results stand in the repair report.


def test_a_huge_integer_window_is_not_a_finite_number():
    data = valid_data()
    data["window_seconds"] = 10**400
    run, defects = load(data)
    assert run is None
    assert "field window_seconds is not a finite number" in defects


def test_a_huge_integer_split_elapsed_is_not_a_finite_number():
    data = valid_data()
    data["splits"] = [{"phase": "plan", "elapsed_seconds": 10**400}]
    run, defects = load(data)
    assert run is None
    assert "split 0 field elapsed_seconds is not a finite number" in defects


def test_a_huge_integer_carried_total_is_not_a_finite_number():
    data = valid_data()
    data["total_seconds"] = 10**400
    run, defects = load(data)
    assert run is None
    assert "field total_seconds is not a finite number" in defects


def test_deeply_nested_input_is_not_valid_json_and_never_raises():
    run, defects = load_run_text("[" * 100000 + "]" * 100000)
    assert run is None
    assert defects == ["input is not valid JSON"]


def test_a_task_holding_an_unpaired_surrogate_is_named():
    data = valid_data()
    data["task"] = "\ud800x"
    run, defects = load(data)
    assert run is None
    assert "field task holds an unpaired surrogate" in defects


def test_a_runner_holding_an_unpaired_surrogate_is_named():
    data = valid_data()
    data["runner"] = "night \ud800fork"
    run, defects = load(data)
    assert run is None
    assert "field runner holds an unpaired surrogate" in defects


def test_a_phase_holding_an_unpaired_surrogate_is_named():
    data = valid_data()
    data["splits"] = [{"phase": "\ud800plan", "elapsed_seconds": 300}]
    run, defects = load(data)
    assert run is None
    assert "split 0 field phase holds an unpaired surrogate" in defects


def test_recorded_at_compact_and_week_forms_are_not_iso_dates():
    for value in ("20260104", "2026-W01-1"):
        data = valid_data()
        data["recorded_at"] = value
        run, defects = load(data)
        assert run is None
        assert "field recorded_at is not an ISO date" in defects


def test_a_split_elapsed_of_infinity_is_not_a_finite_number():
    text = (
        '{"task": "t", "window_seconds": 3600, "runner": "r", '
        '"recorded_at": "2026-01-04", '
        '"splits": [{"phase": "plan", "elapsed_seconds": Infinity}]}'
    )
    run, defects = load_run_text(text)
    assert run is None
    assert "split 0 field elapsed_seconds is not a finite number" in defects


def test_a_carried_total_of_nan_is_not_a_finite_number():
    text = (
        '{"task": "t", "window_seconds": 3600, "runner": "r", '
        '"recorded_at": "2026-01-04", "total_seconds": NaN, '
        '"splits": [{"phase": "plan", "elapsed_seconds": 3}]}'
    )
    run, defects = load_run_text(text)
    assert run is None
    assert "field total_seconds is not a finite number" in defects


def window_run(total, window):
    return {
        "task": "draft the weekly summary",
        "window_seconds": window,
        "runner": "operator",
        "recorded_at": "2026-01-04",
        "splits": [{"phase": "plan", "elapsed_seconds": total}],
        "total_seconds": total,
    }


def test_the_window_line_names_the_seconds_to_spare():
    ghost = window_run(1000, 3600)
    live = window_run(800, 3600)
    lines = race_report(ghost, live)
    assert lines[-2] == (
        "window, the live run finished inside its window "
        "with 2800 seconds to spare"
    )
    assert lines[-1] == "finish ahead by 200 seconds"


def test_the_window_line_names_the_overrun_past_the_window():
    ghost = window_run(1000, 3600)
    live = window_run(3900, 3600)
    lines = race_report(ghost, live)
    assert lines[-2] == "window, the live run ran 300 seconds past its window"
    assert lines[-1] == "finish behind by 2900 seconds"


def test_a_total_exactly_at_the_window_reads_inside_with_zero_to_spare():
    ghost = window_run(1000, 3600)
    live = window_run(3600, 3600)
    lines = race_report(ghost, live)
    assert lines[-2] == (
        "window, the live run finished inside its window "
        "with 0 seconds to spare"
    )


def test_a_sub_microsecond_difference_renders_at_full_precision():
    ghost = make_run("operator", "2026-01-04", [("plan", 1.0000004)])
    live = make_run("operator", "2026-01-05", [("plan", 1.0)])
    lines = race_report(ghost, live)
    difference = 1.0000004 - 1.0
    assert lines[0] == f"plan ahead by {difference!r} seconds"
    assert lines[-1] == f"finish ahead by {difference!r} seconds"
    assert all("by 0 seconds" not in line for line in lines)
