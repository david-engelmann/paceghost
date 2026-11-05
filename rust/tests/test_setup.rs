//! Pins for the repo setup obligations in CONTRACT.md.
//!
//! Translated from tests/test_setup.py, with one addition at the
//! end pinning the Rust port status line the port slice adds. Each
//! pin reads a standing file and fails when the setup text drifts
//! away from the contract, because the setup is part of what later
//! rounds read to understand what this repo promises.

use std::path::PathBuf;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn read(rel: &str) -> String {
    std::fs::read_to_string(root().join(rel)).unwrap_or_else(|_| panic!("{} reads", rel))
}

fn readme_lines() -> Vec<String> {
    read("README.md")
        .lines()
        .map(|line| line.to_string())
        .collect()
}

#[test]
fn test_agents_carries_the_canonical_marker() {
    let text = read("AGENTS.md");
    assert!(text.contains("<!-- repo annex below this line -->"));
    assert!(text.contains("39 repo storefront perfection program"));
}

#[test]
fn test_readme_states_the_closed_round_count() {
    assert!(read("README.md").contains("Closed perfection rounds, 6 of 100"));
}

#[test]
fn test_contract_carries_the_setup_obligations() {
    let text = read("CONTRACT.md");
    for label in ["SETUP-1", "SETUP-2", "SETUP-3", "SETUP-4", "PG-1"] {
        assert!(text.contains(label), "the contract carries {}", label);
    }
}

#[test]
fn test_readme_reports_the_contract_status() {
    let expected = "- Contract, SETUP-1 to SETUP-4 and PG-1 to PG-8 met. PG-1 to PG-4 stand in full form in CONTRACT.md with the Round 1 repair amendments, and Round 1 closed on the merged slice 2 after three reviews, a repair pass, and an approving re-review. PG-5 and PG-6 stand in full form with the Round 2 repair amendment.";
    assert!(readme_lines().iter().any(|found| found == expected));
}

#[test]
fn test_readme_reports_the_round_three_status() {
    let lines = readme_lines();
    let standing = "- Contract, PG-7 and PG-8 met. They stand in full form in CONTRACT.md, and Round 3 closed on the merged slice 4 after three reviews, a repair pass, and an approving re-review.";
    assert!(lines.iter().any(|found| found == standing));
    let slice_line = "- Slice 4 has landed. It carries the field day loader that names every malformed field, the stolen record report that walks the record progression of one window across a field of recorded runs, and the fieldday command, with a synthetic example field day and its golden report under examples.";
    assert!(lines.iter().any(|found| found == slice_line));
}

#[test]
fn test_readme_reports_the_round_four_status() {
    let lines = readme_lines();
    let standing = "- Contract, PG-9 and PG-10 met. They stand in full form in CONTRACT.md with the Round 4 repair amendments, and Round 4 closed on the merged slice 5 after three reviews, a repair pass, and an approving re-review.";
    assert!(lines.iter().any(|found| found == standing));
    let slice_line = "- Slice 5 has landed. It carries the ladder loader that names every malformed field in a ladder file, the ladder report that reads one field day as it stood at each window in a list and closes with a summary line over the closing holders, and the ladder command, with a synthetic example ladder and its golden report under examples.";
    assert!(lines.iter().any(|found| found == slice_line));
    // Adjusted with the Round 5 slice, as the named mechanical
    // consequence of the slice standing built and in review. The
    // count stood at one, the slice 6 status line, and returned
    // to zero when the round closed and the line was worded as
    // landed. Adjusted again with the Round 6 slice, as the named
    // mechanical consequence of slice 7 standing built and in
    // review, so the count stood at one, the slice 7 status
    // line, until the Round 6 closure landed slice 7 and the
    // count returned to zero. Adjusted again with the Round 7
    // slice, as the named mechanical consequence of slice 8
    // standing built and in review, so the count stands at one,
    // the slice 8 status line, until the Round 7 closure lands
    // slice 8 and the count returns to zero.
    assert_eq!(read("README.md").matches("built and in review").count(), 1);
}

