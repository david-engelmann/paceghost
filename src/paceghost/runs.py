"""Recorded runs, the ghost rule, and the race report for paceghost.

Every function in this module is a pure function of its inputs, and the
loader never raises on malformed content. The obligations this module
carries are PG-1 to PG-4 in CONTRACT.md, in the full form section.
"""

import datetime
import json
import math
import pathlib
import re


def _is_number(value):
    return isinstance(value, (int, float)) and not isinstance(value, bool)


def _is_finite_number(value):
    """Return whether a number is finite, without ever raising.

    An int too large to convert to float counts as not finite, so the
    loader names it with the finite number defect instead of raising
    OverflowError on it.
    """
    try:
        return math.isfinite(value)
    except OverflowError:
        return False


def _holds_unpaired_surrogate(text):
    """Return whether a string fails UTF-8 encoding.

    A string holding an unpaired surrogate cannot be encoded, and a
    recorded run carrying one cannot be printed, so the loader names
    it as a defect instead of letting it reach the printer.
    """
    try:
        text.encode("utf-8")
    except UnicodeEncodeError:
        return True
    return False


_RECORDED_AT_SHAPE = re.compile(r"[0-9]{4}-[0-9]{2}-[0-9]{2}")


def format_seconds(value):
    """Render a number of seconds the same way everywhere it appears.

    Values render exactly at up to six decimal places. A nonzero
    value that rounds to zero at that precision renders at full
    precision instead, so a lead never prints as zero.
    """
    number = round(float(value), 6)
    if number == 0 and float(value) != 0:
        return repr(float(value))
    if number == int(number):
        return str(int(number))
    return f"{number:.6f}".rstrip("0").rstrip(".")


def _check_text_field(data, name, defects):
    if name not in data:
        defects.append(f"missing field {name}")
        return None
    value = data[name]
    if not isinstance(value, str):
        defects.append(f"field {name} has the wrong type")
        return None
    if value == "":
        defects.append(f"field {name} is empty")
        return None
    if _holds_unpaired_surrogate(value):
        defects.append(f"field {name} holds an unpaired surrogate")
        return None
    return value


def _load_splits(data, defects):
    """Validate the splits field and return the clean splits, if any."""
    if "splits" not in data:
        defects.append("missing field splits")
        return None
    raw = data["splits"]
    if not isinstance(raw, list):
        defects.append("field splits has the wrong type")
        return None
    if len(raw) == 0:
        defects.append("field splits is empty")
        return None
    clean = []
    last_valid = None
    for index, entry in enumerate(raw):
        if not isinstance(entry, dict):
            defects.append(f"split {index} is not an object")
            continue
        phase = None
        elapsed = None
        if "phase" not in entry:
            defects.append(f"split {index} is missing field phase")
        elif not isinstance(entry["phase"], str):
            defects.append(f"split {index} field phase has the wrong type")
        elif entry["phase"] == "":
            defects.append(f"split {index} field phase is empty")
        elif _holds_unpaired_surrogate(entry["phase"]):
            defects.append(
                f"split {index} field phase holds an unpaired surrogate"
            )
        else:
            phase = entry["phase"]
        if "elapsed_seconds" not in entry:
            defects.append(f"split {index} is missing field elapsed_seconds")
        elif not _is_number(entry["elapsed_seconds"]):
            defects.append(
                f"split {index} field elapsed_seconds has the wrong type"
            )
        elif not _is_finite_number(entry["elapsed_seconds"]):
            defects.append(
                f"split {index} field elapsed_seconds is not a finite number"
            )
        elif entry["elapsed_seconds"] < 0:
            defects.append(
                f"split {index} field elapsed_seconds is negative"
            )
        else:
            elapsed = entry["elapsed_seconds"]
            if last_valid is not None and elapsed <= last_valid:
                defects.append(
                    f"split {index} field elapsed_seconds is not greater "
                    "than the previous split"
                )
                elapsed = None
            else:
                last_valid = elapsed
        if phase is not None and elapsed is not None:
            clean.append({"phase": phase, "elapsed_seconds": elapsed})
    if len(clean) != len(raw):
        return None
    return clean


def load_run_text(text):
    """Parse recorded run text into a run, naming every defect found.

    Returns a pair of the run and the defect list. A malformed input
    returns no run, and this function never raises on bad content.
    """
    try:
        data = json.loads(text)
    except (ValueError, RecursionError):
        return None, ["input is not valid JSON"]
    if not isinstance(data, dict):
        return None, ["input is not a JSON object"]
    defects = []
    task = _check_text_field(data, "task", defects)
    runner = _check_text_field(data, "runner", defects)
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
    recorded_at = None
    if "recorded_at" not in data:
        defects.append("missing field recorded_at")
    elif not isinstance(data["recorded_at"], str):
        defects.append("field recorded_at has the wrong type")
    elif not _RECORDED_AT_SHAPE.fullmatch(data["recorded_at"]):
        defects.append("field recorded_at is not an ISO date")
    else:
        try:
            datetime.date.fromisoformat(data["recorded_at"])
            recorded_at = data["recorded_at"]
        except ValueError:
            defects.append("field recorded_at is not an ISO date")
    splits = _load_splits(data, defects)
    if "total_seconds" in data:
        carried = data["total_seconds"]
        if not _is_number(carried):
            defects.append("field total_seconds has the wrong type")
        elif not _is_finite_number(carried):
            defects.append("field total_seconds is not a finite number")
        elif splits is not None and carried != splits[-1]["elapsed_seconds"]:
            defects.append(
                "field total_seconds does not equal the final split"
            )
    if defects:
        return None, defects
    run = {
        "task": task,
        "window_seconds": window,
        "runner": runner,
        "recorded_at": recorded_at,
        "splits": splits,
        "total_seconds": splits[-1]["elapsed_seconds"],
    }
    return run, []


