//! Tests for the Round 6 command line surface, the meet command.
//!
//! Written for Round 6 in the shape of the Round 5 command tests.

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
        "paceghost-commands6-{}-{}",
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

fn write_meet(dir: &Path, days: &[String], name: &str) -> String {
    let path = dir.join(name);
    let text = format!("{{\"days\": [{}]}}", days.join(", "));
    std::fs::write(&path, text).expect("the scratch file writes");
    path.to_string_lossy().into_owned()
}

fn operator_day(usage: i64, read_usage: i64) -> String {
    day(&format!(
        "[{{\"runner\": \"operator\", \"usage\": {}, \"reads\": [{{\"elapsed_seconds\": 300, \"usage\": {}}}]}}]",
        usage, read_usage
    ))
}

#[test]
fn test_meet_prints_the_report() {
    let dir = scratch("meet-report");
    let days = vec![
        operator_day(900, 150),
        operator_day(800, 100),
        operator_day(700, 50),
    ];
    let path = write_meet(&dir, &days, "meet.json");
    let outcome = cli(&["meet", &path]);
    assert_eq!(outcome.code, 0);
    assert_eq!(
        outcome.stdout,
        vec![
            "meet, 3 days",
            "record at 300 seconds on the first day, operator at a projection of 900",
            "close of the first day, record held by operator at a projection of 900",
            "record at 300 seconds on the second day, operator at a projection of 600",
            "close of the second day, record held by operator at a projection of 600",
            "record at 300 seconds on the third day, operator at a projection of 300",
            "close of the third day, record held by operator at a projection of 300",
            "finish, record held by operator at a projection of 300",
            "record set on the third day at 300 seconds",
            "days read, 3",
        ]
    );
}

#[test]
fn test_meet_prints_the_defects_of_a_malformed_meet() {
    let dir = scratch("meet-bad");
    let path = write_meet(&dir, &[day("[]"), day("[]"), day("[]")], "meet.json");
    let outcome = cli(&["meet", &path]);
    assert_eq!(outcome.code, 1);
    assert_eq!(
        outcome.stdout,
        vec![
            "invalid meet",
            "defect day 0 field runs is empty",
            "defect day 1 field runs is empty",
            "defect day 2 field runs is empty",
        ]
    );
}

#[test]
fn test_meet_prints_the_two_day_refusal() {
    let dir = scratch("meet-two-days");
    let days = vec![operator_day(900, 150), operator_day(800, 100)];
    let path = write_meet(&dir, &days, "meet.json");
    let outcome = cli(&["meet", &path]);
    assert_eq!(outcome.code, 1);
    assert_eq!(
        outcome.stdout,
        vec![
            "invalid meet",
            "defect field days holds two days, the count of the rematch",
        ]
    );
}

#[test]
fn test_meet_prints_the_defect_of_an_unreadable_file() {
    let dir = scratch("meet-missing");
    let missing = dir.join("missing.json").to_string_lossy().into_owned();
    let outcome = cli(&["meet", &missing]);
    assert_eq!(outcome.code, 1);
    assert_eq!(
        outcome.stdout,
        vec!["invalid meet", "defect file cannot be read"]
    );
}

#[test]
fn test_a_forged_label_inside_a_day_prints_no_report() {
    let dir = scratch("meet-forged");
    let days = vec![
        day("[{\"runner\": \"operator\", \"usage\": 900}]"),
        day("[{\"runner\": \"operator\", \"usage\": 900}, {\"runner\": \"fork\\nfinish, no record was set\", \"usage\": 950}]"),
        day("[{\"runner\": \"operator\", \"usage\": 900}]"),
    ];
    let path = write_meet(&dir, &days, "meet.json");
    let outcome = cli(&["meet", &path]);
    assert_eq!(outcome.code, 1);
    assert_eq!(
        outcome.stdout,
        vec![
            "invalid meet",
            "defect day 1 run 1 field runner holds a line break",
        ]
    );
    assert!(!outcome
        .stdout
        .iter()
        .any(|line| line.contains("meet, 3 days")));
}
