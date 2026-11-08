//! Tests for PG-25 and PG-26, the standings reading over a meet.
//!
//! Written for Round 12. Each reading pin states its whole
//! reading, so a pin fails when the walk, a day's close state,
//! a phrase form, or a closing count moves. The corners stand
//! pinned at their edges: a competitor whose first standing
//! falls on a later day, a day holding no marks carrying the
//! standings of the day before into its own section, and a
//! meet holding no marks at all. The relation pins read the
//! report and the table over the same meet and fail when the
//! standings reading drifts from them, and the guard pins hold
//! the reading inside the fencing closure of FIX-PG-11 and the
//! figure guard of FIX-PG-10 from its birth.

use paceghost::fieldday::{DayRun, FieldDay};
use paceghost::meet::{load_meet_text, meet_report, meet_standings, meet_table, Meet};
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

/// A meet over three competitors under one window, where the
/// standings change at every close, one competitor's first
/// standing falls on the second day, and the third day's only
/// marks leave the first competitor's standing carried.
fn close_meet() -> Meet {
    load_meet(&[
        day(
            1000,
            "a",
            &format!(
                "[{}, {}, {}]",
                run("a", 500, &[(500, 250)]),
                run("b", 400, &[(500, 300)]),
                run("c", 300, &[])
            ),
        ),
        day(
            1000,
            "a",
            &format!(
                "[{}, {}, {}]",
                run("a", 450, &[(500, 180)]),
                run("b", 380, &[(250, 95)]),
                run("c", 200, &[(500, 100)])
            ),
        ),
        day(
            1000,
            "a",
            &format!(
                "[{}, {}, {}]",
                run("a", 400, &[]),
                run("b", 350, &[(500, 150)]),
                run("c", 180, &[(500, 80)])
            ),
        ),
    ])
}

#[test]
fn test_the_standings_reading_states_every_close() {
    let lines = meet_standings(&close_meet()).expect("the standings read");
    assert_eq!(
        lines,
        vec![
            "meet standings, 3 days, 3 competitors",
            "day, the first day, standings 2",
            "competitor a, standing projection 500 at the close",
            "competitor b, standing projection 600 at the close",
            "competitor c, no standing projection at the close",
            "close of the first day, record held by a at a projection of 500",
            "day, the second day, standings 3",
            "competitor a, standing projection 360 at the close",
            "competitor b, standing projection 380 at the close",
            "competitor c, standing projection 200 at the close",
            "close of the second day, record held by c at a projection of 200",
            "day, the third day, standings 3",
            "competitor a, standing projection 360 at the close",
            "competitor b, standing projection 300 at the close",
            "competitor c, standing projection 160 at the close",
            "close of the third day, record held by c at a projection of 160",
            "standings counts, days 3, competitors 3, standings held 8",
        ]
    );
}

#[test]
fn test_a_day_holding_no_marks_carries_its_standings_in() {
    let meet = load_meet(&[
        day(
            600,
            "x",
            &format!(
                "[{}, {}]",
                run("x", 500, &[(100, 60)]),
                run("y", 700, &[(200, 100)])
            ),
        ),
        day(600, "x", &format!("[{}]", run("x", 500, &[(100, 50)]))),
        day(
            600,
            "x",
            &format!("[{}, {}]", run("x", 500, &[]), run("y", 700, &[])),
        ),
        day(600, "y", &format!("[{}]", run("y", 700, &[(100, 30)]))),
    ]);
    let lines = meet_standings(&meet).expect("the standings read");
    assert_eq!(
        lines,
        vec![
            "meet standings, 4 days, 2 competitors",
            "day, the first day, standings 2",
            "competitor x, standing projection 360 at the close",
            "competitor y, standing projection 300 at the close",
            "close of the first day, record held by y at a projection of 300",
            "day, the second day, standings 2",
            "competitor x, standing projection 300 at the close",
            "competitor y, standing projection 300 at the close",
            "close of the second day, record held by y at a projection of 300",
            "day, the third day, standings 2",
            "competitor x, standing projection 300 at the close",
            "competitor y, standing projection 300 at the close",
            "close of the third day, record held by y at a projection of 300",
            "day, the fourth day, standings 2",
            "competitor x, standing projection 300 at the close",
            "competitor y, standing projection 180 at the close",
            "close of the fourth day, record held by y at a projection of 180",
            "standings counts, days 4, competitors 2, standings held 8",
        ]
    );
}

#[test]
fn test_a_meet_holding_no_marks_stands_every_competitor_in_the_none_form() {
    let meet = load_meet(&[
        day(600, "solo", &format!("[{}]", run("solo", 500, &[]))),
        day(600, "solo", &format!("[{}]", run("solo", 500, &[]))),
        day(600, "solo", &format!("[{}]", run("solo", 500, &[]))),
    ]);
    let lines = meet_standings(&meet).expect("the standings read");
    assert_eq!(
        lines,
        vec![
            "meet standings, 3 days, 1 competitors",
            "day, the first day, standings 0",
            "competitor solo, no standing projection at the close",
            "close of the first day, no record stood",
            "day, the second day, standings 0",
            "competitor solo, no standing projection at the close",
            "close of the second day, no record stood",
            "day, the third day, standings 0",
            "competitor solo, no standing projection at the close",
            "close of the third day, no record stood",
            "standings counts, days 3, competitors 1, standings held 0",
        ]
    );
}

// The relation pins of PG-26. The section closing lines are
// the closing lines of the meet report for their days, byte
// for byte, the competitor lines of the last section state
// the standings the meet table states at the close, the
// standings held of the counts line is the sum of the section
// openings, and no competitor falls back into the no standing
// form after a section in which it stood with a figure.

