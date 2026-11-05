//! Tests for PG-11 and PG-12, the rematch loader and reading.
//!
//! Written for Round 5. Each reading pin states its whole report,
//! so a pin fails when the walk, the tie discipline, the day
//! ordering, or a closing form moves, and the loader pins state
//! the whole defect list, so a pin fails when a phrase or its day
//! prefix moves.

use std::path::PathBuf;

use num_bigint::BigInt;

use paceghost::fieldday::{field_day_report, load_field_day_text, DayRun, FieldDay};
use paceghost::pynum::Num;
use paceghost::pytext::PText;
use paceghost::rematch::{load_rematch_file, load_rematch_text, rematch_report, Rematch};
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

fn rematch_text(first: &str, second: &str) -> String {
    format!("{{\"first\": {}, \"second\": {}}}", first, second)
}

fn loaded(first: &str, second: &str) -> Rematch {
    let (rematch, defects) = load_rematch_text(&rematch_text(first, second));
    assert_eq!(defects, Vec::<String>::new());
    rematch.expect("the rematch loads")
}

fn report(first: &str, second: &str) -> Vec<String> {
    rematch_report(&loaded(first, second)).expect("the report reads")
}

fn std_day(runs: &[String]) -> String {
    day_json(1800, "operator", &runs_json(runs))
}

fn temp_path(name: &str) -> PathBuf {
    std::env::temp_dir().join(format!("paceghost-rematch-{}-{}", std::process::id(), name))
}

#[test]
fn test_a_clean_rematch_loads_carrying_both_days() {
    let first = std_day(&[
        run_json("operator", 900, &[(300, 150)]),
        run_json("fork", 950, &[]),
    ]);
    let second = day_json(
        900,
        "fork",
        &runs_json(&[run_json("fork", 800, &[(100, 40)])]),
    );
    let rematch = loaded(&first, &second);
    assert_eq!(rematch.first.window_seconds, Num::from_i64(1800));
    assert_eq!(rematch.second.window_seconds, Num::from_i64(900));
    assert!(rematch.first.operator.eq_str("operator"));
    assert!(rematch.second.operator.eq_str("fork"));
    assert_eq!(rematch.first.runs.len(), 2);
    assert_eq!(rematch.second.runs.len(), 1);
    assert!(rematch.second.runs[0].runner.eq_str("fork"));
}

#[test]
fn test_extra_fields_are_ignored() {
    let first = std_day(&[run_json("operator", 900, &[(300, 150)])]);
    let second = std_day(&[run_json("operator", 900, &[(300, 150)])]);
    let text = format!(
        "{{\"first\": {}, \"second\": {}, \"standing\": \"ignored\"}}",
        first, second
    );
    let (rematch, defects) = load_rematch_text(&text);
    assert_eq!(defects, Vec::<String>::new());
    assert!(rematch.is_some());
}

#[test]
fn test_text_that_is_not_json_is_refused() {
    let (rematch, defects) = load_rematch_text("not json");
    assert!(rematch.is_none());
    assert_eq!(defects, vec!["input is not valid JSON"]);
}

#[test]
fn test_input_nested_too_deeply_is_refused() {
    let text = format!("{}{}", "[".repeat(100000), "]".repeat(100000));
    let (rematch, defects) = load_rematch_text(&text);
    assert!(rematch.is_none());
    assert_eq!(defects, vec!["input is not valid JSON"]);
}

#[test]
fn test_json_that_is_not_an_object_is_refused() {
    let (rematch, defects) = load_rematch_text("[1, 2]");
    assert!(rematch.is_none());
    assert_eq!(defects, vec!["input is not a JSON object"]);
}

#[test]
fn test_missing_days_are_each_named() {
    let (rematch, defects) = load_rematch_text("{}");
    assert!(rematch.is_none());
    assert_eq!(defects, vec!["missing field first", "missing field second"]);
}

#[test]
fn test_a_day_that_is_not_an_object_is_named_under_its_key() {
    let second = std_day(&[run_json("operator", 900, &[])]);
    let (rematch, defects) = load_rematch_text(&rematch_text("[]", &second));
    assert!(rematch.is_none());
    assert_eq!(defects, vec!["field first is not an object"]);
    let first = std_day(&[run_json("operator", 900, &[])]);
    let (rematch, defects) = load_rematch_text(&rematch_text(&first, "7"));
    assert!(rematch.is_none());
    assert_eq!(defects, vec!["field second is not an object"]);
}

