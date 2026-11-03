//! Pins for PG-5 in CONTRACT.md, the season over a recorded field.
//!
//! Translated from tests/test_season.py. The field loader names
//! every defect with its run index standing before the PG-1 phrase,
//! and the season figures are computed by hand here so a report that
//! grouped, counted, or ordered differently would fail.

use num_bigint::BigInt;
use num_rational::BigRational;

use paceghost::pynum::Num;
use paceghost::runs::Run;
use paceghost::season::{format_usage, load_field_text, season_report};

fn frac(numer: i64, denom: i64) -> BigRational {
    BigRational::new(BigInt::from(numer), BigInt::from(denom))
}

fn run_data(
    model: Option<&str>,
    total: &str,
    window: &str,
    usage: Option<&str>,
    budget: Option<&str>,
    task: &str,
    extra: &str,
) -> String {
    let mut text = format!(
        "{{\"task\": \"{}\", \"window_seconds\": {}, \"runner\": \"someone\", \"recorded_at\": \"2026-01-04\", \"splits\": [{{\"phase\": \"only\", \"elapsed_seconds\": {}}}]",
        task, window, total
    );
    if let Some(usage) = usage {
        text += &format!(", \"usage\": {}", usage);
    }
    if let Some(budget) = budget {
        text += &format!(", \"budget\": {}", budget);
    }
    if let Some(model) = model {
        text += &format!(", \"model\": {}", model);
    }
    text += extra;
    text += "}";
    text
}

fn std_run(model: &str, total: i64) -> String {
    run_data(
        Some(&format!("\"{}\"", model)),
        &total.to_string(),
        "200",
        Some("40"),
        Some("50"),
        "t",
        "",
    )
}

fn load_field(runs: &[String]) -> (Option<Vec<Run>>, Vec<String>) {
    load_field_text(&format!("{{\"runs\": [{}]}}", runs.join(", ")))
}

fn load_clean_field(runs: &[String]) -> Vec<Run> {
    let (runs, defects) = load_field(runs);
    assert_eq!(defects, Vec::<String>::new());
    runs.expect("the field loads")
}

fn ghost(usage: i64) -> Num {
    Num::from_i64(usage)
}

#[test]
fn test_contract_states_pg5_and_pg6_in_full_form() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let text = std::fs::read_to_string(root.join("CONTRACT.md")).expect("the contract reads");
    assert!(text.contains("PG-5 full form"));
    assert!(text.contains("PG-6 full form"));
}

#[test]
fn test_a_clean_field_loads_with_models_attached() {
    let runs = load_clean_field(&[std_run("peak", 100), std_run("vale", 120)]);
    let models: Vec<String> = runs
        .iter()
        .map(|run| {
            run.model
                .as_ref()
                .expect("a model stands")
                .to_display_string()
        })
        .collect();
    assert_eq!(models, vec!["peak", "vale"]);
    assert_eq!(runs[0].total_seconds, Num::from_i64(100));
}

#[test]
fn test_field_text_that_is_not_json_draws_the_shared_defect() {
    let (runs, defects) = load_field_text("this is not json");
    assert!(runs.is_none());
    assert_eq!(defects, vec!["input is not valid JSON"]);
}

#[test]
fn test_field_top_level_list_draws_the_not_an_object_defect() {
    let (runs, defects) = load_field_text("[]");
    assert!(runs.is_none());
    assert_eq!(defects, vec!["input is not a JSON object"]);
}

#[test]
fn test_field_without_runs_draws_missing_field_runs() {
    let (runs, defects) = load_field_text("{}");
    assert!(runs.is_none());
    assert_eq!(defects, vec!["missing field runs"]);
}

#[test]
fn test_field_runs_of_the_wrong_type_is_named() {
    let (runs, defects) = load_field_text("{\"runs\": {}}");
    assert!(runs.is_none());
    assert_eq!(defects, vec!["field runs has the wrong type"]);
}

#[test]
fn test_field_runs_empty_is_named() {
    let (runs, defects) = load_field_text("{\"runs\": []}");
    assert!(runs.is_none());
    assert_eq!(defects, vec!["field runs is empty"]);
}

