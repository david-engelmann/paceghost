//! Tests for PG-9 and PG-10, the ladder loader and report.
//!
//! Translated from tests/test_ladder.py.

use std::path::PathBuf;

use num_bigint::BigInt;

use paceghost::fieldday::{field_day_report, load_field_day_text, FieldDay};
use paceghost::ladder::{ladder_report, load_ladder_file, load_ladder_text, Ladder};
use paceghost::pynum::Num;
use paceghost::pytext::PText;

fn day_json(window: i64, operator: &str, runs: &str) -> String {
    format!(
        "{{\"window_seconds\": {}, \"operator\": \"{}\", \"runs\": {}}}",
        window, operator, runs
    )
}

fn std_runs() -> String {
    "[{\"runner\": \"operator\", \"usage\": 900, \"reads\": [{\"elapsed_seconds\": 300, \"usage\": 150}]}, {\"runner\": \"fork\", \"usage\": 950, \"reads\": [{\"elapsed_seconds\": 600, \"usage\": 320}]}]".to_string()
}

fn std_day() -> String {
    day_json(1800, "operator", &std_runs())
}

fn ladder_text(windows: &str, day: &str) -> String {
    format!("{{\"day\": {}, \"windows\": {}}}", day, windows)
}

fn loaded_ladder(windows: &str, day: &str) -> Ladder {
    let (ladder, defects) = load_ladder_text(&ladder_text(windows, day));
    assert_eq!(defects, Vec::<String>::new());
    ladder.expect("the ladder loads")
}

fn report_for(runs: &str, windows: &str, operator: &str, day_window: i64) -> Vec<String> {
    let day = day_json(day_window, operator, runs);
    ladder_report(&loaded_ladder(windows, &day)).expect("the report reads")
}

fn temp_path(name: &str) -> PathBuf {
    std::env::temp_dir().join(format!("paceghost-ladder-{}-{}", std::process::id(), name))
}

#[test]
fn test_a_clean_ladder_loads_carrying_the_day_and_the_windows() {
    let ladder = loaded_ladder("[900.0, 600]", &std_day());
    assert_eq!(ladder.windows, vec![BigInt::from(900), BigInt::from(600)]);
    assert!(ladder.day.operator.eq_str("operator"));
    assert_eq!(ladder.day.window_seconds, Num::from_i64(1800));
    let labels: Vec<String> = ladder
        .day
        .runs
        .iter()
        .map(|run| run.runner.to_display_string())
        .collect();
    assert_eq!(labels, vec!["operator", "fork"]);
}

#[test]
fn test_extra_fields_are_ignored() {
    let text = format!(
        "{{\"day\": {}, \"windows\": [600], \"note\": \"kept out\"}}",
        std_day()
    );
    let (ladder, defects) = load_ladder_text(&text);
    assert_eq!(defects, Vec::<String>::new());
    assert_eq!(
        ladder.expect("the ladder loads").windows,
        vec![BigInt::from(600)]
    );
}

#[test]
fn test_text_that_is_not_json_is_refused() {
    let (ladder, defects) = load_ladder_text("not json");
    assert!(ladder.is_none());
    assert_eq!(defects, vec!["input is not valid JSON"]);
}

#[test]
fn test_input_nested_too_deeply_is_refused() {
    let text = format!("{}{}", "[".repeat(100000), "]".repeat(100000));
    let (ladder, defects) = load_ladder_text(&text);
    assert!(ladder.is_none());
    assert_eq!(defects, vec!["input is not valid JSON"]);
}

#[test]
fn test_json_that_is_not_an_object_is_refused() {
    let (ladder, defects) = load_ladder_text("[1, 2]");
    assert!(ladder.is_none());
    assert_eq!(defects, vec!["input is not a JSON object"]);
}

#[test]
fn test_a_missing_day_is_refused() {
    let (ladder, defects) = load_ladder_text("{\"windows\": [600]}");
    assert!(ladder.is_none());
    assert_eq!(defects, vec!["missing field day"]);
}

#[test]
fn test_a_day_that_is_not_an_object_is_refused() {
    let (ladder, defects) = load_ladder_text(&ladder_text("[600]", "[1, 2]"));
    assert!(ladder.is_none());
    assert_eq!(defects, vec!["field day is not an object"]);
}

#[test]
fn test_day_defects_stand_prefixed_and_before_the_window_defects() {
    let day = day_json(1800, "operator", "[]");
    let text = format!("{{\"day\": {}, \"windows\": \"wide\"}}", day);
    let (ladder, defects) = load_ladder_text(&text);
    assert!(ladder.is_none());
    assert_eq!(
        defects,
        vec![
            "day field runs is empty",
            "field windows has the wrong type"
        ]
    );
}