#[test]
fn test_day_defects_stand_prefixed_and_first_day_first() {
    let first = "{\"window_seconds\": 1800, \"operator\": \"a\", \"runs\": []}";
    let second = "{\"window_seconds\": 1800, \"runs\": [{\"runner\": \"a\", \"usage\": 5}]}";
    let (rematch, defects) = load_rematch_text(&rematch_text(first, second));
    assert!(rematch.is_none());
    assert_eq!(
        defects,
        vec!["first field runs is empty", "second missing field operator"]
    );
}

#[test]
fn test_a_line_break_inside_a_day_is_refused_under_its_key() {
    let first = std_day(&[
        run_json("operator", 900, &[]),
        "{\"runner\": \"fo\\nrk\", \"usage\": 950}".to_string(),
    ]);
    let second = std_day(&[run_json("operator", 900, &[])]);
    let (rematch, defects) = load_rematch_text(&rematch_text(&first, &second));
    assert!(rematch.is_none());
    assert_eq!(defects, vec!["first run 1 field runner holds a line break"]);
}

#[test]
fn test_a_duplicate_label_inside_one_day_is_refused_under_its_key() {
    let first = std_day(&[run_json("operator", 900, &[])]);
    let second = day_json(
        1800,
        "a",
        &runs_json(&[run_json("a", 900, &[]), run_json("a", 950, &[])]),
    );
    let (rematch, defects) = load_rematch_text(&rematch_text(&first, &second));
    assert!(rematch.is_none());
    assert_eq!(
        defects,
        vec!["second run 1 field runner duplicates the label of run 0"]
    );
}

#[test]
fn test_the_same_runner_on_both_days_loads_clean() {
    let first = std_day(&[run_json("operator", 900, &[(300, 150)])]);
    let second = std_day(&[run_json("operator", 800, &[(300, 100)])]);
    let (rematch, defects) = load_rematch_text(&rematch_text(&first, &second));
    assert_eq!(defects, Vec::<String>::new());
    let rematch = rematch.expect("the rematch loads");
    assert_eq!(rematch.first.runs.len(), 1);
    assert_eq!(rematch.second.runs.len(), 1);
}

#[test]
fn test_an_unreadable_rematch_file_names_the_single_defect() {
    let (rematch, defects) = load_rematch_file(&temp_path("missing.json"));
    assert!(rematch.is_none());
    assert_eq!(defects, vec!["file cannot be read"]);
}

#[test]
fn test_a_rematch_file_loads_from_disk() {
    let path = temp_path("rematch.json");
    let first = std_day(&[run_json("operator", 900, &[(300, 150)])]);
    let second = std_day(&[run_json("operator", 800, &[(300, 100)])]);
    std::fs::write(&path, rematch_text(&first, &second)).expect("the scratch file writes");
    let (rematch, defects) = load_rematch_file(&path);
    assert_eq!(defects, Vec::<String>::new());
    assert_eq!(rematch.expect("the rematch loads").second.runs.len(), 1);
    let _ = std::fs::remove_file(&path);
}

#[test]
fn test_the_first_day_lines_equal_the_first_days_own_report() {
    let first_runs = [
        run_json("operator", 900, &[(300, 150), (900, 500)]),
        run_json("fork", 950, &[(600, 320)]),
        run_json("quiet", 990, &[]),
    ];
    let first = std_day(&first_runs);
    let second = day_json(
        1800,
        "operator",
        &runs_json(&[
            run_json("operator", 900, &[(300, 100)]),
            run_json("fork", 950, &[(600, 200)]),
        ]),
    );
    let lines = report(&first, &second);
    let (day, defects) = load_field_day_text(&first);
    assert_eq!(defects, Vec::<String>::new());
    let own = field_day_report(&day.expect("the day loads")).expect("the report reads");
    let own_record_lines: Vec<String> = own
        .iter()
        .filter(|line| line.starts_with("record at "))
        .cloned()
        .collect();
    let rematch_first_day_lines: Vec<String> = lines
        .iter()
        .filter(|line| line.contains(" on the first day, "))
        .map(|line| line.replace(" on the first day", ""))
        .collect();
    assert_eq!(rematch_first_day_lines, own_record_lines);
    assert_eq!(
        lines,
        vec![
            "rematch, first day window 1800 seconds, 3 runs, second day window 1800 seconds, 2 runs",
            "record at 300 seconds on the first day, operator at a projection of 900",
            "record at 900 seconds on the first day, fork at a projection of 960",
            "record at 300 seconds on the second day, operator at a projection of 600",
            "finish, record held by operator at a projection of 600",
            "record changed hands on the second day at 300 seconds, operator took the record from fork",
        ]
    );
}

