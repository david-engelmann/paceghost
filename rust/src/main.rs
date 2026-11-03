//! The paceghost command line binary.
//!
//! A thin shell over the library. It collects the process
//! arguments as raw operating system bytes and decodes them the
//! way the Python surrogateescape reading decodes its arguments,
//! so a file name carrying a byte the UTF-8 reading cannot place
//! still opens, hands the file reading to the library dispatch,
//! prints the outcome lines, and exits with the outcome status.
//! When the output stream closes before the lines are written,
//! the process exits 1, as the Python process dies on its broken
//! pipe. The traceback text the Python prints in that case carries
//! paths from the running machine and is not reproduced, and the
//! parity claimed for the closed stream is the exit status alone,
//! as PORT-8 states.

use std::io::Write;

use paceghost::cli::{decode_os_arg, dispatch};
use paceghost::pytext::PText;

#[cfg(unix)]
fn os_arg_bytes(arg: &std::ffi::OsString) -> Vec<u8> {
    use std::os::unix::ffi::OsStrExt;
    arg.as_os_str().as_bytes().to_vec()
}

#[cfg(not(unix))]
fn os_arg_bytes(arg: &std::ffi::OsString) -> Vec<u8> {
    arg.to_string_lossy().into_owned().into_bytes()
}

#[cfg(unix)]
fn path_from_bytes(bytes: &[u8]) -> std::path::PathBuf {
    use std::os::unix::ffi::OsStringExt;
    std::path::PathBuf::from(std::ffi::OsString::from_vec(bytes.to_vec()))
}

#[cfg(not(unix))]
fn path_from_bytes(bytes: &[u8]) -> std::path::PathBuf {
    std::path::PathBuf::from(String::from_utf8_lossy(bytes).into_owned())
}

fn main() {
    let argv: Vec<PText> = std::env::args_os()
        .skip(1)
        .map(|arg| decode_os_arg(&os_arg_bytes(&arg)))
        .collect();
    let outcome = dispatch(&argv, &|bytes: &[u8]| -> Option<String> {
        let data = std::fs::read(path_from_bytes(bytes)).ok()?;
        String::from_utf8(data).ok()
    });
    let mut code = outcome.code;
    let stdout = std::io::stdout();
    let mut stream = stdout.lock();
    let mut stream_failed = false;
    for line in &outcome.stdout {
        if writeln!(stream, "{}", line).is_err() {
            stream_failed = true;
            break;
        }
    }
    if !stream_failed && stream.flush().is_err() {
        stream_failed = true;
    }
    drop(stream);
    let stderr = std::io::stderr();
    let mut error_stream = stderr.lock();
    for line in &outcome.stderr {
        let _ = writeln!(error_stream, "{}", line);
    }
    let _ = error_stream.flush();
    if stream_failed {
        code = 1;
    }
    std::process::exit(code);
}
