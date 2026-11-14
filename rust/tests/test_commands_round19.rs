//! Tests for the Round 19 command line surface, the median
//! command beside the twenty one standing commands.
//!
//! Written for Round 19 in the shape of the Round 18 command
//! tests. The command reads one meet file and prints the
//! median reading of PG-40, and a malformed meet or a missing
//! file prints exactly what the meet command prints for the
//! same file, so the reading never depends on a second
//! loading. The surface pin names the command in the choices
//! of the top level parser, in the top level help, and in
//! its own usage line. The closing pin states the argument
//! outcomes of the command at its own surface at both wrong
//! counts in the one pin and runs the built binary against
//! a closed output stream in the standing form of the closed
//! stream pin of rust/tests/test_cli_widths.rs. Every pin
//! in this file fails before the round by the absence of
//! the command, the median command not standing on the pre
//! round tree, the golden pins and the defect pin failing
//! by invalid choice and the surface and closing pins by
//! the missing subcommand.

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
        "paceghost-commands19-{}-{}",
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
fn test_the_cli_median_command_prints_the_shipped_golden() {
    let path = examples_dir()
        .join("meet.json")
        .to_string_lossy()
        .into_owned();
    let outcome = cli(&["median", &path]);
    assert_eq!(outcome.code, 0);
    assert_eq!(outcome.stdout, golden_lines("meet-median-golden.txt"));
}

#[test]
fn test_the_cli_median_command_prints_the_constructed_golden() {
    let path = examples_dir()
        .join("constructed-meet.json")
        .to_string_lossy()
        .into_owned();
    let outcome = cli(&["median", &path]);
    assert_eq!(outcome.code, 0);
    assert_eq!(
        outcome.stdout,
        golden_lines("constructed-meet-median-golden.txt")
    );
}

#[test]
fn test_the_cli_median_command_prints_what_meet_prints_for_a_malformed_meet() {
    let dir = scratch("median-malformed");
    let path = write_file(&dir, "meet.json", "{\"days\": [{\"window_seconds\": 0}]}");
    let median = cli(&["median", &path]);
    let meet = cli(&["meet", &path]);
    assert_eq!(median.code, 1);
    assert_eq!(median.stdout, meet.stdout);
    assert_eq!(
        median.stdout.first().map(String::as_str),
        Some("invalid meet")
    );
}

#[test]
fn test_the_median_command_stands_in_the_command_surface() {
    let outcome = cli(&["bogus"]);
    assert_eq!(outcome.code, 2);
    assert_eq!(
        outcome.stderr.last().map(String::as_str),
        Some("paceghost: error: argument command: invalid choice: 'bogus' (choose from 'show', 'ghost', 'race', 'season', 'board', 'fieldday', 'ladder', 'rematch', 'meet', 'table', 'reign', 'days', 'forks', 'marks', 'standings', 'traces', 'margins', 'spread', 'middle', 'tails', 'mean', 'median', 'quartiles', 'range', 'midhinge')")
    );
    let outcome = cli(&["--help"]);
    assert_eq!(outcome.code, 0);
    assert!(
        outcome.stdout.iter().any(|line| {
            line.trim_start().starts_with("median")
                && line.contains("print the meet median for three or more field days")
        }),
        "the top level help names the median command and its help line"
    );
    let outcome = cli(&["median", "--help"]);
    assert_eq!(outcome.code, 0);
    assert_eq!(
        outcome.stdout.first().map(String::as_str),
        Some("usage: paceghost median [-h] file")
    );
}

#[test]
fn test_the_cli_median_command_at_the_wrong_argument_counts_and_against_a_closed_output_stream() {
    let outcome = cli(&["median"]);
    assert_eq!(outcome.code, 2);
    assert_eq!(outcome.stdout, Vec::<String>::new());
    assert_eq!(
        outcome.stderr,
        vec![
            "usage: paceghost median [-h] file",
            "paceghost median: error: the following arguments are required: file",
        ]
    );
    let outcome = cli(&["median", "a.json", "b.json"]);
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
    // The closed stream half reads the state of /dev/full
    // before it opens it, in the standing form of the
    // closed stream pin of rust/tests/test_cli_widths.rs.
    // Where the path is absent the open would fail on a
    // healthy tree, and where the path stands as a regular
    // file the write the pin expects to fail succeeds on a
    // healthy tree, so the pin stands skipped in both
    // states, named by the line it prints, and where the
    // path stands as the character device the pin runs.
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
        .arg("median")
        .arg(&example)
        .stdout(full)
        .stderr(std::process::Stdio::null())
        .status()
        .expect("the binary runs");
    assert_eq!(status.code(), Some(1));
    let output = std::process::Command::new(binary)
        .arg("median")
        .arg(&example)
        .output()
        .expect("the binary runs");
    assert_eq!(output.status.code(), Some(0));
}
