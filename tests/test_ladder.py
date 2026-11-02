"""Tests for PG-9 and PG-10, the ladder loader and report."""

import json

import pytest

from paceghost.fieldday import field_day_report, load_field_day_text
from paceghost.ladder import (
    ladder_report,
    load_ladder_file,
    load_ladder_text,
)


def day_data(**overrides):
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
                "reads": [{"elapsed_seconds": 600, "usage": 320}],
            },
        ],
    }
    day.update(overrides)
    return day


def ladder_text(windows=(1800,), day=None, **extra):
    ladder = {
        "day": day_data() if day is None else day,
        "windows": list(windows),
    }
    ladder.update(extra)
    return json.dumps(ladder)


def loaded_ladder(windows=(1800,), day=None):
    ladder, defects = load_ladder_text(ladder_text(windows, day))
    assert defects == []
    return ladder


def report_for(runs, windows, operator="operator", day_window=1800):
    day = {"window_seconds": day_window, "operator": operator, "runs": runs}
    return ladder_report(loaded_ladder(windows, day))


def test_a_clean_ladder_loads_carrying_the_day_and_the_windows():
    ladder = loaded_ladder([900.0, 600])
    assert ladder["windows"] == [900, 600]
    assert ladder["day"]["operator"] == "operator"
    assert ladder["day"]["window_seconds"] == 1800
    assert [run["runner"] for run in ladder["day"]["runs"]] == [
        "operator",
        "fork",
    ]


def test_extra_fields_are_ignored():
    ladder, defects = load_ladder_text(ladder_text([600], note="kept out"))
    assert defects == []
    assert ladder["windows"] == [600]


def test_text_that_is_not_json_is_refused():
    ladder, defects = load_ladder_text("not json")
    assert ladder is None
    assert defects == ["input is not valid JSON"]


def test_input_nested_too_deeply_is_refused():
    ladder, defects = load_ladder_text("[" * 100000 + "]" * 100000)
    assert ladder is None
    assert defects == ["input is not valid JSON"]


def test_json_that_is_not_an_object_is_refused():
    ladder, defects = load_ladder_text("[1, 2]")
    assert ladder is None
    assert defects == ["input is not a JSON object"]


def test_a_missing_day_is_refused():
    ladder, defects = load_ladder_text(json.dumps({"windows": [600]}))
    assert ladder is None
    assert defects == ["missing field day"]


def test_a_day_that_is_not_an_object_is_refused():
    ladder, defects = load_ladder_text(ladder_text([600], day=[1, 2]))
    assert ladder is None
    assert defects == ["field day is not an object"]


def test_day_defects_stand_prefixed_and_before_the_window_defects():
    day = day_data(runs=[])
    ladder, defects = load_ladder_text(
        json.dumps({"day": day, "windows": "wide"})
    )
    assert ladder is None
    assert defects == [
        "day field runs is empty",
        "field windows has the wrong type",
    ]


def test_a_line_break_inside_the_day_is_refused_under_the_day_prefix():
    day = day_data(
        runs=[
            {"runner": "operator", "usage": 900},
            {"runner": "fork\nfinish, no record", "usage": 950},
        ]
    )
    ladder, defects = load_ladder_text(ladder_text([600], day))
    assert ladder is None
    assert defects == ["day run 1 field runner holds a line break"]


def test_missing_windows_are_refused():
    ladder, defects = load_ladder_text(json.dumps({"day": day_data()}))
    assert ladder is None
    assert defects == ["missing field windows"]


def test_windows_of_the_wrong_type_are_refused():
    text = json.dumps({"day": day_data(), "windows": "wide"})
    ladder, defects = load_ladder_text(text)
    assert ladder is None
    assert defects == ["field windows has the wrong type"]


def test_an_empty_window_list_is_refused():
    ladder, defects = load_ladder_text(ladder_text([]))
    assert ladder is None
    assert defects == ["field windows is empty"]


def test_a_window_of_the_wrong_type_is_refused():
    for value in (True, "900"):
        ladder, defects = load_ladder_text(ladder_text([value]))
        assert ladder is None
        assert defects == ["window 0 has the wrong type"]


def test_a_window_that_is_not_finite_is_refused():
    ladder, defects = load_ladder_text(ladder_text([10**400]))
    assert ladder is None
    assert defects == ["window 0 is not a finite number"]


def test_a_window_that_is_not_whole_is_refused():
    ladder, defects = load_ladder_text(ladder_text([600.5]))
    assert ladder is None
    assert defects == ["window 0 is not a whole number"]


def test_a_window_at_or_below_zero_is_refused():
    ladder, defects = load_ladder_text(ladder_text([600, -5]))
    assert ladder is None
    assert defects == ["window 1 must be positive"]


def test_the_positive_check_stands_before_the_whole_check():
    ladder, defects = load_ladder_text(ladder_text([-1.5]))
    assert ladder is None
    assert defects == ["window 0 must be positive"]


def test_a_duplicate_window_is_refused_naming_the_earlier_window():
    ladder, defects = load_ladder_text(ladder_text([600, 900, 600]))
    assert ladder is None
    assert defects == ["window 2 duplicates window 0"]
    ladder, defects = load_ladder_text(ladder_text([900, 900.0]))
    assert ladder is None
    assert defects == ["window 1 duplicates window 0"]


