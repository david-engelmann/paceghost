//! The field day loader and the stolen record, held to PG-7 and PG-8.
//!
//! Translated from tests/test_fieldday.py. Loader pins name each
//! defect phrase of the field day shape, and the report pins walk
//! the record progression at its edges, the tie that never moves the
//! record, the tiebreak across marks when a holder is displaced, the
//! holder whose own projection rises past a rival, and every outcome
//! line the close can produce.

use std::path::PathBuf;

use num_bigint::BigInt;

use paceghost::fieldday::{
    field_day_report, load_field_day_file, load_field_day_text, DayRun, FieldDay,
};
use paceghost::pynum::Num;
use paceghost::pytext::PText;
use paceghost::runs::Read;

fn day_text(window: &str, operator: &str, runs: &str) -> String {
    format!(
        "{{\"window_seconds\": {}, \"operator\": {}, \"runs\": {}}}",
        window, operator, runs
    )
}

fn std_runs() -> String {
    "[{\"runner\": \"operator\", \"usage\": 900, \"reads\": [{\"elapsed_seconds\": 300, \"usage\": 150}]}, {\"runner\": \"fork\", \"usage\": 950, \"reads\": [{\"elapsed_seconds\": 600, \"usage\": 320}]}]".to_string()
}

fn std_day() -> String {
    day_text("1800", "\"operator\"", &std_runs())
}

fn timed(label: &str, reads: Option<&[(i64, i64)]>, usage: i64) -> String {
    let mut text = format!("{{\"runner\": \"{}\", \"usage\": {}", label, usage);
    if let Some(reads) = reads {
        let parts: Vec<String> = reads
            .iter()
            .map(|(elapsed, used)| {
                format!("{{\"elapsed_seconds\": {}, \"usage\": {}}}", elapsed, used)
            })
            .collect();
        text += &format!(", \"reads\": [{}]", parts.join(", "));
    }
    text += "}";
    text
}

fn load_day(runs_json: &str, operator: &str, window: i64) -> FieldDay {
    let text = day_text(&window.to_string(), &format!("\"{}\"", operator), runs_json);
    let (day, defects) = load_field_day_text(&text);
    assert_eq!(defects, Vec::<String>::new());
    day.expect("the day loads")
}

fn report_for(runs: &[String], operator: &str, window: i64) -> Vec<String> {
    let runs_json = format!("[{}]", runs.join(", "));
    let day = load_day(&runs_json, operator, window);
    field_day_report(&day).expect("the report reads")
}

fn report(runs: &[String]) -> Vec<String> {
    report_for(runs, "operator", 1800)
}

// The loader on clean input

#[test]
fn test_a_clean_field_day_loads_with_its_fields_in_file_order() {
    let (day, defects) = load_field_day_text(&std_day());
    assert_eq!(defects, Vec::<String>::new());
    let day = day.expect("the day loads");
    assert_eq!(day.window_seconds, Num::from_i64(1800));
    assert!(day.operator.eq_str("operator"));
    let labels: Vec<String> = day
        .runs
        .iter()
        .map(|run| run.runner.to_display_string())
        .collect();
    assert_eq!(labels, vec!["operator", "fork"]);
    assert_eq!(
        day.runs[0].reads,
        Some(vec![Read {
            elapsed_seconds: Num::from_i64(300),
            usage: Num::from_i64(150),
        }])
    );
}

#[test]
fn test_a_run_without_reads_loads_and_projects_no_figure() {
    let runs = vec![
        timed("operator", None, 10),
        timed("fork", Some(&[(300, 150)]), 900),
    ];
    let lines = report(&runs);
    assert_eq!(
        lines[lines.len() - 1],
        "record never held by operator, whose run projected no figure"
    );
}

#[test]
fn test_a_run_may_carry_usage_without_reads() {
    let text = day_text(
        "1800",
        "\"operator\"",
        "[{\"runner\": \"operator\", \"usage\": 42}]",
    );
    let (day, defects) = load_field_day_text(&text);
    assert_eq!(defects, Vec::<String>::new());
    let day = day.expect("the day loads");
    assert_eq!(
        day.runs[0],
        DayRun {
            runner: PText::from_text("operator"),
            usage: Some(Num::from_i64(42)),
            reads: None,
        }
    );
}