#[test]
fn test_the_record_changes_hands_at_the_first_mark_of_the_second_day() {
    let first = day_json(
        1800,
        "a",
        &runs_json(&[
            run_json("a", 100, &[(300, 150)]),
            run_json("b", 100, &[(600, 100)]),
        ]),
    );
    let second = day_json(1800, "a", &runs_json(&[run_json("a", 100, &[(200, 25)])]));
    assert_eq!(
        report(&first, &second),
        vec![
            "rematch, first day window 1800 seconds, 2 runs, second day window 1800 seconds, 1 runs",
            "record at 300 seconds on the first day, a at a projection of 900",
            "record at 600 seconds on the first day, b at a projection of 300",
            "record at 200 seconds on the second day, a at a projection of 225",
            "finish, record held by a at a projection of 225",
            "record changed hands on the second day at 200 seconds, a took the record from b",
        ]
    );
}

#[test]
fn test_the_record_changes_hands_at_the_last_mark_of_the_second_day() {
    let first = day_json(
        1800,
        "a",
        &runs_json(&[
            run_json("a", 100, &[(300, 100)]),
            run_json("b", 100, &[(600, 150)]),
        ]),
    );
    let second = day_json(
        1800,
        "a",
        &runs_json(&[run_json("a", 100, &[(300, 200), (900, 200)])]),
    );
    assert_eq!(
        report(&first, &second),
        vec![
            "rematch, first day window 1800 seconds, 2 runs, second day window 1800 seconds, 1 runs",
            "record at 300 seconds on the first day, a at a projection of 600",
            "record at 600 seconds on the first day, b at a projection of 450",
            "record at 900 seconds on the second day, a at a projection of 400",
            "finish, record held by a at a projection of 400",
            "record changed hands on the second day at 900 seconds, a took the record from b",
        ]
    );
}

#[test]
fn test_an_equal_figure_on_the_second_day_never_takes_the_record() {
    let first = day_json(1800, "a", &runs_json(&[run_json("a", 100, &[(300, 100)])]));
    let second = day_json(1800, "b", &runs_json(&[run_json("b", 100, &[(300, 100)])]));
    assert_eq!(
        report(&first, &second),
        vec![
            "rematch, first day window 1800 seconds, 1 runs, second day window 1800 seconds, 1 runs",
            "record at 300 seconds on the first day, a at a projection of 600",
            "finish, record held by a at a projection of 600",
            "record never changed hands on the second day",
        ]
    );
}

#[test]
fn test_the_holder_rising_past_another_on_the_second_day_loses_the_record() {
    let first = day_json(
        1800,
        "a",
        &runs_json(&[
            run_json("a", 100, &[(300, 100)]),
            run_json("b", 100, &[(600, 200)]),
        ]),
    );
    let second = day_json(1800, "a", &runs_json(&[run_json("a", 100, &[(300, 300)])]));
    assert_eq!(
        report(&first, &second),
        vec![
            "rematch, first day window 1800 seconds, 2 runs, second day window 1800 seconds, 1 runs",
            "record at 300 seconds on the first day, a at a projection of 600",
            "record at 300 seconds on the second day, b at a projection of 600",
            "finish, record held by b at a projection of 600",
            "record changed hands on the second day at 300 seconds, b took the record from a",
        ]
    );
}

#[test]
fn test_no_record_on_the_first_day_reads_the_first_set_form() {
    let first = day_json(
        1800,
        "a",
        &runs_json(&[run_json("a", 100, &[]), run_json("b", 100, &[])]),
    );
    let second = day_json(1800, "a", &runs_json(&[run_json("a", 100, &[(300, 100)])]));
    assert_eq!(
        report(&first, &second),
        vec![
            "rematch, first day window 1800 seconds, 2 runs, second day window 1800 seconds, 1 runs",
            "record at 300 seconds on the second day, a at a projection of 600",
            "finish, record held by a at a projection of 600",
            "no record stood on the first day",
        ]
    );
}

