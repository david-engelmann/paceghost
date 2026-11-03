//! Pins for the synthetic examples under examples.
//!
//! Translated from tests/test_examples.py. The two example runs are
//! the slice demo. The faster night fork run has to steal the ghost
//! from the operator run, and the stored race report has to stay
//! byte identical to the report the code renders today.

use std::path::PathBuf;

use paceghost::pynum::Num;
use paceghost::runs::{ghost_change, load_run_file, race_report, select_ghost, Run};

fn examples() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples")
}

fn load_example(name: &str) -> Run {
    let (run, defects) = load_run_file(&examples().join(name));
    assert_eq!(defects, Vec::<String>::new());
    run.expect("the example loads")
}

#[test]
fn test_both_example_runs_load_clean() {
    let operator = load_example("operator-run.json");
    let night_fork = load_example("night-fork-run.json");
    assert!(operator.runner.eq_str("operator"));
    assert_eq!(operator.total_seconds, Num::from_i64(1500));
    assert!(night_fork.runner.eq_str("night fork"));
    assert_eq!(night_fork.total_seconds, Num::from_i64(1320));
    assert_eq!(operator.task, night_fork.task);
}

#[test]
fn test_the_example_night_fork_steals_the_ghost() {
    let runs = vec![
        load_example("operator-run.json"),
        load_example("night-fork-run.json"),
    ];
    let ghost = select_ghost(&runs).expect("runs stand");
    assert!(std::ptr::eq(ghost, &runs[1]));
    let (steals, line) = ghost_change(&runs[0], &runs[1]);
    assert!(steals);
    assert_eq!(
        line,
        "ghost changes from operator at 1500 seconds to night fork at 1320 seconds"
    );
}

#[test]
fn test_the_golden_race_report_is_byte_identical() {
    let operator = load_example("operator-run.json");
    let night_fork = load_example("night-fork-run.json");
    let rendered = race_report(&operator, &night_fork).join("\n") + "\n";
    let golden = std::fs::read(examples().join("race-golden.txt")).expect("the golden reads");
    assert_eq!(rendered.as_bytes(), golden.as_slice());
}