#[test]
fn test_unnamed_fields_are_ignored_in_the_day_and_in_a_run() {
    let text = "{\"window_seconds\": 1800, \"operator\": \"operator\", \"extra\": \"ignored\", \"runs\": [{\"runner\": \"operator\", \"usage\": 900, \"surprise\": true, \"reads\": [{\"elapsed_seconds\": 300, \"usage\": 150}]}, {\"runner\": \"fork\", \"usage\": 950, \"reads\": [{\"elapsed_seconds\": 600, \"usage\": 320}]}]}";
    let (day, defects) = load_field_day_text(text);
    assert_eq!(defects, Vec::<String>::new());
    assert!(day.is_some());
}

#[test]
fn test_load_field_day_file_names_an_unreadable_file() {
    let path: PathBuf = std::env::temp_dir().join(format!(
        "paceghost-fieldday-{}-missing.json",
        std::process::id()
    ));
    let (day, defects) = load_field_day_file(&path);
    assert!(day.is_none());
    assert_eq!(defects, vec!["file cannot be read"]);
}

// The loader defect vocabulary

#[test]
fn test_text_that_is_not_json_draws_the_named_defect() {
    let (day, defects) = load_field_day_text("not json at all");
    assert!(day.is_none());
    assert_eq!(defects, vec!["input is not valid JSON"]);
}

#[test]
fn test_deeply_nested_input_draws_the_not_valid_json_defect() {
    let text = format!("{}{}", "[".repeat(100000), "]".repeat(100000));
    let (day, defects) = load_field_day_text(&text);
    assert!(day.is_none());
    assert_eq!(defects, vec!["input is not valid JSON"]);
}

#[test]
fn test_json_that_is_not_an_object_draws_the_named_defect() {
    let (day, defects) = load_field_day_text("[1, 2, 3]");
    assert!(day.is_none());
    assert_eq!(defects, vec!["input is not a JSON object"]);
}

#[test]
fn test_missing_top_level_fields_are_each_named() {
    let (day, defects) = load_field_day_text("{}");
    assert!(day.is_none());
    assert_eq!(
        defects,
        vec![
            "missing field window_seconds",
            "missing field operator",
            "missing field runs",
        ]
    );
}

#[test]
fn test_a_window_of_the_wrong_type_draws_the_named_defect() {
    let (day, defects) = load_field_day_text(&day_text("\"soon\"", "\"operator\"", &std_runs()));
    assert!(day.is_none());
    assert_eq!(defects, vec!["field window_seconds has the wrong type"]);
}

#[test]
fn test_a_boolean_window_is_not_a_number() {
    let (day, defects) = load_field_day_text(&day_text("true", "\"operator\"", &std_runs()));
    assert!(day.is_none());
    assert_eq!(defects, vec!["field window_seconds has the wrong type"]);
}

#[test]
fn test_a_window_too_large_to_be_finite_draws_the_named_defect() {
    let huge = format!("1{}", "0".repeat(400));
    let (day, defects) = load_field_day_text(&day_text(&huge, "\"operator\"", &std_runs()));
    assert!(day.is_none());
    assert_eq!(defects, vec!["field window_seconds is not a finite number"]);
}

#[test]
fn test_a_window_of_zero_draws_the_named_defect() {
    let (day, defects) = load_field_day_text(&day_text("0", "\"operator\"", &std_runs()));
    assert!(day.is_none());
    assert_eq!(defects, vec!["field window_seconds must be positive"]);
}

#[test]
fn test_an_operator_of_the_wrong_type_draws_the_named_defect() {
    let (day, defects) = load_field_day_text(&day_text("1800", "7", &std_runs()));
    assert!(day.is_none());
    assert_eq!(defects, vec!["field operator has the wrong type"]);
}

#[test]
fn test_an_empty_operator_draws_the_named_defect() {
    let (day, defects) = load_field_day_text(&day_text("1800", "\"\"", &std_runs()));
    assert!(day.is_none());
    assert_eq!(defects, vec!["field operator is empty"]);
}

