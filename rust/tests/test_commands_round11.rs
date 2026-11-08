//! Tests for the Round 11 command line surface, the marks
//! command beside the thirteen standing commands.
//!
//! Written for Round 11 in the shape of the Round 10 command
//! tests. The marks command reads one meet file and prints the
//! marks reading, and a malformed meet prints exactly what the
//! meet command prints for the same file, so the reading never
//! depends on a second loading.

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
        "paceghost-commands11-{}-{}",
        std::process::id(),
        tag
    ));
    std::fs::create_dir_all(&dir).expect("the scratch dir is created");
    dir
}

fn write_file(dir: &Path, name: &str, text: &str) -> String {
    let path = dir.join(name);
    std::fs::write(&path, text).expect("the scratch file writes");
    path.to_string_lossy().into_owned()
}

fn day(read_usage: i64) -> String {
    format!(
        "{{\"window_seconds\": 1000, \"operator\": \"alpha\", \"runs\": [{{\"runner\": \"alpha\", \"usage\": 600, \"reads\": [{{\"elapsed_seconds\": 100, \"usage\": {}}}]}}]}}",
        read_usage
    )
}

/// A meet of three days over one competitor, alpha, whose
/// projection improves at every day's only mark, so the first
/// mark sets the record and each later mark extends it.
fn improving_meet(dir: &Path, name: &str) -> String {
    let days = [day(40), day(30), day(20)];
    write_file(dir, name, &format!("{{\"days\": [{}]}}", days.join(", ")))
}

#[test]
fn test_marks_prints_the_marks() {
    let dir = scratch("marks");
    let path = improving_meet(&dir, "meet.json");
    let outcome = cli(&["marks", &path]);
    assert_eq!(outcome.code, 0);
    assert_eq!(
        outcome.stdout,
        vec![
            "meet marks, 3 days",
            "day, the first day, marks 1",
            "mark 1, at 100 seconds on the first day, alpha at a projection of 400, set the standing record",
            "day, the second day, marks 1",
            "mark 2, at 100 seconds on the second day, alpha at a projection of 300, extended its own record, displacing a projection of 400",
            "day, the third day, marks 1",
            "mark 3, at 100 seconds on the third day, alpha at a projection of 200, extended its own record, displacing a projection of 300",
            "marks counts, marks walked 3, record marks 3, days 3",
        ]
    );
}

#[test]
fn test_marks_prints_exactly_what_meet_prints_for_a_malformed_meet() {
    let dir = scratch("malformed");
    let path = write_file(&dir, "meet.json", "{\"days\": []}");
    let marks = cli(&["marks", &path]);
    let meet = cli(&["meet", &path]);
    assert_eq!(marks.code, 1);
    assert_eq!(marks.stdout, meet.stdout);
    assert_eq!(
        marks.stdout.first().map(String::as_str),
        Some("invalid meet")
    );
}

#[test]
fn test_marks_prints_exactly_what_meet_prints_for_a_missing_file() {
    let dir = scratch("missing");
    let path = dir.join("absent.json").to_string_lossy().into_owned();
    let marks = cli(&["marks", &path]);
    let meet = cli(&["meet", &path]);
    assert_eq!(marks.code, 1);
    assert_eq!(marks.stdout, meet.stdout);
    assert_eq!(
        marks.stdout,
        vec!["invalid meet", "defect file cannot be read"]
    );
}
