//! Pins for the synthetic Round 2 examples under examples.
//!
//! Translated from tests/test_examples_round2.py. The example field
//! holds seven runs across three models against the example ghost
//! record, and the stored season report and board have to stay byte
//! identical to what the code renders today.

use std::collections::BTreeSet;
use std::path::PathBuf;

use paceghost::pynum::Num;
use paceghost::runs::{load_run_file, Run};
use paceghost::season::{board_report, load_field_file, season_report};

fn examples() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples")
}

fn load_ghost() -> Run {
    let (ghost, defects) = load_run_file(&examples().join("ghost-run.json"));
    assert_eq!(defects, Vec::<String>::new());
    ghost.expect("the ghost loads")
}

fn ghost_usage() -> Num {
    load_ghost()
        .usage
        .expect("the example ghost carries a usage figure")
}

#[test]
fn test_the_example_field_loads_clean_across_three_models() {
    let (runs, defects) = load_field_file(&examples().join("field.json"));
    assert_eq!(defects, Vec::<String>::new());
    let runs = runs.expect("the field loads");
    assert_eq!(runs.len(), 7);
    let models: BTreeSet<String> = runs
        .iter()
        .map(|run| {
            run.model
                .as_ref()
                .expect("a model stands")
                .to_display_string()
        })
        .collect();
    assert_eq!(
        models,
        ["atlas", "birch", "cedar"]
            .iter()
            .map(|name| name.to_string())
            .collect::<BTreeSet<String>>()
    );
}

#[test]
fn test_the_example_ghost_carries_a_usage_figure() {
    let ghost = load_ghost();
    assert_eq!(ghost.usage, Some(Num::from_i64(1180)));
    assert_eq!(ghost.total_seconds, Num::from_i64(1320));
}

#[test]
fn test_the_golden_season_report_is_byte_identical() {
    let (runs, defects) = load_field_file(&examples().join("field.json"));
    assert_eq!(defects, Vec::<String>::new());
    let runs = runs.expect("the field loads");
    let rendered = season_report(&runs, &ghost_usage()).join("\n") + "\n";
    let golden = std::fs::read(examples().join("season-golden.txt")).expect("the golden reads");
    assert_eq!(rendered.as_bytes(), golden.as_slice());
}

#[test]
fn test_the_example_board_run_loads_clean_with_a_zero_read() {
    let (run, defects) = load_run_file(&examples().join("board-run.json"));
    assert_eq!(defects, Vec::<String>::new());
    let run = run.expect("the run loads");
    let reads = run.reads.expect("the board run carries reads");
    assert_eq!(reads.len(), 4);
    assert_eq!(reads[0].elapsed_seconds, Num::from_i64(0));
}

#[test]
fn test_the_golden_board_is_byte_identical() {
    let (run, defects) = load_run_file(&examples().join("board-run.json"));
    assert_eq!(defects, Vec::<String>::new());
    let run = run.expect("the run loads");
    let rendered = board_report(&run, &ghost_usage()).join("\n") + "\n";
    let golden = std::fs::read(examples().join("board-golden.txt")).expect("the golden reads");
    assert_eq!(rendered.as_bytes(), golden.as_slice());
}