#[test]
fn test_a_run_that_is_not_an_object_is_named_by_index() {
    let (runs, defects) = load_field(&[std_run("peak", 100), "42".to_string()]);
    assert!(runs.is_none());
    assert_eq!(defects, vec!["run 1 is not an object"]);
}

#[test]
fn test_a_run_defect_carries_the_run_index_before_the_pg1_phrase() {
    let bad = "{\"window_seconds\": 200, \"runner\": \"someone\", \"recorded_at\": \"2026-01-04\", \"splits\": [{\"phase\": \"only\", \"elapsed_seconds\": 100}], \"usage\": 40, \"budget\": 50, \"model\": \"peak\"}".to_string();
    let (runs, defects) = load_field(&[std_run("vale", 100), bad]);
    assert!(runs.is_none());
    assert_eq!(defects, vec!["run 1 missing field task"]);
}

#[test]
fn test_a_split_defect_inside_a_run_keeps_both_indexes() {
    let bad = run_data(
        Some("\"peak\""),
        "100",
        "200",
        Some("40"),
        Some("50"),
        "t",
        "",
    )
    .replace(
        "\"splits\": [{\"phase\": \"only\", \"elapsed_seconds\": 100}]",
        "\"splits\": [{\"phase\": \"\", \"elapsed_seconds\": 5}]",
    );
    let (runs, defects) = load_field(&[bad]);
    assert!(runs.is_none());
    assert_eq!(defects, vec!["run 0 split 0 field phase is empty"]);
}

#[test]
fn test_a_run_without_a_model_is_named() {
    let bad = run_data(None, "100", "200", Some("40"), Some("50"), "t", "");
    let (runs, defects) = load_field(&[bad]);
    assert!(runs.is_none());
    assert_eq!(defects, vec!["run 0 missing field model"]);
}

#[test]
fn test_a_model_of_the_wrong_type_is_named() {
    let (runs, defects) = load_field(&[run_data(
        Some("7"),
        "100",
        "200",
        Some("40"),
        Some("50"),
        "t",
        "",
    )]);
    assert!(runs.is_none());
    assert_eq!(defects, vec!["run 0 field model has the wrong type"]);
}

#[test]
fn test_an_empty_model_is_named() {
    let (runs, defects) = load_field(&[run_data(
        Some("\"\""),
        "100",
        "200",
        Some("40"),
        Some("50"),
        "t",
        "",
    )]);
    assert!(runs.is_none());
    assert_eq!(defects, vec!["run 0 field model is empty"]);
}

#[test]
fn test_a_model_holding_an_unpaired_surrogate_is_named() {
    let (runs, defects) = load_field(&[run_data(
        Some("\"pea\\ud800k\""),
        "100",
        "200",
        Some("40"),
        Some("50"),
        "t",
        "",
    )]);
    assert!(runs.is_none());
    assert_eq!(
        defects,
        vec!["run 0 field model holds an unpaired surrogate"]
    );
}

#[test]
fn test_a_run_without_usage_is_named_in_a_field() {
    let bad = run_data(Some("\"peak\""), "100", "200", None, Some("50"), "t", "");
    let (runs, defects) = load_field(&[bad]);
    assert!(runs.is_none());
    assert_eq!(defects, vec!["run 0 missing field usage"]);
}

#[test]
fn test_a_run_without_budget_is_named_in_a_field() {
    let bad = run_data(Some("\"peak\""), "100", "200", Some("40"), None, "t", "");
    let (runs, defects) = load_field(&[bad]);
    assert!(runs.is_none());
    assert_eq!(defects, vec!["run 0 missing field budget"]);
}

#[test]
fn test_a_run_with_a_different_task_is_named() {
    let other = run_data(
        Some("\"vale\""),
        "100",
        "200",
        Some("40"),
        Some("50"),
        "other",
        "",
    );
    let (runs, defects) = load_field(&[std_run("peak", 100), other]);
    assert!(runs.is_none());
    assert_eq!(
        defects,
        vec!["run 1 field task does not match the field task"]
    );
}

