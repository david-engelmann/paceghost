"""Tests for the Round 4 example ladder and its golden report."""

import json
import pathlib

from paceghost.cli import main
from paceghost.fieldday import load_field_day_file
from paceghost.ladder import ladder_report, load_ladder_file

EXAMPLES = pathlib.Path(__file__).resolve().parent.parent / "examples"


def load_example():
    ladder, defects = load_ladder_file(EXAMPLES / "ladder.json")
    assert defects == []
    return ladder


def sections_of(lines):
    sections = []
    for line in lines:
        if line.startswith("ladder, "):
            continue
        if line.startswith("field day, window "):
            sections.append([])
        sections[-1].append(line)
    return sections


def day_as_it_stood(day, window):
    stood = {"window_seconds": window, "operator": day["operator"], "runs": []}
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
    return stood


def test_the_example_ladder_loads_clean():
    ladder = load_example()
    assert ladder["windows"] == [600, 900, 1800]
    assert ladder["day"]["operator"] == "operator"
    assert ladder["day"]["window_seconds"] == 1800
    assert len(ladder["day"]["runs"]) == 4


def test_the_example_embeds_the_round_three_field_day_whole():
    ladder = load_example()
    day, defects = load_field_day_file(EXAMPLES / "field-day.json")
    assert defects == []
    assert ladder["day"] == day


def test_the_example_report_matches_the_golden_bytes():
    ladder = load_example()
    golden = (EXAMPLES / "ladder-golden.txt").read_bytes()
    rendered = "\n".join(ladder_report(ladder)) + "\n"
    assert rendered.encode("utf-8") == golden


def test_the_ladder_command_prints_the_golden_report(capsys):
    assert main(["ladder", str(EXAMPLES / "ladder.json")]) == 0
    golden = (EXAMPLES / "ladder-golden.txt").read_bytes()
    assert capsys.readouterr().out.encode("utf-8") == golden


def test_the_widest_section_matches_the_round_three_golden():
    ladder = load_example()
    sections = sections_of(ladder_report(ladder))
    golden = (EXAMPLES / "field-day-golden.txt").read_text(
        encoding="utf-8"
    ).splitlines()
    assert sections[2] == golden


def test_the_closing_holder_changes_across_the_ladder():
    ladder = load_example()
    lines = ladder_report(ladder)
    finishes = [
        line for line in lines if line.startswith("finish, record held by")
    ]
    assert finishes == [
        "finish, record held by dawn fork at a projection of 400",
        "finish, record held by night fork at a projection of 470",
        "finish, record held by night fork at a projection of 840",
    ]
    assert lines[-1] == (
        "ladder, the closing holder differs from the first window "
        "at 900 seconds, 1800 seconds"
    )


def test_each_section_matches_the_fieldday_command_at_that_window(
    capsys, tmp_path
):
    ladder = load_example()
    sections = sections_of(ladder_report(ladder))
    for window, section in zip(ladder["windows"], sections):
        path = tmp_path / f"day-{window}.json"
        stood = day_as_it_stood(ladder["day"], window)
        path.write_text(json.dumps(stood), encoding="utf-8")
        assert main(["fieldday", str(path)]) == 0
        printed = capsys.readouterr().out.splitlines()
        assert printed == section
