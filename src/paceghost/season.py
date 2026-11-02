"""The season over a recorded field and the live board, for paceghost.

The field loader never raises on malformed content and names every
defect it finds, reusing the PG-1 phrases under a run index prefix.
The season report and the board are pure functions of their inputs,
and every usage figure is computed on exact values and rendered
under the rule FIX-PG-7 states. The obligations this module carries
are PG-5 and PG-6 in CONTRACT.md, in the Round 2 section.
"""

import json
import pathlib
from fractions import Fraction

from paceghost.runs import _check_text_field, _load_run_data, format_seconds


def _round_half_up_units(fraction, places):
    """Return the count of units a fraction rounds to at a precision.

    The count is taken on the exact value, and an exact half rounds
    up. Usage figures are never negative, so the rule needs no sign
    case.
    """
    scaled = fraction * (10 ** places)
    units = scaled.numerator // scaled.denominator
    if 2 * (scaled - units) >= 1:
        units += 1
    return units


def _render_units(units, places):
    """Render a count of units at a precision in plain decimal."""
    whole, part = divmod(units, 10 ** places)
    if part == 0:
        return str(whole)
    digits = f"{part:0{places}d}".rstrip("0")
    return f"{whole}.{digits}"


def _exact_expansion(fraction):
    """Return the exact decimal expansion of a fraction, or None.

    The expansion terminates exactly when the denominator of the
    fraction in its lowest terms carries no prime factor other than
    two and five.
    """
    denominator = fraction.denominator
    twos = 0
    while denominator % 2 == 0:
        denominator //= 2
        twos += 1
    fives = 0
    while denominator % 5 == 0:
        denominator //= 5
        fives += 1
    if denominator != 1:
        return None
    places = max(twos, fives)
    return _render_units(int(fraction * (10 ** places)), places)


def format_usage(value):
    """Render a usage figure from its exact value, never through a float.

    A figure that needs six or fewer decimal places renders exactly.
    A figure that needs more renders rounded at six decimal places,
    with an exact half at the seventh place rounding up on the exact
    value. A nonzero figure that rounds to zero at six places
    renders its exact decimal expansion when that expansion
    terminates, and otherwise renders rounded at twelve decimal
    places with halves rounding up. Every figure renders in plain
    decimal notation and never in
    scientific notation. The rule is FIX-PG-7 in CONTRACT.md,
    amending PG-5 and PG-6.
    """
    fraction = Fraction(value)
    if (fraction * 1000000).denominator == 1:
        return _render_units(int(fraction * 1000000), 6)
    if fraction != 0 and fraction < Fraction(1, 2000000):
        expansion = _exact_expansion(fraction)
        if expansion is not None:
            return expansion
        return _render_units(_round_half_up_units(fraction, 12), 12)
    return _render_units(_round_half_up_units(fraction, 6), 6)


def load_field_text(text):
    """Parse field text into its runs, naming every defect found.

    Returns a pair of the run list and the defect list. A malformed
    input returns no runs, and this function never raises on bad
    content. Each clean run carries its model field.
    """
    try:
        data = json.loads(text)
    except (ValueError, RecursionError):
        return None, ["input is not valid JSON"]
    if not isinstance(data, dict):
        return None, ["input is not a JSON object"]
    defects = []
    if "runs" not in data:
        defects.append("missing field runs")
        return None, defects
    raw = data["runs"]
    if not isinstance(raw, list):
        defects.append("field runs has the wrong type")
        return None, defects
    if len(raw) == 0:
        defects.append("field runs is empty")
        return None, defects
    runs = []
    field_task = None
    for index, entry in enumerate(raw):
        prefix = f"run {index} "
        if not isinstance(entry, dict):
            defects.append(f"{prefix}is not an object")
            continue
        run = _load_run_data(entry, defects, prefix)
        model = _check_text_field(entry, "model", defects, prefix)
        for name in ("usage", "budget"):
            phrase = f"{prefix}missing field {name}"
            if name not in entry and phrase not in defects:
                defects.append(phrase)
        task = _check_text_field(entry, "task", [])
        if task is not None:
            if field_task is None:
                field_task = task
            elif task != field_task:
                defects.append(
                    f"{prefix}field task does not match the field task"
                )
        if run is not None and model is not None:
            run["model"] = model
            runs.append(run)
    if defects:
        return None, defects
    return runs, []


