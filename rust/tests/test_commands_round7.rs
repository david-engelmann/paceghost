//! Tests for the Round 7 command line surface, the table command.
//!
//! Written for Round 7 in the shape of the Round 6 command tests.

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
        "paceghost-commands7-{}-{}",
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
fn test_table_prints_the_table() {
    let dir = scratch("table-report");
    let days = vec![
        operator_day(900, 150),
        operator_day(800, 100),
        operator_day(700, 50),
    ];
    let path = write_meet(&dir, &days, "meet.json");
    let outcome = cli(&["table", &path]);
    assert_eq!(outcome.code, 0);
    assert_eq!(
        outcome.stdout,
        vec![
            "meet table, 3 days",
            "competitor operator, lowest standing projection 300, set on the third day at 300 seconds, record held after 3 marks, standing projection 300 at the close",
            "table counts, competitors 1, record holders 1, marks walked 3",
        ]
    );
}

#[test]
fn test_table_prints_the_defects_of_a_malformed_meet() {
    let dir = scratch("table-bad");
    let path = write_meet(&dir, &[day("[]"), day("[]"), day("[]")], "meet.json");
    let outcome = cli(&["table", &path]);
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
    // The file is a meet file, so the table command prints
    // exactly what the meet command prints over it.
    let meet_outcome = cli(&["meet", &path]);
    assert_eq!(outcome.stdout, meet_outcome.stdout);
    assert_eq!(outcome.stderr, meet_outcome.stderr);
    assert_eq!(outcome.code, meet_outcome.code);
}

#[test]
fn test_table_prints_the_two_day_refusal() {
    let dir = scratch("table-two-days");
    let days = vec![operator_day(900, 150), operator_day(800, 100)];
    let path = write_meet(&dir, &days, "meet.json");
    let outcome = cli(&["table", &path]);
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
fn test_table_prints_the_defect_of_an_unreadable_file() {
    let dir = scratch("table-missing");
    let missing = dir.join("missing.json").to_string_lossy().into_owned();
    let outcome = cli(&["table", &missing]);
    assert_eq!(outcome.code, 1);
    assert_eq!(
        outcome.stdout,
        vec!["invalid meet", "defect file cannot be read"]
    );
}

#[test]
fn test_a_forged_label_inside_a_day_prints_no_table() {
    let dir = scratch("table-forged");
    let days = vec![
        day("[{\"runner\": \"operator\", \"usage\": 900}]"),
        day("[{\"runner\": \"operator\", \"usage\": 900}, {\"runner\": \"fork\\ntable counts, competitors 0\", \"usage\": 950}]"),
        day("[{\"runner\": \"operator\", \"usage\": 900}]"),
    ];
    let path = write_meet(&dir, &days, "meet.json");
    let outcome = cli(&["table", &path]);
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
        .any(|line| line.contains("meet table")));
}

#[test]
fn test_the_table_command_writes_nothing() {
    let dir = scratch("table-writes-nothing");
    let days = vec![
        operator_day(900, 150),
        operator_day(800, 100),
        operator_day(700, 50),
    ];
    let path = write_meet(&dir, &days, "meet.json");
    let before_bytes = std::fs::read(&path).expect("the meet file reads");
    let before: Vec<String> = std::fs::read_dir(&dir)
        .expect("the scratch dir lists")
        .map(|entry| {
            entry
                .expect("the entry reads")
                .file_name()
                .to_string_lossy()
                .into_owned()
        })
        .collect();
    let outcome = cli(&["table", &path]);
    assert_eq!(outcome.code, 0);
    let after: Vec<String> = std::fs::read_dir(&dir)
        .expect("the scratch dir lists")
        .map(|entry| {
            entry
                .expect("the entry reads")
                .file_name()
                .to_string_lossy()
                .into_owned()
        })
        .collect();
    assert_eq!(before, after);
    assert_eq!(
        std::fs::read(&path).expect("the meet file reads"),
        before_bytes
    );
}
