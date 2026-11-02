"""The field day loader and the stolen record, held to PG-7 and PG-8.

Loader pins name each defect phrase of the field day shape, and the
report pins walk the record progression at its edges, the tie that
never moves the record, the tiebreak across marks when a holder is
displaced, the holder whose own projection rises past a rival, and
every outcome line the close can produce.
"""

import json

import pytest

from paceghost.fieldday import (
    field_day_report,
    load_field_day_file,
    load_field_day_text,
)


def day_text(**overrides):
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
    return json.dumps(day)


def load_day(data):
    day, defects = load_field_day_text(json.dumps(data))
    assert defects == []
    return day


def report_for(runs, operator="operator", window=1800):
    day = load_day({"window_seconds": window, "operator": operator, "runs": runs})
    return field_day_report(day)


def timed(label, reads, usage=900):
    run = {"runner": label, "usage": usage}
    if reads is not None:
        run["reads"] = [
            {"elapsed_seconds": elapsed, "usage": used} for elapsed, used in reads
        ]
    return run


# The loader on clean input


def test_a_clean_field_day_loads_with_its_fields_in_file_order():
    day, defects = load_field_day_text(day_text())
    assert defects == []
    assert day["window_seconds"] == 1800
    assert day["operator"] == "operator"
    assert [run["runner"] for run in day["runs"]] == ["operator", "fork"]
    assert day["runs"][0]["reads"] == [{"elapsed_seconds": 300, "usage": 150}]


def test_a_run_without_reads_loads_and_projects_no_figure():
    runs = [timed("operator", None, usage=10), timed("fork", [(300, 150)])]
    lines = report_for(runs)
    assert lines[-1] == (
        "record never held by operator, whose run projected no figure"
    )


def test_a_run_may_carry_usage_without_reads():
    day, defects = load_field_day_text(
        day_text(runs=[{"runner": "operator", "usage": 42}])
    )
    assert defects == []
    assert day["runs"][0] == {"runner": "operator", "usage": 42}


def test_unnamed_fields_are_ignored_in_the_day_and_in_a_run():
    text = day_text(extra="ignored")
    data = json.loads(text)
    data["runs"][0]["surprise"] = True
    day, defects = load_field_day_text(json.dumps(data))
    assert defects == []
    assert "extra" not in day
    assert "surprise" not in day["runs"][0]


def test_load_field_day_file_names_an_unreadable_file(tmp_path):
    day, defects = load_field_day_file(tmp_path / "missing.json")
    assert day is None
    assert defects == ["file cannot be read"]


# The loader defect vocabulary


def test_text_that_is_not_json_draws_the_named_defect():
    day, defects = load_field_day_text("not json at all")
    assert day is None
    assert defects == ["input is not valid JSON"]


def test_deeply_nested_input_draws_the_not_valid_json_defect():
    day, defects = load_field_day_text("[" * 100000 + "]" * 100000)
    assert day is None
    assert defects == ["input is not valid JSON"]


def test_json_that_is_not_an_object_draws_the_named_defect():
    day, defects = load_field_day_text("[1, 2, 3]")
    assert day is None
    assert defects == ["input is not a JSON object"]


def test_missing_top_level_fields_are_each_named():
    day, defects = load_field_day_text("{}")
    assert day is None
    assert defects == [
        "missing field window_seconds",
        "missing field operator",
        "missing field runs",
    ]


def test_a_window_of_the_wrong_type_draws_the_named_defect():
    day, defects = load_field_day_text(day_text(window_seconds="soon"))
    assert defects == ["field window_seconds has the wrong type"]


def test_a_boolean_window_is_not_a_number():
    day, defects = load_field_day_text(day_text(window_seconds=True))
    assert defects == ["field window_seconds has the wrong type"]


def test_a_window_too_large_to_be_finite_draws_the_named_defect():
    text = day_text().replace("1800", "1" + "0" * 400, 1)
    day, defects = load_field_day_text(text)
    assert defects == ["field window_seconds is not a finite number"]


def test_a_window_of_zero_draws_the_named_defect():
    day, defects = load_field_day_text(day_text(window_seconds=0))
    assert defects == ["field window_seconds must be positive"]


def test_an_operator_of_the_wrong_type_draws_the_named_defect():
    day, defects = load_field_day_text(day_text(operator=7))
    assert defects == ["field operator has the wrong type"]


