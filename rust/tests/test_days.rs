//! Tests for PG-19 and PG-20, the days reading over a meet.
//!
//! Written for Round 9 in the shape of the Round 8 reign tests.
//! Each days pin states its whole reading, so a pin fails when
//! the shared walk, the day a mark stands in, a handover and
//! the day it is counted in, a day's close, or a closing count
//! moves, and the reconciliation pins read the meet report,
//! the reign reading, and the days reading off the same meet
//! and fail when the three readings of the one walk disagree.

use num_bigint::BigInt;

use paceghost::fieldday::{DayRun, FieldDay};
use paceghost::meet::{load_meet_text, meet_days, meet_reign, meet_report, Meet};
use paceghost::pynum::Num;
use paceghost::pytext::PText;
use paceghost::runs::Read;

fn day_json(window: i64, operator: &str, runs: &str) -> String {
    format!(
        "{{\"window_seconds\": {}, \"operator\": \"{}\", \"runs\": {}}}",
        window, operator, runs
    )
}

fn run_json(label: &str, usage: i64, reads: &[(i64, i64)]) -> String {
    if reads.is_empty() {
        return format!("{{\"runner\": \"{}\", \"usage\": {}}}", label, usage);
    }
    let reads_text = reads
        .iter()
        .map(|(elapsed, usage)| {
            format!("{{\"elapsed_seconds\": {}, \"usage\": {}}}", elapsed, usage)
        })
        .collect::<Vec<String>>()
        .join(", ");
    format!(
        "{{\"runner\": \"{}\", \"usage\": {}, \"reads\": [{}]}}",
        label, usage, reads_text
    )
}

fn runs_json(runs: &[String]) -> String {
    format!("[{}]", runs.join(", "))
}

fn meet_text(days: &[String]) -> String {
    format!("{{\"days\": [{}]}}", days.join(", "))
}

fn day(window: i64, operator: &str, runs: &[String]) -> String {
    day_json(window, operator, &runs_json(runs))
}

fn loaded(days: &[String]) -> Meet {
    let (meet, defects) = load_meet_text(&meet_text(days));
    assert_eq!(defects, Vec::<String>::new());
    meet.expect("the meet loads")
}

fn days_reading(days: &[String]) -> Vec<String> {
    meet_days(&loaded(days)).expect("the days read")
}

/// The meet of the line for line and reconciliation pins, the
/// hand meet of the Round 7 table tests and the Round 8 reign
/// tests.
///
/// Window 600 on every day. Alpha holds the record after the
/// only mark of the first day at a projection of 300, and
/// after both marks of the second day, its standing falling
/// to 240 at the second mark, and aardvark takes the record
/// at the first mark of the third day at a projection of 60
/// and holds it to the close, so the days read one handover,
/// counted on the third day.
fn hand_meet() -> Vec<String> {
    vec![
        day(
            600,
            "alpha",
            &[
                run_json("alpha", 500, &[(300, 150)]),
                run_json("beta", 900, &[]),
                run_json("gamma", 600, &[(300, 200)]),
            ],
        ),
        day(
            600,
            "alpha",
            &[
                run_json("alpha", 500, &[(200, 80)]),
                run_json("beta", 900, &[(100, 90)]),
            ],
        ),
        day(
            600,
            "beta",
            &[
                run_json("beta", 900, &[(300, 60)]),
                run_json("aardvark", 100, &[(100, 10)]),
            ],
        ),
    ]
}

#[test]
fn test_the_days_of_the_hand_meet_are_stated_line_for_line() {
    assert_eq!(
        days_reading(&hand_meet()),
        vec![
            "meet days, 3 days",
            "day, the first day, marks 1, handovers 0, record at the close held by alpha at a projection of 300",
            "day, the second day, marks 2, handovers 0, record at the close held by alpha at a projection of 240",
            "day, the third day, marks 2, handovers 1, record at the close held by aardvark at a projection of 60",
            "days counts, days 3, marks walked 5, handovers 1",
        ]
    );
}