#[test]
fn test_an_operator_holding_an_unpaired_surrogate_is_named() {
    let (day, defects) = load_field_day_text(&day_text("1800", "\"opera\\ud800tor\"", &std_runs()));
    assert!(day.is_none());
    assert_eq!(defects, vec!["field operator holds an unpaired surrogate"]);
}

#[test]
fn test_an_operator_holding_a_line_break_is_named() {
    let (day, defects) = load_field_day_text(&day_text("1800", "\"opera\\ntor\"", &std_runs()));
    assert!(day.is_none());
    assert_eq!(defects, vec!["field operator holds a line break"]);
}

#[test]
fn test_an_operator_holding_a_carriage_return_is_named() {
    let (day, defects) = load_field_day_text(&day_text("1800", "\"opera\\rtor\"", &std_runs()));
    assert!(day.is_none());
    assert_eq!(defects, vec!["field operator holds a line break"]);
}

#[test]
fn test_a_runs_field_of_the_wrong_type_draws_the_named_defect() {
    let (day, defects) = load_field_day_text(&day_text(
        "1800",
        "\"operator\"",
        "{\"runner\": \"operator\"}",
    ));
    assert!(day.is_none());
    assert_eq!(defects, vec!["field runs has the wrong type"]);
}

#[test]
fn test_an_empty_runs_list_draws_the_named_defect() {
    let (day, defects) = load_field_day_text(&day_text("1800", "\"operator\"", "[]"));
    assert!(day.is_none());
    assert_eq!(defects, vec!["field runs is empty"]);
}

#[test]
fn test_a_run_that_is_not_an_object_is_named_with_its_index() {
    let (day, defects) = load_field_day_text(&day_text("1800", "\"operator\"", "[\"operator\"]"));
    assert!(day.is_none());
    assert_eq!(
        defects,
        vec![
            "run 0 is not an object",
            "field operator names no run in the field day",
        ]
    );
}

#[test]
fn test_a_run_missing_its_runner_is_named_with_its_index() {
    let (day, defects) = load_field_day_text(&day_text("1800", "\"operator\"", "[{\"usage\": 5}]"));
    assert!(day.is_none());
    assert_eq!(
        defects,
        vec![
            "run 0 missing field runner",
            "field operator names no run in the field day",
        ]
    );
}

#[test]
fn test_a_runner_of_the_wrong_type_is_named_with_its_index() {
    let (day, defects) =
        load_field_day_text(&day_text("1800", "\"operator\"", "[{\"runner\": 3}]"));
    assert!(day.is_none());
    assert_eq!(
        defects,
        vec![
            "run 0 field runner has the wrong type",
            "field operator names no run in the field day",
        ]
    );
}

#[test]
fn test_an_empty_runner_is_named_with_its_index() {
    let (day, defects) =
        load_field_day_text(&day_text("1800", "\"operator\"", "[{\"runner\": \"\"}]"));
    assert!(day.is_none());
    assert_eq!(
        defects,
        vec![
            "run 0 field runner is empty",
            "field operator names no run in the field day",
        ]
    );
}

#[test]
fn test_a_runner_holding_an_unpaired_surrogate_is_named() {
    let (day, defects) = load_field_day_text(&day_text(
        "1800",
        "\"operator\"",
        "[{\"runner\": \"fo\\ud800rk\"}]",
    ));
    assert!(day.is_none());
    assert_eq!(
        defects,
        vec![
            "run 0 field runner holds an unpaired surrogate",
            "field operator names no run in the field day",
        ]
    );
}

#[test]
fn test_a_runner_holding_a_line_break_is_named() {
    let (day, defects) = load_field_day_text(&day_text(
        "1800",
        "\"operator\"",
        "[{\"runner\": \"fork\\nrecord at 0 seconds, forged\"}]",
    ));
    assert!(day.is_none());
    assert_eq!(
        defects,
        vec![
            "run 0 field runner holds a line break",
            "field operator names no run in the field day",
        ]
    );
}