#[test]
fn test_a_line_break_inside_the_day_is_refused_under_the_day_prefix() {
    let runs = "[{\"runner\": \"operator\", \"usage\": 900}, {\"runner\": \"fork\\nfinish, no record\", \"usage\": 950}]";
    let day = day_json(1800, "operator", runs);
    let (ladder, defects) = load_ladder_text(&ladder_text("[600]", &day));
    assert!(ladder.is_none());
    assert_eq!(defects, vec!["day run 1 field runner holds a line break"]);
}

#[test]
fn test_missing_windows_are_refused() {
    let text = format!("{{\"day\": {}}}", std_day());
    let (ladder, defects) = load_ladder_text(&text);
    assert!(ladder.is_none());
    assert_eq!(defects, vec!["missing field windows"]);
}

#[test]
fn test_windows_of_the_wrong_type_are_refused() {
    let text = format!("{{\"day\": {}, \"windows\": \"wide\"}}", std_day());
    let (ladder, defects) = load_ladder_text(&text);
    assert!(ladder.is_none());
    assert_eq!(defects, vec!["field windows has the wrong type"]);
}

#[test]
fn test_an_empty_window_list_is_refused() {
    let (ladder, defects) = load_ladder_text(&ladder_text("[]", &std_day()));
    assert!(ladder.is_none());
    assert_eq!(defects, vec!["field windows is empty"]);
}

#[test]
fn test_a_window_of_the_wrong_type_is_refused() {
    for value in ["true", "\"900\""] {
        let windows = format!("[{}]", value);
        let (ladder, defects) = load_ladder_text(&ladder_text(&windows, &std_day()));
        assert!(ladder.is_none());
        assert_eq!(defects, vec!["window 0 has the wrong type"]);
    }
}

#[test]
fn test_a_window_that_is_not_finite_is_refused() {
    let huge = format!("1{}", "0".repeat(400));
    let windows = format!("[{}]", huge);
    let (ladder, defects) = load_ladder_text(&ladder_text(&windows, &std_day()));
    assert!(ladder.is_none());
    assert_eq!(defects, vec!["window 0 is not a finite number"]);
}

#[test]
fn test_a_window_that_is_not_whole_is_refused() {
    let (ladder, defects) = load_ladder_text(&ladder_text("[600.5]", &std_day()));
    assert!(ladder.is_none());
    assert_eq!(defects, vec!["window 0 is not a whole number"]);
}

#[test]
fn test_a_window_at_or_below_zero_is_refused() {
    let (ladder, defects) = load_ladder_text(&ladder_text("[600, -5]", &std_day()));
    assert!(ladder.is_none());
    assert_eq!(defects, vec!["window 1 must be positive"]);
}

#[test]
fn test_the_positive_check_stands_before_the_whole_check() {
    let (ladder, defects) = load_ladder_text(&ladder_text("[-1.5]", &std_day()));
    assert!(ladder.is_none());
    assert_eq!(defects, vec!["window 0 must be positive"]);
}

#[test]
fn test_a_duplicate_window_is_refused_naming_the_earlier_window() {
    let (ladder, defects) = load_ladder_text(&ladder_text("[600, 900, 600]", &std_day()));
    assert!(ladder.is_none());
    assert_eq!(defects, vec!["window 2 duplicates window 0"]);
    let (ladder, defects) = load_ladder_text(&ladder_text("[900, 900.0]", &std_day()));
    assert!(ladder.is_none());
    assert_eq!(defects, vec!["window 1 duplicates window 0"]);
}

#[test]
fn test_one_input_draws_several_defects_in_one_pass() {
    let (ladder, defects) = load_ladder_text("{\"windows\": [0, \"x\"]}");
    assert!(ladder.is_none());
    assert_eq!(
        defects,
        vec![
            "missing field day",
            "window 0 must be positive",
            "window 1 has the wrong type",
        ]
    );
}

#[test]
fn test_an_unreadable_ladder_file_names_the_single_defect() {
    let (ladder, defects) = load_ladder_file(&temp_path("missing.json"));
    assert!(ladder.is_none());
    assert_eq!(defects, vec!["file cannot be read"]);
}