def load_run_file(path):
    """Read a recorded run from disk, with the same contract as text."""
    try:
        text = pathlib.Path(path).read_text(encoding="utf-8")
    except (OSError, UnicodeError):
        return None, ["file cannot be read"]
    return load_run_text(text)


def _recorded_date(run):
    return datetime.date.fromisoformat(run["recorded_at"])


def select_ghost(runs):
    """Return the ghost for one task, or None when there are no runs.

    The ghost holds the lowest total, ties on total break toward the
    earliest recorded date, and full ties keep the input order.
    """
    runs = list(runs)
    if not runs:
        return None
    return min(runs, key=lambda run: (run["total_seconds"], _recorded_date(run)))


def window_line(run, subject):
    """Return the window line for one run.

    The line says the run finished inside its window and names the
    seconds to spare when the total is at or under the window, and
    says the run ran past its window and names the overrun when the
    total is over the window.
    """
    total = run["total_seconds"]
    window = run["window_seconds"]
    if total <= window:
        spare = format_seconds(window - total)
        return (
            f"window, {subject} run finished inside its window "
            f"with {spare} seconds to spare"
        )
    over = format_seconds(total - window)
    return f"window, {subject} run ran {over} seconds past its window"


def race_report(ghost, live):
    """Return the race lines for a live run against the ghost.

    Splits pair by position. The window line for the live run stands
    before the final line, which reports the finish from the two
    totals. The report is a pure function of the two runs.
    """
    lines = []
    ghost_splits = ghost["splits"]
    live_splits = live["splits"]
    for index in range(min(len(ghost_splits), len(live_splits))):
        ghost_split = ghost_splits[index]
        live_split = live_splits[index]
        if ghost_split["phase"] != live_split["phase"]:
            lines.append(
                f"split {index} phase mismatch, ghost {ghost_split['phase']} "
                f"and live {live_split['phase']}"
            )
            continue
        difference = ghost_split["elapsed_seconds"] - live_split["elapsed_seconds"]
        if difference > 0:
            lines.append(
                f"{ghost_split['phase']} ahead by "
                f"{format_seconds(difference)} seconds"
            )
        elif difference < 0:
            lines.append(
                f"{ghost_split['phase']} behind by "
                f"{format_seconds(-difference)} seconds"
            )
        else:
            lines.append(
                f"{ghost_split['phase']} level at "
                f"{format_seconds(live_split['elapsed_seconds'])} seconds"
            )
    if len(ghost_splits) != len(live_splits):
        lines.append(
            f"split counts differ, ghost {len(ghost_splits)} "
            f"and live {len(live_splits)}"
        )
    lines.append(window_line(live, "the live"))
    difference = ghost["total_seconds"] - live["total_seconds"]
    if difference > 0:
        lines.append(f"finish ahead by {format_seconds(difference)} seconds")
    elif difference < 0:
        lines.append(f"finish behind by {format_seconds(-difference)} seconds")
    else:
        lines.append(
            f"finish level at {format_seconds(live['total_seconds'])} seconds"
        )
    return lines


def ghost_change(old_ghost, candidate):
    """Return whether the candidate takes the ghost, with a report line.

    The line names both runners and both totals either way.
    """
    # PG-4 defines the steal by the PG-2 rule applied to the pair, so
    # the candidate steals exactly when its total, and then its
    # recorded date on a tie, sorts strictly before the standing
    # ghost. Stating the comparison here keeps PG-4 from drifting if
    # select_ghost ever changes how it returns its pick.
    candidate_key = (candidate["total_seconds"], _recorded_date(candidate))
    ghost_key = (old_ghost["total_seconds"], _recorded_date(old_ghost))
    steals = candidate_key < ghost_key
    old_total = format_seconds(old_ghost["total_seconds"])
    new_total = format_seconds(candidate["total_seconds"])
    if steals:
        line = (
            f"ghost changes from {old_ghost['runner']} at {old_total} seconds "
            f"to {candidate['runner']} at {new_total} seconds"
        )
    else:
        line = (
            f"ghost stays with {old_ghost['runner']} at {old_total} seconds "
            f"against {candidate['runner']} at {new_total} seconds"
        )
    return steals, line