#[test]
fn test_a_reads_field_of_the_wrong_type_is_named_with_its_run_index() {
    let runs = "[{\"runner\": \"operator\", \"usage\": 5, \"reads\": \"soon\"}]";
    let (day, defects) = load_field_day_text(&day_text("1800", "\"operator\"", runs));
    assert!(day.is_none());
    assert_eq!(defects, vec!["run 0 field reads has the wrong type"]);
}

#[test]
fn test_an_empty_reads_list_is_named_with_its_run_index() {
    let runs = "[{\"runner\": \"operator\", \"usage\": 5, \"reads\": []}]";
    let (day, defects) = load_field_day_text(&day_text("1800", "\"operator\"", runs));
    assert!(day.is_none());
    assert_eq!(defects, vec!["run 0 field reads is empty"]);
}

#[test]
fn test_a_read_defect_carries_the_run_index_before_the_read_index() {
    let runs = "[{\"runner\": \"operator\", \"usage\": 5, \"reads\": [{\"elapsed_seconds\": -1, \"usage\": 0}]}]";
    let (day, defects) = load_field_day_text(&day_text("1800", "\"operator\"", runs));
    assert!(day.is_none());
    assert_eq!(
        defects,
        vec!["run 0 read 0 field elapsed_seconds is negative"]
    );
}

#[test]
fn test_a_read_usage_that_decreases_is_named_under_its_run() {
    let runs = "[{\"runner\": \"operator\", \"usage\": 5, \"reads\": [{\"elapsed_seconds\": 100, \"usage\": 50}, {\"elapsed_seconds\": 200, \"usage\": 40}]}]";
    let (day, defects) = load_field_day_text(&day_text("1800", "\"operator\"", runs));
    assert!(day.is_none());
    assert_eq!(
        defects,
        vec!["run 0 read 1 field usage is less than the previous read"]
    );
}

#[test]
fn test_reads_without_usage_draw_the_missing_usage_defect() {
    let runs = "[{\"runner\": \"operator\", \"reads\": [{\"elapsed_seconds\": 1, \"usage\": 1}]}]";
    let (day, defects) = load_field_day_text(&day_text("1800", "\"operator\"", runs));
    assert!(day.is_none());
    assert_eq!(defects, vec!["run 0 missing field usage"]);
}

#[test]
fn test_a_negative_usage_is_named_under_its_run() {
    let runs = "[{\"runner\": \"operator\", \"usage\": -5}]";
    let (day, defects) = load_field_day_text(&day_text("1800", "\"operator\"", runs));
    assert!(day.is_none());
    assert_eq!(defects, vec!["run 0 field usage is negative"]);
}

#[test]
fn test_a_repeated_label_is_named_with_both_run_indexes() {
    let runs = [
        timed("fork", Some(&[(100, 10)]), 900),
        timed("fork", Some(&[(200, 20)]), 900),
    ];
    let text = day_text("1800", "\"fork\"", &format!("[{}]", runs.join(", ")));
    let (day, defects) = load_field_day_text(&text);
    assert!(day.is_none());
    assert_eq!(
        defects,
        vec!["run 1 field runner duplicates the label of run 0"]
    );
}

#[test]
fn test_an_operator_naming_no_run_is_named() {
    let (day, defects) = load_field_day_text(&day_text("1800", "\"nobody\"", &std_runs()));
    assert!(day.is_none());
    assert_eq!(
        defects,
        vec!["field operator names no run in the field day"]
    );
}

#[test]
fn test_one_input_can_draw_several_defects_in_one_pass() {
    let text = day_text(
        "-1",
        "\"operator\"",
        "[{\"runner\": \"\"}, {\"runner\": \"x\"}, 5]",
    );
    let (day, defects) = load_field_day_text(&text);
    assert!(day.is_none());
    for phrase in [
        "field window_seconds must be positive",
        "run 0 field runner is empty",
        "run 2 is not an object",
        "field operator names no run in the field day",
    ] {
        assert!(
            defects.iter().any(|defect| defect == phrase),
            "missing {}",
            phrase
        );
    }
}

// The report walk

#[test]
fn test_the_header_names_the_window_the_count_and_the_operator() {
    let lines = report(&[timed("operator", Some(&[(300, 150)]), 900)]);
    assert_eq!(
        lines[0],
        "field day, window 1800 seconds, 1 runs, operator operator"
    );
}

