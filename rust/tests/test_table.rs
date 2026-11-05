//! Tests for PG-15 and PG-16, the meet table over a meet.
//!
//! Written for Round 7 in the shape of the Round 6 meet tests.
//! Each table pin states its whole table, so a pin fails when
//! the shared walk, the competitor order, the lowest figure or
//! the mark it was first stood at, a held count, a close
//! figure, or a closing count moves, and the reconciliation
//! pins read the meet report and the table off the same meet
//! and fail when the two readings of the one walk disagree.

use std::path::PathBuf;

use num_bigint::BigInt;

use paceghost::fieldday::{DayRun, FieldDay};
use paceghost::meet::{load_meet_text, meet_report, meet_table, Meet};
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

fn table(days: &[String]) -> Vec<String> {
    meet_table(&loaded(days)).expect("the table reads")
}

/// The meet of the reconciliation and never holder pins.
///
/// Window 600 on every day. Alpha projects 300 on the first
/// day and 240 on the second, gamma projects 400 on the first
/// day and never holds the record, beta projects 540 on the
/// second day and 120 on the third and never holds it either,
/// and aardvark stands only on the third day, projects 60, and
/// takes the record there, so aardvark stands last in the
/// competitor order though its name sorts first.
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
fn test_the_table_of_the_hand_meet_is_stated_line_for_line() {
    assert_eq!(
        table(&hand_meet()),
        vec![
            "meet table, 3 days",
            "competitor alpha, lowest standing projection 240, set on the second day at 200 seconds, record held after 3 marks, standing projection 240 at the close",
            "competitor beta, lowest standing projection 120, set on the third day at 300 seconds, record held after 0 marks, standing projection 120 at the close",
            "competitor gamma, lowest standing projection 400, set on the first day at 300 seconds, record held after 0 marks, standing projection 400 at the close",
            "competitor aardvark, lowest standing projection 60, set on the third day at 100 seconds, record held after 2 marks, standing projection 60 at the close",
            "table counts, competitors 4, record holders 2, marks walked 5",
        ]
    );
}

fn close_figures(lines: &[String]) -> Vec<(String, i64)> {
    let mut figures = Vec::new();
    for line in lines {
        if !line.starts_with("competitor ") || line.contains("no standing projection") {
            continue;
        }
        let label = line
            .trim_start_matches("competitor ")
            .split(',')
            .next()
            .expect("the line names its competitor")
            .to_string();
        let close = line
            .rsplit("standing projection ")
            .next()
            .expect("the line names a close figure")
            .trim_end_matches(" at the close")
            .parse::<i64>()
            .expect("the close figure of this pin is a whole number");
        figures.push((label, close));
    }
    figures
}

fn held_counts(lines: &[String]) -> Vec<i64> {
    lines
        .iter()
        .filter(|line| line.starts_with("competitor "))
        .map(|line| {
            line.split("record held after ")
                .nth(1)
                .expect("the line names a held count")
                .split(' ')
                .next()
                .expect("the held count stands")
                .parse::<i64>()
                .expect("the held count is a whole number")
        })
        .collect()
}

fn finish(lines: &[String]) -> (String, i64) {
    let line = lines
        .iter()
        .find(|line| line.starts_with("finish, record held by "))
        .expect("the report names a finish");
    let rest = line.trim_start_matches("finish, record held by ");
    let (label, figure) = rest
        .split_once(" at a projection of ")
        .expect("the finish line names its figure");
    (
        label.to_string(),
        figure.parse::<i64>().expect("the finish figure is whole"),
    )
}

#[test]
fn test_the_lowest_close_figure_is_the_finish_figure_of_its_holder() {
    let meet = loaded(&hand_meet());
    let report = meet_report(&meet).expect("the report reads");
    let lines = meet_table(&meet).expect("the table reads");
    let figures = close_figures(&lines);
    let (lowest_label, lowest) = figures
        .iter()
        .min_by_key(|(_, figure)| *figure)
        .expect("a close figure stands")
        .clone();
    let (finish_label, finish_figure) = finish(&report);
    assert_eq!(lowest, finish_figure);
    assert_eq!(lowest_label, finish_label);
    assert_eq!(finish_label, "aardvark");
    assert_eq!(finish_figure, 60);
}

