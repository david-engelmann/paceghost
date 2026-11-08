//! Pins for PG-1 to PG-4 in the full form section of CONTRACT.md.
//!
//! Translated from tests/test_runs.py. Each malformed shape the
//! contract names gets a test that fails if the loader stops naming
//! that shape, and each rule gets a test whose numbers would change
//! under the opposite rule.

use std::path::PathBuf;

use paceghost::pynum::{py_float_repr, Num};
use paceghost::pytext::PText;
use paceghost::runs::{
    ghost_change, load_run_file, load_run_text, race_report, select_ghost, Run, Split,
};

fn n(value: i64) -> Num {
    Num::from_i64(value)
}

fn f(value: f64) -> Num {
    Num::from_f64(value)
}

fn run_text(
    task: &str,
    window: &str,
    runner: &str,
    recorded_at: &str,
    splits: &str,
    extra: &str,
) -> String {
    format!(
        "{{\"task\": {}, \"window_seconds\": {}, \"runner\": {}, \"recorded_at\": {}, \"splits\": {}{}}}",
        task, window, runner, recorded_at, splits, extra
    )
}

const TASK: &str = "\"draft the weekly summary\"";
const OPERATOR: &str = "\"operator\"";
const DATE: &str = "\"2026-01-04\"";
const SPLITS: &str =
    "[{\"phase\": \"plan\", \"elapsed_seconds\": 300}, {\"phase\": \"draft\", \"elapsed_seconds\": 900}]";

fn valid_text() -> String {
    run_text(TASK, "3600", OPERATOR, DATE, SPLITS, "")
}

fn load(text: &str) -> (Option<Run>, Vec<String>) {
    load_run_text(text)
}

fn has(defects: &[String], phrase: &str) -> bool {
    defects.iter().any(|defect| defect == phrase)
}

fn make_run(runner: &str, recorded_at: &str, splits: &[(&str, Num)]) -> Run {
    make_run_window(runner, recorded_at, splits, n(3600))
}

fn make_run_window(runner: &str, recorded_at: &str, splits: &[(&str, Num)], window: Num) -> Run {
    let splits: Vec<Split> = splits
        .iter()
        .map(|(phase, elapsed)| Split {
            phase: PText::from_text(phase),
            elapsed_seconds: elapsed.clone(),
        })
        .collect();
    let total_seconds = splits[splits.len() - 1].elapsed_seconds.clone();
    Run {
        task: PText::from_text("draft the weekly summary"),
        window_seconds: window,
        runner: PText::from_text(runner),
        recorded_at: PText::from_text(recorded_at),
        splits,
        total_seconds,
        usage: None,
        budget: None,
        reads: None,
        model: None,
    }
}

fn temp_path(name: &str) -> PathBuf {
    std::env::temp_dir().join(format!("paceghost-runs-{}-{}", std::process::id(), name))
}

// PG-1, clean inputs

#[test]
fn test_valid_run_loads_with_total_from_final_split() {
    let (run, defects) = load(&valid_text());
    assert_eq!(defects, Vec::<String>::new());
    let run = run.expect("a clean run loads");
    assert!(run.task.eq_str("draft the weekly summary"));
    assert!(run.runner.eq_str("operator"));
    assert_eq!(run.window_seconds, n(3600));
    assert!(run.recorded_at.eq_str("2026-01-04"));
    assert_eq!(run.total_seconds, n(900));
    // The key set of the Python run dict, which is exactly these
    // fields when no extension field stands in the file.
    assert!(run.usage.is_none());
    assert!(run.budget.is_none());
    assert!(run.reads.is_none());
    assert!(run.model.is_none());
}

#[test]
fn test_valid_run_with_matching_carried_total_loads() {
    let (run, defects) = load(&valid_text_with_total("900"));
    assert_eq!(defects, Vec::<String>::new());
    assert_eq!(run.expect("the run loads").total_seconds, n(900));
}

fn valid_text_with_total(total: &str) -> String {
    run_text(
        TASK,
        "3600",
        OPERATOR,
        DATE,
        SPLITS,
        &format!(", \"total_seconds\": {}", total),
    )
}

