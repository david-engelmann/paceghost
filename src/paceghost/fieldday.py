"""The field day and the stolen record, for paceghost.

The field day loader never raises on malformed content and names
every defect it finds, reusing the phrases of the recorded run
shape and of the Round 2 run extension under a run index prefix.
The field day report is a pure function of the loaded day, and
every projection is computed on exact values and rendered under
the rules the contract already states. The obligations this module
carries are PG-7 and PG-8 in CONTRACT.md, in the Round 3 section.
"""

import json
import pathlib
from fractions import Fraction

from paceghost.runs import (
    _check_text_field,
    _is_finite_number,
    _is_number,
    _load_reads,
    _load_usage_value,
    format_seconds,
)
from paceghost.season import format_usage


def _check_label(data, name, defects, prefix=""):
    """Validate a label field and return its value, if sound.

    The phrases are the ones _check_text_field draws, plus the line
    break phrase, because a label reaches the printer in the field
    day report and a label holding a line break would forge a
    report line.
    """
    value = _check_text_field(data, name, defects, prefix)
    if value is None:
        return None
    if "\n" in value or "\r" in value:
        defects.append(f"{prefix}field {name} holds a line break")
        return None
    return value


def _load_day_run(entry, index, defects):
    """Validate one run object of a field day.

    Returns a pair of the clean run, or None when this run drew any
    defect, and the label of the run, or None when the label itself
    drew a defect. The label is reported even for a run that drew
    other defects, so the duplicate and operator checks can read it.
    """
    prefix = f"run {index} "
    if not isinstance(entry, dict):
        defects.append(f"{prefix}is not an object")
        return None, None
    before = len(defects)
    label = _check_label(entry, "runner", defects, prefix)
    usage = _load_usage_value(entry, "usage", defects, prefix)
    reads = _load_reads(entry, defects, prefix)
    if "reads" in entry and "usage" not in entry:
        defects.append(f"{prefix}missing field usage")
    if len(defects) > before:
        return None, label
    run = {"runner": label}
    if "usage" in entry:
        run["usage"] = usage
    if "reads" in entry:
        run["reads"] = reads
    return run, label


def load_field_day_text(text):
    """Parse field day text into a field day, naming every defect.

    Returns a pair of the field day and the defect list. A malformed
    input returns no field day, and this function never raises on
    bad content.
    """
    try:
        data = json.loads(text)
    except (ValueError, RecursionError):
        return None, ["input is not valid JSON"]
    if not isinstance(data, dict):
        return None, ["input is not a JSON object"]
    defects = []
    window = None
    if "window_seconds" not in data:
        defects.append("missing field window_seconds")
    elif not _is_number(data["window_seconds"]):
        defects.append("field window_seconds has the wrong type")
    elif not _is_finite_number(data["window_seconds"]):
        defects.append("field window_seconds is not a finite number")
    elif data["window_seconds"] <= 0:
        defects.append("field window_seconds must be positive")
    else:
        window = data["window_seconds"]
    operator = _check_label(data, "operator", defects)
    runs = None
    labels = []
    if "runs" not in data:
        defects.append("missing field runs")
    elif not isinstance(data["runs"], list):
        defects.append("field runs has the wrong type")
    elif len(data["runs"]) == 0:
        defects.append("field runs is empty")
    else:
        runs = []
        for index, entry in enumerate(data["runs"]):
            run, label = _load_day_run(entry, index, defects)
            runs.append(run)
            labels.append(label)
        first_seen = {}
        for index, label in enumerate(labels):
            if label is None:
                continue
            if label in first_seen:
                defects.append(
                    f"run {index} field runner duplicates "
                    f"the label of run {first_seen[label]}"
                )
            else:
                first_seen[label] = index
        if operator is not None and operator not in first_seen:
            defects.append("field operator names no run in the field day")
    if defects:
        return None, defects
    day = {"window_seconds": window, "operator": operator, "runs": runs}
    return day, []


def load_field_day_file(path):
    """Read a field day from disk, with the same contract as text."""
    try:
        text = pathlib.Path(path).read_text(encoding="utf-8")
    except (OSError, UnicodeError):
        return None, ["file cannot be read"]
    return load_field_day_text(text)


def field_day_report(day):
    """Return the stolen record lines for one loaded field day.

    The walk reads every run through its timed reads in elapsed
    order. A run projects the final usage of PG-6 from each read,
    the record after each mark is the lowest standing projection,
    and the holder keeps the record while it stands at the record
    figure, so an equal figure alone never moves the record.
    """
    window = Fraction(day["window_seconds"])
    runs = day["runs"]
    operator = day["operator"]
    operator_index = next(
        index for index, run in enumerate(runs) if run["runner"] == operator
    )
    reads_by_run = []
    marks = set()
    for run in runs:
        by_elapsed = {}
        for read in run.get("reads", []):
            elapsed = Fraction(read["elapsed_seconds"])
            if elapsed > 0:
                by_elapsed[elapsed] = Fraction(read["usage"])
                marks.add(elapsed)
        reads_by_run.append(by_elapsed)
    lines = [
        f"field day, window {format_seconds(day['window_seconds'])} "
        f"seconds, {len(runs)} runs, operator {operator}"
    ]
    standing = [None] * len(runs)
    set_at = [None] * len(runs)
    holder = None
    emitted = None
    operator_held = False
    last_stolen = None
    for mark in sorted(marks):
        for index in range(len(runs)):
            usage = reads_by_run[index].get(mark)
            if usage is not None:
                standing[index] = usage * window / mark
                set_at[index] = mark
        figures = [figure for figure in standing if figure is not None]
        if not figures:
            continue
        record = min(figures)
        if holder is not None and standing[holder] == record:
            new_holder = holder
        else:
            candidates = [
                index for index, figure in enumerate(standing)
                if figure == record
            ]
            new_holder = min(candidates, key=lambda index: (set_at[index], index))
        if new_holder != holder:
            if holder == operator_index:
                last_stolen = (mark, new_holder)
            holder = new_holder
        if holder == operator_index:
            operator_held = True
        if emitted != (holder, record):
            lines.append(
                f"record at {format_seconds(mark)} seconds, "
                f"{runs[holder]['runner']} at a projection of "
                f"{format_usage(record)}"
            )
            emitted = (holder, record)
    if holder is None:
        lines.append("finish, no record was set")
        return lines
    record = standing[holder]
    lines.append(
        f"finish, record held by {runs[holder]['runner']} "
        f"at a projection of {format_usage(record)}"
    )
    if holder == operator_index:
        others = [
            figure
            for index, figure in enumerate(standing)
            if index != operator_index and figure is not None
        ]
        line = f"record kept by {operator} to the close"
        if others:
            line += f", margin {format_usage(min(others) - record)} at the close"
        lines.append(line)
    elif operator_held:
        mark, thief = last_stolen
        margin = standing[operator_index] - record
        lines.append(
            f"record stolen from {operator} at {format_seconds(mark)} "
            f"seconds by {runs[thief]['runner']}, "
            f"margin {format_usage(margin)} at the close"
        )
    elif standing[operator_index] is not None:
        margin = standing[operator_index] - record
        lines.append(
            f"record never held by {operator}, "
            f"margin {format_usage(margin)} at the close"
        )
    else:
        lines.append(
            f"record never held by {operator}, whose run projected no figure"
        )
    return lines
