//! Tests for the Round 20 command line surface, the
//! quartiles command, and the standing form repair over
//! the whole usage error from Round 19.
//!
//! Written for Round 20. Each pin's failure mode stands
//! stated here pin by pin, not as one blanket claim.
//! Pins 28 to 34 stand in this file. Pin 28 and pin 29
//! are the note's pins of those numbers, the shipped and
//! the constructed example by the command line. The
//! built pin 30, the help pin, discharges the note's pin
//! 31, the command surface, asserting the twenty three
//! name brace list in the top level help, the quartiles
//! help line verbatim, and the quartiles usage line by
//! the command's own help. The built pin 31 rejects a
//! near miss name, a missing file, and too many
//! arguments, and its third part stands as the argument
//! count half of the note's pin 32. The built pin 32 is
//! the standing form repair over the whole usage error,
//! beside Rounds 16 and 19's repair tests. Pin 33 stands
//! as the note's pin 30, the malformed meet by the
//! command line, and pin 34 stands as the closed stdout
//! half of the note's pin 32.
//!
//! Pins 28 to 32 fail before the round, pin 28 and pin
//! 29 by the command's absence, pin 30 by the help text
//! carrying twenty two names and pin 31 by the invalid
//! choice error carrying twenty two names and pin 32 by
//! the usage error carrying twenty two names in its
//! brace list, and pins 33 and 34 fail before the round
//! by the command's absence, the quartiles name standing
//! as an invalid choice on the pre round tree. On the
//! built tree, pin 28 passes under all six mutations of
//! this round, and pin 29 carries the round's one command
//! level mutation failure mode, failing under Q-4 on the
//! constructed example, whose closest lower quartile
//! margin stands at a constructed tie. Pins 30, 31, and
//! 32 pass under all six mutations, no mutation of this
//! round touching the table, the header, the parse, or
//! the usage error. Pin 33 passes under all six mutations
//! by design, the loader standing ahead of every
//! selection, and pin 34 passes under all six by
//! construction, its assertions standing on exit codes
//! alone over the shipped meet, no mutation of this round
//! touching the output stream discipline.

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

const EXAMPLE_FIXTURE: &str = include_str!("../../examples/meet.json");
const CONSTRUCTED_FIXTURE: &str = include_str!("../../examples/constructed-meet.json");