def test_an_empty_operator_draws_the_named_defect():
    day, defects = load_field_day_text(day_text(operator=""))
    assert defects == ["field operator is empty"]


def test_an_operator_holding_an_unpaired_surrogate_is_named():
    day, defects = load_field_day_text(day_text(operator="opera\ud800tor"))
    assert defects == ["field operator holds an unpaired surrogate"]


def test_an_operator_holding_a_line_break_is_named():
    day, defects = load_field_day_text(day_text(operator="opera\ntor"))
    assert defects == ["field operator holds a line break"]


def test_an_operator_holding_a_carriage_return_is_named():
    day, defects = load_field_day_text(day_text(operator="opera\rtor"))
    assert defects == ["field operator holds a line break"]


def test_a_runs_field_of_the_wrong_type_draws_the_named_defect():
    day, defects = load_field_day_text(day_text(runs={"runner": "operator"}))
    assert defects == ["field runs has the wrong type"]


def test_an_empty_runs_list_draws_the_named_defect():
    day, defects = load_field_day_text(day_text(runs=[]))
    assert defects == ["field runs is empty"]


def test_a_run_that_is_not_an_object_is_named_with_its_index():
    day, defects = load_field_day_text(day_text(runs=["operator"]))
    assert defects == [
        "run 0 is not an object",
        "field operator names no run in the field day",
    ]


def test_a_run_missing_its_runner_is_named_with_its_index():
    day, defects = load_field_day_text(day_text(runs=[{"usage": 5}]))
    assert defects == [
        "run 0 missing field runner",
        "field operator names no run in the field day",
    ]


def test_a_runner_of_the_wrong_type_is_named_with_its_index():
    day, defects = load_field_day_text(day_text(runs=[{"runner": 3}]))
    assert defects == [
        "run 0 field runner has the wrong type",
        "field operator names no run in the field day",
    ]


def test_an_empty_runner_is_named_with_its_index():
    day, defects = load_field_day_text(day_text(runs=[{"runner": ""}]))
    assert defects == [
        "run 0 field runner is empty",
        "field operator names no run in the field day",
    ]


def test_a_runner_holding_an_unpaired_surrogate_is_named():
    day, defects = load_field_day_text(day_text(runs=[{"runner": "fo\ud800rk"}]))
    assert defects == [
        "run 0 field runner holds an unpaired surrogate",
        "field operator names no run in the field day",
    ]


def test_a_runner_holding_a_line_break_is_named():
    day, defects = load_field_day_text(
        day_text(runs=[{"runner": "fork\nrecord at 0 seconds, forged"}])
    )
    assert defects == [
        "run 0 field runner holds a line break",
        "field operator names no run in the field day",
    ]


def test_a_reads_field_of_the_wrong_type_is_named_with_its_run_index():
    runs = [{"runner": "operator", "usage": 5, "reads": "soon"}]
    day, defects = load_field_day_text(day_text(runs=runs))
    assert defects == ["run 0 field reads has the wrong type"]


def test_an_empty_reads_list_is_named_with_its_run_index():
    runs = [{"runner": "operator", "usage": 5, "reads": []}]
    day, defects = load_field_day_text(day_text(runs=runs))
    assert defects == ["run 0 field reads is empty"]


def test_a_read_defect_carries_the_run_index_before_the_read_index():
    runs = [
        {
            "runner": "operator",
            "usage": 5,
            "reads": [{"elapsed_seconds": -1, "usage": 0}],
        }
    ]
    day, defects = load_field_day_text(day_text(runs=runs))
    assert defects == ["run 0 read 0 field elapsed_seconds is negative"]


def test_a_read_usage_that_decreases_is_named_under_its_run():
    runs = [
        {
            "runner": "operator",
            "usage": 5,
            "reads": [
                {"elapsed_seconds": 100, "usage": 50},
                {"elapsed_seconds": 200, "usage": 40},
            ],
        }
    ]
    day, defects = load_field_day_text(day_text(runs=runs))
    assert defects == ["run 0 read 1 field usage is less than the previous read"]


def test_reads_without_usage_draw_the_missing_usage_defect():
    runs = [{"runner": "operator", "reads": [{"elapsed_seconds": 1, "usage": 1}]}]
    day, defects = load_field_day_text(day_text(runs=runs))
    assert defects == ["run 0 missing field usage"]