#[test]
fn test_the_operator_keeps_the_record_and_the_margin_is_the_gap() {
    let runs = vec![
        timed("operator", Some(&[(300, 150)]), 900),
        timed("fork", Some(&[(300, 200)]), 900),
    ];
    assert_eq!(
        report(&runs),
        vec![
            "field day, window 1800 seconds, 2 runs, operator operator",
            "record at 300 seconds, operator at a projection of 900",
            "finish, record held by operator at a projection of 900",
            "record kept by operator to the close, margin 300 at the close",
        ]
    );
}

#[test]
fn test_the_operator_keeping_a_sole_record_states_no_margin() {
    let lines = report(&[timed("operator", Some(&[(300, 150)]), 900)]);
    assert_eq!(
        lines[lines.len() - 1],
        "record kept by operator to the close"
    );
}

#[test]
fn test_an_equal_projection_never_takes_the_record() {
    let runs = vec![
        timed("operator", Some(&[(300, 150)]), 900),
        timed("fork", Some(&[(600, 300)]), 900),
    ];
    assert_eq!(
        report(&runs),
        vec![
            "field day, window 1800 seconds, 2 runs, operator operator",
            "record at 300 seconds, operator at a projection of 900",
            "finish, record held by operator at a projection of 900",
            "record kept by operator to the close, margin 0 at the close",
        ]
    );
}

#[test]
fn test_a_tie_at_the_first_mark_breaks_toward_the_earlier_run() {
    let runs = vec![
        timed("fork", Some(&[(300, 150)]), 900),
        timed("operator", Some(&[(300, 150)]), 900),
    ];
    let lines = report(&runs);
    assert_eq!(
        lines[1],
        "record at 300 seconds, fork at a projection of 900"
    );
    assert_eq!(
        lines[lines.len() - 1],
        "record never held by operator, margin 0 at the close"
    );
}

#[test]
fn test_a_displaced_record_goes_to_the_figure_set_at_the_earliest_mark() {
    let runs = vec![
        timed("operator", Some(&[(300, 150), (600, 340)]), 900),
        timed("r2", Some(&[(500, 250)]), 900),
        timed("r1", Some(&[(400, 200)]), 900),
    ];
    assert_eq!(
        report(&runs),
        vec![
            "field day, window 1800 seconds, 3 runs, operator operator",
            "record at 300 seconds, operator at a projection of 900",
            "record at 600 seconds, r1 at a projection of 900",
            "finish, record held by r1 at a projection of 900",
            "record stolen from operator at 600 seconds by r1, margin 120 at the close",
        ]
    );
}

#[test]
fn test_the_record_is_stolen_and_the_margin_is_read_at_the_close() {
    let runs = vec![
        timed("operator", Some(&[(300, 150), (900, 500)]), 900),
        timed("fork", Some(&[(600, 250)]), 900),
    ];
    assert_eq!(
        report(&runs),
        vec![
            "field day, window 1800 seconds, 2 runs, operator operator",
            "record at 300 seconds, operator at a projection of 900",
            "record at 600 seconds, fork at a projection of 750",
            "finish, record held by fork at a projection of 750",
            "record stolen from operator at 600 seconds by fork, margin 250 at the close",
        ]
    );
}

#[test]
fn test_a_regained_record_that_is_kept_reports_the_kept_line() {
    let runs = vec![
        timed("operator", Some(&[(300, 150), (700, 300)]), 900),
        timed("fork", Some(&[(500, 220), (900, 430)]), 900),
    ];
    assert_eq!(
        report(&runs),
        vec![
            "field day, window 1800 seconds, 2 runs, operator operator",
            "record at 300 seconds, operator at a projection of 900",
            "record at 500 seconds, fork at a projection of 792",
            "record at 700 seconds, operator at a projection of 771.428571",
            "finish, record held by operator at a projection of 771.428571",
            "record kept by operator to the close, margin 88.571429 at the close",
        ]
    );
}