#[test]
fn test_the_season_figures_are_the_hand_computed_ones() {
    let field = vec![
        run_data(
            Some("\"peak\""),
            "100",
            "200",
            Some("40"),
            Some("50"),
            "t",
            "",
        ),
        run_data(
            Some("\"peak\""),
            "250",
            "200",
            Some("30"),
            Some("50"),
            "t",
            "",
        ),
        run_data(
            Some("\"vale\""),
            "150",
            "200",
            Some("44"),
            Some("40"),
            "t",
            "",
        ),
        run_data(
            Some("\"vale\""),
            "120",
            "200",
            Some("20"),
            Some("40"),
            "t",
            "",
        ),
        run_data(
            Some("\"wick\""),
            "90",
            "200",
            Some("44"),
            Some("100"),
            "t",
            "",
        ),
    ];
    let runs = load_clean_field(&field);
    assert_eq!(
        season_report(&runs, &ghost(45)),
        vec![
            "season for task t",
            "model vale",
            "runs raced 2",
            "inside window 2",
            "inside budget 1",
            "total usage 64",
            "mean usage 32",
            "head to head against the ghost, wins 2 of 2",
            "model wick",
            "runs raced 1",
            "inside window 1",
            "inside budget 1",
            "total usage 44",
            "mean usage 44",
            "head to head against the ghost, wins 1 of 1",
            "model peak",
            "runs raced 2",
            "inside window 1",
            "inside budget 2",
            "total usage 70",
            "mean usage 35",
            "head to head against the ghost, wins 1 of 2",
        ]
    );
}

#[test]
fn test_a_win_needs_the_window_and_less_usage_than_the_ghost() {
    let field = vec![
        run_data(
            Some("\"solo\""),
            "250",
            "200",
            Some("10"),
            Some("50"),
            "t",
            "",
        ),
        run_data(
            Some("\"solo\""),
            "100",
            "200",
            Some("45"),
            Some("50"),
            "t",
            "",
        ),
        run_data(
            Some("\"solo\""),
            "100",
            "200",
            Some("44"),
            Some("50"),
            "t",
            "",
        ),
    ];
    let runs = load_clean_field(&field);
    let lines = season_report(&runs, &ghost(45));
    assert_eq!(
        lines[lines.len() - 1],
        "head to head against the ghost, wins 1 of 3"
    );
}

#[test]
fn test_ordering_breaks_a_wins_and_total_tie_by_model_name() {
    let field = vec![
        run_data(
            Some("\"beta\""),
            "50",
            "200",
            Some("10"),
            Some("50"),
            "t",
            "",
        ),
        run_data(
            Some("\"alpha\""),
            "50",
            "200",
            Some("10"),
            Some("50"),
            "t",
            "",
        ),
    ];
    let runs = load_clean_field(&field);
    let lines = season_report(&runs, &ghost(45));
    assert_eq!(lines[1], "model alpha");
    assert_eq!(lines[8], "model beta");
}

#[test]
fn test_the_mean_renders_at_six_decimal_places() {
    let field = vec![
        run_data(
            Some("\"trio\""),
            "100",
            "200",
            Some("40"),
            Some("50"),
            "t",
            "",
        ),
        run_data(
            Some("\"trio\""),
            "100",
            "200",
            Some("30"),
            Some("50"),
            "t",
            "",
        ),
        run_data(
            Some("\"trio\""),
            "100",
            "200",
            Some("30"),
            Some("50"),
            "t",
            "",
        ),
    ];
    let runs = load_clean_field(&field);
    let lines = season_report(&runs, &ghost(45));
    assert!(lines.iter().any(|line| line == "total usage 100"));
    assert!(lines.iter().any(|line| line == "mean usage 33.333333"));
}

#[test]
fn test_usage_figures_render_exactly() {
    assert_eq!(format_usage(&frac(0, 1)), "0");
    assert_eq!(format_usage(&frac(2170, 1)), "2170");
    assert_eq!(format_usage(&Num::from_f64(2.5).to_rational()), "2.5");
    assert_eq!(format_usage(&Num::from_f64(0.1 + 0.2).to_rational()), "0.3");
    assert_eq!(
        format_usage(&Num::from_f64(1.0 / 3.0).to_rational()),
        "0.333333"
    );
    assert_eq!(format_usage(&frac(4, 10000000)), "0.0000004");
}