def test_a_negative_usage_is_named_under_its_run():
    runs = [{"runner": "operator", "usage": -5}]
    day, defects = load_field_day_text(day_text(runs=runs))
    assert defects == ["run 0 field usage is negative"]


def test_a_repeated_label_is_named_with_both_run_indexes():
    runs = [timed("fork", [(100, 10)]), timed("fork", [(200, 20)])]
    day, defects = load_field_day_text(
        day_text(operator="fork", runs=runs)
    )
    assert defects == ["run 1 field runner duplicates the label of run 0"]


def test_an_operator_naming_no_run_is_named():
    day, defects = load_field_day_text(day_text(operator="nobody"))
    assert defects == ["field operator names no run in the field day"]


def test_one_input_can_draw_several_defects_in_one_pass():
    day, defects = load_field_day_text(
        day_text(window_seconds=-1, runs=[{"runner": ""}, {"runner": "x"}, 5])
    )
    assert "field window_seconds must be positive" in defects
    assert "run 0 field runner is empty" in defects
    assert "run 2 is not an object" in defects
    assert "field operator names no run in the field day" in defects


# The report walk


def test_the_header_names_the_window_the_count_and_the_operator():
    lines = report_for([timed("operator", [(300, 150)])])
    assert lines[0] == "field day, window 1800 seconds, 1 runs, operator operator"


def test_the_operator_keeps_the_record_and_the_margin_is_the_gap():
    runs = [
        timed("operator", [(300, 150)]),
        timed("fork", [(300, 200)]),
    ]
    lines = report_for(runs)
    assert lines == [
        "field day, window 1800 seconds, 2 runs, operator operator",
        "record at 300 seconds, operator at a projection of 900",
        "finish, record held by operator at a projection of 900",
        "record kept by operator to the close, margin 300 at the close",
    ]


def test_the_operator_keeping_a_sole_record_states_no_margin():
    lines = report_for([timed("operator", [(300, 150)])])
    assert lines[-1] == "record kept by operator to the close"


def test_an_equal_projection_never_takes_the_record():
    runs = [
        timed("operator", [(300, 150)]),
        timed("fork", [(600, 300)]),
    ]
    lines = report_for(runs)
    assert lines == [
        "field day, window 1800 seconds, 2 runs, operator operator",
        "record at 300 seconds, operator at a projection of 900",
        "finish, record held by operator at a projection of 900",
        "record kept by operator to the close, margin 0 at the close",
    ]


def test_a_tie_at_the_first_mark_breaks_toward_the_earlier_run():
    runs = [
        timed("fork", [(300, 150)]),
        timed("operator", [(300, 150)]),
    ]
    lines = report_for(runs)
    assert lines[1] == "record at 300 seconds, fork at a projection of 900"
    assert lines[-1] == (
        "record never held by operator, margin 0 at the close"
    )


def test_a_displaced_record_goes_to_the_figure_set_at_the_earliest_mark():
    runs = [
        timed("operator", [(300, 150), (600, 340)]),
        timed("r2", [(500, 250)]),
        timed("r1", [(400, 200)]),
    ]
    lines = report_for(runs)
    assert lines == [
        "field day, window 1800 seconds, 3 runs, operator operator",
        "record at 300 seconds, operator at a projection of 900",
        "record at 600 seconds, r1 at a projection of 900",
        "finish, record held by r1 at a projection of 900",
        "record stolen from operator at 600 seconds by r1, "
        "margin 120 at the close",
    ]


def test_the_record_is_stolen_and_the_margin_is_read_at_the_close():
    runs = [
        timed("operator", [(300, 150), (900, 500)]),
        timed("fork", [(600, 250)]),
    ]
    lines = report_for(runs)
    assert lines == [
        "field day, window 1800 seconds, 2 runs, operator operator",
        "record at 300 seconds, operator at a projection of 900",
        "record at 600 seconds, fork at a projection of 750",
        "finish, record held by fork at a projection of 750",
        "record stolen from operator at 600 seconds by fork, "
        "margin 250 at the close",
    ]


def test_a_regained_record_that_is_kept_reports_the_kept_line():
    runs = [
        timed("operator", [(300, 150), (700, 300)]),
        timed("fork", [(500, 220), (900, 430)]),
    ]
    lines = report_for(runs)
    assert lines == [
        "field day, window 1800 seconds, 2 runs, operator operator",
        "record at 300 seconds, operator at a projection of 900",
        "record at 500 seconds, fork at a projection of 792",
        "record at 700 seconds, operator at a projection of 771.428571",
        "finish, record held by operator at a projection of 771.428571",
        "record kept by operator to the close, margin 88.571429 at the close",
    ]