#[test]
fn test_the_held_counts_sum_to_the_marks_a_record_stood_after() {
    let meet = loaded(&hand_meet());
    let lines = meet_table(&meet).expect("the table reads");
    let counts = lines.last().expect("the counts line stands");
    let walked: i64 = counts
        .rsplit("marks walked ")
        .next()
        .expect("the counts line names the marks")
        .parse()
        .expect("the marks count is a whole number");
    assert_eq!(walked, 5);
    assert_eq!(held_counts(&lines).iter().sum::<i64>(), walked);
}

#[test]
fn test_a_holder_displaced_by_its_own_rise_stops_holding() {
    // Ace holds the record at 300 through the first day. On the
    // second day ace projects 540, its own standing projection
    // rising past bee standing at 390, so bee holds the record
    // after that mark though bee projected nothing there, and
    // ace takes the record back on the third day at 240.
    let days = vec![
        day(
            600,
            "ace",
            &[
                run_json("ace", 500, &[(100, 50)]),
                run_json("bee", 600, &[(200, 130)]),
            ],
        ),
        day(600, "ace", &[run_json("ace", 600, &[(100, 90)])]),
        day(
            600,
            "ace",
            &[
                run_json("ace", 400, &[(100, 40)]),
                run_json("bee", 700, &[(500, 300)]),
            ],
        ),
    ];
    assert_eq!(
        table(&days),
        vec![
            "meet table, 3 days",
            "competitor ace, lowest standing projection 240, set on the third day at 100 seconds, record held after 4 marks, standing projection 240 at the close",
            "competitor bee, lowest standing projection 360, set on the third day at 500 seconds, record held after 1 marks, standing projection 360 at the close",
            "table counts, competitors 2, record holders 2, marks walked 5",
        ]
    );
}

#[test]
fn test_an_equal_figure_alone_never_moves_the_record() {
    // Ace and bee both project 300 at the same mark of the
    // first day, and ace stands earlier in the competitor
    // order, so ace holds. Bee projects 300 again on the
    // second day and ace still stands at the figure, so ace
    // keeps holding. On the third day ace rises to 400 and bee
    // takes the record at the figure it has stood at since the
    // first day.
    let days = vec![
        day(
            600,
            "ace",
            &[
                run_json("ace", 300, &[(100, 50)]),
                run_json("bee", 300, &[(100, 50)]),
            ],
        ),
        day(
            600,
            "ace",
            &[
                run_json("ace", 300, &[]),
                run_json("bee", 300, &[(200, 100)]),
            ],
        ),
        day(
            600,
            "ace",
            &[
                run_json("ace", 400, &[(300, 200)]),
                run_json("bee", 300, &[]),
            ],
        ),
    ];
    assert_eq!(
        table(&days),
        vec![
            "meet table, 3 days",
            "competitor ace, lowest standing projection 300, set on the first day at 100 seconds, record held after 2 marks, standing projection 400 at the close",
            "competitor bee, lowest standing projection 300, set on the first day at 100 seconds, record held after 1 marks, standing projection 300 at the close",
            "table counts, competitors 2, record holders 2, marks walked 3",
        ]
    );
}

#[test]
fn test_the_lowest_figure_names_the_first_mark_it_was_stood_at() {
    // Ace projects 300 on every day, the same lowest figure at
    // three marks, and the table names the first of them.
    let days = vec![
        day(
            600,
            "ace",
            &[
                run_json("ace", 300, &[(100, 50)]),
                run_json("idle", 900, &[]),
            ],
        ),
        day(600, "ace", &[run_json("ace", 300, &[(200, 100)])]),
        day(600, "ace", &[run_json("ace", 300, &[(300, 150)])]),
    ];
    assert_eq!(
        table(&days),
        vec![
            "meet table, 3 days",
            "competitor ace, lowest standing projection 300, set on the first day at 100 seconds, record held after 3 marks, standing projection 300 at the close",
            "competitor idle, no standing projection at any mark, record held after 0 marks, no standing projection at the close",
            "table counts, competitors 2, record holders 1, marks walked 3",
        ]
    );
}

