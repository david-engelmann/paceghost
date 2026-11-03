//! Pins for the season and board commands in CONTRACT.md.
//!
//! Translated from tests/test_commands_round2.py. Each command gets
//! a clean run with its exact printed lines and each refusal with
//! its exit status and its single line, so a command that started
//! printing a report for a refused input would fail.

use std::path::{Path, PathBuf};

use paceghost::cli::{dispatch, CliOutcome};
use paceghost::pytext::PText;

fn reader(bytes: &[u8]) -> Option<String> {
    let path = String::from_utf8_lossy(bytes).into_owned();
    let data = std::fs::read(path).ok()?;
    String::from_utf8(data).ok()
}

fn cli(args: &[&str]) -> CliOutcome {
    let argv: Vec<PText> = args.iter().map(|arg| PText::from_text(arg)).collect();
    dispatch(&argv, &reader)
}

fn scratch(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "paceghost-commands2-{}-{}",
        std::process::id(),
        tag
    ));
    std::fs::create_dir_all(&dir).expect("the scratch dir is created");
    dir
}

fn write(dir: &Path, name: &str, text: &str) -> String {
    let path = dir.join(name);
    std::fs::write(&path, text).expect("the scratch file writes");
    path.to_string_lossy().into_owned()
}

fn run_data(
    total: i64,
    usage: Option<i64>,
    budget: Option<i64>,
    model: Option<&str>,
    task: &str,
    extra: &str,
) -> String {
    let mut text = format!(
        "{{\"task\": \"{}\", \"window_seconds\": 200, \"runner\": \"someone\", \"recorded_at\": \"2026-01-04\", \"splits\": [{{\"phase\": \"only\", \"elapsed_seconds\": {}}}]",
        task, total
    );
    if let Some(usage) = usage {
        text += &format!(", \"usage\": {}", usage);
    }
    if let Some(budget) = budget {
        text += &format!(", \"budget\": {}", budget);
    }
    if let Some(model) = model {
        text += &format!(", \"model\": \"{}\"", model);
    }
    text += extra;
    text += "}";
    text
}

fn field_file(dir: &Path) -> String {
    let runs = [
        run_data(100, Some(40), Some(50), Some("peak"), "t", ""),
        run_data(120, Some(20), Some(50), Some("vale"), "t", ""),
    ];
    write(
        dir,
        "field.json",
        &format!("{{\"runs\": [{}]}}", runs.join(", ")),
    )
}

fn ghost_file(dir: &Path, usage: Option<i64>, task: &str) -> String {
    write(
        dir,
        "ghost.json",
        &run_data(180, usage, None, None, task, ""),
    )
}

#[test]
fn test_season_prints_the_report() {
    let dir = scratch("season-report");
    let field = field_file(&dir);
    let ghost = ghost_file(&dir, Some(45), "t");
    let outcome = cli(&["season", &field, &ghost]);
    assert_eq!(outcome.code, 0);
    assert_eq!(
        outcome.stdout,
        vec![
            "season for task t",
            "model vale",
            "runs raced 1",
            "inside window 1",
            "inside budget 1",
            "total usage 20",
            "mean usage 20",
            "head to head against the ghost, wins 1 of 1",
            "model peak",
            "runs raced 1",
            "inside window 1",
            "inside budget 1",
            "total usage 40",
            "mean usage 40",
            "head to head against the ghost, wins 1 of 1",
        ]
    );
}

#[test]
fn test_season_on_a_malformed_field_prints_its_defects() {
    let dir = scratch("season-bad-field");
    let field = write(&dir, "field.json", "{\"runs\": [{\"task\": \"t\"}]}");
    let ghost = ghost_file(&dir, Some(45), "t");
    let outcome = cli(&["season", &field, &ghost]);
    assert_eq!(outcome.code, 1);
    assert_eq!(outcome.stdout[0], "invalid field");
    assert!(outcome
        .stdout
        .iter()
        .any(|line| line == "defect run 0 missing field splits"));
}