def test_a_steal_after_a_regain_names_the_last_away_change():
    runs = [
        timed("operator", [(300, 150), (700, 270), (1200, 700)]),
        timed("fork", [(500, 200), (900, 340)]),
    ]
    lines = report_for(runs)
    assert lines[-2] == "finish, record held by fork at a projection of 680"
    assert lines[-1] == (
        "record stolen from operator at 900 seconds by fork, "
        "margin 370 at the close"
    )


def test_the_holder_rising_past_a_rival_hands_the_record_over():
    runs = [
        timed("operator", [(300, 150), (600, 340)]),
        timed("fork", [(400, 210)]),
    ]
    lines = report_for(runs)
    assert lines == [
        "field day, window 1800 seconds, 2 runs, operator operator",
        "record at 300 seconds, operator at a projection of 900",
        "record at 600 seconds, fork at a projection of 945",
        "finish, record held by fork at a projection of 945",
        "record stolen from operator at 600 seconds by fork, "
        "margin 75 at the close",
    ]


def test_the_holder_improving_its_own_figure_moves_the_record_line():
    runs = [
        timed("operator", [(300, 150)]),
        timed("fork", [(400, 200), (800, 300)]),
    ]
    lines = report_for(runs)
    assert lines[1] == "record at 300 seconds, operator at a projection of 900"
    assert lines[2] == "record at 800 seconds, fork at a projection of 675"
    assert lines[-1] == (
        "record stolen from operator at 800 seconds by fork, "
        "margin 225 at the close"
    )


def test_a_zero_elapsed_read_projects_no_figure():
    runs = [
        timed("operator", [(0, 0), (300, 150)]),
        timed("fork", [(0, 500)]),
    ]
    lines = report_for(runs)
    assert lines[1] == "record at 300 seconds, operator at a projection of 900"
    assert lines[-1] == "record kept by operator to the close"


def test_no_record_stands_when_no_read_projects_a_figure():
    runs = [timed("operator", [(0, 0)]), timed("fork", None, usage=10)]
    lines = report_for(runs)
    assert lines == [
        "field day, window 1800 seconds, 2 runs, operator operator",
        "finish, no record was set",
    ]


def test_a_run_with_no_reads_never_holds_the_record_on_its_usage():
    runs = [
        timed("operator", [(600, 400)]),
        timed("fork", None, usage=1),
    ]
    lines = report_for(runs)
    assert lines[-2] == "finish, record held by operator at a projection of 1200"
    assert lines[-1] == "record kept by operator to the close"


def test_the_operator_who_never_held_reads_the_margin_at_the_close():
    runs = [
        timed("operator", [(600, 400)]),
        timed("fork", [(300, 100)]),
    ]
    lines = report_for(runs)
    assert lines == [
        "field day, window 1800 seconds, 2 runs, operator operator",
        "record at 300 seconds, fork at a projection of 600",
        "finish, record held by fork at a projection of 600",
        "record never held by operator, margin 600 at the close",
    ]


def test_the_closing_margin_can_be_zero_when_the_figures_tie():
    runs = [
        timed("operator", [(300, 200), (700, 350)]),
        timed("fork", [(500, 250)]),
    ]
    lines = report_for(runs)
    assert lines[-2] == "finish, record held by fork at a projection of 900"
    assert lines[-1] == (
        "record stolen from operator at 500 seconds by fork, "
        "margin 0 at the close"
    )


def test_a_projection_renders_at_full_precision_below_the_rounding_bound():
    runs = [timed("operator", [(700, 430)]), timed("fork", [(300, 200)])]
    lines = report_for(runs)
    assert (
        "record at 700 seconds, operator at a projection of 1105.714286"
        in lines
    )


# The report on a value the loader refuses, the discipline of FIX-PG-8
def test_the_report_raises_a_value_error_when_the_operator_names_no_run():
    day = {
        "window_seconds": 1800,
        "operator": "ghost",
        "runs": [timed("operator", [(300, 150)]), timed("fork", [(600, 320)])],
    }
    with pytest.raises(
        ValueError, match="the operator names no run among the runs"
    ):
        field_day_report(day)
