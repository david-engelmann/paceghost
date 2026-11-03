//! Pins for PG-6 in CONTRACT.md and the Round 2 run extension.
//!
//! Translated from tests/test_board.py. The loader names every
//! malformed extension field, a recorded run without reads loads
//! exactly as Round 1 defined it, and the board projections are
//! computed by hand here so a board that extrapolated, compared, or
//! judged differently would fail.

use paceghost::pynum::Num;
use paceghost::runs::{load_run_text, Run};
use paceghost::season::board_report;

fn base(extra: &str) -> String {
    format!(
        "{{\"task\": \"t\", \"window_seconds\": 100, \"runner\": \"someone\", \"recorded_at\": \"2026-01-04\", \"splits\": [{{\"phase\": \"only\", \"elapsed_seconds\": 90}}]{}}}",
        extra
    )
}

fn board_run(reads: &str, usage: &str, budget: &str, window: &str) -> String {
    format!(
        "{{\"task\": \"t\", \"window_seconds\": {}, \"runner\": \"someone\", \"recorded_at\": \"2026-01-04\", \"splits\": [{{\"phase\": \"only\", \"elapsed_seconds\": 90}}], \"usage\": {}, \"budget\": {}, \"reads\": {}}}",
        window, usage, budget, reads
    )
}

fn load(text: &str) -> (Option<Run>, Vec<String>) {
    load_run_text(text)
}

fn load_clean(text: &str) -> Run {
    let (run, defects) = load(text);
    assert_eq!(defects, Vec::<String>::new());
    run.expect("the run loads")
}

fn ghost(usage: i64) -> Num {
    Num::from_i64(usage)
}

#[test]
fn test_a_round_1_record_without_reads_loads_exactly_as_before() {
    let (run, defects) = load(&base(""));
    assert_eq!(defects, Vec::<String>::new());
    let run = run.expect("the run loads");
    // The key set of the Python run dict for a Round 1 record.
    assert!(run.usage.is_none());
    assert!(run.budget.is_none());
    assert!(run.reads.is_none());
    assert!(run.model.is_none());
    assert_eq!(run.total_seconds, Num::from_i64(90));
}

#[test]
fn test_a_run_with_usage_and_budget_but_no_reads_loads_clean() {
    let run = load_clean(&base(", \"usage\": 12, \"budget\": 20"));
    assert_eq!(run.usage, Some(Num::from_i64(12)));
    assert_eq!(run.budget, Some(Num::from_i64(20)));
    assert!(run.reads.is_none());
}

#[test]
fn test_usage_of_the_wrong_type_is_named() {
    let (run, defects) = load(&base(", \"usage\": \"many\""));
    assert!(run.is_none());
    assert_eq!(defects, vec!["field usage has the wrong type"]);
}

#[test]
fn test_usage_that_is_not_finite_is_named() {
    let text = "{\"task\": \"t\", \"window_seconds\": 100, \"runner\": \"r\", \"recorded_at\": \"2026-01-04\", \"splits\": [{\"phase\": \"a\", \"elapsed_seconds\": 1}], \"usage\": NaN}";
    let (run, defects) = load(text);
    assert!(run.is_none());
    assert_eq!(defects, vec!["field usage is not a finite number"]);
}

#[test]
fn test_negative_usage_is_named() {
    let (run, defects) = load(&base(", \"usage\": -1"));
    assert!(run.is_none());
    assert_eq!(defects, vec!["field usage is negative"]);
}

#[test]
fn test_budget_of_the_wrong_type_is_named() {
    let (run, defects) = load(&base(", \"budget\": \"some\""));
    assert!(run.is_none());
    assert_eq!(defects, vec!["field budget has the wrong type"]);
}

#[test]
fn test_budget_that_is_not_finite_is_named() {
    let text = "{\"task\": \"t\", \"window_seconds\": 100, \"runner\": \"r\", \"recorded_at\": \"2026-01-04\", \"splits\": [{\"phase\": \"a\", \"elapsed_seconds\": 1}], \"budget\": Infinity}";
    let (run, defects) = load(text);
    assert!(run.is_none());
    assert_eq!(defects, vec!["field budget is not a finite number"]);
}

#[test]
fn test_a_budget_of_zero_is_named() {
    let (run, defects) = load(&base(", \"budget\": 0"));
    assert!(run.is_none());
    assert_eq!(defects, vec!["field budget must be positive"]);
}

#[test]
fn test_reads_of_the_wrong_type_is_named() {
    let (run, defects) = load(&base(", \"usage\": 1, \"budget\": 2, \"reads\": {}"));
    assert!(run.is_none());
    assert_eq!(defects, vec!["field reads has the wrong type"]);
}

