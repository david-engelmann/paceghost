//! Tests for PG-27 and PG-28, the traces reading over a meet.
//!
//! Written for Round 13. Each reading pin states its whole
//! reading, so a pin fails when the walk, a day's close state,
//! a phrase form, or a closing count moves. The corners stand
//! pinned at their edges: a competitor whose first standing
//! falls on a later day, a standing carried over a day holding
//! no marks, and a meet holding no marks at all. The relation
//! pins read the standings reading and the table over the same
//! meet and fail when the traces reading drifts from them, the
//! transposition holding line by line, and the guard pins hold
//! the reading inside the fencing closure of FIX-PG-11 and the
//! figure guard of FIX-PG-10 from its birth. Every pin in
//! this file fails before the round by the absence of the
//! reading, the meet_traces function not standing on the pre
//! round tree, and on the built tree each fails against a
//! wrong construction of the reading.

use paceghost::fieldday::{DayRun, FieldDay};
use paceghost::meet::{load_meet_text, meet_standings, meet_table, meet_traces, Meet};
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
/// marks leave the first competitor's standing carried. The
/// standings of this meet stand pinned whole in the standings
/// suite, and the traces pin below is their transposition.
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

/// A meet over four days whose third day holds no marks at
/// all, so the standings of the second day carry into the
/// third day's close in every section.
fn carried_meet() -> Meet {
    load_meet(&[
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
    ])
}

/// The close meet with a fourth competitor who holds no
/// reads on any day and so never stands, beside the third
/// competitor whose first standing falls on the second day.
fn late_and_never_meet() -> Meet {
    load_meet(&[
        day(
            1000,
            "a",
            &format!(
                "[{}, {}, {}, {}]",
                run("a", 500, &[(500, 250)]),
                run("b", 400, &[(500, 300)]),
                run("c", 300, &[]),
                run("d", 250, &[])
            ),
        ),
        day(
            1000,
            "a",
            &format!(
                "[{}, {}, {}, {}]",
                run("a", 450, &[(500, 180)]),
                run("b", 380, &[(250, 95)]),
                run("c", 200, &[(500, 100)]),
                run("d", 250, &[])
            ),
        ),
        day(
            1000,
            "a",
            &format!(
                "[{}, {}, {}, {}]",
                run("a", 400, &[]),
                run("b", 350, &[(500, 150)]),
                run("c", 180, &[(500, 80)]),
                run("d", 250, &[])
            ),
        ),
    ])
}

#[test]
fn test_the_traces_reading_states_every_competitor_at_every_close() {
    let lines = meet_traces(&close_meet()).expect("the traces read");
    assert_eq!(
        lines,
        vec![
            "meet traces, 3 days, 3 competitors",
            "competitor a",
            "the first day, standing projection 500 at the close",
            "the second day, standing projection 360 at the close",
            "the third day, standing projection 360 at the close",
            "close, standings held at 3 closes, standing projection 360 at the close",
            "competitor b",
            "the first day, standing projection 600 at the close",
            "the second day, standing projection 380 at the close",
            "the third day, standing projection 300 at the close",
            "close, standings held at 3 closes, standing projection 300 at the close",
            "competitor c",
            "the first day, no standing projection at the close",
            "the second day, standing projection 200 at the close",
            "the third day, standing projection 160 at the close",
            "close, standings held at 2 closes, standing projection 160 at the close",
            "traces counts, competitors 3, days 3, standings held 8",
        ]
    );
}