#[test]
fn test_extra_fields_are_ignored() {
    let text = run_text(
        TASK,
        "3600",
        OPERATOR,
        DATE,
        SPLITS,
        ", \"notes\": \"written after the run\"",
    );
    let (run, defects) = load(&text);
    assert_eq!(defects, Vec::<String>::new());
    assert!(run.is_some());
}

#[test]
fn test_first_split_at_zero_is_valid() {
    let splits = "[{\"phase\": \"plan\", \"elapsed_seconds\": 0}, {\"phase\": \"draft\", \"elapsed_seconds\": 900}]";
    let (run, defects) = load(&run_text(TASK, "3600", OPERATOR, DATE, splits, ""));
    assert_eq!(defects, Vec::<String>::new());
    assert_eq!(run.expect("the run loads").splits[0].elapsed_seconds, n(0));
}

// PG-1, malformed inputs the loader must name

#[test]
fn test_non_json_input_is_named() {
    let (run, defects) = load("this is not json");
    assert!(run.is_none());
    assert_eq!(defects, vec!["input is not valid JSON"]);
}

#[test]
fn test_non_object_json_is_named() {
    for text in ["[1, 2]", "\"just a string\"", "42"] {
        let (run, defects) = load(text);
        assert!(run.is_none());
        assert_eq!(defects, vec!["input is not a JSON object"], "for {}", text);
    }
}

#[test]
fn test_each_missing_top_level_field_is_named() {
    let cases: Vec<String> = vec![
        // no task
        "{\"window_seconds\": 3600, \"runner\": \"operator\", \"recorded_at\": \"2026-01-04\", \"splits\": []}".to_string(),
        // no runner
        "{\"task\": \"t\", \"window_seconds\": 3600, \"recorded_at\": \"2026-01-04\", \"splits\": []}".to_string(),
        // no window_seconds
        "{\"task\": \"t\", \"runner\": \"operator\", \"recorded_at\": \"2026-01-04\", \"splits\": []}".to_string(),
        // no recorded_at
        "{\"task\": \"t\", \"window_seconds\": 3600, \"runner\": \"operator\", \"splits\": []}".to_string(),
        // no splits
        "{\"task\": \"t\", \"window_seconds\": 3600, \"runner\": \"operator\", \"recorded_at\": \"2026-01-04\"}".to_string(),
    ];
    let fields = ["task", "runner", "window_seconds", "recorded_at", "splits"];
    for (text, field) in cases.iter().zip(fields.iter()) {
        let (run, defects) = load(text);
        assert!(run.is_none());
        assert!(
            has(&defects, &format!("missing field {}", field)),
            "for field {} in {:?}",
            field,
            defects
        );
    }
}

#[test]
fn test_task_wrong_type_and_empty_are_named() {
    let (run, defects) = load(&run_text("5", "3600", OPERATOR, DATE, SPLITS, ""));
    assert!(run.is_none());
    assert!(has(&defects, "field task has the wrong type"));
    let (run, defects) = load(&run_text("\"\"", "3600", OPERATOR, DATE, SPLITS, ""));
    assert!(run.is_none());
    assert!(has(&defects, "field task is empty"));
}

#[test]
fn test_runner_wrong_type_and_empty_are_named() {
    let (run, defects) = load(&run_text(TASK, "3600", "[\"operator\"]", DATE, SPLITS, ""));
    assert!(run.is_none());
    assert!(has(&defects, "field runner has the wrong type"));
    let (run, defects) = load(&run_text(TASK, "3600", "\"\"", DATE, SPLITS, ""));
    assert!(run.is_none());
    assert!(has(&defects, "field runner is empty"));
}

#[test]
fn test_window_wrong_type_is_named() {
    let (run, defects) = load(&run_text(TASK, "\"3600\"", OPERATOR, DATE, SPLITS, ""));
    assert!(run.is_none());
    assert!(has(&defects, "field window_seconds has the wrong type"));
}

#[test]
fn test_window_boolean_is_not_a_number() {
    let (run, defects) = load(&run_text(TASK, "true", OPERATOR, DATE, SPLITS, ""));
    assert!(run.is_none());
    assert!(has(&defects, "field window_seconds has the wrong type"));
}