#[test]
fn test_a_meet_where_nothing_projects_renders_the_none_forms() {
    // Still reads only at elapsed zero, which projects no
    // figure, and mute holds no reads at all, so no mark stands
    // anywhere in the walk and every competitor reads in the
    // none forms.
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
        table(&days),
        vec![
            "meet table, 3 days",
            "competitor still, no standing projection at any mark, record held after 0 marks, no standing projection at the close",
            "competitor mute, no standing projection at any mark, record held after 0 marks, no standing projection at the close",
            "table counts, competitors 2, record holders 0, marks walked 0",
        ]
    );
}

#[test]
fn test_the_table_is_a_pure_function_of_the_meet() {
    let meet = loaded(&hand_meet());
    let first = meet_table(&meet).expect("the table reads");
    let second = meet_table(&meet).expect("the table reads again");
    assert_eq!(first, second);
    let report = meet_report(&meet).expect("the report reads");
    assert_ne!(first, report);
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
    let error =
        meet_table(&Meet { days: Vec::new() }).expect_err("the table names the precondition");
    assert_eq!(error, "the meet holds no days");
}

#[test]
fn test_a_hand_built_meet_past_the_ordinal_bound_names_the_precondition() {
    let days: Vec<FieldDay> = (0..11).map(|_| ordinary_day()).collect();
    let error = meet_table(&Meet { days }).expect_err("the table names the precondition");
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
    let error = meet_table(&bad_first).expect_err("the table names the precondition");
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
    let error = meet_table(&bad_last).expect_err("the table names the precondition");
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
    let error = meet_table(&meet).expect_err("the table names the precondition");
    assert_eq!(error, "a read usage value is not a finite number");
}

#[test]
fn test_a_hand_built_meet_with_no_runs_renders_an_empty_table() {
    let days: Vec<FieldDay> = (0..3)
        .map(|_| hand_day(Num::from_i64(1800), Vec::new()))
        .collect();
    let lines = meet_table(&Meet { days }).expect("the table reads");
    assert_eq!(
        lines,
        vec![
            "meet table, 3 days",
            "table counts, competitors 0, record holders 0, marks walked 0",
        ]
    );
}

#[test]
fn test_the_example_table_stands_as_hand_computed() {
    let examples = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples");
    let (meet, defects) = paceghost::meet::load_meet_file(&examples.join("meet.json"));
    assert_eq!(defects, Vec::<String>::new());
    let meet = meet.expect("the meet loads");
    let lines = meet_table(&meet).expect("the table reads");
    assert_eq!(
        lines,
        vec![
            "meet table, 3 days",
            "competitor operator, lowest standing projection 750, set on the third day at 600 seconds, record held after 3 marks, standing projection 750 at the close",
            "competitor night fork, lowest standing projection 720, set on the third day at 500 seconds, record held after 5 marks, standing projection 720 at the close",
            "competitor dawn fork, lowest standing projection 560, set on the third day at 450 seconds, record held after 8 marks, standing projection 560 at the close",
            "competitor quiet fork, no standing projection at any mark, record held after 0 marks, no standing projection at the close",
            "table counts, competitors 4, record holders 3, marks walked 16",
        ]
    );
    let report = meet_report(&meet).expect("the report reads");
    let figures = close_figures(&lines);
    let (lowest_label, lowest) = figures
        .iter()
        .min_by_key(|(_, figure)| *figure)
        .expect("a close figure stands")
        .clone();
    let (finish_label, finish_figure) = finish(&report);
    assert_eq!(lowest, finish_figure);
    assert_eq!(lowest_label, finish_label);
    assert_eq!(held_counts(&lines).iter().sum::<i64>(), 16);
}
