//! Tests for the Round 4 command line surface, the ladder command.
//!
//! Translated from tests/test_commands_round4.py.

use std::path::{Path, PathBuf};

use paceghost::cli::{dispatch, CliOutcome};
use paceghost::pytext::PText;

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
    let dir = std::env::temp_dir().join(format!(
        "paceghost-commands4-{}-{}",
        std::process::id(),
        tag
    ));
    std::fs::create_dir_all(&dir).expect("the scratch dir is created");
    dir
}

fn ladder_day(runs: &str) -> String {
    format!(
        "{{\"window_seconds\": 1800, \"operator\": \"operator\", \"runs\": {}}}",
        runs
    )
}

fn std_runs() -> String {
    "[{\"runner\": \"operator\", \"usage\": 900, \"reads\": [{\"elapsed_seconds\": 300, \"usage\": 210}]}, {\"runner\": \"fork\", \"usage\": 950, \"reads\": [{\"elapsed_seconds\": 600, \"usage\": 200}]}]".to_string()
}

fn write_ladder(dir: &Path, windows: &str, day: &str, name: &str) -> String {
    let path = dir.join(name);
    let text = format!("{{\"day\": {}, \"windows\": {}}}", day, windows);
    std::fs::write(&path, text).expect("the scratch file writes");
    path.to_string_lossy().into_owned()
}

#[test]
fn test_ladder_prints_the_report() {
    let dir = scratch("ladder-report");
    let path = write_ladder(&dir, "[300, 600]", &ladder_day(&std_runs()), "ladder.json");
    let outcome = cli(&["ladder", &path]);
    assert_eq!(outcome.code, 0);
    assert_eq!(
        outcome.stdout,
        vec![
            "field day, window 300 seconds, 2 runs, operator operator",
            "record at 300 seconds, operator at a projection of 210",
            "finish, record held by operator at a projection of 210",
            "record kept by operator to the close",
            "field day, window 600 seconds, 2 runs, operator operator",
            "record at 300 seconds, operator at a projection of 420",
            "record at 600 seconds, fork at a projection of 200",
            "finish, record held by fork at a projection of 200",
            "record stolen from operator at 600 seconds by fork, margin 220 at the close",
            "ladder, the closing holder differs from the first window at 600 seconds",
        ]
    );
}

#[test]
fn test_ladder_prints_the_defects_of_a_malformed_ladder() {
    let dir = scratch("ladder-bad");
    let path = write_ladder(&dir, "[]", &ladder_day(&std_runs()), "ladder.json");
    let outcome = cli(&["ladder", &path]);
    assert_eq!(outcome.code, 1);
    assert_eq!(
        outcome.stdout,
        vec!["invalid ladder", "defect field windows is empty"]
    );
}

#[test]
fn test_ladder_prints_the_day_defects_of_a_malformed_day() {
    let dir = scratch("ladder-bad-day");
    let path = write_ladder(&dir, "[300, 600]", &ladder_day("[]"), "ladder.json");
    let outcome = cli(&["ladder", &path]);
    assert_eq!(outcome.code, 1);
    assert_eq!(
        outcome.stdout,
        vec!["invalid ladder", "defect day field runs is empty"]
    );
}

#[test]
fn test_ladder_prints_the_defect_of_an_unreadable_file() {
    let dir = scratch("ladder-missing");
    let missing = dir.join("missing.json").to_string_lossy().into_owned();
    let outcome = cli(&["ladder", &missing]);
    assert_eq!(outcome.code, 1);
    assert_eq!(
        outcome.stdout,
        vec!["invalid ladder", "defect file cannot be read"]
    );
}

#[test]
fn test_a_forged_label_inside_the_day_prints_no_report() {
    let dir = scratch("ladder-forged");
    let runs = "[{\"runner\": \"operator\", \"usage\": 900}, {\"runner\": \"fork\\nfinish, no record was set\", \"usage\": 950}]";
    let path = write_ladder(&dir, "[300, 600]", &ladder_day(runs), "ladder.json");
    let outcome = cli(&["ladder", &path]);
    assert_eq!(outcome.code, 1);
    assert_eq!(
        outcome.stdout,
        vec![
            "invalid ladder",
            "defect day run 1 field runner holds a line break",
        ]
    );
    assert!(!outcome
        .stdout
        .iter()
        .any(|line| line.contains("field day, window")));
}