#[test]
fn test_window_must_be_positive() {
    for value in ["0", "-5"] {
        let (run, defects) = load(&run_text(TASK, value, OPERATOR, DATE, SPLITS, ""));
        assert!(run.is_none());
        assert!(has(&defects, "field window_seconds must be positive"));
    }
}

#[test]
fn test_window_nan_is_not_a_finite_number() {
    let text = "{\"task\": \"t\", \"window_seconds\": NaN, \"runner\": \"r\", \"recorded_at\": \"2026-01-04\", \"splits\": [{\"phase\": \"plan\", \"elapsed_seconds\": 3}]}";
    let (run, defects) = load(text);
    assert!(run.is_none());
    assert!(has(&defects, "field window_seconds is not a finite number"));
}

#[test]
fn test_recorded_at_wrong_type_is_named() {
    let (run, defects) = load(&run_text(TASK, "3600", OPERATOR, "20260104", SPLITS, ""));
    assert!(run.is_none());
    assert!(has(&defects, "field recorded_at has the wrong type"));
}

#[test]
fn test_recorded_at_must_be_an_iso_date() {
    for value in ["last Tuesday", "2026-13-40", "04/01/2026"] {
        let quoted = format!("\"{}\"", value);
        let (run, defects) = load(&run_text(TASK, "3600", OPERATOR, &quoted, SPLITS, ""));
        assert!(run.is_none());
        assert!(has(&defects, "field recorded_at is not an ISO date"));
    }
}

#[test]
fn test_splits_wrong_type_and_empty_are_named() {
    let (run, defects) = load(&run_text(
        TASK,
        "3600",
        OPERATOR,
        DATE,
        "{\"phase\": \"plan\"}",
        "",
    ));
    assert!(run.is_none());
    assert!(has(&defects, "field splits has the wrong type"));
    let (run, defects) = load(&run_text(TASK, "3600", OPERATOR, DATE, "[]", ""));
    assert!(run.is_none());
    assert!(has(&defects, "field splits is empty"));
}

#[test]
fn test_split_that_is_not_an_object_is_named() {
    let (run, defects) = load(&run_text(TASK, "3600", OPERATOR, DATE, "[42]", ""));
    assert!(run.is_none());
    assert!(has(&defects, "split 0 is not an object"));
}

#[test]
fn test_split_missing_fields_are_named() {
    let (run, defects) = load(&run_text(
        TASK,
        "3600",
        OPERATOR,
        DATE,
        "[{\"elapsed_seconds\": 300}]",
        "",
    ));
    assert!(run.is_none());
    assert!(has(&defects, "split 0 is missing field phase"));
    let (run, defects) = load(&run_text(
        TASK,
        "3600",
        OPERATOR,
        DATE,
        "[{\"phase\": \"plan\"}]",
        "",
    ));
    assert!(run.is_none());
    assert!(has(&defects, "split 0 is missing field elapsed_seconds"));
}

#[test]
fn test_split_phase_wrong_type_and_empty_are_named() {
    let (run, defects) = load(&run_text(
        TASK,
        "3600",
        OPERATOR,
        DATE,
        "[{\"phase\": 7, \"elapsed_seconds\": 300}]",
        "",
    ));
    assert!(run.is_none());
    assert!(has(&defects, "split 0 field phase has the wrong type"));
    let (run, defects) = load(&run_text(
        TASK,
        "3600",
        OPERATOR,
        DATE,
        "[{\"phase\": \"\", \"elapsed_seconds\": 300}]",
        "",
    ));
    assert!(run.is_none());
    assert!(has(&defects, "split 0 field phase is empty"));
}

#[test]
fn test_split_elapsed_wrong_type_and_boolean_are_named() {
    for value in ["\"300\"", "true"] {
        let splits = format!("[{{\"phase\": \"plan\", \"elapsed_seconds\": {}}}]", value);
        let (run, defects) = load(&run_text(TASK, "3600", OPERATOR, DATE, &splits, ""));
        assert!(run.is_none());
        assert!(has(
            &defects,
            "split 0 field elapsed_seconds has the wrong type"
        ));
    }
}