#[test]
fn test_an_empty_reads_list_is_named() {
    let (run, defects) = load(&base(", \"usage\": 1, \"budget\": 2, \"reads\": []"));
    assert!(run.is_none());
    assert_eq!(defects, vec!["field reads is empty"]);
}

#[test]
fn test_a_read_that_is_not_an_object_is_named() {
    let (run, defects) = load(&board_run("[7]", "30", "50", "100"));
    assert!(run.is_none());
    assert_eq!(defects, vec!["read 0 is not an object"]);
}

#[test]
fn test_a_read_missing_a_field_is_named() {
    let (run, defects) = load(&board_run("[{\"elapsed_seconds\": 5}]", "30", "50", "100"));
    assert!(run.is_none());
    assert_eq!(defects, vec!["read 0 is missing field usage"]);
}

#[test]
fn test_a_read_field_of_the_wrong_type_is_named() {
    let (run, defects) = load(&board_run(
        "[{\"elapsed_seconds\": \"5\", \"usage\": 1}]",
        "30",
        "50",
        "100",
    ));
    assert!(run.is_none());
    assert_eq!(
        defects,
        vec!["read 0 field elapsed_seconds has the wrong type"]
    );
}

#[test]
fn test_a_read_elapsed_that_is_not_finite_is_named() {
    let text = "{\"task\": \"t\", \"window_seconds\": 100, \"runner\": \"r\", \"recorded_at\": \"2026-01-04\", \"splits\": [{\"phase\": \"a\", \"elapsed_seconds\": 1}], \"usage\": 1, \"budget\": 2, \"reads\": [{\"elapsed_seconds\": NaN, \"usage\": 1}]}";
    let (run, defects) = load(text);
    assert!(run.is_none());
    assert_eq!(
        defects,
        vec!["read 0 field elapsed_seconds is not a finite number"]
    );
}

#[test]
fn test_a_negative_read_elapsed_is_named() {
    let (run, defects) = load(&board_run(
        "[{\"elapsed_seconds\": -5, \"usage\": 1}]",
        "30",
        "50",
        "100",
    ));
    assert!(run.is_none());
    assert_eq!(defects, vec!["read 0 field elapsed_seconds is negative"]);
}

#[test]
fn test_a_read_elapsed_that_fails_to_increase_is_named() {
    let reads =
        "[{\"elapsed_seconds\": 50, \"usage\": 1}, {\"elapsed_seconds\": 50, \"usage\": 2}]";
    let (run, defects) = load(&board_run(reads, "30", "50", "100"));
    assert!(run.is_none());
    assert_eq!(
        defects,
        vec!["read 1 field elapsed_seconds is not greater than the previous read"]
    );
}

#[test]
fn test_a_read_usage_that_is_not_finite_is_named() {
    let text = "{\"task\": \"t\", \"window_seconds\": 100, \"runner\": \"r\", \"recorded_at\": \"2026-01-04\", \"splits\": [{\"phase\": \"a\", \"elapsed_seconds\": 1}], \"usage\": 1, \"budget\": 2, \"reads\": [{\"elapsed_seconds\": 5, \"usage\": NaN}]}";
    let (run, defects) = load(text);
    assert!(run.is_none());
    assert_eq!(defects, vec!["read 0 field usage is not a finite number"]);
}

#[test]
fn test_a_negative_read_usage_is_named() {
    let (run, defects) = load(&board_run(
        "[{\"elapsed_seconds\": 5, \"usage\": -1}]",
        "30",
        "50",
        "100",
    ));
    assert!(run.is_none());
    assert_eq!(defects, vec!["read 0 field usage is negative"]);
}

#[test]
fn test_a_read_usage_that_decreases_is_named() {
    let reads = "[{\"elapsed_seconds\": 5, \"usage\": 10}, {\"elapsed_seconds\": 9, \"usage\": 4}]";
    let (run, defects) = load(&board_run(reads, "30", "50", "100"));
    assert!(run.is_none());
    assert_eq!(
        defects,
        vec!["read 1 field usage is less than the previous read"]
    );
}

#[test]
fn test_reads_without_usage_and_budget_are_named() {
    let text = base(", \"reads\": [{\"elapsed_seconds\": 5, \"usage\": 1}]");
    let (run, defects) = load(&text);
    assert!(run.is_none());
    assert_eq!(defects, vec!["missing field usage", "missing field budget"]);
}

#[test]
fn test_reads_with_usage_but_no_budget_names_only_the_budget() {
    let text = base(", \"usage\": 9, \"reads\": [{\"elapsed_seconds\": 5, \"usage\": 1}]");
    let (run, defects) = load(&text);
    assert!(run.is_none());
    assert_eq!(defects, vec!["missing field budget"]);
}

