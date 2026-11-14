//! Tests for the Round 18 command line surface, the mean
//! command beside the twenty standing commands.
//!
//! Written for Round 18 in the shape of the Round 17 command
//! tests. The command reads one meet file and prints the
//! mean reading of PG-38, and a malformed meet or a missing
//! file prints exactly what the meet command prints for the
//! same file, so the reading never depends on a second
//! loading. The usage pin states the argument outcomes of
//! the command at its own surface at both wrong counts in
//! the one pin, and the closed stream pin runs the built
//! binary in the standing form of the closed stream pin of
//! rust/tests/test_cli_widths.rs. Every pin in this file
//! fails before the round by the absence of the command, the
//! mean command not standing on the pre round tree, the
//! golden pin and the two defect pins failing by invalid
//! choice and the usage and closed stream pins by the
//! missing subcommand.

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
        "paceghost-commands18-{}-{}",
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
fn test_the_cli_mean_command_prints_the_shipped_golden() {
    let path = examples_dir()
        .join("meet.json")
        .to_string_lossy()
        .into_owned();
    let outcome = cli(&["mean", &path]);
    assert_eq!(outcome.code, 0);
    assert_eq!(outcome.stdout, golden_lines("meet-mean-golden.txt"));
}

#[test]
fn test_the_cli_mean_command_prints_what_meet_prints_for_a_malformed_meet() {
    let dir = scratch("mean-malformed");
    let path = write_file(&dir, "meet.json", "{\"days\": [{\"window_seconds\": 0}]}");
    let mean = cli(&["mean", &path]);
    let meet = cli(&["meet", &path]);
    assert_eq!(mean.code, 1);
    assert_eq!(mean.stdout, meet.stdout);
    assert_eq!(
        mean.stdout.first().map(String::as_str),
        Some("invalid meet")
    );
}

#[test]
fn test_the_cli_mean_command_at_the_wrong_argument_counts_prints_the_usage_message() {
    let outcome = cli(&["mean"]);
    assert_eq!(outcome.code, 2);
    assert_eq!(outcome.stdout, Vec::<String>::new());
    assert_eq!(
        outcome.stderr,
        vec![
            "usage: paceghost mean [-h] file",
            "paceghost mean: error: the following arguments are required: file",
        ]
    );
    let outcome = cli(&["mean", "a.json", "b.json"]);
    assert_eq!(outcome.code, 2);
    assert_eq!(outcome.stdout, Vec::<String>::new());
    assert_eq!(
        outcome.stderr,
        vec![
            "usage: paceghost [-h]",
            "                 {show,ghost,race,season,board,fieldday,ladder,rematch,meet,table,reign,days,forks,marks,standings,traces,margins,spread,middle,tails,mean,median,quartiles,range,midhinge}",
            "                 ...",
            "paceghost: error: unrecognized arguments: b.json",
        ]
    );
}

#[test]
fn test_the_cli_mean_command_prints_what_meet_prints_for_a_missing_file() {
    let dir = scratch("mean-missing");
    let path = dir.join("absent.json").to_string_lossy().into_owned();
    let mean = cli(&["mean", &path]);
    let meet = cli(&["meet", &path]);
    assert_eq!(mean.code, 1);
    assert_eq!(mean.stdout, meet.stdout);
    assert_eq!(
        mean.stdout,
        vec!["invalid meet", "defect file cannot be read"]
    );
}

#[test]
fn test_the_cli_mean_command_against_a_closed_output_stream_exits_one() {
    // The pin reads the state of /dev/full before it opens
    // it, in the standing form of the closed stream pin of
    // rust/tests/test_cli_widths.rs. Where the path is
    // absent the open would fail on a healthy tree, and
    // where the path stands as a regular file the write the
    // pin expects to fail succeeds on a healthy tree, so
    // the pin stands skipped in both states, named by the
    // line it prints, and where the path stands as the
    // character device the pin runs.
    use std::os::unix::fs::FileTypeExt;
    match std::fs::metadata("/dev/full") {
        Err(_) => {
            println!("skipping the closed stream pin, /dev/full is absent on this machine");
            return;
        }
        Ok(metadata) if !metadata.file_type().is_char_device() => {
            let state = if metadata.file_type().is_file() {
                "a regular file"
            } else {
                "not a character device"
            };
            println!(
                "skipping the closed stream pin, /dev/full is {} on this machine",
                state
            );
            return;
        }
        Ok(_) => {}
    }
    let binary = env!("CARGO_BIN_EXE_paceghost");
    let example = examples_dir().join("meet.json");
    let full = std::fs::File::create("/dev/full").expect("the full device opens");
    let status = std::process::Command::new(binary)
        .arg("mean")
        .arg(&example)
        .stdout(full)
        .stderr(std::process::Stdio::null())
        .status()
        .expect("the binary runs");
    assert_eq!(status.code(), Some(1));
    let output = std::process::Command::new(binary)
        .arg("mean")
        .arg(&example)
        .output()
        .expect("the binary runs");
    assert_eq!(output.status.code(), Some(0));
}