#[test]
fn test_split_elapsed_negative_is_named() {
    let (run, defects) = load(&run_text(
        TASK,
        "3600",
        OPERATOR,
        DATE,
        "[{\"phase\": \"plan\", \"elapsed_seconds\": -1}]",
        "",
    ));
    assert!(run.is_none());
    assert!(has(&defects, "split 0 field elapsed_seconds is negative"));
}

#[test]
fn test_split_elapsed_must_increase() {
    for second in [300, 200] {
        let splits = format!(
            "[{{\"phase\": \"plan\", \"elapsed_seconds\": 300}}, {{\"phase\": \"draft\", \"elapsed_seconds\": {}}}]",
            second
        );
        let (run, defects) = load(&run_text(TASK, "3600", OPERATOR, DATE, &splits, ""));
        assert!(run.is_none());
        assert!(has(
            &defects,
            "split 1 field elapsed_seconds is not greater than the previous split"
        ));
    }
}

#[test]
fn test_carried_total_mismatch_is_named() {
    let (run, defects) = load(&valid_text_with_total("901"));
    assert!(run.is_none());
    assert!(has(
        &defects,
        "field total_seconds does not equal the final split"
    ));
}

#[test]
fn test_carried_total_wrong_type_is_named() {
    let (run, defects) = load(&valid_text_with_total("\"900\""));
    assert!(run.is_none());
    assert!(has(&defects, "field total_seconds has the wrong type"));
}

#[test]
fn test_several_defects_are_named_in_one_pass() {
    let (run, defects) = load("{\"task\": 5, \"splits\": []}");
    assert!(run.is_none());
    for phrase in [
        "field task has the wrong type",
        "missing field runner",
        "missing field window_seconds",
        "missing field recorded_at",
        "field splits is empty",
    ] {
        assert!(has(&defects, phrase), "missing {}", phrase);
    }
    assert!(defects.len() >= 5);
}

#[test]
fn test_loader_never_raises_on_garbage() {
    for text in [
        "",
        "{",
        "null",
        "{\"splits\": \"x\"}",
        "{\"a\": 1} trailing",
    ] {
        let (run, defects) = load(text);
        assert!(run.is_none(), "for {:?}", text);
        assert!(!defects.is_empty(), "for {:?}", text);
    }
}

#[test]
fn test_load_run_file_reports_an_unreadable_file() {
    let path = temp_path("no such run.json");
    let (run, defects) = load_run_file(&path);
    assert!(run.is_none());
    assert_eq!(defects, vec!["file cannot be read"]);
}

#[test]
fn test_load_run_file_round_trip() {
    let path = temp_path("run.json");
    std::fs::write(&path, valid_text()).expect("the scratch file writes");
    let (run, defects) = load_run_file(&path);
    assert_eq!(defects, Vec::<String>::new());
    assert_eq!(run.expect("the run loads").total_seconds, n(900));
    let _ = std::fs::remove_file(&path);
}

// PG-2, the ghost rule

#[test]
fn test_ghost_is_the_lowest_total() {
    let runs = vec![
        make_run(
            "operator",
            "2026-01-04",
            &[("plan", n(300)), ("draft", n(900))],
        ),
        make_run(
            "night fork",
            "2026-01-05",
            &[("plan", n(200)), ("draft", n(700))],
        ),
        make_run(
            "weekend fork",
            "2026-01-06",
            &[("plan", n(250)), ("draft", n(800))],
        ),
    ];
    let ghost = select_ghost(&runs).expect("runs stand");
    assert!(std::ptr::eq(ghost, &runs[1]));
}

#[test]
fn test_ghost_tie_breaks_toward_the_earliest_recorded_run() {
    let runs = vec![
        make_run(
            "operator",
            "2026-01-06",
            &[("plan", n(200)), ("draft", n(700))],
        ),
        make_run(
            "night fork",
            "2026-01-03",
            &[("plan", n(240)), ("draft", n(700))],
        ),
    ];
    let ghost = select_ghost(&runs).expect("runs stand");
    assert!(std::ptr::eq(ghost, &runs[1]));
}

