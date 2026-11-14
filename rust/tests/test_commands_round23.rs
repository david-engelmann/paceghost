//! Tests for the Round 24 command line surface, the
//! midrange command.
//!
//! Written for Round 24. Each pin's failure mode stands
//! stated here pin by pin, not as one blanket claim. The
//! five pins of this file fail before the round by the
//! command's absence, the CLI carrying no midrange entry,
//! the midrange name standing as an invalid choice on the
//! pre round tree and the help text carrying twenty five
//! names. On the built tree, the shipped meet pin fails
//! under S-1, S-5, and S-6, and passes under S-2, S-3,
//! and S-4 by design, the shipped meet's fields never
//! passing two figures so its midrange coincides with
//! its median and its mean at every mark, and its closest
//! midrange margin, 90 at the second mark, standing
//! unique. The constructed meet pin fails under S-1, S-5,
//! and S-6, and passes under S-2 and S-3 by design, its
//! fields counting one figure, and it fails under S-4,
//! the constructed meet's closest midrange margin, 50,
//! standing tied at its third and sixth marks, so the
//! loosened scan renames it to the sixth and the pin
//! stands as an S-4 catcher beside pin 8 of the reading
//! file. The malformed meet pin and the command surface
//! pin pass under all six mutations by design. The
//! closed-stream pin passes under all six mutations by
//! design, the edits reaching no stream behaviour, and
//! it carries the /dev/full posture of the design
//! record, skipping under its FIX-PG-13 posture in a
//! regular-file seating and standing decided by the
//! hosted gate there.

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

const EXAMPLE_FIXTURE: &str = include_str!("../../examples/meet.json");
const CONSTRUCTED_FIXTURE: &str = include_str!("../../examples/constructed-meet.json");

fn scratch(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "paceghost-commands23-{}-{}",
        std::process::id(),
        tag
    ));
    fs::create_dir_all(&dir).expect("the scratch directory is made");
    dir
}

fn write(dir: &Path, name: &str, contents: &str) -> PathBuf {
    let path = dir.join(name);
    fs::write(&path, contents).expect("the fixture is written");
    path
}

fn paceghost() -> Command {
    Command::new(env!("CARGO_BIN_EXE_paceghost"))
}

struct Output {
    status: Option<i32>,
    stdout: Vec<u8>,
    stderr: Vec<u8>,
}

fn run(args: &[&str], stdin_bytes: &[u8]) -> Output {
    let mut command = paceghost();
    command
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let mut child = command.spawn().expect("the binary starts");
    child
        .stdin
        .as_mut()
        .expect("stdin is piped")
        .write_all(stdin_bytes)
        .expect("stdin is written");
    let output = child.wait_with_output().expect("the binary finishes");
    Output {
        status: output.status.code(),
        stdout: output.stdout,
        stderr: output.stderr,
    }
}

fn path_str(path: &Path) -> &str {
    path.to_str().expect("the scratch path is utf8")
}

#[test]
fn test_the_midrange_command_over_the_example_meet_by_the_cli() {
    let dir = scratch("midrange");
    let example = write(&dir, "meet.json", EXAMPLE_FIXTURE);
    let out = run(&["midrange", path_str(&example)], &[]);
    assert_eq!(out.status, Some(0));
    let text = String::from_utf8(out.stdout).expect("the output is utf8");
    assert_eq!(
        text,
        include_str!("../../examples/meet-midrange-golden.txt")
    );
}

#[test]
fn test_the_midrange_command_over_the_constructed_example_by_the_cli() {
    let dir = scratch("midrange-constructed");
    let example = write(&dir, "constructed-meet.json", CONSTRUCTED_FIXTURE);
    let out = run(&["midrange", path_str(&example)], &[]);
    assert_eq!(out.status, Some(0));
    let text = String::from_utf8(out.stdout).expect("the output is utf8");
    assert_eq!(
        text,
        include_str!("../../examples/constructed-meet-midrange-golden.txt")
    );
}

// The malformed meet pin. The fixture is the malformed
// discipline of Round 20's commands pin carried to this
// round's command, one day holding a window of zero and
// no operator and no runs, drawing four defects in the
// loader's own order, the day count first and then the
// day's three in the order the day loader draws them. The
// pin breaks if the midrange command loads the meet by
// any second loading of its own, its defect text parting
// from the meet command's, or if it answers a malformed
// meet with any status but one.

#[test]
fn test_the_midrange_command_prints_what_meet_prints_for_a_malformed_meet() {
    let dir = scratch("midrange-malformed");
    let bad = write(&dir, "meet.json", "{\"days\": [{\"window_seconds\": 0}]}");
    let midrange = run(&["midrange", path_str(&bad)], &[]);
    let meet = run(&["meet", path_str(&bad)], &[]);
    assert_eq!(midrange.status, Some(1));
    assert_eq!(meet.status, Some(1));
    assert_eq!(midrange.stdout, meet.stdout);
    assert_eq!(
        String::from_utf8(midrange.stdout).expect("the output is utf8"),
        "invalid meet\ndefect field days holds one day\ndefect day 0 field window_seconds must be positive\ndefect day 0 missing field operator\ndefect day 0 missing field runs\n"
    );
}