def test_one_input_draws_several_defects_in_one_pass():
    ladder, defects = load_ladder_text(
        json.dumps({"windows": [0, "x"]})
    )
    assert ladder is None
    assert defects == [
        "missing field day",
        "window 0 must be positive",
        "window 1 has the wrong type",
    ]


def test_an_unreadable_ladder_file_names_the_single_defect(tmp_path):
    ladder, defects = load_ladder_file(tmp_path / "missing.json")
    assert ladder is None
    assert defects == ["file cannot be read"]


def test_a_ladder_file_loads_from_disk(tmp_path):
    path = tmp_path / "ladder.json"
    path.write_text(ladder_text([600]), encoding="utf-8")
    ladder, defects = load_ladder_file(path)
    assert defects == []
    assert ladder["windows"] == [600]


def test_the_report_reads_each_window_as_the_day_stood_at_it():
    lines = report_for(day_data()["runs"], [1800, 300])
    assert lines == [
        "field day, window 1800 seconds, 2 runs, operator operator",
        "record at 300 seconds, operator at a projection of 900",
        "finish, record held by operator at a projection of 900",
        "record kept by operator to the close, margin 60 at the close",
        "field day, window 300 seconds, 2 runs, operator operator",
        "record at 300 seconds, operator at a projection of 150",
        "finish, record held by operator at a projection of 150",
        "record kept by operator to the close",
        "ladder, the closing holder never changed across the ladder",
    ]


def test_a_window_holding_no_read_sets_no_record():
    lines = report_for(day_data()["runs"], [100])
    assert lines == [
        "field day, window 100 seconds, 2 runs, operator operator",
        "finish, no record was set",
        "ladder, the closing holder never changed across the ladder",
    ]


def test_two_windows_with_no_record_do_not_differ():
    lines = report_for(day_data()["runs"], [100, 200])
    assert lines[-1] == (
        "ladder, the closing holder never changed across the ladder"
    )


def test_a_holder_after_no_record_differs_from_no_holder():
    lines = report_for(day_data()["runs"], [100, 300])
    assert lines[-1] == (
        "ladder, the closing holder differs from the first window "
        "at 300 seconds"
    )


def test_the_closing_holder_change_is_named_in_the_summary():
    runs = [
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
    ]
    lines = report_for(runs, [300, 600])
    assert lines == [
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


def test_a_change_at_the_last_window_only_names_that_window():
    runs = [
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
    ]
    lines = report_for(runs, [600, 300])
    assert lines[0].startswith("field day, window 600 seconds")
    assert lines[-1] == (
        "ladder, the closing holder differs from the first window "
        "at 300 seconds"
    )


def test_the_day_window_plays_no_part_in_the_reading():
    runs = [
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
    ]
    wide = report_for(runs, [300, 600], day_window=1800)
    narrow = report_for(runs, [300, 600], day_window=42)
    assert wide == narrow


def test_an_equal_figure_never_takes_the_record_inside_a_window():
    runs = [
        {
            "runner": "operator",
            "usage": 900,
            "reads": [{"elapsed_seconds": 300, "usage": 150}],
        },
        {
            "runner": "fork",
            "usage": 950,
            "reads": [{"elapsed_seconds": 600, "usage": 300}],
        },
    ]
    lines = report_for(runs, [600])
    assert lines == [
        "field day, window 600 seconds, 2 runs, operator operator",
        "record at 300 seconds, operator at a projection of 300",
        "finish, record held by operator at a projection of 300",
        "record kept by operator to the close, margin 0 at the close",
        "ladder, the closing holder never changed across the ladder",
    ]


def test_each_section_equals_the_field_day_report_of_the_day_as_it_stood():
    day = {
        "window_seconds": 1800,
        "operator": "operator",
        "runs": [
            {
                "runner": "operator",
                "usage": 900,
                "reads": [
                    {"elapsed_seconds": 300, "usage": 150},
                    {"elapsed_seconds": 900, "usage": 500},
                ],
            },
            {
                "runner": "fork",
                "usage": 950,
                "reads": [{"elapsed_seconds": 600, "usage": 320}],
            },
            {"runner": "quiet", "usage": 990},
        ],
    }
    for window in (300, 600, 900, 1800):
        stood = {"window_seconds": window, "operator": "operator", "runs": []}
        for run in day["runs"]:
            clone = {"runner": run["runner"], "usage": run["usage"]}
            kept = [
                read
                for read in run.get("reads", [])
                if read["elapsed_seconds"] <= window
            ]
            if kept:
                clone["reads"] = kept
            stood["runs"].append(clone)
        loaded, defects = load_field_day_text(json.dumps(stood))
        assert defects == []
        section = ladder_report(loaded_ladder([window], day))[:-1]
        assert section == field_day_report(loaded)


# The report on values the loader refuses, the discipline of FIX-PG-8
def test_the_report_raises_a_value_error_on_an_empty_windows_list():
    with pytest.raises(ValueError, match="the ladder holds no windows"):
        ladder_report({"day": day_data(), "windows": []})


def test_the_report_raises_a_value_error_when_the_operator_names_no_run():
    day = day_data(operator="ghost")
    with pytest.raises(
        ValueError, match="the operator names no run among the runs"
    ):
        ladder_report({"day": day, "windows": [600]})


def test_the_report_raises_a_value_error_when_the_day_holds_no_runs():
    day = day_data(runs=[])
    with pytest.raises(
        ValueError, match="the operator names no run among the runs"
    ):
        ladder_report({"day": day, "windows": [600]})