#[test]
fn test_a_day_holding_no_marks_carries_its_standings_in() {
    let lines = meet_traces(&carried_meet()).expect("the traces read");
    assert_eq!(
        lines,
        vec![
            "meet traces, 4 days, 2 competitors",
            "competitor x",
            "the first day, standing projection 360 at the close",
            "the second day, standing projection 300 at the close",
            "the third day, standing projection 300 at the close",
            "the fourth day, standing projection 300 at the close",
            "close, standings held at 4 closes, standing projection 300 at the close",
            "competitor y",
            "the first day, standing projection 300 at the close",
            "the second day, standing projection 300 at the close",
            "the third day, standing projection 300 at the close",
            "the fourth day, standing projection 180 at the close",
            "close, standings held at 4 closes, standing projection 180 at the close",
            "traces counts, competitors 2, days 4, standings held 8",
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
    let lines = meet_traces(&meet).expect("the traces read");
    assert_eq!(
        lines,
        vec![
            "meet traces, 3 days, 1 competitors",
            "competitor solo",
            "the first day, no standing projection at the close",
            "the second day, no standing projection at the close",
            "the third day, no standing projection at the close",
            "close, standings held at 0 closes, no standing projection at the close",
            "traces counts, competitors 1, days 3, standings held 0",
        ]
    );
}

// The relation pins of PG-28. The traces reading is the
// standings reading transposed, so every day line of a section
// states the close form the standings reading states for that
// competitor in that day's section, the last day line of a
// section states the standing the meet table states at the
// close, the standings held of the counts line equals the
// standings held of the standings counts line and the sum of
// the section closings, and no competitor falls back into the
// no standing form inside a section after a figured day.

fn close_form(line: &str) -> String {
    if line.contains("no standing projection at the close") {
        return "no standing projection at the close".to_string();
    }
    let start = line
        .find("standing projection ")
        .expect("a figured line names its figure");
    line[start..].to_string()
}

fn trace_sections(lines: &[String]) -> Vec<Vec<String>> {
    let mut out: Vec<Vec<String>> = Vec::new();
    for line in lines.iter().skip(1) {
        if line.starts_with("competitor ") {
            out.push(Vec::new());
        }
        if let Some(section) = out.last_mut() {
            section.push(line.clone());
        }
    }
    out
}

fn standings_sections(lines: &[String]) -> Vec<Vec<String>> {
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

fn assert_transposition(meet: &Meet, competitors: usize, days: usize) {
    let traces = meet_traces(meet).expect("the traces read");
    let standings = meet_standings(meet).expect("the standings read");
    let by_competitor = trace_sections(&traces);
    let by_day = standings_sections(&standings);
    assert_eq!(by_competitor.len(), competitors);
    assert_eq!(by_day.len(), days);
    for (competitor, section) in by_competitor.iter().enumerate() {
        let day_lines: Vec<&String> = section
            .iter()
            .filter(|line| line.starts_with("the "))
            .collect();
        assert_eq!(day_lines.len(), by_day.len());
        for (day_index, day_line) in day_lines.iter().enumerate() {
            let standings_line = by_day[day_index]
                .iter()
                .filter(|line| line.starts_with("competitor "))
                .nth(competitor)
                .expect("the standings section names the competitor");
            assert_eq!(
                close_form(day_line),
                close_form(standings_line),
                "competitor {} on day {}",
                competitor,
                day_index
            );
        }
    }
}

#[test]
fn test_the_traces_reading_transposes_the_standings_reading() {
    assert_transposition(&close_meet(), 3, 3);
    assert_transposition(&late_and_never_meet(), 4, 3);
}

#[test]
fn test_the_last_day_line_of_each_section_states_the_table_close() {
    let meet = close_meet();
    let traces = meet_traces(&meet).expect("the traces read");
    let table = meet_table(&meet).expect("the table reads");
    let table_lines: Vec<&String> = table
        .iter()
        .filter(|line| line.starts_with("competitor "))
        .collect();
    let sections = trace_sections(&traces);
    assert_eq!(sections.len(), table_lines.len());
    for (section, table_line) in sections.iter().zip(table_lines.iter()) {
        let last_day = section
            .iter()
            .rfind(|line| line.starts_with("the "))
            .expect("a section holds a day line");
        let form = close_form(last_day);
        if form == "no standing projection at the close" {
            assert!(
                table_line.ends_with("no standing projection at the close"),
                "{}",
                table_line
            );
        } else {
            assert!(table_line.ends_with(&form), "{}", table_line);
        }
    }
}

#[test]
fn test_the_counts_line_sums_the_section_closings() {
    let meet = close_meet();
    let traces = meet_traces(&meet).expect("the traces read");
    let standings = meet_standings(&meet).expect("the standings read");
    let closings: usize = traces
        .iter()
        .filter(|line| line.starts_with("close, standings held at "))
        .map(|line| {
            line.split("standings held at ")
                .nth(1)
                .and_then(|rest| rest.split(' ').next())
                .and_then(|word| word.parse::<usize>().ok())
                .expect("a section closing names its count")
        })
        .sum();
    let traces_counts = traces.last().expect("the reading closes");
    assert!(
        traces_counts.ends_with(&format!("standings held {}", closings)),
        "{}",
        traces_counts
    );
    let standings_counts = standings.last().expect("the standings close");
    let standings_held = standings_counts
        .rsplit(' ')
        .next()
        .expect("the standings counts name the held count");
    assert!(
        traces_counts.ends_with(&format!("standings held {}", standings_held)),
        "{}",
        traces_counts
    );
}

#[test]
fn test_no_competitor_falls_back_into_the_none_form_inside_a_section() {
    let lines = meet_traces(&close_meet()).expect("the traces read");
    for section in trace_sections(&lines) {
        let mut stood = false;
        for line in section.iter().filter(|line| line.starts_with("the ")) {
            if line.contains("no standing projection at the close") {
                assert!(!stood, "{}", line);
            } else {
                stood = true;
            }
        }
    }
}

// The guard pins. The traces reading stands inside the
// fencing closure of FIX-PG-11 from its birth, and under the
// figure guard of FIX-PG-10 and the day count errors of
// PG-13, in the one walk it shares with the seven readings
// before it.

#[test]
fn test_a_hand_built_meet_holding_no_days_names_the_precondition() {
    let meet = Meet { days: vec![] };
    let error = meet_traces(&meet).expect_err("the reading names the precondition");
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
    let error = meet_traces(&meet).expect_err("the reading names the precondition");
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
    let error = meet_traces(&meet).expect_err("the reading names the precondition");
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
    let error = meet_traces(&meet).expect_err("the reading names the precondition");
    assert_eq!(error, "a runner label holds a line break");
}
