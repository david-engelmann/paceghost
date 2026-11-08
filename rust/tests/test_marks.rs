//! Tests for PG-23 and PG-24, the marks reading over a meet.
//!
//! Written for Round 11. Each reading pin states its whole
//! reading, so a pin fails when the walk, the classification of
//! a mark against the state before it, a phrase form, or a
//! closing count moves. The classification corners stand pinned
//! at their edges: a mark that takes the record at an unchanged
//! figure when the standing holder's own read rises past it, a
//! mark whose own read raises the record its setter already
//! holds, a day holding no marks, and a meet holding no marks
//! at all. The relation pins read the days, reign, and report
//! readings over the same meet and fail when the marks reading
//! drifts from them.

use paceghost::fieldday::{DayRun, FieldDay};
use paceghost::meet::{load_meet_text, meet_days, meet_marks, meet_reign, meet_report, Meet};
use paceghost::pynum::Num;
use paceghost::pytext::PText;
use paceghost::runs::Read;

fn day(window: i64, operator: &str, runs: &str) -> String {
    format!(
        "{{\"window_seconds\": {}, \"operator\": \"{}\", \"runs\": {}}}",
        window, operator, runs
    )
}

fn run(label: &str, usage: i64, reads: &[(i64, i64)]) -> String {
    if reads.is_empty() {
        return format!("{{\"runner\": \"{}\", \"usage\": {}}}", label, usage);
    }
    let parts: Vec<String> = reads
        .iter()
        .map(|(elapsed, used)| format!("{{\"elapsed_seconds\": {}, \"usage\": {}}}", elapsed, used))
        .collect();
    format!(
        "{{\"runner\": \"{}\", \"usage\": {}, \"reads\": [{}]}}",
        label,
        usage,
        parts.join(", ")
    )
}

fn load_meet(days: &[String]) -> Meet {
    let text = format!("{{\"days\": [{}]}}", days.join(", "));
    let (meet, defects) = load_meet_text(&text);
    assert_eq!(defects, Vec::<String>::new());
    meet.expect("the meet loads")
}

/// A meet over two competitors whose marks cover all four
/// standings, including a taking at an unchanged figure on the
/// second day, when x raises its own standing past the record
/// and y, standing at the old figure, takes the record at it.
fn standing_meet() -> Meet {
    load_meet(&[
        day(
            600,
            "x",
            &format!(
                "[{}, {}]",
                run("x", 500, &[(100, 60), (300, 140), (400, 160)]),
                run("y", 700, &[(200, 100), (500, 200)])
            ),
        ),
        day(600, "x", &format!("[{}]", run("x", 500, &[(100, 50)]))),
        day(
            600,
            "x",
            &format!("[{}, {}]", run("x", 500, &[]), run("y", 700, &[(100, 30)])),
        ),
    ])
}

#[test]
fn test_the_marks_reading_states_every_mark_against_the_state_before_it() {
    let lines = meet_marks(&standing_meet()).expect("the marks read");
    assert_eq!(
        lines,
        vec![
            "meet marks, 3 days",
            "day, the first day, marks 5",
            "mark 1, at 100 seconds on the first day, x at a projection of 360, set the standing record",
            "mark 2, at 200 seconds on the first day, y at a projection of 300, took the record from x, displacing a projection of 360",
            "mark 3, at 300 seconds on the first day, x at a projection of 280, took the record from y, displacing a projection of 300",
            "mark 4, at 400 seconds on the first day, x at a projection of 240, extended its own record, displacing a projection of 280",
            "mark 5, at 500 seconds on the first day, x at a projection of 240, stood below the standing record of 240",
            "day, the second day, marks 1",
            "mark 6, at 100 seconds on the second day, y at a projection of 240, took the record from x, displacing a projection of 240",
            "day, the third day, marks 1",
            "mark 7, at 100 seconds on the third day, y at a projection of 180, extended its own record, displacing a projection of 240",
            "marks counts, marks walked 7, record marks 6, days 3",
        ]
    );
}

#[test]
fn test_a_mark_raising_the_record_its_holder_holds_stands_below() {
    let meet = load_meet(&[
        day(1000, "a", &format!("[{}]", run("a", 400, &[(100, 40)]))),
        day(1000, "a", &format!("[{}]", run("a", 500, &[(100, 50)]))),
        day(1000, "a", &format!("[{}]", run("a", 600, &[(100, 60)]))),
    ]);
    let lines = meet_marks(&meet).expect("the marks read");
    assert_eq!(
        lines,
        vec![
            "meet marks, 3 days",
            "day, the first day, marks 1",
            "mark 1, at 100 seconds on the first day, a at a projection of 400, set the standing record",
            "day, the second day, marks 1",
            "mark 2, at 100 seconds on the second day, a at a projection of 500, stood below the standing record of 400",
            "day, the third day, marks 1",
            "mark 3, at 100 seconds on the third day, a at a projection of 600, stood below the standing record of 500",
            "marks counts, marks walked 3, record marks 1, days 3",
        ]
    );
}

