//! Tests for the Round 5 example rematches and their golden reports.
//!
//! Written for Round 5 in the shape of the Round 4 example tests.
//! The first example record changes hands on the second day, and
//! the held example record is never taken from the first day.

use std::path::PathBuf;

use paceghost::cli::{dispatch, CliOutcome};
use paceghost::fieldday::load_field_day_file;
use paceghost::pytext::PText;
use paceghost::rematch::{load_rematch_file, rematch_report, Rematch};

fn examples() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples")
}

fn reader(bytes: &[u8]) -> Option<String> {
    let path = String::from_utf8_lossy(bytes).into_owned();
    let data = std::fs::read(path).ok()?;
    String::from_utf8(data).ok()
}

fn cli(args: &[&str]) -> CliOutcome {
    let argv: Vec<PText> = args.iter().map(|arg| PText::from_text(arg)).collect();
    dispatch(&argv, &reader)
}

fn load_example(name: &str) -> Rematch {
    let (rematch, defects) = load_rematch_file(&examples().join(name));
    assert_eq!(defects, Vec::<String>::new());
    rematch.expect("the rematch loads")
}

#[test]
fn test_the_example_rematch_loads_clean() {
    let rematch = load_example("rematch.json");
    assert_eq!(rematch.first.runs.len(), 4);
    assert_eq!(rematch.second.runs.len(), 3);
    assert!(rematch.first.operator.eq_str("operator"));
    assert!(rematch.second.operator.eq_str("operator"));
}

#[test]
fn test_the_examples_embed_the_round_three_field_day_as_the_first_day() {
    let (day, defects) = load_field_day_file(&examples().join("field-day.json"));
    assert_eq!(defects, Vec::<String>::new());
    let day = day.expect("the field day loads");
    assert_eq!(load_example("rematch.json").first, day);
    assert_eq!(load_example("rematch-held.json").first, day);
}

#[test]
fn test_the_example_report_matches_the_golden_bytes() {
    let rematch = load_example("rematch.json");
    let golden = std::fs::read(examples().join("rematch-golden.txt")).expect("the golden reads");
    let rendered = rematch_report(&rematch)
        .expect("the report reads")
        .join("\n")
        + "\n";
    assert_eq!(rendered.as_bytes(), golden.as_slice());
}

#[test]
fn test_the_rematch_command_prints_the_golden_report() {
    let path = examples()
        .join("rematch.json")
        .to_string_lossy()
        .into_owned();
    let outcome = cli(&["rematch", &path]);
    assert_eq!(outcome.code, 0);
    let golden = std::fs::read(examples().join("rematch-golden.txt")).expect("the golden reads");
    let printed = outcome.stdout.join("\n") + "\n";
    assert_eq!(printed.as_bytes(), golden.as_slice());
}

#[test]
fn test_the_held_example_report_matches_the_golden_bytes() {
    let rematch = load_example("rematch-held.json");
    let golden =
        std::fs::read(examples().join("rematch-held-golden.txt")).expect("the golden reads");
    let rendered = rematch_report(&rematch)
        .expect("the report reads")
        .join("\n")
        + "\n";
    assert_eq!(rendered.as_bytes(), golden.as_slice());
}

#[test]
fn test_the_held_command_prints_the_golden_report() {
    let path = examples()
        .join("rematch-held.json")
        .to_string_lossy()
        .into_owned();
    let outcome = cli(&["rematch", &path]);
    assert_eq!(outcome.code, 0);
    let golden =
        std::fs::read(examples().join("rematch-held-golden.txt")).expect("the golden reads");
    let printed = outcome.stdout.join("\n") + "\n";
    assert_eq!(printed.as_bytes(), golden.as_slice());
}

#[test]
fn test_the_first_day_lines_equal_the_field_day_golden_record_lines() {
    let golden =
        std::fs::read_to_string(examples().join("field-day-golden.txt")).expect("the golden reads");
    let own_record_lines: Vec<String> = golden
        .lines()
        .filter(|line| line.starts_with("record at "))
        .map(|line| line.to_string())
        .collect();
    for name in ["rematch.json", "rematch-held.json"] {
        let lines = rematch_report(&load_example(name)).expect("the report reads");
        let first_day_lines: Vec<String> = lines
            .iter()
            .filter(|line| line.contains(" on the first day, "))
            .map(|line| line.replace(" on the first day", ""))
            .collect();
        assert_eq!(first_day_lines, own_record_lines, "for {}", name);
    }
}

#[test]
fn test_the_closing_lines_state_the_change_of_hands_and_the_stand() {
    let lines = rematch_report(&load_example("rematch.json")).expect("the report reads");
    assert_eq!(
        lines[lines.len() - 2],
        "finish, record held by dawn fork at a projection of 675"
    );
    assert_eq!(
        lines[lines.len() - 1],
        "record changed hands on the second day at 400 seconds, dawn fork took the record from night fork"
    );
    let held = rematch_report(&load_example("rematch-held.json")).expect("the report reads");
    assert_eq!(
        held[held.len() - 2],
        "finish, record held by night fork at a projection of 840"
    );
    assert_eq!(
        held[held.len() - 1],
        "record never changed hands on the second day"
    );
}