def load_field_file(path):
    """Read a field from disk, with the same contract as text."""
    try:
        text = pathlib.Path(path).read_text(encoding="utf-8")
    except (OSError, UnicodeError):
        return None, ["file cannot be read"]
    return load_field_text(text)


def season_report(runs, ghost):
    """Return the season lines for a field against one ghost.

    The ghost carries a usage field. Blocks order by wins descending,
    then total usage ascending, then model name. A run wins when it
    finished inside its own window on less usage than the ghost.
    """
    ghost_usage = Fraction(ghost["usage"])
    grouped = {}
    for run in runs:
        grouped.setdefault(run["model"], []).append(run)
    rows = []
    for model, model_runs in grouped.items():
        raced = len(model_runs)
        inside_window = 0
        inside_budget = 0
        wins = 0
        total_usage = Fraction(0)
        for run in model_runs:
            usage = Fraction(run["usage"])
            total_usage += usage
            in_window = run["total_seconds"] <= run["window_seconds"]
            if in_window:
                inside_window += 1
            if usage <= Fraction(run["budget"]):
                inside_budget += 1
            if in_window and usage < ghost_usage:
                wins += 1
        rows.append(
            (model, raced, inside_window, inside_budget, total_usage, wins)
        )
    rows.sort(key=lambda row: (-row[5], row[4], row[0]))
    lines = [f"season for task {runs[0]['task']}"]
    for model, raced, inside_window, inside_budget, total_usage, wins in rows:
        lines.append(f"model {model}")
        lines.append(f"runs raced {raced}")
        lines.append(f"inside window {inside_window}")
        lines.append(f"inside budget {inside_budget}")
        lines.append(f"total usage {format_usage(total_usage)}")
        lines.append(f"mean usage {format_usage(total_usage / raced)}")
        lines.append(f"head to head against the ghost, wins {wins} of {raced}")
    return lines


def _ghost_verdict(figure, ghost_usage):
    if figure < ghost_usage:
        return "inside the ghost usage"
    if figure > ghost_usage:
        return "past the ghost usage"
    return "level with the ghost usage"


def _budget_verdict(figure, budget):
    if figure <= budget:
        return "inside the budget"
    return "past the budget"


def board_report(run, ghost):
    """Return the live board lines for a run carrying timed reads.

    The run carries reads, usage, and budget fields, and the ghost
    carries a usage field. Each read projects the final usage from
    the rate so far, and the final line sets the actual usage beside
    the last projection and says whether the board called it.
    """
    ghost_usage = Fraction(ghost["usage"])
    budget = Fraction(run["budget"])
    window = Fraction(run["window_seconds"])
    lines = []
    last = None
    for index, read in enumerate(run["reads"]):
        elapsed = read["elapsed_seconds"]
        usage = Fraction(read["usage"])
        head = (
            f"read {index} at {format_seconds(elapsed)} seconds, "
            f"usage {format_usage(usage)}"
        )
        if Fraction(elapsed) == 0:
            lines.append(f"{head}, projects no figure")
            continue
        projection = usage * window / Fraction(elapsed)
        ghost_verdict = _ghost_verdict(projection, ghost_usage)
        budget_verdict = _budget_verdict(projection, budget)
        lines.append(
            f"{head}, projection {format_usage(projection)}, "
            f"{ghost_verdict}, {budget_verdict}"
        )
        last = (projection, ghost_verdict, budget_verdict)
    actual = Fraction(run["usage"])
    ghost_verdict = _ghost_verdict(actual, ghost_usage)
    budget_verdict = _budget_verdict(actual, budget)
    if last is None:
        lines.append(
            f"finish, actual usage {format_usage(actual)}, "
            f"no projection was made, {ghost_verdict}, {budget_verdict}"
        )
    else:
        called = last[1] == ghost_verdict and last[2] == budget_verdict
        judgement = "the board called it" if called else "the board missed it"
        lines.append(
            f"finish, actual usage {format_usage(actual)} against "
            f"a last projection of {format_usage(last[0])}, "
            f"{ghost_verdict}, {budget_verdict}, {judgement}"
        )
    return lines
