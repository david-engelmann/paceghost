//! Pins for the synthetic Round 3 example under examples.
//!
//! Translated from tests/test_examples_round3.py. The example field
//! day holds four runs in one window, three of them carrying timed
//! reads and one carrying none, and the operator run loses the
//! record to the night fork partway through the window. The stored
//! golden report has to stay byte identical to what the code renders
//! today, and to what the fieldday command prints.

use std::path::PathBuf;

use paceghost::cli::dispatch;
use paceghost::fieldday::{field_day_report, load_field_day_file, FieldDay};
use paceghost::pynum::Num;
use paceghost::pytext::PText;

fn examples() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples")
}

fn load_day() -> FieldDay {
    let (day, defects) = load_field_day_file(&examples().join("field-day.json"));
    assert_eq!(defects, Vec::<String>::new());
    day.expect("the field day loads")
}

#[test]
fn test_the_example_field_day_loads_clean_with_four_runs() {
    let day = load_day();
    assert_eq!(day.window_seconds, Num::from_i64(1800));
    assert!(day.operator.eq_str("operator"));
    let labels: Vec<String> = day
        .runs
        .iter()
        .map(|run| run.runner.to_display_string())
        .collect();
    assert_eq!(
        labels,
        vec!["operator", "night fork", "dawn fork", "quiet fork"]
    );
}

#[test]
fn test_the_example_carries_a_run_with_no_timed_reads() {
    let day = load_day();
    let quiet = &day.runs[3];
    assert!(quiet.runner.eq_str("quiet fork"));
    assert!(quiet.reads.is_none());
}

#[test]
fn test_the_golden_field_day_report_is_byte_identical() {
    let day = load_day();
    let rendered = field_day_report(&day).expect("the report reads").join("\n") + "\n";
    let golden = std::fs::read(examples().join("field-day-golden.txt")).expect("the golden reads");
    assert_eq!(rendered.as_bytes(), golden.as_slice());
}

#[test]
fn test_the_fieldday_command_prints_the_golden_report() {
    let path = examples()
        .join("field-day.json")
        .to_string_lossy()
        .into_owned();
    let outcome = dispatch(
        &[PText::from_text("fieldday"), PText::from_text(&path)],
        &|bytes: &[u8]| -> Option<String> {
            let path = String::from_utf8_lossy(bytes).into_owned();
            let data = std::fs::read(path).ok()?;
            String::from_utf8(data).ok()
        },
    );
    assert_eq!(outcome.code, 0);
    let golden =
        std::fs::read_to_string(examples().join("field-day-golden.txt")).expect("the golden reads");
    assert_eq!(outcome.stdout.join("\n") + "\n", golden);
}

#[test]
fn test_the_example_operator_loses_the_record_partway_through() {
    let day = load_day();
    let lines = field_day_report(&day).expect("the report reads");
    assert_eq!(
        lines[1],
        "record at 300 seconds, operator at a projection of 1260"
    );
    assert_eq!(
        lines[lines.len() - 1],
        "record stolen from operator at 900 seconds by night fork, margin 300 at the close"
    );
}