#[test]
fn test_the_midrange_command_stands_in_the_command_surface() {
    let out = run(&["--help"], &[]);
    assert_eq!(out.status, Some(0));
    let text = String::from_utf8(out.stdout).expect("the help is utf8");
    assert!(text.contains("{show,ghost,race,season,board,fieldday,ladder,rematch,meet,table,reign,days,forks,marks,standings,traces,margins,spread,middle,tails,mean,median,quartiles,range,midhinge,midrange}"), "help names usage, {}", text);
    assert!(text.contains("midrange"), "help lists midrange, {}", text);
    assert!(
        text.contains("print the meet midrange for three or more field days"),
        "help carries the midrange help line, {}",
        text
    );
    let sub = run(&["midrange", "--help"], &[]);
    assert_eq!(sub.status, Some(0));
    let sub_text = String::from_utf8(sub.stdout).expect("the help is utf8");
    assert_eq!(
        sub_text.lines().next(),
        Some("usage: paceghost midrange [-h] file"),
        "help shows the midrange usage, {}",
        sub_text
    );
    let out = run(&["midrang", "x"], &[]);
    assert_eq!(out.status, Some(2));
    assert_eq!(
        String::from_utf8(out.stderr).expect("the error is utf8"),
        "usage: paceghost [-h]\n                 {show,ghost,race,season,board,fieldday,ladder,rematch,meet,table,reign,days,forks,marks,standings,traces,margins,spread,middle,tails,mean,median,quartiles,range,midhinge,midrange}\n                 ...\npaceghost: error: argument command: invalid choice: 'midrang' (choose from 'show', 'ghost', 'race', 'season', 'board', 'fieldday', 'ladder', 'rematch', 'meet', 'table', 'reign', 'days', 'forks', 'marks', 'standings', 'traces', 'margins', 'spread', 'middle', 'tails', 'mean', 'median', 'quartiles', 'range', 'midhinge', 'midrange')\n"
    );
    let dir = scratch("midrange-missing");
    let missing = dir.join("no-such-meet.json");
    let out = run(&["midrange", path_str(&missing)], &[]);
    assert_eq!(out.status, Some(1));
    assert_eq!(
        String::from_utf8(out.stdout).expect("the output is utf8"),
        "invalid meet\ndefect file cannot be read\n"
    );
    assert_eq!(
        String::from_utf8(out.stderr).expect("the error is utf8"),
        ""
    );
    let dir = scratch("midrange-extra");
    let example = write(&dir, "meet.json", EXAMPLE_FIXTURE);
    let out = run(&["midrange", path_str(&example), "extra"], &[]);
    assert_eq!(out.status, Some(2));
    assert_eq!(
        String::from_utf8(out.stderr).expect("the error is utf8"),
        "usage: paceghost [-h]\n                 {show,ghost,race,season,board,fieldday,ladder,rematch,meet,table,reign,days,forks,marks,standings,traces,margins,spread,middle,tails,mean,median,quartiles,range,midhinge,midrange}\n                 ...\npaceghost: error: unrecognized arguments: extra\n"
    );
}

// The closed stream pin, in the standing form of the
// closed stream pin of test_commands_round22.rs. The
// pin reads the state of /dev/full before it opens it.
// Where the path is absent the open would fail on a
// healthy tree, and where the path stands as a regular
// file the write the pin expects to fail succeeds on a
// healthy tree, so the pin stands skipped in both states,
// named by the line it prints, and where the path stands
// as the character device the pin runs. The pin breaks if
// the midrange command prints its reading through an
// unchecked stream, the failed write at /dev/full leaving
// the exit at zero, or if the command answers a sound
// meet with any status but zero on a healthy stream. The
// local read of this pin carries the environmental
// qualification that the /dev/full state is time varying
// on this machine, and the hosted gate is its deciding
// read.

#[test]
fn test_the_midrange_command_against_a_closed_output_stream() {
    use std::os::unix::fs::FileTypeExt;
    match fs::metadata("/dev/full") {
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
    let dir = scratch("midrange-closed");
    let example = write(&dir, "meet.json", EXAMPLE_FIXTURE);
    let full = fs::File::create("/dev/full").expect("the full device opens");
    let status = paceghost()
        .arg("midrange")
        .arg(&example)
        .stdout(full)
        .stderr(Stdio::null())
        .status()
        .expect("the binary runs");
    assert_eq!(status.code(), Some(1));
    let out = run(&["midrange", path_str(&example)], &[]);
    assert_eq!(out.status, Some(0));
}