#[test]
fn test_the_board_lines_are_the_hand_computed_ones() {
    let reads = "[{\"elapsed_seconds\": 0, \"usage\": 0}, {\"elapsed_seconds\": 50, \"usage\": 10}, {\"elapsed_seconds\": 150, \"usage\": 45}]";
    let run = load_clean(&board_run(reads, "30", "50", "100"));
    assert_eq!(
        board_report(&run, &ghost(40)),
        vec![
            "read 0 at 0 seconds, usage 0, projects no figure",
            "read 1 at 50 seconds, usage 10, projection 20, inside the ghost usage, inside the budget",
            "read 2 at 150 seconds, usage 45, projection 30, inside the ghost usage, inside the budget",
            "finish, actual usage 30 against a last projection of 30, inside the ghost usage, inside the budget, the board called it",
        ]
    );
}

#[test]
fn test_a_projection_past_both_limits_says_so() {
    let run = load_clean(&board_run(
        "[{\"elapsed_seconds\": 10, \"usage\": 30}]",
        "30",
        "50",
        "100",
    ));
    let lines = board_report(&run, &ghost(40));
    assert_eq!(
        lines[0],
        "read 0 at 10 seconds, usage 30, projection 300, past the ghost usage, past the budget"
    );
}

#[test]
fn test_a_projection_level_with_the_ghost_usage_says_so() {
    let run = load_clean(&board_run(
        "[{\"elapsed_seconds\": 25, \"usage\": 10}]",
        "40",
        "50",
        "100",
    ));
    let lines = board_report(&run, &ghost(40));
    assert_eq!(
        lines[0],
        "read 0 at 25 seconds, usage 10, projection 40, level with the ghost usage, inside the budget"
    );
    assert_eq!(
        lines[lines.len() - 1],
        "finish, actual usage 40 against a last projection of 40, level with the ghost usage, inside the budget, the board called it"
    );
}

#[test]
fn test_the_board_missed_when_the_projection_verdicts_differ() {
    let run = load_clean(&board_run(
        "[{\"elapsed_seconds\": 50, \"usage\": 30}]",
        "30",
        "50",
        "100",
    ));
    let lines = board_report(&run, &ghost(40));
    assert_eq!(
        lines[lines.len() - 1],
        "finish, actual usage 30 against a last projection of 60, inside the ghost usage, inside the budget, the board missed it"
    );
}

#[test]
fn test_a_board_of_only_a_zero_read_makes_no_projection() {
    let run = load_clean(&board_run(
        "[{\"elapsed_seconds\": 0, \"usage\": 0}]",
        "30",
        "50",
        "100",
    ));
    assert_eq!(
        board_report(&run, &ghost(40)),
        vec![
            "read 0 at 0 seconds, usage 0, projects no figure",
            "finish, actual usage 30, no projection was made, inside the ghost usage, inside the budget",
        ]
    );
}

#[test]
fn test_a_tiny_projection_renders_at_twelve_places_in_plain_notation() {
    let run = load_clean(&board_run(
        "[{\"elapsed_seconds\": 30000000, \"usage\": 1}]",
        "1",
        "50",
        "1",
    ));
    let lines = board_report(&run, &ghost(40));
    assert_eq!(
        lines[0],
        "read 0 at 30000000 seconds, usage 1, projection 0.000000033333, inside the ghost usage, inside the budget"
    );
}

#[test]
fn test_a_projection_of_exactly_half_the_smallest_unit_rounds_up() {
    let run = load_clean(&board_run(
        "[{\"elapsed_seconds\": 2000000, \"usage\": 1}]",
        "1",
        "50",
        "1",
    ));
    let lines = board_report(&run, &ghost(40));
    assert_eq!(
        lines[0],
        "read 0 at 2000000 seconds, usage 1, projection 0.000001, inside the ghost usage, inside the budget"
    );
}

#[test]
fn test_an_exact_half_projection_rounds_up() {
    let run = load_clean(&board_run(
        "[{\"elapsed_seconds\": 2000000, \"usage\": 4000001}]",
        "30",
        "50",
        "1",
    ));
    let lines = board_report(&run, &ghost(40));
    assert_eq!(
        lines[0],
        "read 0 at 2000000 seconds, usage 4000001, projection 2.000001, inside the ghost usage, inside the budget"
    );
}

#[test]
fn test_the_board_is_byte_deterministic() {
    let reads = "[{\"elapsed_seconds\": 0, \"usage\": 0}, {\"elapsed_seconds\": 40, \"usage\": 7}]";
    let run = load_clean(&board_run(reads, "30", "50", "100"));
    assert_eq!(
        board_report(&run, &ghost(40)),
        board_report(&run, &ghost(40))
    );
}
