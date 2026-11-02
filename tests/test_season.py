"""Pins for PG-5 in CONTRACT.md, the season over a recorded field.

The field loader names every defect with its run index standing
before the PG-1 phrase, and the season figures are computed by hand
here so a report that grouped, counted, or ordered differently
would fail.
"""

import json
import pathlib
from fractions import Fraction

from paceghost.season import format_usage, load_field_text, season_report

ROOT = pathlib.Path(__file__).resolve().parent.parent


def run_data(model, total, window=200, usage=40, budget=50, task="t", **extra):
    data = {
        "task": task,
        "window_seconds": window,
        "runner": "someone",
        "recorded_at": "2026-01-04",
        "splits": [{"phase": "only", "elapsed_seconds": total}],
        "usage": usage,
        "budget": budget,
        "model": model,
    }
    data.update(extra)
    return data


def load_field(runs):
    return load_field_text(json.dumps({"runs": runs}))


def ghost(usage=45):
    return {"task": "t", "usage": usage}


def test_contract_states_pg5_and_pg6_in_full_form():
    text = (ROOT / "CONTRACT.md").read_text(encoding="utf-8")
    assert "PG-5 full form" in text
    assert "PG-6 full form" in text


def test_a_clean_field_loads_with_models_attached():
    runs, defects = load_field([run_data("peak", 100), run_data("vale", 120)])
    assert defects == []
    assert [run["model"] for run in runs] == ["peak", "vale"]
    assert runs[0]["total_seconds"] == 100


def test_field_text_that_is_not_json_draws_the_shared_defect():
    runs, defects = load_field_text("this is not json")
    assert runs is None
    assert defects == ["input is not valid JSON"]


def test_field_top_level_list_draws_the_not_an_object_defect():
    runs, defects = load_field_text("[]")
    assert runs is None
    assert defects == ["input is not a JSON object"]


def test_field_without_runs_draws_missing_field_runs():
    runs, defects = load_field_text("{}")
    assert runs is None
    assert defects == ["missing field runs"]


def test_field_runs_of_the_wrong_type_is_named():
    runs, defects = load_field_text('{"runs": {}}')
    assert runs is None
    assert defects == ["field runs has the wrong type"]


def test_field_runs_empty_is_named():
    runs, defects = load_field_text('{"runs": []}')
    assert runs is None
    assert defects == ["field runs is empty"]


def test_a_run_that_is_not_an_object_is_named_by_index():
    runs, defects = load_field([run_data("peak", 100), 42])
    assert runs is None
    assert defects == ["run 1 is not an object"]


def test_a_run_defect_carries_the_run_index_before_the_pg1_phrase():
    bad = run_data("peak", 100)
    del bad["task"]
    runs, defects = load_field([run_data("vale", 100), bad])
    assert runs is None
    assert defects == ["run 1 missing field task"]


def test_a_split_defect_inside_a_run_keeps_both_indexes():
    bad = run_data("peak", 100, splits=[{"phase": "", "elapsed_seconds": 5}])
    runs, defects = load_field([bad])
    assert runs is None
    assert defects == ["run 0 split 0 field phase is empty"]


def test_a_run_without_a_model_is_named():
    bad = run_data("peak", 100)
    del bad["model"]
    runs, defects = load_field([bad])
    assert runs is None
    assert defects == ["run 0 missing field model"]


def test_a_model_of_the_wrong_type_is_named():
    runs, defects = load_field([run_data(7, 100)])
    assert runs is None
    assert defects == ["run 0 field model has the wrong type"]


def test_an_empty_model_is_named():
    runs, defects = load_field([run_data("", 100)])
    assert runs is None
    assert defects == ["run 0 field model is empty"]


def test_a_model_holding_an_unpaired_surrogate_is_named():
    runs, defects = load_field([run_data("pea\ud800k", 100)])
    assert runs is None
    assert defects == ["run 0 field model holds an unpaired surrogate"]


def test_a_run_without_usage_is_named_in_a_field():
    bad = run_data("peak", 100)
    del bad["usage"]
    runs, defects = load_field([bad])
    assert runs is None
    assert defects == ["run 0 missing field usage"]


def test_a_run_without_budget_is_named_in_a_field():
    bad = run_data("peak", 100)
    del bad["budget"]
    runs, defects = load_field([bad])
    assert runs is None
    assert defects == ["run 0 missing field budget"]


def test_a_run_with_a_different_task_is_named():
    runs, defects = load_field(
        [run_data("peak", 100), run_data("vale", 100, task="other")]
    )
    assert runs is None
    assert defects == ["run 1 field task does not match the field task"]