fn day_counts(lines: &[String]) -> Vec<(i64, i64)> {
    lines
        .iter()
        .filter(|line| line.starts_with("day, "))
        .map(|line| {
            let marks = line
                .split("marks ")
                .nth(1)
                .expect("the day line names its marks")
                .split(',')
                .next()
                .expect("the marks count stands")
                .parse::<i64>()
                .expect("the marks count is a whole number");
            let handovers = line
                .split("handovers ")
                .nth(1)
                .expect("the day line names its handovers")
                .split(',')
                .next()
                .expect("the handovers count stands")
                .parse::<i64>()
                .expect("the handovers count is a whole number");
            (marks, handovers)
        })
        .collect()
}

fn counts_line_figures(lines: &[String]) -> (i64, i64, i64) {
    let counts = lines.last().expect("the counts line stands");
    let segments: Vec<&str> = counts.split(", ").collect();
    assert_eq!(segments.len(), 4, "the counts line holds four parts");
    assert_eq!(segments[0], "days counts");
    let days = segments[1]
        .strip_prefix("days ")
        .expect("the counts line names the days")
        .parse::<i64>()
        .expect("the days count is a whole number");
    let walked = segments[2]
        .strip_prefix("marks walked ")
        .expect("the counts line names the marks")
        .parse::<i64>()
        .expect("the marks count is a whole number");
    let handovers = segments[3]
        .strip_prefix("handovers ")
        .expect("the counts line names the handovers")
        .parse::<i64>()
        .expect("the handovers count is a whole number");
    (days, walked, handovers)
}

#[test]
fn test_day_marks_sum_to_the_reign_marks_walked() {
    let meet = loaded(&hand_meet());
    let lines = meet_days(&meet).expect("the days read");
    let reign = meet_reign(&meet).expect("the reign reads");
    let reign_walked: i64 = reign
        .last()
        .expect("the reign counts line stands")
        .split("marks walked ")
        .nth(1)
        .expect("the reign counts line names the marks")
        .split(',')
        .next()
        .expect("the marks count stands")
        .parse()
        .expect("the marks count is a whole number");
    let summed: i64 = day_counts(&lines).iter().map(|(marks, _)| marks).sum();
    let (_, walked, _) = counts_line_figures(&lines);
    assert_eq!(summed, walked);
    assert_eq!(walked, reign_walked);
    assert_eq!(walked, 5);
}

#[test]
fn test_handovers_equal_the_spells_minus_one() {
    let meet = loaded(&hand_meet());
    let lines = meet_days(&meet).expect("the days read");
    let reign = meet_reign(&meet).expect("the reign reads");
    let spells: i64 = reign
        .last()
        .expect("the reign counts line stands")
        .split("spells ")
        .nth(1)
        .expect("the reign counts line names the spells")
        .split(',')
        .next()
        .expect("the spells count stands")
        .parse()
        .expect("the spells count is a whole number");
    let summed: i64 = day_counts(&lines)
        .iter()
        .map(|(_, handovers)| handovers)
        .sum();
    let (_, _, handovers) = counts_line_figures(&lines);
    assert_eq!(summed, handovers);
    assert_eq!(handovers, spells - 1);
    assert_eq!(handovers, 1);
}

#[test]
fn test_day_closes_agree_with_the_meet_report_closes() {
    let meet = loaded(&hand_meet());
    let lines = meet_days(&meet).expect("the days read");
    let report = meet_report(&meet).expect("the report reads");
    let ordinals = ["first", "second", "third"];
    for (index, ordinal) in ordinals.iter().enumerate() {
        let close = report
            .iter()
            .find(|line| line.starts_with(&format!("close of the {} day, ", ordinal)))
            .expect("the report closes the day");
        let day_line = &lines[index + 1];
        if let Some(rest) =
            close.strip_prefix(&format!("close of the {} day, record held by ", ordinal))
        {
            assert_eq!(
                day_line.rsplit(", record at the close held by ").next(),
                Some(rest),
                "the day line close names the report close for the {} day",
                ordinal
            );
        } else {
            assert!(
                close.ends_with("no record stood"),
                "the report close for the {} day is one of its two forms",
                ordinal
            );
            assert!(
                day_line.ends_with("no record stood at the close"),
                "the day line for the {} day closes with no record stood",
                ordinal
            );
        }
    }
    let finish = report
        .iter()
        .find(|line| line.starts_with("finish, record held by "))
        .expect("the report finish line stands");
    let finish_rest = finish
        .strip_prefix("finish, record held by ")
        .expect("the finish line names its holder");
    let last_day = lines
        .iter()
        .rev()
        .find(|line| line.starts_with("day, "))
        .expect("a day line stands");
    assert_eq!(
        last_day.rsplit(", record at the close held by ").next(),
        Some(finish_rest),
        "the last day line close names the report finish"
    );
}