fn scratch(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "paceghost-commands20-{}-{}",
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

#[test]
fn test_the_quartiles_command_over_the_example_meet_by_the_cli() {
    let dir = scratch("quartiles");
    let example = write(&dir, "meet.json", EXAMPLE_FIXTURE);
    let out = run(&["quartiles", path_str(&example)], &[]);
    assert_eq!(out.status, Some(0));
    let text = String::from_utf8(out.stdout).expect("the output is utf8");
    assert_eq!(
        text,
        include_str!("../../examples/meet-quartiles-golden.txt")
    );
}

#[test]
fn test_the_quartiles_command_over_the_constructed_example_by_the_cli() {
    let dir = scratch("quartiles-constructed");
    let example = write(&dir, "constructed-meet.json", CONSTRUCTED_FIXTURE);
    let out = run(&["quartiles", path_str(&example)], &[]);
    assert_eq!(out.status, Some(0));
    let text = String::from_utf8(out.stdout).expect("the output is utf8");
    assert_eq!(
        text,
        include_str!("../../examples/constructed-meet-quartiles-golden.txt")
    );
}

#[test]
fn test_the_help_names_the_new_command_and_its_help_line() {
    let out = run(&["--help"], &[]);
    assert_eq!(out.status, Some(0));
    let text = String::from_utf8(out.stdout).expect("the help is utf8");
    assert!(text.contains("{show,ghost,race,season,board,fieldday,ladder,rematch,meet,table,reign,days,forks,marks,standings,traces,margins,spread,middle,tails,mean,median,quartiles}"), "help names usage, {}", text);
    assert!(text.contains("quartiles"), "help lists quartiles, {}", text);
    assert!(
        text.contains("print the meet quartiles for three or more field days"),
        "help carries the quartiles help line, {}",
        text
    );
    let sub = run(&["quartiles", "--help"], &[]);
    assert_eq!(sub.status, Some(0));
    let sub_text = String::from_utf8(sub.stdout).expect("the help is utf8");
    assert_eq!(
        sub_text.lines().next(),
        Some("usage: paceghost quartiles [-h] file"),
        "help shows the quartiles usage, {}",
        sub_text
    );
}

#[test]
fn test_the_quartiles_command_rejects_a_near_miss_name_and_a_missing_file_and_too_many_arguments() {
    let out = run(&["quartile", "x"], &[]);
    assert_eq!(out.status, Some(2));
    assert_eq!(
        String::from_utf8(out.stderr).expect("the error is utf8"),
        "usage: paceghost [-h]\n                 {show,ghost,race,season,board,fieldday,ladder,rematch,meet,table,reign,days,forks,marks,standings,traces,margins,spread,middle,tails,mean,median,quartiles}\n                 ...\npaceghost: error: argument command: invalid choice: 'quartile' (choose from 'show', 'ghost', 'race', 'season', 'board', 'fieldday', 'ladder', 'rematch', 'meet', 'table', 'reign', 'days', 'forks', 'marks', 'standings', 'traces', 'margins', 'spread', 'middle', 'tails', 'mean', 'median', 'quartiles')\n"
    );
    let dir = scratch("quartiles-missing");
    let missing = dir.join("no-such-meet.json");
    let out = run(&["quartiles", path_str(&missing)], &[]);
    assert_eq!(out.status, Some(1));
    assert_eq!(
        String::from_utf8(out.stdout).expect("the output is utf8"),
        "invalid meet\ndefect file cannot be read\n"
    );
    assert_eq!(
        String::from_utf8(out.stderr).expect("the error is utf8"),
        ""
    );
    let dir = scratch("quartiles-extra");
    let example = write(&dir, "meet.json", EXAMPLE_FIXTURE);
    let out = run(&["quartiles", path_str(&example), "extra"], &[]);
    assert_eq!(out.status, Some(2));
    assert_eq!(
        String::from_utf8(out.stderr).expect("the error is utf8"),
        "usage: paceghost [-h]\n                 {show,ghost,race,season,board,fieldday,ladder,rematch,meet,table,reign,days,forks,marks,standings,traces,margins,spread,middle,tails,mean,median,quartiles}\n                 ...\npaceghost: error: unrecognized arguments: extra\n"
    );
}

// The standing form repair. Round 19's usage error over the
// whole rendering named the subcommand's own position by the
// first option token argparse read after it, the parse
// landing on the file value of the earlier subcommand where
// the filesystem took the shell's redirection as one more
// position. The two halves of this pin stand as one test
// function beside Rounds 16 and 19's repair test. The first
// half is the regression in live form, the bare two argument
// shape of argparse on a synthetic abbreviation run through
// the shipped binary, the dispatcher's returned error text
// on those bytes asserted whole against the text the
// standing form produces. The repair's first form of this
// half asserted that same text over a literal of its own,
// an assertion no binary behaviour could fail, and it
// stands restated in live form here. The
// second half runs the same bytes through the shipped binary
// with its standard error taken to /dev/full, the standing
// oracle for the usage error over the whole rendering, and
// matches on the exit the port's form states for it. The
// /dev/full state is read at runtime and the half skips only
// where its oracle is repaired, the state standing describable
// in the text of this pin where the half states it.

#[test]
fn test_a_usage_error_over_the_whole_usage_renders_it_before_refusing_a_closed_error_stream() {
    // The regression, in live form. The bare two argument
    // shape of argparse on a synthetic abbreviation renders
    // the usage error over the whole usage, and the error
    // text the dispatcher returns on those bytes stands
    // asserted whole. Triple slash stands pasted in the two
    // argument shape the note fixes, as the family's own
    // early Rounds 4 and 5 fixtures' own slash tallies stand
    // untallied beside them.
    let out = run(&["quartile", "x"], &[]);
    assert_eq!(out.status, Some(2));
    assert_eq!(
        String::from_utf8(out.stderr).expect("the error is utf8"),
        "usage: paceghost [-h]\n                 {show,ghost,race,season,board,fieldday,ladder,rematch,meet,table,reign,days,forks,marks,standings,traces,margins,spread,middle,tails,mean,median,quartiles}\n                 ...\npaceghost: error: argument command: invalid choice: 'quartile' (choose from 'show', 'ghost', 'race', 'season', 'board', 'fieldday', 'ladder', 'rematch', 'meet', 'table', 'reign', 'days', 'forks', 'marks', 'standings', 'traces', 'margins', 'spread', 'middle', 'tails', 'mean', 'median', 'quartiles')\n"
    );
    // The shipped half. The same bytes through the shipped
    // binary with its error stream taken to /dev/full, the
    // error stream closed before the error is rendered.
    match fs::metadata("/dev/full") {
        Ok(metadata) if metadata.is_file() => {}
        Ok(_) => {
            let out = run_closed_stderr(&["quartile", "x"]);
            assert_eq!(
                out.exit_code, 2,
                "a full error stream leaves the exit at argparse's refusal, \
                 the error stream's own refusal moving nothing, as the port states"
            );
        }
        Err(_) => {}
    }
}

// Pin 33, the note's pin 30, a malformed meet by the
// command line. The fixture is the malformed discipline
// of Round 19's median pin carried to this round's
// command, one day holding a window of zero and no
// operator and no runs, drawing four defects in the
// loader's own order, the day count first and then the
// day's three in the order the day loader draws them.
// The pin breaks if the quartiles command loads the
// meet by any second loading of its own, its defect
// text parting from the meet command's, or if it
// answers a malformed meet with any status but one.

#[test]
fn test_the_quartiles_command_prints_what_meet_prints_for_a_malformed_meet() {
    let dir = scratch("quartiles-malformed");
    let bad = write(&dir, "meet.json", "{\"days\": [{\"window_seconds\": 0}]}");
    let quartiles = run(&["quartiles", path_str(&bad)], &[]);
    let meet = run(&["meet", path_str(&bad)], &[]);
    assert_eq!(quartiles.status, Some(1));
    assert_eq!(meet.status, Some(1));
    assert_eq!(quartiles.stdout, meet.stdout);
    assert_eq!(
        String::from_utf8(quartiles.stdout).expect("the output is utf8"),
        "invalid meet\ndefect field days holds one day\ndefect day 0 field window_seconds must be positive\ndefect day 0 missing field operator\ndefect day 0 missing field runs\n"
    );
}

// Pin 34, the closed stdout half of the note's pin 32,
// in the standing form of the closed stream pin of
// rust/tests/test_cli_widths.rs and of the closed half
// of Round 19's closing pin. The pin reads the state
// of /dev/full before it opens it. Where the path is
// absent the open would fail on a healthy tree, and
// where the path stands as a regular file the write
// the pin expects to fail succeeds on a healthy tree,
// so the pin stands skipped in both states, named by
// the line it prints, and where the path stands as the
// character device the pin runs. The pin breaks if the
// quartiles command prints its reading through an
// unchecked stream, the failed write at /dev/full
// leaving the exit at zero, or if the command answers
// a sound meet with any status but zero on a healthy
// stream. The argument count half of the note's pin 32
// stands in the built pin 31's third part and is not
// duplicated here.

#[test]
fn test_the_quartiles_command_against_a_closed_output_stream() {
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
    let dir = scratch("quartiles-closed");
    let example = write(&dir, "meet.json", EXAMPLE_FIXTURE);
    let full = fs::File::create("/dev/full").expect("the full device opens");
    let status = paceghost()
        .arg("quartiles")
        .arg(&example)
        .stdout(full)
        .stderr(Stdio::null())
        .status()
        .expect("the binary runs");
    assert_eq!(status.code(), Some(1));
    let out = run(&["quartiles", path_str(&example)], &[]);
    assert_eq!(out.status, Some(0));
}

fn path_str(path: &Path) -> &str {
    path.to_str().expect("the scratch path is utf8")
}

struct ClosedOutput {
    exit_code: i32,
}

fn run_closed_stderr(args: &[&str]) -> ClosedOutput {
    let mut command = paceghost();
    command
        .args(args)
        .stdin(Stdio::null())
        .stdout(Stdio::null());
    #[cfg(unix)]
    {
        use std::os::unix::io::FromRawFd;
        let file = fs::File::open("/dev/full").expect("the full device opens");
        let fd = {
            use std::os::unix::io::AsRawFd;
            file.as_raw_fd()
        };
        let dup = unsafe { libc_dup(fd) };
        let stderr = unsafe { Stdio::from_raw_fd(dup) };
        command.stderr(stderr);
        std::mem::forget(file);
    }
    let status = command
        .status()
        .expect("the binary runs with a closed error stream");
    ClosedOutput {
        exit_code: status.code().unwrap_or(-1),
    }
}

#[cfg(unix)]
unsafe fn libc_dup(fd: i32) -> i32 {
    extern "C" {
        fn dup(oldfd: i32) -> i32;
    }
    dup(fd)
}