#[test]
fn test_ghost_full_tie_keeps_the_input_order() {
    let runs = vec![
        make_run(
            "operator",
            "2026-01-04",
            &[("plan", n(200)), ("draft", n(700))],
        ),
        make_run(
            "night fork",
            "2026-01-04",
            &[("plan", n(260)), ("draft", n(700))],
        ),
    ];
    let ghost = select_ghost(&runs).expect("runs stand");
    assert!(std::ptr::eq(ghost, &runs[0]));
}

#[test]
fn test_select_ghost_with_no_runs_returns_none() {
    assert!(select_ghost(&[]).is_none());
}

// PG-3, the race report

#[test]
fn test_race_reports_ahead_behind_and_the_finish_with_exact_seconds() {
    let ghost = make_run(
        "operator",
        "2026-01-04",
        &[("plan", n(300)), ("draft", n(900)), ("polish", n(1500))],
    );
    let live = make_run(
        "operator",
        "2026-01-05",
        &[("plan", n(240)), ("draft", n(960)), ("polish", n(1440))],
    );
    assert_eq!(
        race_report(&ghost, &live),
        vec![
            "plan ahead by 60 seconds",
            "draft behind by 60 seconds",
            "polish ahead by 60 seconds",
            "window, the live run finished inside its window with 2160 seconds to spare",
            "finish ahead by 60 seconds",
        ]
    );
}

#[test]
fn test_race_reports_level_splits_and_a_level_finish() {
    let splits: &[(&str, Num)] = &[("plan", n(300)), ("draft", n(900))];
    let ghost = make_run("operator", "2026-01-04", splits);
    let live = make_run("night fork", "2026-01-05", splits);
    assert_eq!(
        race_report(&ghost, &live),
        vec![
            "plan level at 300 seconds",
            "draft level at 900 seconds",
            "window, the live run finished inside its window with 2700 seconds to spare",
            "finish level at 900 seconds",
        ]
    );
}

#[test]
fn test_race_reports_a_finish_behind() {
    let ghost = make_run(
        "operator",
        "2026-01-04",
        &[("plan", n(300)), ("draft", n(900))],
    );
    let live = make_run(
        "operator",
        "2026-01-05",
        &[("plan", n(310)), ("draft", n(990))],
    );
    let lines = race_report(&ghost, &live);
    assert_eq!(lines[lines.len() - 1], "finish behind by 90 seconds");
}

#[test]
fn test_race_names_a_phase_mismatch_without_a_difference() {
    let ghost = make_run(
        "operator",
        "2026-01-04",
        &[("plan", n(300)), ("draft", n(900))],
    );
    let live = make_run(
        "operator",
        "2026-01-05",
        &[("plan", n(240)), ("write", n(800))],
    );
    let lines = race_report(&ghost, &live);
    assert_eq!(lines[0], "plan ahead by 60 seconds");
    assert_eq!(
        lines[1],
        "split 1 phase mismatch, ghost draft and live write"
    );
    assert_eq!(lines[lines.len() - 1], "finish ahead by 100 seconds");
}

#[test]
fn test_race_reports_different_split_counts_before_the_finish() {
    let ghost = make_run(
        "operator",
        "2026-01-04",
        &[("plan", n(300)), ("draft", n(900)), ("polish", n(1500))],
    );
    let live = make_run(
        "operator",
        "2026-01-05",
        &[
            ("plan", n(240)),
            ("draft", n(800)),
            ("polish", n(1300)),
            ("send", n(1400)),
        ],
    );
    let lines = race_report(&ghost, &live);
    assert_eq!(
        lines[lines.len() - 3],
        "split counts differ, ghost 3 and live 4"
    );
    assert_eq!(
        lines[lines.len() - 2],
        "window, the live run finished inside its window with 2200 seconds to spare"
    );
    assert_eq!(lines[lines.len() - 1], "finish ahead by 100 seconds");
}