#[test]
fn test_no_record_on_either_day_reads_no_record_was_set() {
    let first = day_json(1800, "a", &runs_json(&[run_json("a", 100, &[])]));
    let second = day_json(1800, "b", &runs_json(&[run_json("b", 100, &[])]));
    assert_eq!(
        report(&first, &second),
        vec![
            "rematch, first day window 1800 seconds, 1 runs, second day window 1800 seconds, 1 runs",
            "finish, no record was set",
        ]
    );
}

#[test]
fn test_each_day_projects_under_its_own_window() {
    let first = day_json(900, "a", &runs_json(&[run_json("a", 100, &[(300, 100)])]));
    let second = day_json(1800, "b", &runs_json(&[run_json("b", 100, &[(300, 40)])]));
    assert_eq!(
        report(&first, &second),
        vec![
            "rematch, first day window 900 seconds, 1 runs, second day window 1800 seconds, 1 runs",
            "record at 300 seconds on the first day, a at a projection of 300",
            "record at 300 seconds on the second day, b at a projection of 240",
            "finish, record held by b at a projection of 240",
            "record changed hands on the second day at 300 seconds, b took the record from a",
        ]
    );
}

#[test]
fn test_a_first_day_mark_stands_before_an_earlier_second_day_elapsed() {
    let first = day_json(1800, "a", &runs_json(&[run_json("a", 100, &[(1500, 250)])]));
    let second = day_json(1800, "b", &runs_json(&[run_json("b", 100, &[(100, 15)])]));
    assert_eq!(
        report(&first, &second),
        vec![
            "rematch, first day window 1800 seconds, 1 runs, second day window 1800 seconds, 1 runs",
            "record at 1500 seconds on the first day, a at a projection of 300",
            "record at 100 seconds on the second day, b at a projection of 270",
            "finish, record held by b at a projection of 270",
            "record changed hands on the second day at 100 seconds, b took the record from a",
        ]
    );
}

#[test]
fn test_the_change_of_hands_names_the_first_displacement_only() {
    let first = day_json(1800, "a", &runs_json(&[run_json("a", 100, &[(300, 100)])]));
    let second = day_json(
        1800,
        "b",
        &runs_json(&[
            run_json("b", 100, &[(300, 50)]),
            run_json("c", 100, &[(600, 25)]),
        ]),
    );
    assert_eq!(
        report(&first, &second),
        vec![
            "rematch, first day window 1800 seconds, 1 runs, second day window 1800 seconds, 2 runs",
            "record at 300 seconds on the first day, a at a projection of 600",
            "record at 300 seconds on the second day, b at a projection of 300",
            "record at 600 seconds on the second day, c at a projection of 75",
            "finish, record held by c at a projection of 75",
            "record changed hands on the second day at 300 seconds, b took the record from a",
        ]
    );
}

#[test]
fn test_a_tie_at_one_mark_breaks_toward_the_earlier_competitor() {
    let first = day_json(1800, "a", &runs_json(&[run_json("a", 100, &[(300, 200)])]));
    let second = day_json(
        1800,
        "b",
        &runs_json(&[
            run_json("b", 100, &[(300, 100)]),
            run_json("c", 100, &[(300, 100)]),
        ]),
    );
    assert_eq!(
        report(&first, &second),
        vec![
            "rematch, first day window 1800 seconds, 1 runs, second day window 1800 seconds, 2 runs",
            "record at 300 seconds on the first day, a at a projection of 1200",
            "record at 300 seconds on the second day, b at a projection of 600",
            "finish, record held by b at a projection of 600",
            "record changed hands on the second day at 300 seconds, b took the record from a",
        ]
    );
}

#[test]
fn test_a_standing_projection_carries_across_the_days() {
    let first = day_json(
        1800,
        "a",
        &runs_json(&[
            run_json("a", 100, &[(300, 100)]),
            run_json("b", 100, &[(300, 50)]),
        ]),
    );
    let second = day_json(
        1800,
        "c",
        &runs_json(&[
            run_json("c", 100, &[(200, 100)]),
            run_json("a", 100, &[(400, 30)]),
        ]),
    );
    assert_eq!(
        report(&first, &second),
        vec![
            "rematch, first day window 1800 seconds, 2 runs, second day window 1800 seconds, 2 runs",
            "record at 300 seconds on the first day, b at a projection of 300",
            "record at 400 seconds on the second day, a at a projection of 135",
            "finish, record held by a at a projection of 135",
            "record changed hands on the second day at 400 seconds, a took the record from b",
        ]
    );
}