#[test]
fn test_an_exact_half_at_the_seventh_place_rounds_up() {
    assert_eq!(format_usage(&frac(1, 2000000)), "0.000001");
    assert_eq!(format_usage(&frac(20000005, 10000000)), "2.000001");
    assert_eq!(format_usage(&frac(10000005, 10000000)), "1.000001");
    assert_eq!(format_usage(&frac(25, 10000000)), "0.000003");
    assert_eq!(format_usage(&frac(15, 10000000)), "0.000002");
    assert_eq!(
        format_usage(&BigRational::new(
            BigInt::from(1234567891234565i64),
            BigInt::from(10000000)
        )),
        "123456789.123457"
    );
}

#[test]
fn test_a_figure_above_half_the_smallest_unit_rounds_at_six_places() {
    assert_eq!(format_usage(&frac(9, 10000000)), "0.000001");
    assert_eq!(format_usage(&frac(2, 3000000)), "0.000001");
}

#[test]
fn test_a_tiny_figure_renders_its_exact_expansion_when_it_terminates() {
    assert_eq!(format_usage(&frac(1, 10000000)), "0.0000001");
    assert_eq!(
        format_usage(&BigRational::new(
            BigInt::from(1),
            BigInt::from(2000000000000i64)
        )),
        "0.0000000000005"
    );
}

#[test]
fn test_a_tiny_figure_that_never_terminates_renders_at_twelve_places() {
    assert_eq!(format_usage(&frac(1, 30000000)), "0.000000033333");
}

#[test]
fn test_a_season_mean_too_small_for_six_places_renders_at_twelve() {
    let field = vec![
        run_data(
            Some("\"peak\""),
            "100",
            "200",
            Some("0.0000001"),
            Some("50"),
            "t",
            "",
        ),
        run_data(
            Some("\"peak\""),
            "100",
            "200",
            Some("0"),
            Some("50"),
            "t",
            "",
        ),
        run_data(
            Some("\"peak\""),
            "100",
            "200",
            Some("0"),
            Some("50"),
            "t",
            "",
        ),
    ];
    let runs = load_clean_field(&field);
    let lines = season_report(&runs, &ghost(45));
    assert!(lines.iter().any(|line| line == "mean usage 0.000000033333"));
}

#[test]
fn test_a_field_run_with_a_negative_usage_is_named_under_its_index() {
    let (runs, defects) = load_field(&[run_data(
        Some("\"peak\""),
        "100",
        "200",
        Some("-1"),
        Some("50"),
        "t",
        "",
    )]);
    assert!(runs.is_none());
    assert_eq!(defects, vec!["run 0 field usage is negative"]);
}

#[test]
fn test_a_field_run_with_a_budget_of_zero_is_named_under_its_index() {
    let (runs, defects) = load_field(&[run_data(
        Some("\"peak\""),
        "100",
        "200",
        Some("40"),
        Some("0"),
        "t",
        "",
    )]);
    assert!(runs.is_none());
    assert_eq!(defects, vec!["run 0 field budget must be positive"]);
}

#[test]
fn test_a_malformed_read_inside_a_field_run_keeps_both_indexes() {
    let bad = run_data(
        Some("\"peak\""),
        "100",
        "200",
        Some("40"),
        Some("50"),
        "t",
        ", \"reads\": [{\"elapsed_seconds\": 10, \"usage\": 5}, {\"elapsed_seconds\": 20, \"usage\": 4}]",
    );
    let (runs, defects) = load_field(&[bad]);
    assert!(runs.is_none());
    assert_eq!(
        defects,
        vec!["run 0 read 1 field usage is less than the previous read"]
    );
}

#[test]
fn test_the_season_report_is_byte_deterministic() {
    let field = vec![
        std_run("peak", 100),
        run_data(
            Some("\"vale\""),
            "120",
            "200",
            Some("20"),
            Some("50"),
            "t",
            "",
        ),
    ];
    let runs = load_clean_field(&field);
    assert_eq!(
        season_report(&runs, &ghost(45)),
        season_report(&runs, &ghost(45))
    );
}