#[test]
fn test_a_day_holding_no_marks_holds_an_empty_section() {
    let meet = load_meet(&[
        day(1000, "a", &format!("[{}]", run("a", 400, &[(100, 40)]))),
        day(
            1000,
            "a",
            &format!("[{}, {}]", run("a", 400, &[]), run("b", 900, &[])),
        ),
        day(1000, "b", &format!("[{}]", run("b", 300, &[(100, 30)]))),
    ]);
    let lines = meet_marks(&meet).expect("the marks read");
    assert_eq!(
        lines,
        vec![
            "meet marks, 3 days",
            "day, the first day, marks 1",
            "mark 1, at 100 seconds on the first day, a at a projection of 400, set the standing record",
            "day, the second day, marks 0",
            "day, the third day, marks 1",
            "mark 2, at 100 seconds on the third day, b at a projection of 300, took the record from a, displacing a projection of 400",
            "marks counts, marks walked 2, record marks 2, days 3",
        ]
    );
}

#[test]
fn test_a_meet_holding_no_marks_at_all_reads_empty_sections() {
    let meet = load_meet(&[
        day(1000, "a", &format!("[{}]", run("a", 400, &[]))),
        day(1000, "a", &format!("[{}]", run("a", 400, &[]))),
        day(1000, "a", &format!("[{}]", run("a", 400, &[]))),
    ]);
    let lines = meet_marks(&meet).expect("the marks read");
    assert_eq!(
        lines,
        vec![
            "meet marks, 3 days",
            "day, the first day, marks 0",
            "day, the second day, marks 0",
            "day, the third day, marks 0",
            "marks counts, marks walked 0, record marks 0, days 3",
        ]
    );
}

#[test]
fn test_the_closings_bind_to_the_days_reign_and_report_readings() {
    let meet = standing_meet();
    let marks = meet_marks(&meet).expect("the marks read");
    let set_or_took = marks
        .iter()
        .filter(|line| {
            line.contains(", set the standing record") || line.contains(", took the record from")
        })
        .count();
    let reign = meet_reign(&meet).expect("the reign reads");
    let spells: usize = reign
        .last()
        .expect("the reign closes")
        .split(", spells ")
        .nth(1)
        .and_then(|rest| rest.split(',').next())
        .and_then(|text| text.parse().ok())
        .expect("the reign closing names its spells");
    assert_eq!(set_or_took, spells);
    let days = meet_days(&meet).expect("the days read");
    let per_day: usize = days
        .iter()
        .filter(|line| line.starts_with("day, the "))
        .map(|line| {
            line.split(", marks ")
                .nth(1)
                .and_then(|rest| rest.split(',').next())
                .and_then(|text| text.parse::<usize>().ok())
                .expect("a day line names its marks")
        })
        .sum();
    let closing = marks.last().expect("the marks close");
    assert!(
        closing.contains(&format!("marks walked {}", per_day)),
        "{}",
        closing
    );
    let report = meet_report(&meet).expect("the report reads");
    assert_eq!(report.last().map(String::as_str), Some("days read, 3"));
    assert!(closing.ends_with("days 3"), "{}", closing);
}

// The guard pins. The marks reading stands inside the fencing
// closure of FIX-PG-11 from its birth, and under the figure
// guard of FIX-PG-10 and the day count errors of PG-13, in the
// one walk it shares with the five readings before it.

#[test]
fn test_a_hand_built_meet_holding_no_days_names_the_precondition() {
    let meet = Meet { days: vec![] };
    let error = meet_marks(&meet).expect_err("the reading names the precondition");
    assert_eq!(error, "the meet holds no days");
}

#[test]
fn test_a_break_bearing_label_names_the_precondition() {
    let clean_day = |label: &str| FieldDay {
        window_seconds: Num::from_i64(1000),
        operator: PText::from_text(label),
        runs: vec![DayRun {
            runner: PText::from_text(label),
            usage: None,
            reads: Some(vec![Read {
                elapsed_seconds: Num::from_i64(100),
                usage: Num::from_i64(40),
            }]),
        }],
    };
    let mut broken_day = clean_day("a");
    broken_day.runs.push(DayRun {
        runner: PText::from_text("b\nb"),
        usage: None,
        reads: None,
    });
    let meet = Meet {
        days: vec![clean_day("a"), broken_day, clean_day("a")],
    };
    let error = meet_marks(&meet).expect_err("the reading names the precondition");
    assert_eq!(error, "a runner label holds a line break");
}