#[test]
fn test_season_on_a_ghost_without_usage_is_refused() {
    let dir = scratch("season-no-usage");
    let field = field_file(&dir);
    let ghost = ghost_file(&dir, None, "t");
    let outcome = cli(&["season", &field, &ghost]);
    assert_eq!(outcome.code, 1);
    assert_eq!(outcome.stdout, vec!["ghost record carries no usage"]);
}

#[test]
fn test_season_on_a_task_mismatch_is_refused() {
    let dir = scratch("season-mismatch");
    let field = field_file(&dir);
    let ghost = ghost_file(&dir, Some(45), "other");
    let outcome = cli(&["season", &field, &ghost]);
    assert_eq!(outcome.code, 1);
    assert_eq!(outcome.stdout, vec!["runs do not share one task"]);
}

fn board_file(dir: &Path) -> String {
    let text = "{\"task\": \"t\", \"window_seconds\": 100, \"runner\": \"someone\", \"recorded_at\": \"2026-01-04\", \"splits\": [{\"phase\": \"only\", \"elapsed_seconds\": 90}], \"usage\": 30, \"budget\": 50, \"reads\": [{\"elapsed_seconds\": 0, \"usage\": 0}, {\"elapsed_seconds\": 50, \"usage\": 10}]}";
    write(dir, "run.json", text)
}

#[test]
fn test_board_prints_the_board() {
    let dir = scratch("board-report");
    let run = board_file(&dir);
    let ghost = ghost_file(&dir, Some(40), "t");
    let outcome = cli(&["board", &run, &ghost]);
    assert_eq!(outcome.code, 0);
    assert_eq!(
        outcome.stdout,
        vec![
            "read 0 at 0 seconds, usage 0, projects no figure",
            "read 1 at 50 seconds, usage 10, projection 20, inside the ghost usage, inside the budget",
            "finish, actual usage 30 against a last projection of 20, inside the ghost usage, inside the budget, the board called it",
        ]
    );
}

#[test]
fn test_board_on_a_run_without_reads_is_refused() {
    let dir = scratch("board-no-reads");
    let run = write(
        &dir,
        "run.json",
        &run_data(90, Some(30), Some(50), None, "t", ""),
    );
    let ghost = ghost_file(&dir, Some(40), "t");
    let outcome = cli(&["board", &run, &ghost]);
    assert_eq!(outcome.code, 1);
    assert_eq!(outcome.stdout, vec!["run carries no timed reads"]);
}

#[test]
fn test_board_on_a_ghost_without_usage_is_refused() {
    let dir = scratch("board-no-usage");
    let run = board_file(&dir);
    let ghost = ghost_file(&dir, None, "t");
    let outcome = cli(&["board", &run, &ghost]);
    assert_eq!(outcome.code, 1);
    assert_eq!(outcome.stdout, vec!["ghost record carries no usage"]);
}

#[test]
fn test_board_on_a_task_mismatch_is_refused() {
    let dir = scratch("board-mismatch");
    let run = board_file(&dir);
    let ghost = ghost_file(&dir, Some(40), "other");
    let outcome = cli(&["board", &run, &ghost]);
    assert_eq!(outcome.code, 1);
    assert_eq!(outcome.stdout, vec!["runs do not share one task"]);
}

#[test]
fn test_board_on_a_malformed_run_prints_its_defects() {
    let dir = scratch("board-bad-run");
    let run = write(&dir, "run.json", "{\"task\": \"t\"}");
    let ghost = ghost_file(&dir, Some(40), "t");
    let outcome = cli(&["board", &run, &ghost]);
    assert_eq!(outcome.code, 1);
    assert_eq!(outcome.stdout[0], "invalid recorded run");
    assert!(outcome
        .stdout
        .iter()
        .any(|line| line == "defect missing field splits"));
}