#[test]
fn test_a_steal_after_a_regain_names_the_last_away_change() {
    let runs = vec![
        timed(
            "operator",
            Some(&[(300, 150), (700, 270), (1200, 700)]),
            900,
        ),
        timed("fork", Some(&[(500, 200), (900, 340)]), 900),
    ];
    let lines = report(&runs);
    assert_eq!(
        lines[lines.len() - 2],
        "finish, record held by fork at a projection of 680"
    );
    assert_eq!(
        lines[lines.len() - 1],
        "record stolen from operator at 900 seconds by fork, margin 370 at the close"
    );
}

#[test]
fn test_the_holder_rising_past_a_rival_hands_the_record_over() {
    let runs = vec![
        timed("operator", Some(&[(300, 150), (600, 340)]), 900),
        timed("fork", Some(&[(400, 210)]), 900),
    ];
    assert_eq!(
        report(&runs),
        vec![
            "field day, window 1800 seconds, 2 runs, operator operator",
            "record at 300 seconds, operator at a projection of 900",
            "record at 600 seconds, fork at a projection of 945",
            "finish, record held by fork at a projection of 945",
            "record stolen from operator at 600 seconds by fork, margin 75 at the close",
        ]
    );
}

#[test]
fn test_the_holder_improving_its_own_figure_moves_the_record_line() {
    let runs = vec![
        timed("operator", Some(&[(300, 150)]), 900),
        timed("fork", Some(&[(400, 200), (800, 300)]), 900),
    ];
    let lines = report(&runs);
    assert_eq!(
        lines[1],
        "record at 300 seconds, operator at a projection of 900"
    );
    assert_eq!(
        lines[2],
        "record at 800 seconds, fork at a projection of 675"
    );
    assert_eq!(
        lines[lines.len() - 1],
        "record stolen from operator at 800 seconds by fork, margin 225 at the close"
    );
}

#[test]
fn test_a_zero_elapsed_read_projects_no_figure() {
    let runs = vec![
        timed("operator", Some(&[(0, 0), (300, 150)]), 900),
        timed("fork", Some(&[(0, 500)]), 900),
    ];
    let lines = report(&runs);
    assert_eq!(
        lines[1],
        "record at 300 seconds, operator at a projection of 900"
    );
    assert_eq!(
        lines[lines.len() - 1],
        "record kept by operator to the close"
    );
}

#[test]
fn test_no_record_stands_when_no_read_projects_a_figure() {
    let runs = vec![
        timed("operator", Some(&[(0, 0)]), 900),
        timed("fork", None, 10),
    ];
    assert_eq!(
        report(&runs),
        vec![
            "field day, window 1800 seconds, 2 runs, operator operator",
            "finish, no record was set",
        ]
    );
}

#[test]
fn test_a_run_with_no_reads_never_holds_the_record_on_its_usage() {
    let runs = vec![
        timed("operator", Some(&[(600, 400)]), 900),
        timed("fork", None, 1),
    ];
    let lines = report(&runs);
    assert_eq!(
        lines[lines.len() - 2],
        "finish, record held by operator at a projection of 1200"
    );
    assert_eq!(
        lines[lines.len() - 1],
        "record kept by operator to the close"
    );
}

#[test]
fn test_the_operator_who_never_held_reads_the_margin_at_the_close() {
    let runs = vec![
        timed("operator", Some(&[(600, 400)]), 900),
        timed("fork", Some(&[(300, 100)]), 900),
    ];
    assert_eq!(
        report(&runs),
        vec![
            "field day, window 1800 seconds, 2 runs, operator operator",
            "record at 300 seconds, fork at a projection of 600",
            "finish, record held by fork at a projection of 600",
            "record never held by operator, margin 600 at the close",
        ]
    );
}

#[test]
fn test_the_closing_margin_can_be_zero_when_the_figures_tie() {
    let runs = vec![
        timed("operator", Some(&[(300, 200), (700, 350)]), 900),
        timed("fork", Some(&[(500, 250)]), 900),
    ];
    let lines = report(&runs);
    assert_eq!(
        lines[lines.len() - 2],
        "finish, record held by fork at a projection of 900"
    );
    assert_eq!(
        lines[lines.len() - 1],
        "record stolen from operator at 500 seconds by fork, margin 0 at the close"
    );
}