#[test]
fn test_race_renders_fractional_seconds_exactly() {
    let ghost = make_run("operator", "2026-01-04", &[("plan", f(100.5))]);
    let live = make_run("operator", "2026-01-05", &[("plan", n(88))]);
    assert_eq!(
        race_report(&ghost, &live),
        vec![
            "plan ahead by 12.5 seconds",
            "window, the live run finished inside its window with 3512 seconds to spare",
            "finish ahead by 12.5 seconds",
        ]
    );
}

// PG-4, the ghost changing hands

#[test]
fn test_a_faster_candidate_steals_the_ghost_with_both_totals() {
    let old = make_run(
        "operator",
        "2026-01-04",
        &[("plan", n(300)), ("polish", n(1500))],
    );
    let candidate = make_run(
        "night fork",
        "2026-01-05",
        &[("plan", n(240)), ("polish", n(1320))],
    );
    let (steals, line) = ghost_change(&old, &candidate).expect("a clean pair reports");
    assert!(steals);
    assert_eq!(
        line,
        "ghost changes from operator at 1500 seconds to night fork at 1320 seconds"
    );
}

#[test]
fn test_a_slower_candidate_leaves_the_ghost_with_both_totals() {
    let old = make_run(
        "operator",
        "2026-01-04",
        &[("plan", n(300)), ("polish", n(1500))],
    );
    let candidate = make_run(
        "night fork",
        "2026-01-05",
        &[("plan", n(400)), ("polish", n(1800))],
    );
    let (steals, line) = ghost_change(&old, &candidate).expect("a clean pair reports");
    assert!(!steals);
    assert_eq!(
        line,
        "ghost stays with operator at 1500 seconds against night fork at 1800 seconds"
    );
}

#[test]
fn test_an_equal_total_with_an_earlier_date_steals() {
    let old = make_run(
        "operator",
        "2026-01-06",
        &[("plan", n(300)), ("polish", n(1500))],
    );
    let candidate = make_run(
        "night fork",
        "2026-01-02",
        &[("plan", n(200)), ("polish", n(1500))],
    );
    let (steals, line) = ghost_change(&old, &candidate).expect("a clean pair reports");
    assert!(steals);
    assert_eq!(
        line,
        "ghost changes from operator at 1500 seconds to night fork at 1500 seconds"
    );
}

#[test]
fn test_an_equal_total_with_a_later_or_same_date_does_not_steal() {
    let old = make_run(
        "operator",
        "2026-01-04",
        &[("plan", n(300)), ("polish", n(1500))],
    );
    for date in ["2026-01-04", "2026-01-09"] {
        let candidate = make_run("night fork", date, &[("plan", n(200)), ("polish", n(1500))]);
        let (steals, line) = ghost_change(&old, &candidate).expect("a clean pair reports");
        assert!(!steals);
        assert_eq!(
            line,
            "ghost stays with operator at 1500 seconds against night fork at 1500 seconds"
        );
    }
}

// Round 1 repair pins, FIX-PG-1 to FIX-PG-5

fn huge_integer() -> String {
    format!("1{}", "0".repeat(400))
}

#[test]
fn test_a_huge_integer_window_is_not_a_finite_number() {
    let (run, defects) = load(&run_text(TASK, &huge_integer(), OPERATOR, DATE, SPLITS, ""));
    assert!(run.is_none());
    assert!(has(&defects, "field window_seconds is not a finite number"));
}

#[test]
fn test_a_huge_integer_split_elapsed_is_not_a_finite_number() {
    let splits = format!(
        "[{{\"phase\": \"plan\", \"elapsed_seconds\": {}}}]",
        huge_integer()
    );
    let (run, defects) = load(&run_text(TASK, "3600", OPERATOR, DATE, &splits, ""));
    assert!(run.is_none());
    assert!(has(
        &defects,
        "split 0 field elapsed_seconds is not a finite number"
    ));
}

#[test]
fn test_a_huge_integer_carried_total_is_not_a_finite_number() {
    let (run, defects) = load(&valid_text_with_total(&huge_integer()));
    assert!(run.is_none());
    assert!(has(&defects, "field total_seconds is not a finite number"));
}

#[test]
fn test_deeply_nested_input_is_not_valid_json_and_never_raises() {
    let text = format!("{}{}", "[".repeat(100000), "]".repeat(100000));
    let (run, defects) = load(&text);
    assert!(run.is_none());
    assert_eq!(defects, vec!["input is not valid JSON"]);
}

