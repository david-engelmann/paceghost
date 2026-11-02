"""The paceghost command line surface.

Three commands read recorded runs from local files and print reports.
The show command prints one run or its defects, the ghost command
prints the standing ghost for a task, and the race command prints a
live run raced against a ghost. Nothing here touches the network.
"""

import argparse
import sys

from paceghost.runs import (
    format_seconds,
    load_run_file,
    race_report,
    select_ghost,
    window_line,
)


def run_summary_lines(run):
    """Return the summary lines the show command prints for one run."""
    lines = [
        f"task {run['task']}",
        f"runner {run['runner']}",
        f"recorded_at {run['recorded_at']}",
        f"window_seconds {format_seconds(run['window_seconds'])}",
        f"total_seconds {format_seconds(run['total_seconds'])}",
        f"splits {len(run['splits'])}",
    ]
    for split in run["splits"]:
        lines.append(
            f"split {split['phase']} at "
            f"{format_seconds(split['elapsed_seconds'])} seconds"
        )
    lines.append(window_line(run, "this"))
    return lines


def _load_or_print_defects(path):
    run, defects = load_run_file(path)
    if defects:
        print("invalid recorded run")
        for defect in defects:
            print(f"defect {defect}")
        return None
    return run


def _command_show(args):
    run = _load_or_print_defects(args.file)
    if run is None:
        return 1
    for line in run_summary_lines(run):
        print(line)
    return 0


def _command_ghost(args):
    runs = []
    failed = False
    for path in args.files:
        run = _load_or_print_defects(path)
        if run is None:
            failed = True
        else:
            runs.append(run)
    if failed:
        return 1
    if len({run["task"] for run in runs}) != 1:
        print("runs do not share one task")
        return 1
    ghost = select_ghost(runs)
    print(f"ghost for task {ghost['task']}")
    print(f"runner {ghost['runner']}")
    print(f"recorded_at {ghost['recorded_at']}")
    print(f"total_seconds {format_seconds(ghost['total_seconds'])}")
    return 0


def _command_race(args):
    ghost = _load_or_print_defects(args.ghost_file)
    live = _load_or_print_defects(args.live_file)
    if ghost is None or live is None:
        return 1
    if ghost["task"] != live["task"]:
        print("runs do not share one task")
        return 1
    for line in race_report(ghost, live):
        print(line)
    return 0


def main(argv=None):
    """Run the paceghost command line and return the exit status."""
    parser = argparse.ArgumentParser(
        prog="paceghost",
        description="Race a recorded run against the ghost of your best run.",
    )
    commands = parser.add_subparsers(dest="command", required=True)
    show = commands.add_parser("show", help="print one recorded run")
    show.add_argument("file")
    show.set_defaults(handler=_command_show)
    ghost = commands.add_parser("ghost", help="print the standing ghost")
    ghost.add_argument("files", nargs="+")
    ghost.set_defaults(handler=_command_ghost)
    race = commands.add_parser("race", help="race a live run against a ghost")
    race.add_argument("ghost_file")
    race.add_argument("live_file")
    race.set_defaults(handler=_command_race)
    args = parser.parse_args(argv)
    return args.handler(args)


if __name__ == "__main__":
    sys.exit(main())
