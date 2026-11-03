//! Pins for the codepoint scan scope stated in SETUP-3 as amended
//! by FIX-PG-6.
//!
//! Translated from tests/test_scan.py. The scan is run as a
//! subprocess against a planted tree, because the rule is about
//! which files on disk the scan opens, and only a real run against a
//! real tree can pin that. Each planted tree carries the scan script
//! itself under scripts, which is where the scan looks for its own
//! root.

use std::path::{Path, PathBuf};
use std::process::Command;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn plant_tree(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("paceghost-scan-{}-{}", std::process::id(), tag));
    let scripts = dir.join("scripts");
    std::fs::create_dir_all(&scripts).expect("the planted tree is created");
    std::fs::copy(
        root().join("scripts").join("codepoint_scan.py"),
        scripts.join("codepoint_scan.py"),
    )
    .expect("the scan script is planted");
    dir
}

struct ScanResult {
    code: i32,
    stdout: String,
}

fn run_scan(root: &Path) -> ScanResult {
    let output = Command::new("python3")
        .arg(root.join("scripts").join("codepoint_scan.py"))
        .output()
        .expect("the scan runs");
    ScanResult {
        code: output.status.code().expect("the scan exits with a status"),
        stdout: String::from_utf8(output.stdout).expect("the scan prints text"),
    }
}

const EM_DASH: &str = "\u{2014}";
const EN_DASH: &str = "\u{2013}";

#[test]
fn test_a_clean_planted_tree_passes() {
    let root = plant_tree("clean");
    std::fs::write(root.join("notes.md"), "plain text\n").expect("the note writes");
    let result = run_scan(&root);
    assert_eq!(result.code, 0);
    assert!(result.stdout.contains("U+2014=0"));
    assert!(result.stdout.contains("U+2013=0"));
}

#[test]
fn test_a_dash_in_an_extensionless_file_fails_the_scan() {
    let root = plant_tree("extensionless");
    std::fs::write(
        root.join("LICENSE"),
        format!("licence text {} more text\n", EM_DASH),
    )
    .expect("the licence writes");
    let result = run_scan(&root);
    assert_eq!(result.code, 1);
    assert!(result.stdout.contains("U+2014=1"));
}

#[test]
fn test_a_dash_in_a_python_file_fails_the_scan() {
    let root = plant_tree("python");
    std::fs::write(
        root.join("module.py"),
        format!("# a comment {} here\n", EN_DASH),
    )
    .expect("the module writes");
    let result = run_scan(&root);
    assert_eq!(result.code, 1);
    assert!(result.stdout.contains("U+2013=1"));
}

#[test]
fn test_dashes_inside_an_ignored_directory_pass_the_scan() {
    let root = plant_tree("ignored");
    let venv = root.join(".venv");
    std::fs::create_dir_all(&venv).expect("the ignored dir is created");
    std::fs::write(
        venv.join("planted.md"),
        format!("{} and {}\n", EM_DASH, EN_DASH),
    )
    .expect("the planted file writes");
    let result = run_scan(&root);
    assert_eq!(result.code, 0);
    assert!(result.stdout.contains("U+2014=0"));
    assert!(result.stdout.contains("U+2013=0"));
}

#[test]
fn test_a_dash_in_a_rust_file_fails_the_scan() {
    // Added with the port review repairs, by addition only. PORT-9
    // extends the scan scope to the Rust tree.
    let root = plant_tree("rust");
    std::fs::write(
        root.join("module.rs"),
        format!("// a comment {} here\n", EM_DASH),
    )
    .expect("the module writes");
    let result = run_scan(&root);
    assert_eq!(result.code, 1);
    assert!(result.stdout.contains("U+2014=1"));
}