#[test]
fn test_a_ladder_file_loads_from_disk() {
    let path = temp_path("ladder.json");
    std::fs::write(&path, ladder_text("[600]", &std_day())).expect("the scratch file writes");
    let (ladder, defects) = load_ladder_file(&path);
    assert_eq!(defects, Vec::<String>::new());
    assert_eq!(
        ladder.expect("the ladder loads").windows,
        vec![BigInt::from(600)]
    );
    let _ = std::fs::remove_file(&path);
}

#[test]
fn test_the_report_reads_each_window_as_the_day_stood_at_it() {
    let lines = report_for(&std_runs(), "[1800, 300]", "operator", 1800);
    assert_eq!(
        lines,
        vec![
            "field day, window 1800 seconds, 2 runs, operator operator",
            "record at 300 seconds, operator at a projection of 900",
            "finish, record held by operator at a projection of 900",
            "record kept by operator to the close, margin 60 at the close",
            "field day, window 300 seconds, 2 runs, operator operator",
            "record at 300 seconds, operator at a projection of 150",
            "finish, record held by operator at a projection of 150",
            "record kept by operator to the close",
            "ladder, the closing holder never changed across the ladder",
        ]
    );
}

#[test]
fn test_a_window_holding_no_read_sets_no_record() {
    let lines = report_for(&std_runs(), "[100]", "operator", 1800);
    assert_eq!(
        lines,
        vec![
            "field day, window 100 seconds, 2 runs, operator operator",
            "finish, no record was set",
            "ladder, the closing holder never changed across the ladder",
        ]
    );
}

#[test]
fn test_two_windows_with_no_record_do_not_differ() {
    let lines = report_for(&std_runs(), "[100, 200]", "operator", 1800);
    assert_eq!(
        lines[lines.len() - 1],
        "ladder, the closing holder never changed across the ladder"
    );
}

#[test]
fn test_a_holder_after_no_record_differs_from_no_holder() {
    let lines = report_for(&std_runs(), "[100, 300]", "operator", 1800);
    assert_eq!(
        lines[lines.len() - 1],
        "ladder, the closing holder differs from the first window at 300 seconds"
    );
}

fn change_runs() -> String {
    "[{\"runner\": \"operator\", \"usage\": 900, \"reads\": [{\"elapsed_seconds\": 300, \"usage\": 210}]}, {\"runner\": \"fork\", \"usage\": 950, \"reads\": [{\"elapsed_seconds\": 600, \"usage\": 200}]}]".to_string()
}

#[test]
fn test_the_closing_holder_change_is_named_in_the_summary() {
    let lines = report_for(&change_runs(), "[300, 600]", "operator", 1800);
    assert_eq!(
        lines,
        vec![
            "field day, window 300 seconds, 2 runs, operator operator",
            "record at 300 seconds, operator at a projection of 210",
            "finish, record held by operator at a projection of 210",
            "record kept by operator to the close",
            "field day, window 600 seconds, 2 runs, operator operator",
            "record at 300 seconds, operator at a projection of 420",
            "record at 600 seconds, fork at a projection of 200",
            "finish, record held by fork at a projection of 200",
            "record stolen from operator at 600 seconds by fork, margin 220 at the close",
            "ladder, the closing holder differs from the first window at 600 seconds",
        ]
    );
}

#[test]
fn test_a_change_at_the_last_window_only_names_that_window() {
    let lines = report_for(&change_runs(), "[600, 300]", "operator", 1800);
    assert!(lines[0].starts_with("field day, window 600 seconds"));
    assert_eq!(
        lines[lines.len() - 1],
        "ladder, the closing holder differs from the first window at 300 seconds"
    );
}

#[test]
fn test_the_day_window_plays_no_part_in_the_reading() {
    let wide = report_for(&change_runs(), "[300, 600]", "operator", 1800);
    let narrow = report_for(&change_runs(), "[300, 600]", "operator", 42);
    assert_eq!(wide, narrow);
}

#[test]
fn test_an_equal_figure_never_takes_the_record_inside_a_window() {
    let runs = "[{\"runner\": \"operator\", \"usage\": 900, \"reads\": [{\"elapsed_seconds\": 300, \"usage\": 150}]}, {\"runner\": \"fork\", \"usage\": 950, \"reads\": [{\"elapsed_seconds\": 600, \"usage\": 300}]}]";
    let lines = report_for(runs, "[600]", "operator", 1800);
    assert_eq!(
        lines,
        vec![
            "field day, window 600 seconds, 2 runs, operator operator",
            "record at 300 seconds, operator at a projection of 300",
            "finish, record held by operator at a projection of 300",
            "record kept by operator to the close, margin 0 at the close",
            "ladder, the closing holder never changed across the ladder",
        ]
    );
}

