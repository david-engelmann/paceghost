//! Tests for the Round 12 command line surface, the standings
//! command beside the fourteen standing commands, and the
//! dispatch pins of the FIX-PG-14 fencing closure.
//!
//! Written for Round 12 in the shape of the Round 11 command
//! tests. The FIX-PG-14 pins draw the red team's own shapes
//! against the real commands, a run file whose task holds a
//! break against show and ghost, a phase holding a break
//! against race in the ghost file and in the live file, a model
//! and a task holding a break inside a field file against
//! season, and a recorded_at holding a break against show,
//! where it is refused as no date under FIX-PG-4. On the
//! unrepaired code the task, phase, and model files load clean
//! and the forged lines print, so those pins fail there for a
//! real reason.

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
        "paceghost-commands12-{}-{}",
        std::process::id(),
        tag
    ));
    std::fs::create_dir_all(&dir).expect("the scratch dir is created");
    dir
}

fn write_file(dir: &Path, name: &str, text: &str) -> String {
    let path = dir.join(name);
    std::fs::write(&path, text).expect("the scratch file writes");
    path.to_string_lossy().into_owned()
}

fn run_file(task: &str, phase: &str, recorded_at: &str) -> String {
    format!(
        "{{\"task\": \"{}\", \"window_seconds\": 3600, \"runner\": \"operator\", \"recorded_at\": \"{}\", \"splits\": [{{\"phase\": \"{}\", \"elapsed_seconds\": 300}}]}}",
        task, recorded_at, phase
    )
}

fn field_file(model: &str, task: &str) -> String {
    format!(
        "{{\"runs\": [{{\"task\": \"{}\", \"window_seconds\": 200, \"runner\": \"someone\", \"recorded_at\": \"2026-01-04\", \"splits\": [{{\"phase\": \"only\", \"elapsed_seconds\": 100}}], \"usage\": 40, \"budget\": 50, \"model\": \"{}\"}}]}}",
        task, model
    )
}

#[test]
fn test_show_refuses_a_run_whose_task_holds_a_break() {
    let dir = scratch("show-task");
    let path = write_file(
        &dir,
        "run.json",
        &run_file("draft\\nthe summary", "plan", "2026-01-04"),
    );
    let outcome = cli(&["show", &path]);
    assert_eq!(outcome.code, 1);
    assert_eq!(
        outcome.stdout,
        vec![
            "invalid recorded run",
            "defect field task holds a line break"
        ]
    );
}

#[test]
fn test_ghost_refuses_a_run_whose_task_holds_a_break() {
    let dir = scratch("ghost-task");
    let clean = write_file(&dir, "clean.json", &run_file("t", "plan", "2026-01-04"));
    let broken = write_file(
        &dir,
        "broken.json",
        &run_file("draft\\nthe summary", "plan", "2026-01-05"),
    );
    let outcome = cli(&["ghost", &clean, &broken]);
    assert_eq!(outcome.code, 1);
    assert_eq!(
        outcome.stdout,
        vec![
            "invalid recorded run",
            "defect field task holds a line break"
        ]
    );
}

#[test]
fn test_race_refuses_a_phase_break_in_the_ghost_file() {
    let dir = scratch("race-ghost");
    let ghost = write_file(&dir, "ghost.json", &run_file("t", "pla\\nn", "2026-01-04"));
    let live = write_file(&dir, "live.json", &run_file("t", "plan", "2026-01-05"));
    let outcome = cli(&["race", &ghost, &live]);
    assert_eq!(outcome.code, 1);
    assert_eq!(
        outcome.stdout,
        vec![
            "invalid recorded run",
            "defect split 0 field phase holds a line break"
        ]
    );
}

#[test]
fn test_race_refuses_a_phase_break_in_the_live_file() {
    let dir = scratch("race-live");
    let ghost = write_file(&dir, "ghost.json", &run_file("t", "plan", "2026-01-04"));
    let live = write_file(&dir, "live.json", &run_file("t", "pla\\nn", "2026-01-05"));
    let outcome = cli(&["race", &ghost, &live]);
    assert_eq!(outcome.code, 1);
    assert_eq!(
        outcome.stdout,
        vec![
            "invalid recorded run",
            "defect split 0 field phase holds a line break"
        ]
    );
}

