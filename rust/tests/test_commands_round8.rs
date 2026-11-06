//! Tests for the Round 8 command line surface, the reign
//! command beside the ten standing commands.
//!
//! Written for Round 8 in the shape of the Round 7 command
//! tests. The reign command reads one meet file and prints the
//! reign reading, and a malformed meet prints exactly what the
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
        "paceghost-commands8-{}-{}",
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

fn write_file(dir: &Path, name: &str, text: &str) -> String {
    let path = dir.join(name);
    std::fs::write(&path, text).expect("the scratch file writes");
    path.to_string_lossy().into_owned()
}

fn operator_day(read_usage: i64) -> String {
    day(&format!(
        "[{{\"runner\": \"operator\", \"usage\": 600, \"reads\": [{{\"elapsed_seconds\": 300, \"usage\": {}}}]}}]",
        read_usage
    ))
}

/// A meet of three days over one competitor, operator, whose
/// projection improves at every day's only mark, so the walk
/// is one spell of operator of three marks, one per day, every
/// mark at 300 seconds.
fn operator_meet(dir: &Path, name: &str) -> String {
    let days = [operator_day(150), operator_day(100), operator_day(50)];
    write_file(dir, name, &format!("{{\"days\": [{}]}}", days.join(", ")))
}

#[test]
fn test_reign_prints_the_reign() {
    let dir = scratch("reign");
    let path = operator_meet(&dir, "meet.json");
    let outcome = cli(&["reign", &path]);
    assert_eq!(outcome.code, 0);
    assert_eq!(
        outcome.stdout,
        vec![
            "meet reign, 3 days",
            "spell, operator, first mark at 300 seconds on the first day, last mark at 300 seconds on the third day, 3 marks",
            "reign counts, marks walked 3, spells 1, holders 1, longest spell 3 marks held by operator",
        ]
    );
    assert!(outcome.stderr.is_empty());
}

#[test]
fn test_reign_prints_the_defects_of_a_malformed_meet() {
    let dir = scratch("reign-defects");
    let path = write_file(&dir, "bad.json", "{\"days\": \"x\"}");
    let meet_outcome = cli(&["meet", &path]);
    let outcome = cli(&["reign", &path]);
    assert_eq!(outcome.code, 1);
    assert_eq!(
        outcome.stdout,
        vec!["invalid meet", "defect field days has the wrong type"]
    );
    assert_eq!(outcome.stdout, meet_outcome.stdout);
    assert_eq!(outcome.stderr, meet_outcome.stderr);
}

#[test]
fn test_reign_prints_the_two_day_refusal() {
    let dir = scratch("reign-two-days");
    let days = [operator_day(150), operator_day(100)];
    let path = write_file(
        &dir,
        "two.json",
        &format!("{{\"days\": [{}]}}", days.join(", ")),
    );
    let outcome = cli(&["reign", &path]);
    assert_eq!(outcome.code, 1);
    assert_eq!(
        outcome.stdout,
        vec![
            "invalid meet",
            "defect field days holds two days, the count of the rematch"
        ]
    );
}

#[test]
fn test_reign_prints_the_defect_of_an_unreadable_file() {
    let dir = scratch("reign-missing");
    let missing = dir.join("missing.json").to_string_lossy().into_owned();
    let outcome = cli(&["reign", &missing]);
    assert_eq!(outcome.code, 1);
    assert_eq!(
        outcome.stdout,
        vec!["invalid meet", "defect file cannot be read"]
    );
}

#[test]
fn test_a_forged_label_inside_a_day_prints_no_reign() {
    let dir = scratch("reign-forged");
    // The label carries a line break as an escape inside the
    // JSON string, so the parsed runner value itself holds the
    // break and a forged closing line besides, and the field
    // day validation refuses it, and no reading of the meet
    // prints at all.
    let days = [
        day("[{\"runner\": \"operator\", \"usage\": 900}]"),
        day("[{\"runner\": \"operator\", \"usage\": 900}, {\"runner\": \"fork\\nreign counts, marks walked 0\", \"usage\": 950}]"),
        day("[{\"runner\": \"operator\", \"usage\": 900}]"),
    ];
    let path = write_file(
        &dir,
        "forged.json",
        &format!("{{\"days\": [{}]}}", days.join(", ")),
    );
    let outcome = cli(&["reign", &path]);
    assert_eq!(outcome.code, 1);
    assert!(!outcome
        .stdout
        .iter()
        .any(|line| line.contains("meet reign")));
    assert!(outcome
        .stdout
        .iter()
        .any(|line| line == "defect day 1 run 1 field runner holds a line break"));
}

#[test]
fn test_the_reign_command_writes_nothing() {
    let dir = scratch("reign-purity");
    let path = operator_meet(&dir, "meet.json");
    let before = std::fs::read(&path).expect("the meet reads");
    let outcome = cli(&["reign", &path]);
    assert_eq!(outcome.code, 0);
    let entries: Vec<String> = std::fs::read_dir(&dir)
        .expect("the scratch directory lists")
        .map(|entry| {
            entry
                .expect("the entry reads")
                .file_name()
                .to_string_lossy()
                .into_owned()
        })
        .collect();
    assert_eq!(entries, vec!["meet.json"]);
    assert_eq!(std::fs::read(&path).expect("the meet reads again"), before);
}