def test_the_season_figures_are_the_hand_computed_ones():
    field = [
        run_data("peak", 100, usage=40, budget=50),
        run_data("peak", 250, usage=30, budget=50),
        run_data("vale", 150, usage=44, budget=40),
        run_data("vale", 120, usage=20, budget=40),
        run_data("wick", 90, usage=44, budget=100),
    ]
    runs, defects = load_field(field)
    assert defects == []
    assert season_report(runs, ghost()) == [
        "season for task t",
        "model vale",
        "runs raced 2",
        "inside window 2",
        "inside budget 1",
        "total usage 64",
        "mean usage 32",
        "head to head against the ghost, wins 2 of 2",
        "model wick",
        "runs raced 1",
        "inside window 1",
        "inside budget 1",
        "total usage 44",
        "mean usage 44",
        "head to head against the ghost, wins 1 of 1",
        "model peak",
        "runs raced 2",
        "inside window 1",
        "inside budget 2",
        "total usage 70",
        "mean usage 35",
        "head to head against the ghost, wins 1 of 2",
    ]


def test_a_win_needs_the_window_and_less_usage_than_the_ghost():
    field = [
        run_data("solo", 250, usage=10, budget=50),
        run_data("solo", 100, usage=45, budget=50),
        run_data("solo", 100, usage=44, budget=50),
    ]
    runs, defects = load_field(field)
    assert defects == []
    lines = season_report(runs, ghost(usage=45))
    assert lines[-1] == "head to head against the ghost, wins 1 of 3"


def test_ordering_breaks_a_wins_and_total_tie_by_model_name():
    field = [run_data("beta", 50, usage=10), run_data("alpha", 50, usage=10)]
    runs, defects = load_field(field)
    assert defects == []
    lines = season_report(runs, ghost())
    assert lines[1] == "model alpha"
    assert lines[8] == "model beta"


def test_the_mean_renders_at_six_decimal_places():
    field = [
        run_data("trio", 100, usage=40),
        run_data("trio", 100, usage=30),
        run_data("trio", 100, usage=30),
    ]
    runs, defects = load_field(field)
    assert defects == []
    lines = season_report(runs, ghost())
    assert "total usage 100" in lines
    assert "mean usage 33.333333" in lines


def test_usage_figures_render_exactly():
    assert format_usage(0) == "0"
    assert format_usage(2170) == "2170"
    assert format_usage(2.5) == "2.5"
    assert format_usage(0.1 + 0.2) == "0.3"
    assert format_usage(1 / 3) == "0.333333"
    assert format_usage(Fraction(4, 10000000)) == "0.0000004"


def test_an_exact_half_at_the_seventh_place_rounds_up():
    assert format_usage(Fraction(1, 2000000)) == "0.000001"
    assert format_usage(Fraction(20000005, 10000000)) == "2.000001"
    assert format_usage(Fraction(10000005, 10000000)) == "1.000001"
    assert format_usage(Fraction(25, 10000000)) == "0.000003"
    assert format_usage(Fraction(15, 10000000)) == "0.000002"
    assert format_usage(Fraction(1234567891234565, 10000000)) == "123456789.123457"


def test_a_figure_above_half_the_smallest_unit_rounds_at_six_places():
    assert format_usage(Fraction(9, 10000000)) == "0.000001"
    assert format_usage(Fraction(2, 3000000)) == "0.000001"


def test_a_tiny_figure_renders_its_exact_expansion_when_it_terminates():
    assert format_usage(Fraction(1, 10000000)) == "0.0000001"
    assert format_usage(Fraction(1, 2000000000000)) == "0.0000000000005"


def test_a_tiny_figure_that_never_terminates_renders_at_twelve_places():
    assert format_usage(Fraction(1, 30000000)) == "0.000000033333"


def test_a_season_mean_too_small_for_six_places_renders_at_twelve():
    field = [
        run_data("peak", 100, usage=0.0000001),
        run_data("peak", 100, usage=0),
        run_data("peak", 100, usage=0),
    ]
    runs, defects = load_field(field)
    assert defects == []
    lines = season_report(runs, ghost())
    assert "mean usage 0.000000033333" in lines


def test_a_field_run_with_a_negative_usage_is_named_under_its_index():
    runs, defects = load_field([run_data("peak", 100, usage=-1)])
    assert runs is None
    assert defects == ["run 0 field usage is negative"]


def test_a_field_run_with_a_budget_of_zero_is_named_under_its_index():
    runs, defects = load_field([run_data("peak", 100, budget=0)])
    assert runs is None
    assert defects == ["run 0 field budget must be positive"]


def test_a_malformed_read_inside_a_field_run_keeps_both_indexes():
    bad = run_data(
        "peak",
        100,
        reads=[
            {"elapsed_seconds": 10, "usage": 5},
            {"elapsed_seconds": 20, "usage": 4},
        ],
    )
    runs, defects = load_field([bad])
    assert runs is None
    assert defects == ["run 0 read 1 field usage is less than the previous read"]


def test_the_season_report_is_byte_deterministic():
    field = [run_data("peak", 100), run_data("vale", 120, usage=20)]
    runs, defects = load_field(field)
    assert defects == []
    first = season_report(runs, ghost())
    second = season_report(runs, ghost())
    assert first == second
