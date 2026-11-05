//! Tests for the Round 6 example meet and its golden report.
//!
//! Written for Round 6 in the shape of the Round 5 example tests.
//! The example meet embeds the rematch example's two days whole
//! as its first two days, and its third day takes the record.

use std::path::PathBuf;

use paceghost::cli::{dispatch, CliOutcome};
use paceghost::meet::{load_meet_file, meet_report, Meet};
use paceghost::pytext::PText;
use paceghost::rematch::load_rematch_file;

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

fn load_example(name: &str) -> Meet {
    let (meet, defects) = load_meet_file(&examples().join(name));
    assert_eq!(defects, Vec::<String>::new());
    meet.expect("the meet loads")
}

#[test]
fn test_the_example_meet_loads_clean() {
    let meet = load_example("meet.json");
    assert_eq!(meet.days.len(), 3);
    assert_eq!(meet.days[0].runs.len(), 4);
    assert_eq!(meet.days[1].runs.len(), 3);
    assert_eq!(meet.days[2].runs.len(), 3);
    assert!(meet.days[2].operator.eq_str("operator"));
}

#[test]
fn test_the_example_meet_embeds_the_rematch_days_whole() {
    let (rematch, defects) = load_rematch_file(&examples().join("rematch.json"));
    assert_eq!(defects, Vec::<String>::new());
    let rematch = rematch.expect("the rematch loads");
    let meet = load_example("meet.json");
    assert_eq!(meet.days[0], rematch.first);
    assert_eq!(meet.days[1], rematch.second);
}

#[test]
fn test_the_example_report_matches_the_golden_bytes() {
    let meet = load_example("meet.json");
    let golden = std::fs::read(examples().join("meet-golden.txt")).expect("the golden reads");
    let rendered = meet_report(&meet).expect("the report reads").join("\n") + "\n";
    assert_eq!(rendered.as_bytes(), golden.as_slice());
}

#[test]
fn test_the_meet_command_prints_the_golden_report() {
    let path = examples().join("meet.json").to_string_lossy().into_owned();
    let outcome = cli(&["meet", &path]);
    assert_eq!(outcome.code, 0);
    let golden = std::fs::read(examples().join("meet-golden.txt")).expect("the golden reads");
    let printed = outcome.stdout.join("\n") + "\n";
    assert_eq!(printed.as_bytes(), golden.as_slice());
}

#[test]
fn test_the_first_two_days_record_lines_equal_the_rematch_golden_lines() {
    let golden =
        std::fs::read_to_string(examples().join("rematch-golden.txt")).expect("the golden reads");
    let rematch_records: Vec<String> = golden
        .lines()
        .filter(|line| line.starts_with("record at "))
        .map(|line| line.to_string())
        .collect();
    let lines = meet_report(&load_example("meet.json")).expect("the report reads");
    let meet_prefix_records: Vec<String> = lines
        .iter()
        .filter(|line| {
            line.starts_with("record at ")
                && (line.contains(" on the first day, ") || line.contains(" on the second day, "))
        })
        .cloned()
        .collect();
    assert_eq!(meet_prefix_records, rematch_records);
}

#[test]
fn test_the_third_day_takes_the_record() {
    let lines = meet_report(&load_example("meet.json")).expect("the report reads");
    assert_eq!(
        lines[lines.len() - 4],
        "close of the third day, record held by dawn fork at a projection of 560"
    );
    assert_eq!(
        lines[lines.len() - 3],
        "finish, record held by dawn fork at a projection of 560"
    );
    assert_eq!(
        lines[lines.len() - 2],
        "record set on the third day at 450 seconds"
    );
    assert_eq!(lines[lines.len() - 1], "days read, 3");
}