#[test]
fn test_readme_reports_the_round_five_status() {
    let lines = readme_lines();
    let standing = "- Contract, PG-11 and PG-12 met. They stand in full form in CONTRACT.md with the Round 5 repair amendment, and Round 5 closed on the merged slice 6 after three reviews, a repair pass, and an approving re-review.";
    assert!(lines.iter().any(|found| found == standing));
    let slice_line = "- Slice 6 has landed. It carries the rematch loader that names every malformed field in a rematch file, the rematch report that reads two field days as one record progression and closes by naming where the record changed hands between the days, and the rematch command, with two synthetic example rematches and their golden reports under examples.";
    assert!(lines.iter().any(|found| found == slice_line));
    assert!(read("README.md").contains("Closed perfection rounds, 6 of 100"));
}

#[test]
fn test_readme_reports_the_round_six_status() {
    let lines = readme_lines();
    let standing = "- Contract, PG-13 and PG-14 met. They stand in full form in CONTRACT.md, and Round 6 closed on the merged slice 7 after three reviews, a repair pass, and an approving re-review.";
    assert!(lines.iter().any(|found| found == standing));
    let slice_line = "- Slice 7 has landed. It carries the meet loader that names every malformed field in a meet file, the meet report that reads three or more field days as one record progression and names the closing holder of each day and the day the standing record was set, and the meet command, with a synthetic example meet and its golden report under examples.";
    assert!(lines.iter().any(|found| found == slice_line));
    assert!(read("README.md").contains("Closed perfection rounds, 6 of 100"));
}

#[test]
fn test_readme_reports_the_round_seven_slice() {
    let lines = readme_lines();
    let slice_line = "- Slice 8 is built and in review. It carries the meet table that reads a meet competitor by competitor over the one record progression of the meet report, naming the lowest standing projection of each competitor and the day and mark at which it was set, the count of marks each competitor held the record after, and their standing projection at the close, and the table command, with the golden meet table of the standing example meet under examples.";
    assert!(lines.iter().any(|found| found == slice_line));
    assert!(read("README.md").contains("Closed perfection rounds, 6 of 100"));
}

#[test]
fn test_ci_runs_the_same_gate_as_the_local_gate() {
    let live: Vec<String> = read(".github/workflows/ci.yml")
        .lines()
        .map(|line| line.split('#').next().unwrap_or("").to_string())
        .collect();
    assert!(live.iter().any(|line| line.contains("cargo test")));
    assert!(live.iter().any(|line| line.contains("cargo fmt --check")));
    assert!(live
        .iter()
        .any(|line| line.contains("cargo clippy --all-targets -- -D warnings")));
    // Adjusted with the removal slice of PORT-10, as the named
    // mechanical consequence of the Python gate leaving CI. The
    // two pins that read the Python commands in CI are replaced by
    // pins that read their absence, so the cargo gate stands alone
    // under guard rather than merely unopposed.
    assert!(!live
        .iter()
        .any(|line| line.contains("python3 scripts/codepoint_scan.py")));
    assert!(!live
        .iter()
        .any(|line| line.contains("python3 -m pytest tests -q")));
    assert!(!live.iter().any(|line| line.contains("setup-python")));
}

// Added by the Rust port slice, not a translation, and adjusted
// with the removal slice of PORT-10 to the landed wording. The
// port status line must stand in the README so the programme
// status the port reports is pinned where a reader of the repo
// finds it. The line is worded so the Round 4 pin above, which
// counts the plain phrase the brief suggested zero times across
// the README, still holds.
#[test]
fn test_readme_reports_the_rust_port_status() {
    let line = "- The Rust port has landed. It carries the behavior the Python package carried, checked against the same goldens, and the Python tree stands removed under PORT-10.";
    assert_eq!(
        readme_lines()
            .iter()
            .filter(|found| found.as_str() == line)
            .count(),
        1
    );
}