#[test]
fn test_the_days_operators_take_no_part_in_the_reading() {
    let first_runs = &runs_json(&[
        run_json("a", 100, &[(300, 150)]),
        run_json("b", 100, &[(600, 100)]),
    ]);
    let second_runs = &runs_json(&[run_json("a", 100, &[(200, 25)])]);
    let one = report(
        &day_json(1800, "a", first_runs),
        &day_json(1800, "a", second_runs),
    );
    let two = report(
        &day_json(1800, "b", first_runs),
        &day_json(1800, "a", second_runs),
    );
    assert_eq!(one, two);
}

#[test]
fn test_the_opening_line_names_fractional_windows_and_counts() {
    let first = "{\"window_seconds\": 900.5, \"operator\": \"a\", \"runs\": [{\"runner\": \"a\", \"usage\": 5}]}";
    let second = day_json(
        1800,
        "a",
        &runs_json(&[run_json("a", 100, &[]), run_json("b", 100, &[])]),
    );
    assert_eq!(
        report(first, &second),
        vec![
            "rematch, first day window 900.5 seconds, 1 runs, second day window 1800 seconds, 2 runs",
            "finish, no record was set",
        ]
    );
}

#[test]
fn test_a_hand_built_rematch_with_no_runs_reads_no_record() {
    let day = FieldDay {
        window_seconds: Num::from_i64(1800),
        operator: PText::from_text("ghost"),
        runs: Vec::new(),
    };
    let lines = rematch_report(&Rematch {
        first: day.clone(),
        second: day,
    })
    .expect("the report reads");
    assert_eq!(
        lines,
        vec![
            "rematch, first day window 1800 seconds, 0 runs, second day window 1800 seconds, 0 runs",
            "finish, no record was set",
        ]
    );
}

#[test]
fn test_a_hand_built_repeated_label_stands_as_one_competitor() {
    let read = |elapsed: i64, usage: i64| Read {
        elapsed_seconds: Num::from_i64(elapsed),
        usage: Num::from_i64(usage),
    };
    let first = FieldDay {
        window_seconds: Num::from_i64(1800),
        operator: PText::from_text("a"),
        runs: vec![
            DayRun {
                runner: PText::from_text("a"),
                usage: None,
                reads: Some(vec![read(300, 100)]),
            },
            DayRun {
                runner: PText::from_text("a"),
                usage: None,
                reads: Some(vec![read(600, 50), read(600, 25)]),
            },
        ],
    };
    let second = FieldDay {
        window_seconds: Num::from_i64(1800),
        operator: PText::from_text("a"),
        runs: vec![DayRun {
            runner: PText::from_text("a"),
            usage: None,
            reads: None,
        }],
    };
    assert_eq!(
        rematch_report(&Rematch { first, second }).expect("the report reads"),
        vec![
            "rematch, first day window 1800 seconds, 2 runs, second day window 1800 seconds, 1 runs",
            "record at 300 seconds on the first day, a at a projection of 600",
            "record at 600 seconds on the first day, a at a projection of 75",
            "finish, record held by a at a projection of 75",
            "record never changed hands on the second day",
        ]
    );
}

// The FIX-PG-10 pins, added with the Round 5 repair. Each one hands
// the reading a figure its loader would refuse as not finite and
// states the error the reading must return. On the unrepaired code
// every non finite case below panics inside the exact conversion
// and the oversized cases render an infinity in a seconds
// position, so each pin fails there for a real reason.

fn hand_day(window: Num, runs: Vec<DayRun>) -> FieldDay {
    FieldDay {
        window_seconds: window,
        operator: PText::from_text("a"),
        runs,
    }
}

fn hand_run(reads: Vec<Read>) -> DayRun {
    DayRun {
        runner: PText::from_text("a"),
        usage: None,
        reads: Some(reads),
    }
}

fn ordinary_run() -> DayRun {
    hand_run(vec![Read {
        elapsed_seconds: Num::from_i64(300),
        usage: Num::from_i64(100),
    }])
}

