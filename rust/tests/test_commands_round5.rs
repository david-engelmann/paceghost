//! Tests for the Round 5 command line surface, the rematch command.
//!
//! Written for Round 5 in the shape of the Round 4 command tests.

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
        "paceghost-commands5-{}-{}",
        std::process::id(),
        tag
    ));
    std::fs::create_dir_all(&dir).expect("the scratch dir is created");
    dir
}

fn day(runs: &str) -> String {
    format!(
        "{{\"window_seconds\": 1800, \"operator\": \"operator\", \"runs\": {}}}",
        runs
    )
}

fn write_rematch(dir: &Path, first: &str, second: &str, name: &str) -> String {
    let path = dir.join(name);
    let text = format!("{{\"first\": {}, \"second\": {}}}", first, second);
    std::fs::write(&path, text).expect("the scratch file writes");
    path.to_string_lossy().into_owned()
}

#[test]
fn test_rematch_prints_the_report() {
    let dir = scratch("rematch-report");
    let first = day("[{\"runner\": \"operator\", \"usage\": 900, \"reads\": [{\"elapsed_seconds\": 300, \"usage\": 150}]}]");
    let second = day("[{\"runner\": \"operator\", \"usage\": 800, \"reads\": [{\"elapsed_seconds\": 300, \"usage\": 100}]}]");
    let path = write_rematch(&dir, &first, &second, "rematch.json");
    let outcome = cli(&["rematch", &path]);
    assert_eq!(outcome.code, 0);
    assert_eq!(
        outcome.stdout,
        vec![
            "rematch, first day window 1800 seconds, 1 runs, second day window 1800 seconds, 1 runs",
            "record at 300 seconds on the first day, operator at a projection of 900",
            "record at 300 seconds on the second day, operator at a projection of 600",
            "finish, record held by operator at a projection of 600",
            "record never changed hands on the second day",
        ]
    );
}

#[test]
fn test_rematch_prints_the_defects_of_a_malformed_rematch() {
    let dir = scratch("rematch-bad");
    let path = write_rematch(&dir, &day("[]"), &day("[]"), "rematch.json");
    let outcome = cli(&["rematch", &path]);
    assert_eq!(outcome.code, 1);
    assert_eq!(
        outcome.stdout,
        vec![
            "invalid rematch",
            "defect first field runs is empty",
            "defect second field runs is empty",
        ]
    );
}

#[test]
fn test_rematch_prints_the_defect_of_a_missing_day() {
    let dir = scratch("rematch-missing-day");
    let path = dir.join("rematch.json");
    std::fs::write(&path, format!("{{\"second\": {}}}", day("[]")))
        .expect("the scratch file writes");
    let path_text = path.to_string_lossy().into_owned();
    let outcome = cli(&["rematch", &path_text]);
    assert_eq!(outcome.code, 1);
    assert_eq!(
        outcome.stdout,
        vec![
            "invalid rematch",
            "defect missing field first",
            "defect second field runs is empty",
        ]
    );
}

#[test]
fn test_rematch_prints_the_defect_of_an_unreadable_file() {
    let dir = scratch("rematch-missing");
    let missing = dir.join("missing.json").to_string_lossy().into_owned();
    let outcome = cli(&["rematch", &missing]);
    assert_eq!(outcome.code, 1);
    assert_eq!(
        outcome.stdout,
        vec!["invalid rematch", "defect file cannot be read"]
    );
}

#[test]
fn test_a_forged_label_inside_a_day_prints_no_report() {
    let dir = scratch("rematch-forged");
    let first = day("[{\"runner\": \"operator\", \"usage\": 900}]");
    let second = day("[{\"runner\": \"operator\", \"usage\": 900}, {\"runner\": \"fork\\nfinish, no record was set\", \"usage\": 950}]");
    let path = write_rematch(&dir, &first, &second, "rematch.json");
    let outcome = cli(&["rematch", &path]);
    assert_eq!(outcome.code, 1);
    assert_eq!(
        outcome.stdout,
        vec![
            "invalid rematch",
            "defect second run 1 field runner holds a line break",
        ]
    );
    assert!(!outcome
        .stdout
        .iter()
        .any(|line| line.contains("rematch, first day window")));
}