#[test]
fn test_a_handover_at_the_first_mark_of_a_day_counts_in_that_day() {
    // Alpha holds the only mark of the first day. Beta takes
    // the record at the first mark of the second day and
    // holds both of its marks, so the one handover of the
    // meet stands on the second day and not on the first.
    let days = vec![
        day(600, "alpha", &[run_json("alpha", 500, &[(300, 150)])]),
        day(
            600,
            "beta",
            &[run_json("beta", 900, &[(100, 10), (200, 30)])],
        ),
        day(600, "beta", &[run_json("beta", 900, &[(100, 5)])]),
    ];
    assert_eq!(
        days_reading(&days),
        vec![
            "meet days, 3 days",
            "day, the first day, marks 1, handovers 0, record at the close held by alpha at a projection of 300",
            "day, the second day, marks 2, handovers 1, record at the close held by beta at a projection of 90",
            "day, the third day, marks 1, handovers 0, record at the close held by beta at a projection of 30",
            "days counts, days 3, marks walked 4, handovers 1",
        ]
    );
}

#[test]
fn test_a_middle_day_with_no_marks_carries_the_record_at_its_close() {
    // The second day carries reads only at elapsed zero,
    // which project no figure, so the day holds no mark and
    // no handover and closes with the record of the first
    // day carried standing.
    let days = vec![
        day(600, "alpha", &[run_json("alpha", 500, &[(300, 150)])]),
        day(
            600,
            "alpha",
            &[
                run_json("alpha", 500, &[(0, 10)]),
                run_json("beta", 900, &[(0, 20)]),
            ],
        ),
        day(600, "alpha", &[run_json("alpha", 500, &[(100, 10)])]),
    ];
    assert_eq!(
        days_reading(&days),
        vec![
            "meet days, 3 days",
            "day, the first day, marks 1, handovers 0, record at the close held by alpha at a projection of 300",
            "day, the second day, marks 0, handovers 0, record at the close held by alpha at a projection of 300",
            "day, the third day, marks 1, handovers 0, record at the close held by alpha at a projection of 60",
            "days counts, days 3, marks walked 2, handovers 0",
        ]
    );
}

#[test]
fn test_a_day_before_the_first_projection_closes_with_no_record_stood() {
    // No read of the first day projects, its runs holding no
    // reads at all, so the first day closes with no record
    // stood and the reading opens on the second day.
    let days = vec![
        day(
            600,
            "alpha",
            &[run_json("alpha", 500, &[]), run_json("beta", 900, &[])],
        ),
        day(600, "alpha", &[run_json("alpha", 500, &[(200, 80)])]),
        day(600, "alpha", &[run_json("alpha", 500, &[(100, 10)])]),
    ];
    assert_eq!(
        days_reading(&days),
        vec![
            "meet days, 3 days",
            "day, the first day, marks 0, handovers 0, no record stood at the close",
            "day, the second day, marks 1, handovers 0, record at the close held by alpha at a projection of 240",
            "day, the third day, marks 1, handovers 0, record at the close held by alpha at a projection of 60",
            "days counts, days 3, marks walked 2, handovers 0",
        ]
    );
}

#[test]
fn test_a_meet_where_nothing_projects_reads_every_day_at_zero() {
    // Reads only at elapsed zero project no figure and mute
    // holds no reads at all, so no mark stands anywhere in
    // the walk and every day reads at zero with no record
    // stood at its close.
    let days = vec![
        day(
            600,
            "still",
            &[
                run_json("still", 100, &[(0, 10)]),
                run_json("mute", 100, &[]),
            ],
        ),
        day(600, "still", &[run_json("still", 100, &[(0, 20)])]),
        day(600, "still", &[run_json("still", 100, &[(0, 30)])]),
    ];
    assert_eq!(
        days_reading(&days),
        vec![
            "meet days, 3 days",
            "day, the first day, marks 0, handovers 0, no record stood at the close",
            "day, the second day, marks 0, handovers 0, no record stood at the close",
            "day, the third day, marks 0, handovers 0, no record stood at the close",
            "days counts, days 3, marks walked 0, handovers 0",
        ]
    );
}

