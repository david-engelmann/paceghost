//! Tests for the Round 7 example meet table and its golden.
//!
//! Written for Round 7 in the shape of the Round 6 example
//! tests. The example is the standing meet of Round 6, which
//! does not move, and its table was counted by hand from the
//! meet file before the golden was produced.

use std::path::PathBuf;

use paceghost::cli::{dispatch, CliOutcome};
use paceghost::meet::{load_meet_file, meet_table, Meet};
use paceghost::pytext::PText;

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
fn test_the_example_table_matches_the_golden_bytes() {
    let meet = load_example("meet.json");
    let golden = std::fs::read(examples().join("meet-table-golden.txt")).expect("the golden reads");
    let rendered = meet_table(&meet).expect("the table reads").join("\n") + "\n";
    assert_eq!(rendered.as_bytes(), golden.as_slice());
}

#[test]
fn test_the_table_command_prints_the_golden_table() {
    let path = examples().join("meet.json").to_string_lossy().into_owned();
    let outcome = cli(&["table", &path]);
    assert_eq!(outcome.code, 0);
    let golden = std::fs::read(examples().join("meet-table-golden.txt")).expect("the golden reads");
    let printed = outcome.stdout.join("\n") + "\n";
    assert_eq!(printed.as_bytes(), golden.as_slice());
}

#[test]
fn test_the_example_table_closes_on_the_hand_counts() {
    let lines = meet_table(&load_example("meet.json")).expect("the table reads");
    assert_eq!(
        lines.last().map(|line| line.as_str()),
        Some("table counts, competitors 4, record holders 3, marks walked 16")
    );
    assert_eq!(
        lines[3],
        "competitor dawn fork, lowest standing projection 560, set on the third day at 450 seconds, record held after 8 marks, standing projection 560 at the close"
    );
    assert_eq!(
        lines[4],
        "competitor quiet fork, no standing projection at any mark, record held after 0 marks, no standing projection at the close"
    );
}