#[test]
fn test_a_projection_renders_at_full_precision_below_the_rounding_bound() {
    let runs = vec![
        timed("operator", Some(&[(700, 430)]), 900),
        timed("fork", Some(&[(300, 200)]), 900),
    ];
    let lines = report(&runs);
    assert!(lines
        .iter()
        .any(|line| line == "record at 700 seconds, operator at a projection of 1105.714286"));
}

// The report on a value the loader refuses, the discipline of FIX-PG-8

#[test]
fn test_the_report_raises_a_value_error_when_the_operator_names_no_run() {
    let day = FieldDay {
        window_seconds: Num::from_i64(1800),
        operator: PText::from_text("ghost"),
        runs: vec![
            DayRun {
                runner: PText::from_text("operator"),
                usage: Some(Num::from_i64(900)),
                reads: Some(vec![Read {
                    elapsed_seconds: Num::from_i64(300),
                    usage: Num::from_i64(150),
                }]),
            },
            DayRun {
                runner: PText::from_text("fork"),
                usage: Some(Num::from_i64(900)),
                reads: Some(vec![Read {
                    elapsed_seconds: Num::from_i64(600),
                    usage: Num::from_i64(320),
                }]),
            },
        ],
    };
    let error = field_day_report(&day).expect_err("the report names the precondition");
    assert_eq!(error, "the operator names no run among the runs");
}

// The FIX-PG-10 pins, added with the Round 5 repair. The record
// walk stands under the rematch reading as well as this one, so
// the shared guard is pinned on this surface too. On the
// unrepaired code the non finite cases panic inside the exact
// conversion and the oversized case renders an infinity in the
// opening line, so each pin fails there for a real reason.

fn hand_day_with(window: Num, reads: Vec<Read>) -> FieldDay {
    FieldDay {
        window_seconds: window,
        operator: PText::from_text("operator"),
        runs: vec![DayRun {
            runner: PText::from_text("operator"),
            usage: None,
            reads: Some(reads),
        }],
    }
}

fn ordinary_read() -> Read {
    Read {
        elapsed_seconds: Num::from_i64(300),
        usage: Num::from_i64(150),
    }
}

#[test]
fn test_a_non_finite_window_names_the_precondition() {
    let day = hand_day_with(Num::from_f64(f64::NAN), vec![ordinary_read()]);
    let error = field_day_report(&day).expect_err("the report names the precondition");
    assert_eq!(error, "the window is not a finite number");
}

#[test]
fn test_a_non_finite_read_figure_names_the_precondition() {
    let day = hand_day_with(
        Num::from_i64(1800),
        vec![Read {
            elapsed_seconds: Num::from_f64(f64::NEG_INFINITY),
            usage: Num::from_i64(150),
        }],
    );
    let error = field_day_report(&day).expect_err("the report names the precondition");
    assert_eq!(error, "a read elapsed value is not a finite number");
    let day = hand_day_with(
        Num::from_i64(1800),
        vec![Read {
            elapsed_seconds: Num::from_i64(300),
            usage: Num::from_f64(f64::INFINITY),
        }],
    );
    let error = field_day_report(&day).expect_err("the report names the precondition");
    assert_eq!(error, "a read usage value is not a finite number");
}

#[test]
fn test_a_non_finite_run_usage_names_the_precondition() {
    let day = FieldDay {
        window_seconds: Num::from_i64(1800),
        operator: PText::from_text("operator"),
        runs: vec![DayRun {
            runner: PText::from_text("operator"),
            usage: Some(Num::from_f64(f64::INFINITY)),
            reads: Some(vec![ordinary_read()]),
        }],
    };
    let error = field_day_report(&day).expect_err("the report names the precondition");
    assert_eq!(error, "a run usage value is not a finite number");
}

#[test]
fn test_an_oversized_window_names_the_precondition_instead_of_rendering_inf() {
    let mut value = BigInt::from(1);
    for _ in 0..400 {
        value *= 10;
    }
    let day = hand_day_with(Num::Int(value), vec![ordinary_read()]);
    let error = field_day_report(&day).expect_err("the report names the precondition");
    assert_eq!(error, "the window is not a finite number");
}