fn sections(lines: &[String]) -> Vec<Vec<String>> {
    let mut out: Vec<Vec<String>> = Vec::new();
    for line in lines {
        if line.starts_with("day, the ") {
            out.push(Vec::new());
        }
        if let Some(section) = out.last_mut() {
            section.push(line.clone());
        }
    }
    out
}

#[test]
fn test_the_section_closings_are_the_report_closings() {
    let meet = close_meet();
    let lines = meet_standings(&meet).expect("the standings read");
    let report = meet_report(&meet).expect("the report reads");
    let report_closes: Vec<&String> = report
        .iter()
        .filter(|line| line.starts_with("close of the "))
        .collect();
    let sections = sections(&lines);
    assert_eq!(sections.len(), report_closes.len());
    for (section, close) in sections.iter().zip(report_closes.iter()) {
        let closing = section
            .iter()
            .find(|line| line.starts_with("close of the "))
            .expect("a section closes");
        assert_eq!(closing.as_str(), close.as_str());
    }
    assert_eq!(
        lines.last().map(String::as_str),
        Some("standings counts, days 3, competitors 3, standings held 8")
    );
    assert_eq!(report.last().map(String::as_str), Some("days read, 3"));
}

#[test]
fn test_the_last_section_states_the_table_close() {
    let meet = close_meet();
    let lines = meet_standings(&meet).expect("the standings read");
    let table = meet_table(&meet).expect("the table reads");
    let sections = sections(&lines);
    let last = sections.last().expect("a last section stands");
    let competitor_lines: Vec<&String> = last
        .iter()
        .filter(|line| line.starts_with("competitor "))
        .collect();
    let table_lines: Vec<&String> = table
        .iter()
        .filter(|line| line.starts_with("competitor "))
        .collect();
    assert_eq!(competitor_lines.len(), table_lines.len());
    for (line, table_line) in competitor_lines.iter().zip(table_lines.iter()) {
        if line.contains("no standing projection at the close") {
            assert!(
                table_line.ends_with("no standing projection at the close"),
                "{}",
                table_line
            );
        } else {
            let figure = line
                .strip_prefix("competitor ")
                .and_then(|rest| rest.split(", standing projection ").nth(1))
                .and_then(|rest| rest.strip_suffix(" at the close"))
                .expect("a competitor line names its figure");
            assert!(
                table_line.ends_with(&format!("standing projection {} at the close", figure)),
                "{}",
                table_line
            );
        }
    }
}

#[test]
fn test_the_counts_line_sums_the_section_openings() {
    let meet = close_meet();
    let lines = meet_standings(&meet).expect("the standings read");
    let opened: usize = lines
        .iter()
        .filter(|line| line.starts_with("day, the "))
        .map(|line| {
            line.rsplit(' ')
                .next()
                .and_then(|word| word.parse::<usize>().ok())
                .expect("a section opening names its count")
        })
        .sum();
    let counts = lines.last().expect("the reading closes");
    assert!(
        counts.ends_with(&format!("standings held {}", opened)),
        "{}",
        counts
    );
}

#[test]
fn test_no_competitor_falls_back_into_the_none_form() {
    let lines = meet_standings(&close_meet()).expect("the standings read");
    let sections = sections(&lines);
    let mut stood: Vec<bool> = Vec::new();
    for section in &sections {
        for (index, line) in section
            .iter()
            .filter(|line| line.starts_with("competitor "))
            .enumerate()
        {
            if stood.len() <= index {
                stood.push(false);
            }
            if line.contains("no standing projection at the close") {
                assert!(!stood[index], "{}", line);
            } else {
                stood[index] = true;
            }
        }
    }
}

// The guard pins. The standings reading stands inside the
// fencing closure of FIX-PG-11 from its birth, and under the
// figure guard of FIX-PG-10 and the day count errors of
// PG-13, in the one walk it shares with the six readings
// before it.

#[test]
fn test_a_hand_built_meet_holding_no_days_names_the_precondition() {
    let meet = Meet { days: vec![] };
    let error = meet_standings(&meet).expect_err("the reading names the precondition");
    assert_eq!(error, "the meet holds no days");
}

#[test]
fn test_a_hand_built_meet_holding_too_many_days_names_the_precondition() {
    let clean_day = || FieldDay {
        window_seconds: Num::from_i64(1000),
        operator: PText::from_text("a"),
        runs: vec![DayRun {
            runner: PText::from_text("a"),
            usage: None,
            reads: None,
        }],
    };
    let meet = Meet {
        days: (0..11).map(|_| clean_day()).collect(),
    };
    let error = meet_standings(&meet).expect_err("the reading names the precondition");
    assert_eq!(
        error,
        "the meet holds more days than the ordinal table names"
    );
}

#[test]
fn test_a_non_finite_window_names_the_precondition() {
    let clean_day = |window: Num| FieldDay {
        window_seconds: window,
        operator: PText::from_text("a"),
        runs: vec![DayRun {
            runner: PText::from_text("a"),
            usage: None,
            reads: Some(vec![Read {
                elapsed_seconds: Num::from_i64(100),
                usage: Num::from_i64(40),
            }]),
        }],
    };
    let meet = Meet {
        days: vec![
            clean_day(Num::from_i64(1000)),
            clean_day(Num::from_f64(f64::INFINITY)),
            clean_day(Num::from_i64(1000)),
        ],
    };
    let error = meet_standings(&meet).expect_err("the reading names the precondition");
    assert_eq!(error, "the window is not a finite number");
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
    let error = meet_standings(&meet).expect_err("the reading names the precondition");
    assert_eq!(error, "a runner label holds a line break");
}