#[test]
fn test_a_task_holding_an_unpaired_surrogate_is_named() {
    let (run, defects) = load(&run_text(
        "\"\\ud800x\"",
        "3600",
        OPERATOR,
        DATE,
        SPLITS,
        "",
    ));
    assert!(run.is_none());
    assert!(has(&defects, "field task holds an unpaired surrogate"));
}

#[test]
fn test_a_runner_holding_an_unpaired_surrogate_is_named() {
    let (run, defects) = load(&run_text(
        TASK,
        "3600",
        "\"night \\ud800fork\"",
        DATE,
        SPLITS,
        "",
    ));
    assert!(run.is_none());
    assert!(has(&defects, "field runner holds an unpaired surrogate"));
}

#[test]
fn test_a_phase_holding_an_unpaired_surrogate_is_named() {
    let (run, defects) = load(&run_text(
        TASK,
        "3600",
        OPERATOR,
        DATE,
        "[{\"phase\": \"\\ud800plan\", \"elapsed_seconds\": 300}]",
        "",
    ));
    assert!(run.is_none());
    assert!(has(
        &defects,
        "split 0 field phase holds an unpaired surrogate"
    ));
}

#[test]
fn test_recorded_at_compact_and_week_forms_are_not_iso_dates() {
    for value in ["20260104", "2026-W01-1"] {
        let quoted = format!("\"{}\"", value);
        let (run, defects) = load(&run_text(TASK, "3600", OPERATOR, &quoted, SPLITS, ""));
        assert!(run.is_none());
        assert!(has(&defects, "field recorded_at is not an ISO date"));
    }
}

#[test]
fn test_a_split_elapsed_of_infinity_is_not_a_finite_number() {
    let text = "{\"task\": \"t\", \"window_seconds\": 3600, \"runner\": \"r\", \"recorded_at\": \"2026-01-04\", \"splits\": [{\"phase\": \"plan\", \"elapsed_seconds\": Infinity}]}";
    let (run, defects) = load(text);
    assert!(run.is_none());
    assert!(has(
        &defects,
        "split 0 field elapsed_seconds is not a finite number"
    ));
}

#[test]
fn test_a_carried_total_of_nan_is_not_a_finite_number() {
    let text = "{\"task\": \"t\", \"window_seconds\": 3600, \"runner\": \"r\", \"recorded_at\": \"2026-01-04\", \"total_seconds\": NaN, \"splits\": [{\"phase\": \"plan\", \"elapsed_seconds\": 3}]}";
    let (run, defects) = load(text);
    assert!(run.is_none());
    assert!(has(&defects, "field total_seconds is not a finite number"));
}

#[test]
fn test_the_window_line_names_the_seconds_to_spare() {
    let ghost = make_run_window("operator", "2026-01-04", &[("plan", n(1000))], n(3600));
    let live = make_run_window("operator", "2026-01-04", &[("plan", n(800))], n(3600));
    let lines = race_report(&ghost, &live);
    assert_eq!(
        lines[lines.len() - 2],
        "window, the live run finished inside its window with 2800 seconds to spare"
    );
    assert_eq!(lines[lines.len() - 1], "finish ahead by 200 seconds");
}

#[test]
fn test_the_window_line_names_the_overrun_past_the_window() {
    let ghost = make_run_window("operator", "2026-01-04", &[("plan", n(1000))], n(3600));
    let live = make_run_window("operator", "2026-01-04", &[("plan", n(3900))], n(3600));
    let lines = race_report(&ghost, &live);
    assert_eq!(
        lines[lines.len() - 2],
        "window, the live run ran 300 seconds past its window"
    );
    assert_eq!(lines[lines.len() - 1], "finish behind by 2900 seconds");
}

#[test]
fn test_a_total_exactly_at_the_window_reads_inside_with_zero_to_spare() {
    let ghost = make_run_window("operator", "2026-01-04", &[("plan", n(1000))], n(3600));
    let live = make_run_window("operator", "2026-01-04", &[("plan", n(3600))], n(3600));
    let lines = race_report(&ghost, &live);
    assert_eq!(
        lines[lines.len() - 2],
        "window, the live run finished inside its window with 0 seconds to spare"
    );
}