#[test]
fn test_season_refuses_a_field_whose_model_holds_a_break() {
    let dir = scratch("season-model");
    let field = write_file(&dir, "field.json", &field_file("pea\\nk", "t"));
    let ghost = write_file(&dir, "ghost.json", &run_file("t", "plan", "2026-01-04"));
    let outcome = cli(&["season", &field, &ghost]);
    assert_eq!(outcome.code, 1);
    assert_eq!(
        outcome.stdout,
        vec![
            "invalid field",
            "defect run 0 field model holds a line break"
        ]
    );
}

#[test]
fn test_season_refuses_a_field_whose_task_holds_a_break() {
    let dir = scratch("season-task");
    let field = write_file(&dir, "field.json", &field_file("peak", "t\\nx"));
    let ghost = write_file(&dir, "ghost.json", &run_file("t", "plan", "2026-01-04"));
    let outcome = cli(&["season", &field, &ghost]);
    assert_eq!(outcome.code, 1);
    assert_eq!(
        outcome.stdout,
        vec![
            "invalid field",
            "defect run 0 field task holds a line break"
        ]
    );
}

#[test]
fn test_show_refuses_a_recorded_at_holding_a_break_as_no_date() {
    let dir = scratch("show-date");
    let path = write_file(&dir, "run.json", &run_file("t", "plan", "2026-01-04\\nx"));
    let outcome = cli(&["show", &path]);
    assert_eq!(outcome.code, 1);
    assert_eq!(
        outcome.stdout,
        vec![
            "invalid recorded run",
            "defect field recorded_at is not an ISO date"
        ]
    );
}

// The standings command pins of PG-26. The command reads one
// meet file and prints the standings reading, and a malformed
// meet prints exactly what the meet command prints for the
// same file, so the reading never depends on a second loading.

fn meet_day(read_usage: i64) -> String {
    format!(
        "{{\"window_seconds\": 1000, \"operator\": \"alpha\", \"runs\": [{{\"runner\": \"alpha\", \"usage\": 600, \"reads\": [{{\"elapsed_seconds\": 100, \"usage\": {}}}]}}]}}",
        read_usage
    )
}

#[test]
fn test_standings_prints_the_standings() {
    let dir = scratch("standings");
    let days = [meet_day(40), meet_day(30), meet_day(20)];
    let path = write_file(
        &dir,
        "meet.json",
        &format!("{{\"days\": [{}]}}", days.join(", ")),
    );
    let outcome = cli(&["standings", &path]);
    assert_eq!(outcome.code, 0);
    assert_eq!(
        outcome.stdout,
        vec![
            "meet standings, 3 days, 1 competitors",
            "day, the first day, standings 1",
            "competitor alpha, standing projection 400 at the close",
            "close of the first day, record held by alpha at a projection of 400",
            "day, the second day, standings 1",
            "competitor alpha, standing projection 300 at the close",
            "close of the second day, record held by alpha at a projection of 300",
            "day, the third day, standings 1",
            "competitor alpha, standing projection 200 at the close",
            "close of the third day, record held by alpha at a projection of 200",
            "standings counts, days 3, competitors 1, standings held 3",
        ]
    );
}

#[test]
fn test_standings_prints_a_malformed_meet_exactly_as_meet_does() {
    let dir = scratch("standings-malformed");
    let path = write_file(&dir, "meet.json", "{\"days\": [{\"window_seconds\": 0}]}");
    let standings = cli(&["standings", &path]);
    let meet = cli(&["meet", &path]);
    assert_eq!(standings.code, 1);
    assert_eq!(standings.stdout, meet.stdout);
    assert_eq!(
        standings.stdout.first().map(String::as_str),
        Some("invalid meet")
    );
}
