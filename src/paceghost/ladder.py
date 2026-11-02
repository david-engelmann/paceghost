"""The window ladder over one field day, for paceghost.

The ladder loader never raises on malformed content and names
every defect it finds, running the field day validation over the
day the ladder file embeds and prefixing its defects with the
word day, and naming the window list in phrases of the same
shape. The ladder report reads the day as it stood at each
window in the list through the same record walk that computes
the field day report, and closes with one summary line over the
closing holders. The obligations this module carries are PG-9
and PG-10 in CONTRACT.md, in the Round 4 section.
"""

import json
import pathlib
from fractions import Fraction

from paceghost.fieldday import _load_field_day_data, _record_walk
from paceghost.runs import _is_finite_number, _is_number, format_seconds


def _load_windows(data, defects):
    """Validate the windows field of a ladder and return its list.

    Returns the window lengths as whole numbers in list order, or
    None when the field is absent or of the wrong kind or empty.
    A length that draws a defect takes no place in the list, and
    the duplicate check reads only the lengths that loaded clean.
    """
    if "windows" not in data:
        defects.append("missing field windows")
        return None
    raw = data["windows"]
    if not isinstance(raw, list):
        defects.append("field windows has the wrong type")
        return None
    if len(raw) == 0:
        defects.append("field windows is empty")
        return None
    windows = []
    first_seen = {}
    for index, value in enumerate(raw):
        prefix = f"window {index} "
        if not _is_number(value):
            defects.append(f"{prefix}has the wrong type")
            continue
        if not _is_finite_number(value):
            defects.append(f"{prefix}is not a finite number")
            continue
        if value <= 0:
            defects.append(f"{prefix}must be positive")
            continue
        fraction = Fraction(value)
        if fraction.denominator != 1:
            defects.append(f"{prefix}is not a whole number")
            continue
        whole = int(fraction)
        if whole in first_seen:
            defects.append(
                f"{prefix}duplicates window {first_seen[whole]}"
            )
            continue
        first_seen[whole] = index
        windows.append(whole)
    return windows


def load_ladder_text(text):
    """Parse ladder text into a ladder, naming every defect.

    Returns a pair of the ladder and the defect list. A malformed
    input returns no ladder, and this function never raises on
    bad content.
    """
    try:
        data = json.loads(text)
    except (ValueError, RecursionError):
        return None, ["input is not valid JSON"]
    if not isinstance(data, dict):
        return None, ["input is not a JSON object"]
    defects = []
    day = None
    if "day" not in data:
        defects.append("missing field day")
    elif not isinstance(data["day"], dict):
        defects.append("field day is not an object")
    else:
        day, day_defects = _load_field_day_data(data["day"])
        defects.extend(f"day {defect}" for defect in day_defects)
    windows = _load_windows(data, defects)
    if defects:
        return None, defects
    return {"day": day, "windows": windows}, []


def load_ladder_file(path):
    """Read a ladder from disk, with the same contract as text."""
    try:
        text = pathlib.Path(path).read_text(encoding="utf-8")
    except (OSError, UnicodeError):
        return None, ["file cannot be read"]
    return load_ladder_text(text)


def ladder_report(ladder):
    """Return the ladder lines for one loaded ladder.

    One section stands per window in list order, each computed by
    the record walk with the window standing as the horizon, so
    the section is the field day report of the day as it stood at
    that window. The summary line names every window after the
    first whose closing holder is not the closing holder of the
    first window, and reads that the closing holder never changed
    when no window after the first differs. Handed a value the
    loader would refuse, the report raises a ValueError naming
    the broken precondition, an empty windows list here and,
    through the walk, an operator naming no run among the runs,
    in the discipline of FIX-PG-8.
    """
    if not ladder["windows"]:
        raise ValueError("the ladder holds no windows")
    day = ladder["day"]
    lines = []
    holders = []
    for window in ladder["windows"]:
        section, holder = _record_walk(
            day["runs"], day["operator"], window, Fraction(window)
        )
        lines.extend(section)
        holders.append(holder)
    first = holders[0]
    differing = [
        ladder["windows"][index]
        for index in range(1, len(holders))
        if holders[index] != first
    ]
    if not differing:
        lines.append("ladder, the closing holder never changed across the ladder")
    else:
        named = ", ".join(
            f"{format_seconds(window)} seconds" for window in differing
        )
        lines.append(
            f"ladder, the closing holder differs from the first window at {named}"
        )
    return lines
