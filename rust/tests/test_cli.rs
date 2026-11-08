//! Pins for the command line surface in CONTRACT.md.
//!
//! Translated from tests/test_cli.py. Each command gets a clean run
//! with its exact printed lines and a malformed run with its exit
//! status and named defects, so a command that started swallowing
//! defects or printing extra lines would fail.

use std::path::{Path, PathBuf};

use paceghost::cli::{dispatch, run_summary_lines, CliOutcome};
use paceghost::pynum::Num;
use paceghost::pytext::PText;
use paceghost::runs::{Run, Split};

fn reader(bytes: &[u8]) -> Option<String> {
    let path = String::from_utf8_lossy(bytes).into_owned();
    let data = std::fs::read(path).ok()?;
    String::from_utf8(data).ok()
}

fn cli(args: &[&str]) -> CliOutcome {
    let argv: Vec<PText> = args.iter().map(|arg| PText::from_text(arg)).collect();
    dispatch(&argv, &reader)
}

fn scratch(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("paceghost-cli-{}-{}", std::process::id(), tag));
    std::fs::create_dir_all(&dir).expect("the scratch dir is created");
    dir
}

fn write_run(dir: &Path, name: &str, text: &str) -> String {
    let path = dir.join(name);
    std::fs::write(&path, text).expect("the scratch file writes");
    path.to_string_lossy().into_owned()
}

fn run_data(runner: &str, recorded_at: &str, splits: &[(&str, i64)], task: &str) -> String {
    let parts: Vec<String> = splits
        .iter()
        .map(|(phase, elapsed)| {
            format!(
                "{{\"phase\": \"{}\", \"elapsed_seconds\": {}}}",
                phase, elapsed
            )
        })
        .collect();
    format!(
        "{{\"task\": \"{}\", \"window_seconds\": 3600, \"runner\": \"{}\", \"recorded_at\": \"{}\", \"splits\": [{}]}}",
        task,
        runner,
        recorded_at,
        parts.join(", ")
    )
}

const TASK: &str = "draft the weekly summary";

#[test]
fn test_show_prints_the_run_summary() {
    let dir = scratch("show-summary");
    let path = write_run(
        &dir,
        "run.json",
        &run_data(
            "operator",
            "2026-01-04",
            &[("plan", 300), ("draft", 900)],
            TASK,
        ),
    );
    let outcome = cli(&["show", &path]);
    assert_eq!(outcome.code, 0);
    assert_eq!(
        outcome.stdout,
        vec![
            "task draft the weekly summary",
            "runner operator",
            "recorded_at 2026-01-04",
            "window_seconds 3600",
            "total_seconds 900",
            "splits 2",
            "split plan at 300 seconds",
            "split draft at 900 seconds",
            "window, this run finished inside its window with 2700 seconds to spare",
        ]
    );
}

#[test]
fn test_show_prints_defects_and_exits_one_on_a_malformed_run() {
    let dir = scratch("show-bad");
    let path = write_run(&dir, "bad.json", "{\"task\": 5, \"splits\": []}");
    let outcome = cli(&["show", &path]);
    assert_eq!(outcome.code, 1);
    assert!(outcome
        .stdout
        .iter()
        .any(|line| line == "invalid recorded run"));
    assert!(outcome
        .stdout
        .iter()
        .any(|line| line == "defect field task has the wrong type"));
    assert!(outcome
        .stdout
        .iter()
        .any(|line| line == "defect field splits is empty"));
}

#[test]
fn test_show_reports_an_unreadable_file() {
    let dir = scratch("show-missing");
    let missing = dir.join("missing.json").to_string_lossy().into_owned();
    let outcome = cli(&["show", &missing]);
    assert_eq!(outcome.code, 1);
    assert!(outcome
        .stdout
        .iter()
        .any(|line| line == "defect file cannot be read"));
}

#[test]
fn test_ghost_prints_the_faster_run_as_the_ghost() {
    let dir = scratch("ghost-fast");
    let slow = write_run(
        &dir,
        "slow.json",
        &run_data(
            "operator",
            "2026-01-04",
            &[("plan", 300), ("draft", 900), ("polish", 1500)],
            TASK,
        ),
    );
    let fast = write_run(
        &dir,
        "fast.json",
        &run_data(
            "night fork",
            "2026-01-05",
            &[("plan", 240), ("draft", 780), ("polish", 1320)],
            TASK,
        ),
    );
    let outcome = cli(&["ghost", &slow, &fast]);
    assert_eq!(outcome.code, 0);
    assert_eq!(
        outcome.stdout,
        vec![
            "ghost for task draft the weekly summary",
            "runner night fork",
            "recorded_at 2026-01-05",
            "total_seconds 1320",
        ]
    );
}

#[test]
fn test_ghost_rejects_runs_of_more_than_one_task() {
    let dir = scratch("ghost-tasks");
    let one = write_run(
        &dir,
        "one.json",
        &run_data("operator", "2026-01-04", &[("plan", 300)], TASK),
    );
    let two = write_run(
        &dir,
        "two.json",
        &run_data("operator", "2026-01-04", &[("plan", 200)], "another task"),
    );
    let outcome = cli(&["ghost", &one, &two]);
    assert_eq!(outcome.code, 1);
    assert!(outcome
        .stdout
        .iter()
        .any(|line| line == "runs do not share one task"));
}

