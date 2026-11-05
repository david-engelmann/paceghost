//! Pins for the codepoint scan scope stated in SETUP-3 as amended
//! by FIX-PG-6.
//!
//! Translated from tests/test_scan.py, and rewritten with the
//! removal slice of PORT-10 to read the library scan directly
//! instead of running the Python script as a subprocess, because
//! the scan lives in the crate now. The rule is still about which
//! files on disk the scan opens, so each pin plants a real tree
//! and reads the counts the library returns for it.

use std::path::PathBuf;

use paceghost::scan::{report_line, scan_tree};

fn plant_tree(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("paceghost-scan-{}-{}", std::process::id(), tag));
    std::fs::create_dir_all(&dir).expect("the planted tree is created");
    dir
}

const EM_DASH: &str = "\u{2014}";
const EN_DASH: &str = "\u{2013}";

#[test]
fn test_a_clean_planted_tree_passes() {
    let root = plant_tree("clean");
    std::fs::write(root.join("notes.md"), "plain text\n").expect("the note writes");
    let counts = scan_tree(&root);
    assert!(counts.is_clean());
    assert_eq!(counts.em_dashes, 0);
    assert_eq!(counts.en_dashes, 0);
    assert_eq!(report_line(&counts), "codepoint scan U+2014=0 U+2013=0");
}

#[test]
fn test_a_dash_in_an_extensionless_file_fails_the_scan() {
    let root = plant_tree("extensionless");
    std::fs::write(
        root.join("LICENSE"),
        format!("licence text {} more text\n", EM_DASH),
    )
    .expect("the licence writes");
    let counts = scan_tree(&root);
    assert!(!counts.is_clean());
    assert_eq!(counts.em_dashes, 1);
}

#[test]
fn test_a_dash_in_a_python_file_fails_the_scan() {
    let root = plant_tree("python");
    std::fs::write(
        root.join("module.py"),
        format!("# a comment {} here\n", EN_DASH),
    )
    .expect("the module writes");
    let counts = scan_tree(&root);
    assert!(!counts.is_clean());
    assert_eq!(counts.en_dashes, 1);
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
    let counts = scan_tree(&root);
    assert!(counts.is_clean());
    assert_eq!(counts.em_dashes, 0);
    assert_eq!(counts.en_dashes, 0);
}

#[test]
fn test_dashes_inside_a_target_directory_pass_the_scan() {
    // Added with the Round 5 standards repair. A cargo target
    // directory standing inside a scanned tree stands out of
    // scope beside the other ignored directories, so the counts
    // the scan reports never depend on whether the tree was
    // built in place.
    let root = plant_tree("target");
    let target = root.join("target");
    std::fs::create_dir_all(&target).expect("the target dir is created");
    std::fs::write(
        target.join("fingerprint.json"),
        format!("{{\"note\": \"one {} here\"}}\n", EM_DASH),
    )
    .expect("the fingerprint writes");
    std::fs::write(target.join("artifact"), format!("one {} here\n", EN_DASH))
        .expect("the artifact writes");
    let counts = scan_tree(&root);
    assert!(counts.is_clean());
    assert_eq!(counts.em_dashes, 0);
    assert_eq!(counts.en_dashes, 0);
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
    let counts = scan_tree(&root);
    assert!(!counts.is_clean());
    assert_eq!(counts.em_dashes, 1);
}