fn non_finite_figures() -> Vec<Num> {
    vec![
        Num::from_f64(f64::NAN),
        Num::from_f64(f64::INFINITY),
        Num::from_f64(f64::NEG_INFINITY),
    ]
}

fn oversized_integer() -> Num {
    let mut value = BigInt::from(1);
    for _ in 0..400 {
        value *= 10;
    }
    Num::Int(value)
}

#[test]
fn test_a_non_finite_window_on_either_day_names_the_precondition() {
    for figure in non_finite_figures() {
        let rematch = Rematch {
            first: hand_day(figure, vec![ordinary_run()]),
            second: hand_day(Num::from_i64(1800), vec![ordinary_run()]),
        };
        let error = rematch_report(&rematch).expect_err("the report names the precondition");
        assert_eq!(error, "the window is not a finite number");
    }
    let rematch = Rematch {
        first: hand_day(Num::from_i64(1800), vec![ordinary_run()]),
        second: hand_day(Num::from_f64(f64::INFINITY), vec![ordinary_run()]),
    };
    let error = rematch_report(&rematch).expect_err("the report names the precondition");
    assert_eq!(error, "the window is not a finite number");
    let rematch = Rematch {
        first: hand_day(Num::from_i64(1800), vec![ordinary_run()]),
        second: hand_day(Num::from_f64(f64::NAN), Vec::new()),
    };
    let error = rematch_report(&rematch).expect_err("the report names the precondition");
    assert_eq!(error, "the window is not a finite number");
}

#[test]
fn test_a_non_finite_read_elapsed_on_the_first_day_names_the_precondition() {
    for figure in non_finite_figures() {
        let bad = hand_run(vec![Read {
            elapsed_seconds: figure,
            usage: Num::from_i64(100),
        }]);
        let rematch = Rematch {
            first: hand_day(Num::from_i64(1800), vec![bad]),
            second: hand_day(Num::from_i64(1800), vec![ordinary_run()]),
        };
        let error = rematch_report(&rematch).expect_err("the report names the precondition");
        assert_eq!(error, "a read elapsed value is not a finite number");
    }
}

#[test]
fn test_a_non_finite_read_usage_on_the_second_day_names_the_precondition() {
    for figure in non_finite_figures() {
        let bad = hand_run(vec![Read {
            elapsed_seconds: Num::from_i64(300),
            usage: figure,
        }]);
        let rematch = Rematch {
            first: hand_day(Num::from_i64(1800), vec![ordinary_run()]),
            second: hand_day(Num::from_i64(1800), vec![bad]),
        };
        let error = rematch_report(&rematch).expect_err("the report names the precondition");
        assert_eq!(error, "a read usage value is not a finite number");
    }
}

#[test]
fn test_a_non_finite_run_usage_on_the_second_day_names_the_precondition() {
    let bad = DayRun {
        runner: PText::from_text("a"),
        usage: Some(Num::from_f64(f64::NAN)),
        reads: Some(vec![Read {
            elapsed_seconds: Num::from_i64(300),
            usage: Num::from_i64(100),
        }]),
    };
    let rematch = Rematch {
        first: hand_day(Num::from_i64(1800), vec![ordinary_run()]),
        second: hand_day(Num::from_i64(1800), vec![bad]),
    };
    let error = rematch_report(&rematch).expect_err("the report names the precondition");
    assert_eq!(error, "a run usage value is not a finite number");
}

#[test]
fn test_an_oversized_window_names_the_precondition_instead_of_rendering_inf() {
    let rematch = Rematch {
        first: hand_day(oversized_integer(), vec![ordinary_run()]),
        second: hand_day(Num::from_i64(1800), vec![ordinary_run()]),
    };
    let error = rematch_report(&rematch).expect_err("the report names the precondition");
    assert_eq!(error, "the window is not a finite number");
}

#[test]
fn test_an_oversized_read_elapsed_names_the_precondition_instead_of_rendering_inf() {
    let bad = hand_run(vec![Read {
        elapsed_seconds: oversized_integer(),
        usage: Num::from_i64(100),
    }]);
    let rematch = Rematch {
        first: hand_day(Num::from_i64(1800), vec![bad]),
        second: hand_day(Num::from_i64(1800), vec![ordinary_run()]),
    };
    let error = rematch_report(&rematch).expect_err("the report names the precondition");
    assert_eq!(error, "a read elapsed value is not a finite number");
}
