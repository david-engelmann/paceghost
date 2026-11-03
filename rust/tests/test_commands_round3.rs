//! Pins for the fieldday command in CONTRACT.md.
//!
//! Translated from tests/test_commands_round3.py. The command gets
//! a clean run over a small field day with its exact printed lines,
//! and each refusal gets its exit status and its printed defects, so
//! a command that started printing a report for a refused input
//! would fail.

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
        "paceghost-commands3-{}-{}",
        std::process::id(),
        tag
    ));
    std::fs::create_dir_all(&dir).expect("the scratch dir is created");
    dir
}

fn write(dir: &Path, name: &str, text: &str) -> String {
    let path = dir.join(name);
    std::fs::write(&path, text).expect("the scratch file writes");
    path.to_string_lossy().into_owned()
}

fn day_file(dir: &Path) -> String {
    let text = "{\"window_seconds\": 1800, \"operator\": \"operator\", \"runs\": [{\"runner\": \"operator\", \"usage\": 900, \"reads\": [{\"elapsed_seconds\": 300, \"usage\": 150}]}, {\"runner\": \"fork\", \"usage\": 950, \"reads\": [{\"elapsed_seconds\": 600, \"usage\": 250}]}]}";
    write(dir, "day.json", text)
}

#[test]
fn test_fieldday_prints_the_report() {
    let dir = scratch("fieldday-report");
    let day = day_file(&dir);
    let outcome = cli(&["fieldday", &day]);
    assert_eq!(outcome.code, 0);
    assert_eq!(
        outcome.stdout,
        vec![
            "field day, window 1800 seconds, 2 runs, operator operator",
            "record at 300 seconds, operator at a projection of 900",
            "record at 600 seconds, fork at a projection of 750",
            "finish, record held by fork at a projection of 750",
            "record stolen from operator at 600 seconds by fork, margin 150 at the close",
        ]
    );
}

#[test]
fn test_fieldday_prints_the_defects_of_a_malformed_day() {
    let dir = scratch("fieldday-bad");
    let path = write(&dir, "bad.json", "{\"window_seconds\": 1800, \"runs\": []}");
    let outcome = cli(&["fieldday", &path]);
    assert_eq!(outcome.code, 1);
    assert_eq!(
        outcome.stdout,
        vec![
            "invalid field day",
            "defect missing field operator",
            "defect field runs is empty",
        ]
    );
}

#[test]
fn test_fieldday_names_an_unreadable_file() {
    let dir = scratch("fieldday-missing");
    let missing = dir.join("missing.json").to_string_lossy().into_owned();
    let outcome = cli(&["fieldday", &missing]);
    assert_eq!(outcome.code, 1);
    assert_eq!(
        outcome.stdout,
        vec!["invalid field day", "defect file cannot be read"]
    );
}

#[test]
fn test_fieldday_prints_no_report_for_a_day_with_a_forged_label() {
    let dir = scratch("fieldday-forged");
    let text = "{\"window_seconds\": 1800, \"operator\": \"operator\", \"runs\": [{\"runner\": \"operator\", \"usage\": 1}, {\"runner\": \"fork\\nfinish, record held by fork\", \"usage\": 2}]}";
    let path = write(&dir, "forged.json", text);
    let outcome = cli(&["fieldday", &path]);
    assert_eq!(outcome.code, 1);
    assert_eq!(
        outcome.stdout,
        vec![
            "invalid field day",
            "defect run 1 field runner holds a line break",
        ]
    );
}