#[test]
fn test_each_section_equals_the_field_day_report_of_the_day_as_it_stood() {
    // The day of the Python pin, as (label, usage, reads) triples.
    type ReadMarks = Vec<(i64, i64)>;
    let source: Vec<(&str, i64, ReadMarks)> = vec![
        ("operator", 900, vec![(300, 150), (900, 500)]),
        ("fork", 950, vec![(600, 320)]),
        ("quiet", 990, vec![]),
    ];
    let day_runs_json = format!(
        "[{}]",
        source
            .iter()
            .map(|(label, usage, reads)| {
                let mut run = format!("{{\"runner\": \"{}\", \"usage\": {}", label, usage);
                if !reads.is_empty() {
                    let parts: Vec<String> = reads
                        .iter()
                        .map(|(e, u)| format!("{{\"elapsed_seconds\": {}, \"usage\": {}}}", e, u))
                        .collect();
                    run += &format!(", \"reads\": [{}]", parts.join(", "));
                }
                run += "}";
                run
            })
            .collect::<Vec<String>>()
            .join(", ")
    );
    let day = day_json(1800, "operator", &day_runs_json);
    for window in [300, 600, 900, 1800] {
        let stood_runs_json = format!(
            "[{}]",
            source
                .iter()
                .map(|(label, usage, reads)| {
                    let kept: Vec<&(i64, i64)> =
                        reads.iter().filter(|(e, _)| *e <= window).collect();
                    let mut run = format!("{{\"runner\": \"{}\", \"usage\": {}", label, usage);
                    if !kept.is_empty() {
                        let parts: Vec<String> = kept
                            .iter()
                            .map(|(e, u)| {
                                format!("{{\"elapsed_seconds\": {}, \"usage\": {}}}", e, u)
                            })
                            .collect();
                        run += &format!(", \"reads\": [{}]", parts.join(", "));
                    }
                    run += "}";
                    run
                })
                .collect::<Vec<String>>()
                .join(", ")
        );
        let stood_text = day_json(window, "operator", &stood_runs_json);
        let (stood, defects) = load_field_day_text(&stood_text);
        assert_eq!(defects, Vec::<String>::new());
        let stood = stood.expect("the stood day loads");
        let windows = format!("[{}]", window);
        let ladder = loaded_ladder(&windows, &day);
        let lines = ladder_report(&ladder).expect("the report reads");
        let section = &lines[..lines.len() - 1];
        assert_eq!(
            section,
            field_day_report(&stood).expect("the report reads"),
            "at window {}",
            window
        );
    }
}

// The report on values the loader refuses, the discipline of FIX-PG-8

fn hand_day(operator: &str, runs: Vec<paceghost::fieldday::DayRun>) -> FieldDay {
    FieldDay {
        window_seconds: Num::from_i64(1800),
        operator: PText::from_text(operator),
        runs,
    }
}

#[test]
fn test_the_report_raises_a_value_error_on_an_empty_windows_list() {
    let (day, defects) = load_field_day_text(&std_day());
    assert_eq!(defects, Vec::<String>::new());
    let ladder = Ladder {
        day: day.expect("the day loads"),
        windows: vec![],
    };
    let error = ladder_report(&ladder).expect_err("the report names the precondition");
    assert_eq!(error, "the ladder holds no windows");
}

#[test]
fn test_the_report_raises_a_value_error_when_the_operator_names_no_run() {
    let (day, defects) = load_field_day_text(&day_json(1800, "ghost", &std_runs()));
    // The loader refuses this day, so the ladder is built by hand.
    assert!(day.is_none());
    assert!(!defects.is_empty());
    let (_, clean_defects) = load_field_day_text(&std_day());
    assert_eq!(clean_defects, Vec::<String>::new());
    let runs = vec![paceghost::fieldday::DayRun {
        runner: PText::from_text("operator"),
        usage: Some(Num::from_i64(900)),
        reads: None,
    }];
    let ladder = Ladder {
        day: hand_day("ghost", runs),
        windows: vec![BigInt::from(600)],
    };
    let error = ladder_report(&ladder).expect_err("the report names the precondition");
    assert_eq!(error, "the operator names no run among the runs");
}

#[test]
fn test_the_report_raises_a_value_error_when_the_day_holds_no_runs() {
    let ladder = Ladder {
        day: hand_day("operator", vec![]),
        windows: vec![BigInt::from(600)],
    };
    let error = ladder_report(&ladder).expect_err("the report names the precondition");
    assert_eq!(error, "the operator names no run among the runs");
}