#[test]
fn test_a_sub_microsecond_difference_renders_at_full_precision() {
    let ghost = make_run("operator", "2026-01-04", &[("plan", f(1.0000004))]);
    let live = make_run("operator", "2026-01-05", &[("plan", f(1.0))]);
    let lines = race_report(&ghost, &live);
    let expected = py_float_repr(1.0000004f64 - 1.0f64);
    assert_eq!(lines[0], format!("plan ahead by {} seconds", expected));
    assert_eq!(
        lines[lines.len() - 1],
        format!("finish ahead by {} seconds", expected)
    );
    assert!(lines.iter().all(|line| !line.contains("by 0 seconds")));
}

// The FIX-PG-12 pins, added with the Round 11 repair. Each one
// hands the ghost change report a runner label holding a line
// break, a value the recorded run loader states no line break
// rule against, and states the named error the report must
// return on its error channel. On the unrepaired code the report
// returns its line with the break standing raw inside it, so
// each pin fails there for a real reason.

#[test]
fn test_ghost_change_refuses_a_break_bearing_candidate() {
    let old = make_run(
        "operator",
        "2026-01-04",
        &[("plan", n(300)), ("polish", n(1500))],
    );
    let candidate = make_run(
        "b\nb",
        "2026-01-05",
        &[("plan", n(240)), ("polish", n(1320))],
    );
    let error = ghost_change(&old, &candidate).expect_err("the report names the precondition");
    assert_eq!(error, "a runner label holds a line break");
}

#[test]
fn test_ghost_change_refuses_a_break_bearing_standing_ghost() {
    let old = make_run(
        "b\nb",
        "2026-01-04",
        &[("plan", n(300)), ("polish", n(1500))],
    );
    let candidate = make_run(
        "night fork",
        "2026-01-05",
        &[("plan", n(240)), ("polish", n(1320))],
    );
    let error = ghost_change(&old, &candidate).expect_err("the report names the precondition");
    assert_eq!(error, "a runner label holds a line break");
}

// The FIX-PG-14 pins, added with the Round 12 repair. The run
// loader's text field rule widens to line breaks for the task
// and the split phases, in the check_label form the field day
// loader states, so a file carrying a break in either field is
// refused at load instead of forging a line in the readings
// that print it. On the unrepaired code the files under the
// two break pins load clean, so those pins fail there for a
// real reason. The order pin passes there by design, because
// its file already draws the unpaired surrogate defect there,
// check_text_field drawing that defect and returning nothing,
// and so the pin discriminates the check order instead, one
// defect standing for a text field, the text checks first and
// the break phrase last, so a task holding a surrogate beside
// its break draws the surrogate phrase alone.

#[test]
fn test_a_task_holding_a_line_break_is_named() {
    let (run, defects) = load(&run_text(
        "\"draft\\nthe weekly summary\"",
        "3600",
        OPERATOR,
        DATE,
        SPLITS,
        "",
    ));
    assert!(run.is_none());
    assert_eq!(defects, vec!["field task holds a line break"]);
}

#[test]
fn test_a_phase_holding_a_carriage_return_is_named() {
    let splits = "[{\"phase\": \"plan\", \"elapsed_seconds\": 300}, {\"phase\": \"dra\\rft\", \"elapsed_seconds\": 900}]";
    let (run, defects) = load(&run_text(TASK, "3600", OPERATOR, DATE, splits, ""));
    assert!(run.is_none());
    assert_eq!(defects, vec!["split 1 field phase holds a line break"]);
}

#[test]
fn test_a_task_holding_a_surrogate_beside_a_break_draws_the_surrogate_phrase() {
    let (run, defects) = load(&run_text(
        "\"\\ud800\\nx\"",
        "3600",
        OPERATOR,
        DATE,
        SPLITS,
        "",
    ));
    assert!(run.is_none());
    assert_eq!(defects, vec!["field task holds an unpaired surrogate"]);
}