#[test]
fn test_ghost_rejects_a_malformed_file() {
    let dir = scratch("ghost-bad");
    let good = write_run(
        &dir,
        "good.json",
        &run_data("operator", "2026-01-04", &[("plan", 300)], TASK),
    );
    let bad = write_run(&dir, "bad.json", "{\"task\": \"x\"}");
    let outcome = cli(&["ghost", &good, &bad]);
    assert_eq!(outcome.code, 1);
    assert!(outcome
        .stdout
        .iter()
        .any(|line| line == "defect missing field runner"));
}

#[test]
fn test_race_prints_the_race_report() {
    let dir = scratch("race-report");
    let ghost = write_run(
        &dir,
        "ghost.json",
        &run_data(
            "operator",
            "2026-01-04",
            &[("plan", 300), ("draft", 900), ("polish", 1500)],
            TASK,
        ),
    );
    let live = write_run(
        &dir,
        "live.json",
        &run_data(
            "night fork",
            "2026-01-05",
            &[("plan", 240), ("draft", 780), ("polish", 1320)],
            TASK,
        ),
    );
    let outcome = cli(&["race", &ghost, &live]);
    assert_eq!(outcome.code, 0);
    assert_eq!(
        outcome.stdout,
        vec![
            "plan ahead by 60 seconds",
            "draft ahead by 120 seconds",
            "polish ahead by 180 seconds",
            "window, the live run finished inside its window with 2280 seconds to spare",
            "finish ahead by 180 seconds",
        ]
    );
}

#[test]
fn test_race_rejects_a_malformed_input() {
    let dir = scratch("race-bad");
    let ghost = write_run(
        &dir,
        "ghost.json",
        &run_data("operator", "2026-01-04", &[("plan", 300)], TASK),
    );
    let bad = write_run(&dir, "bad.json", "{\"splits\": \"no\"}");
    let outcome = cli(&["race", &ghost, &bad]);
    assert_eq!(outcome.code, 1);
    assert!(outcome
        .stdout
        .iter()
        .any(|line| line == "invalid recorded run"));
}

#[test]
fn test_show_prints_defects_for_a_task_with_an_unpaired_surrogate() {
    let dir = scratch("show-surrogate");
    let text = "{\"task\": \"\\ud800x\", \"window_seconds\": 3600, \"runner\": \"operator\", \"recorded_at\": \"2026-01-04\", \"splits\": [{\"phase\": \"plan\", \"elapsed_seconds\": 300}]}";
    let path = write_run(&dir, "surrogate.json", text);
    let outcome = cli(&["show", &path]);
    assert_eq!(outcome.code, 1);
    assert!(outcome
        .stdout
        .iter()
        .any(|line| line == "invalid recorded run"));
    assert!(outcome
        .stdout
        .iter()
        .any(|line| line == "defect field task holds an unpaired surrogate"));
}

#[test]
fn test_race_rejects_runs_of_different_tasks() {
    let dir = scratch("race-tasks");
    let ghost = write_run(
        &dir,
        "ghost.json",
        &run_data("operator", "2026-01-04", &[("plan", 300)], TASK),
    );
    let live = write_run(
        &dir,
        "live.json",
        &run_data("operator", "2026-01-05", &[("plan", 200)], "another task"),
    );
    let outcome = cli(&["race", &ghost, &live]);
    assert_eq!(outcome.code, 1);
    assert!(outcome
        .stdout
        .iter()
        .any(|line| line == "runs do not share one task"));
}

// The FIX-PG-12 pins, added with the Round 11 repair. A run file
// can carry a runner label holding a line break, because the
// recorded run loader states no line break rule for a runner, so
// the show and ghost printings refuse such a label in the one
// named error instead of printing it raw. On the unrepaired code
// both commands print the label with the break standing inside
// the runner line, so each pin fails there for a real reason.

#[test]
fn test_run_summary_lines_refuse_a_break_bearing_runner() {
    let run = Run {
        task: PText::from_text(TASK),
        window_seconds: Num::from_i64(3600),
        runner: PText::from_text("b\nb"),
        recorded_at: PText::from_text("2026-01-04"),
        splits: vec![Split {
            phase: PText::from_text("plan"),
            elapsed_seconds: Num::from_i64(300),
        }],
        total_seconds: Num::from_i64(300),
        usage: None,
        budget: None,
        reads: None,
        model: None,
    };
    let error = run_summary_lines(&run).expect_err("the lines name the precondition");
    assert_eq!(error, "a runner label holds a line break");
}

#[test]
fn test_show_prints_the_named_error_for_a_break_bearing_runner() {
    let dir = scratch("show-break");
    let path = write_run(
        &dir,
        "run.json",
        &run_data("b\\nb", "2026-01-04", &[("plan", 300)], TASK),
    );
    let outcome = cli(&["show", &path]);
    assert_eq!(outcome.stdout, vec!["a runner label holds a line break"]);
    assert_eq!(outcome.code, 1);
}

#[test]
fn test_ghost_prints_the_named_error_for_a_break_bearing_ghost() {
    let dir = scratch("ghost-break");
    let clean = write_run(
        &dir,
        "clean.json",
        &run_data("operator", "2026-01-04", &[("plan", 1500)], TASK),
    );
    let broken = write_run(
        &dir,
        "broken.json",
        &run_data("b\\nb", "2026-01-05", &[("plan", 1320)], TASK),
    );
    let outcome = cli(&["ghost", &clean, &broken]);
    assert_eq!(outcome.stdout, vec!["a runner label holds a line break"]);
    assert_eq!(outcome.code, 1);
}
