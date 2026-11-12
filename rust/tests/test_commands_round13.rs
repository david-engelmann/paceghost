//! Tests for the Round 13 command line surface, the traces
//! command beside the fifteen standing commands.
//!
//! Written for Round 13 in the shape of the Round 12 command
//! tests. The command reads one meet file and prints the
//! traces reading of PG-28, and a malformed meet or a missing
//! file prints exactly what the meet command prints for the
//! same file, so the reading never depends on a second
//! loading. The usage pins state the argument outcomes of the
//! command at its own surface. Every pin in this file fails
//! before the round by the absence of the command, the traces
//! command not standing on the pre round tree.

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
        "paceghost-commands13-{}-{}",
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

fn examples_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples")
}

fn golden_lines(name: &str) -> Vec<String> {
    let golden = std::fs::read_to_string(examples_dir().join(name)).expect("the golden file reads");
    assert!(golden.ends_with('\n'));
    golden.lines().map(str::to_string).collect()
}

#[test]
fn test_traces_prints_the_shipped_golden() {
    let path = examples_dir()
        .join("meet.json")
        .to_string_lossy()
        .into_owned();
    let outcome = cli(&["traces", &path]);
    assert_eq!(outcome.code, 0);
    assert_eq!(outcome.stdout, golden_lines("meet-traces-golden.txt"));
}

#[test]
fn test_traces_prints_a_malformed_meet_exactly_as_meet_does() {
    let dir = scratch("traces-malformed");
    let path = write_file(&dir, "meet.json", "{\"days\": [{\"window_seconds\": 0}]}");
    let traces = cli(&["traces", &path]);
    let meet = cli(&["meet", &path]);
    assert_eq!(traces.code, 1);
    assert_eq!(traces.stdout, meet.stdout);
    assert_eq!(
        traces.stdout.first().map(String::as_str),
        Some("invalid meet")
    );
}

#[test]
fn test_traces_prints_a_missing_file_exactly_as_meet_does() {
    let dir = scratch("traces-missing");
    let path = dir.join("absent.json").to_string_lossy().into_owned();
    let traces = cli(&["traces", &path]);
    let meet = cli(&["meet", &path]);
    assert_eq!(traces.code, 1);
    assert_eq!(traces.stdout, meet.stdout);
    assert_eq!(
        traces.stdout,
        vec!["invalid meet", "defect file cannot be read"]
    );
}

#[test]
fn test_traces_without_its_argument_is_a_usage_outcome() {
    let outcome = cli(&["traces"]);
    assert_eq!(outcome.code, 2);
    assert_eq!(outcome.stdout, Vec::<String>::new());
    assert_eq!(
        outcome.stderr,
        vec![
            "usage: paceghost traces [-h] file",
            "paceghost traces: error: the following arguments are required: file",
        ]
    );
}

#[test]
fn test_traces_with_an_extra_argument_is_a_usage_outcome() {
    let outcome = cli(&["traces", "a.json", "b.json"]);
    assert_eq!(outcome.code, 2);
    assert_eq!(outcome.stdout, Vec::<String>::new());
    assert_eq!(
        outcome.stderr,
        vec![
            "usage: paceghost [-h]",
            "                 {show,ghost,race,season,board,fieldday,ladder,rematch,meet,table,reign,days,forks,marks,standings,traces,margins,spread,middle,tails,mean,median}",
            "                 ...",
            "paceghost: error: unrecognized arguments: b.json",
        ]
    );
}
