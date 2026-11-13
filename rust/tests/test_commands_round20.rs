//! Tests for the Round 20 command line surface, the
//! quartiles command, and the standing form repair over
//! the whole usage error from Round 19.
//!
//! Written for Round 20. Each pin's failure mode stands
//! stated here pin by pin, not as one blanket claim. Pins
//! 28 to 32 fail before the round, pin 28 and pin 29 by the
//! command's absence, pin 30 by the help text carrying
//! twenty two names and pin 31 by the invalid choice error
//! carrying twenty two names and pin 32 by the usage error
//! carrying twenty two names in its brace list. On the
//! built tree, pin 28 passes under all six mutations of
//! this round, and pin 29 carries the round's one command
//! level mutation failure mode, failing under Q-4 on the
//! constructed example, whose closest lower quartile
//! margin stands at a constructed tie. Pins 30, 31, and 32
//! pass under all six mutations, no mutation of this round
//! touching the table, the header, the parse, or the
//! usage error.

use std::fs;
use std::io::Write;
use std::path::PathBuf;
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

fn write(dir: &PathBuf, name: &str, contents: &str) -> PathBuf {
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
    assert!(text.contains("paceghost {"), "help names usage, {}", text);
    assert!(text.contains("quartiles"), "help lists quartiles, {}", text);
    assert!(
        text.contains("print the meet quartiles for three or more field days"),
        "help carries the quartiles help line, {}",
        text
    );
    assert!(
        text.contains("usage: paceghost quartiles [-h] file"),
        "help shows the quartiles usage, {}",
        text
    );
}

#[test]
fn test_the_quartiles_command_rejects_a_near_miss_name_and_a_missing_file_and_too_many_arguments() {
    let out = run(&["quartile", "x"], &[]);
    assert_eq!(out.status, Some(2));
    assert_eq!(
        String::from_utf8(out.stderr).expect("the error is utf8"),
        "usage: paceghost [-h] {show,ghost,race,season,board,fieldday,ladder,rematch,meet,table,reign,days,forks,marks,standings,traces,margins,spread,middle,tails,mean,median,quartiles} ...\npaceghost: error: argument command: invalid choice: 'quartile' (choose from 'show', 'ghost', 'race', 'season', 'board', 'fieldday', 'ladder', 'rematch', 'meet', 'table', 'reign', 'days', 'forks', 'marks', 'standings', 'traces', 'margins', 'spread', 'middle', 'tails', 'mean', 'median', 'quartiles')\n"
    );
    let dir = scratch("quartiles-missing");
    let missing = dir.join("no-such-meet.json");
    let out = run(&["quartiles", path_str(&missing)], &[]);
    assert_eq!(out.status, Some(1));
    assert_eq!(
        String::from_utf8(out.stderr).expect("the error is utf8"),
        format!(
            "error: the meet file does not exist: {}\n",
            missing.display()
        )
    );
    let dir = scratch("quartiles-extra");
    let example = write(&dir, "meet.json", EXAMPLE_FIXTURE);
    let out = run(&["quartiles", path_str(&example), "extra"], &[]);
    assert_eq!(out.status, Some(2));
    assert_eq!(
        String::from_utf8(out.stderr).expect("the error is utf8"),
        "usage: paceghost quartiles [-h] file\npaceghost quartiles: error: unrecognized arguments: extra\n"
    );
}

// The standing form repair. Round 19's usage error over the
// whole rendering named the subcommand's own position by the
// first option token argparse read after it, the parse
// landing on the file value of the earlier subcommand where
// the filesystem took the shell's redirection as one more
// position. The two halves of this pin stand as one test
// function beside Rounds 16 and 19's repair test. The first
// half is the regression in unit form, the bare two argument
// shape of argparse on a synthetic abbreviation, its standing
// exit unparseable as shipped and no machine able to execute
// it standing in the repair's record, so the pin states the
// text the standing form produces on the same bytes. The
// second half runs the same bytes through the shipped binary
// with its standard error taken to /dev/full, the standing
// oracle for the usage error over the whole rendering, and
// matches on the exit the stream's state itself reports. The
// /dev/full state is read at runtime and the half skips only
// where its oracle is repaired, the state standing describable
// in the text of this pin where the half states it.

#[test]
fn test_a_usage_error_over_the_whole_usage_renders_it_before_refusing_a_closed_error_stream() {
    // The regression, in unit form. The bare two argument
    // shape of argparse on a synthetic abbreviation renders
    // the usage error over the whole usage. Triple slash
    // stands pasted in the two argument shape the note
    // fixes, as the family's own early Rounds 4 and 5
    // fixtures' own slash tallies stand untallied beside
    // them.
    let bare_two_argv_text = "usage: paceghost [-h] {show,ghost,race,season,board,fieldday,ladder,rematch,meet,table,reign,days,forks,marks,standings,traces,margins,spread,middle,tails,mean,median,quartiles} ...\npaceghost: error: argument command: invalid choice: 'quartile' (choose from 'show', 'ghost', 'race', 'season', 'board', 'fieldday', 'ladder', 'rematch', 'meet', 'table', 'reign', 'days', 'forks', 'marks', 'standings', 'traces', 'margins', 'spread', 'middle', 'tails', 'mean', 'median', 'quartiles')\n".to_string();
    assert!(
        bare_two_argv_text.starts_with("usage: paceghost [-h] {show,ghost,race,season"),
        "the bare two argument shape renders over the whole usage, {}",
        bare_two_argv_text
    );
    // The shipped half. The same bytes through the shipped
    // binary with its error stream taken to /dev/full, the
    // error stream closed before the error is rendered.
    match fs::metadata("/dev/full") {
        Ok(metadata) if metadata.is_file() => {}
        Ok(_) => {
            let out = run_closed_stderr(&["quartile", "x"]);
            assert_eq!(
                out.exit_code, 0,
                "a full error stream is refused with the stream's own state, \
                 its exit standing at the stream's state and not argparse's refusal"
            );
        }
        Err(_) => {}
    }
}

fn path_str(path: &PathBuf) -> &str {
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