#[test]
fn test_the_days_are_a_pure_function_of_the_meet() {
    let meet = loaded(&hand_meet());
    let first = meet_days(&meet).expect("the days read");
    let second = meet_days(&meet).expect("the days read again");
    assert_eq!(first, second);
    let reign = meet_reign(&meet).expect("the reign reads");
    assert_ne!(first, reign);
}

fn hand_day(window: Num, runs: Vec<DayRun>) -> FieldDay {
    FieldDay {
        window_seconds: window,
        operator: PText::from_text("a"),
        runs,
    }
}

fn ordinary_run() -> DayRun {
    DayRun {
        runner: PText::from_text("a"),
        usage: None,
        reads: Some(vec![Read {
            elapsed_seconds: Num::from_i64(300),
            usage: Num::from_i64(100),
        }]),
    }
}

fn ordinary_day() -> FieldDay {
    hand_day(Num::from_i64(1800), vec![ordinary_run()])
}

#[test]
fn test_a_hand_built_meet_holding_no_days_names_the_precondition() {
    let error = meet_days(&Meet { days: Vec::new() }).expect_err("the days name the precondition");
    assert_eq!(error, "the meet holds no days");
}

#[test]
fn test_a_hand_built_meet_past_the_ordinal_bound_names_the_precondition() {
    let days: Vec<FieldDay> = (0..11).map(|_| ordinary_day()).collect();
    let error = meet_days(&Meet { days }).expect_err("the days name the precondition");
    assert_eq!(
        error,
        "the meet holds more days than the ordinal table names"
    );
}

#[test]
fn test_a_hand_built_meet_with_a_non_finite_window_names_it_in_day_order() {
    let bad_first = Meet {
        days: vec![
            ordinary_day(),
            hand_day(Num::from_f64(f64::NAN), vec![ordinary_run()]),
            ordinary_day(),
        ],
    };
    let error = meet_days(&bad_first).expect_err("the days name the precondition");
    assert_eq!(error, "the window is not a finite number");
    let oversized = {
        let mut value = BigInt::from(1);
        for _ in 0..400 {
            value *= 10;
        }
        Num::Int(value)
    };
    let bad_last = Meet {
        days: vec![
            ordinary_day(),
            ordinary_day(),
            hand_day(oversized, vec![ordinary_run()]),
        ],
    };
    let error = meet_days(&bad_last).expect_err("the days name the precondition");
    assert_eq!(error, "the window is not a finite number");
}

#[test]
fn test_a_hand_built_meet_with_a_non_finite_read_figure_names_it() {
    let bad_run = DayRun {
        runner: PText::from_text("a"),
        usage: None,
        reads: Some(vec![Read {
            elapsed_seconds: Num::from_i64(300),
            usage: Num::from_f64(f64::INFINITY),
        }]),
    };
    let meet = Meet {
        days: vec![
            hand_day(Num::from_i64(1800), vec![bad_run]),
            ordinary_day(),
            ordinary_day(),
        ],
    };
    let error = meet_days(&meet).expect_err("the days name the precondition");
    assert_eq!(error, "a read usage value is not a finite number");
}

#[test]
fn test_a_hand_built_meet_with_no_runs_reads_every_day_at_zero() {
    let days: Vec<FieldDay> = (0..3)
        .map(|_| hand_day(Num::from_i64(1800), Vec::new()))
        .collect();
    let lines = meet_days(&Meet { days }).expect("the days read");
    assert_eq!(
        lines,
        vec![
            "meet days, 3 days",
            "day, the first day, marks 0, handovers 0, no record stood at the close",
            "day, the second day, marks 0, handovers 0, no record stood at the close",
            "day, the third day, marks 0, handovers 0, no record stood at the close",
            "days counts, days 3, marks walked 0, handovers 0",
        ]
    );
}
