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
    assert!(read("README.md").contains("Closed perfection rounds, 22 of 100"));
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
    // standing built and in review, so the count stood at one,
    // the slice 8 status line, until the Round 7 closure landed
    // slice 8 and the count returned to zero. Adjusted again
    // with the Round 8 slice, as the named mechanical
    // consequence of slice 9 standing built and in review, so
    // the count stood at one, the slice 9 status line, until
    // the Round 8 closure landed slice 9 and the count
    // returned to zero. Adjusted again with the Round 9 slice,
    // as the named mechanical consequence of slice 10 standing
    // built and in review, so the count stood at one, the
    // slice 10 status line, until the Round 9 closure landed
    // slice 10 and the count returned to zero. Adjusted again
    // with the Round 10 slice, as the named mechanical
    // consequence of slice 11 standing built and in review, so
    // the count stood at one, the slice 11 status line, until
    // the Round 10 closure landed slice 11 and the count
    // returned to zero. Adjusted again with the Round 11
    // slice, as the named mechanical consequence of slice 12
    // standing built and in review, so the count stood at
    // one, the slice 12 status line, until the Round 11
    // closure landed slice 12 and the count returned to zero.
    // Adjusted again with the Round 12 slice, as the named
    // mechanical consequence of slice 13 standing built and
    // in review, so the count stood at one, the slice 13
    // status line, until the Round 12 closure landed slice 13
    // and the count returned to zero. Adjusted again with
    // the Round 13 slice, as the named mechanical consequence
    // of slice 14 standing built and in review, so the count
    // stood at one, the slice 14 status line, until the
    // Round 13 closure landed slice 14 and the count returned
    // to zero. Adjusted again with the Round 14 slice, as
    // the named mechanical consequence of slice 15 standing
    // built and in review, so the count stood at one, the
    // slice 15 status line, until the Round 14 closure landed
    // slice 15 and the count returned to zero. Adjusted
    // again with the Round 15 slice, as the named mechanical
    // consequence of slice 16 standing built and in review,
    // so the count stood at one, the slice 16 status line,
    // until the Round 15 closure landed slice 16 and the
    // count returned to zero. Adjusted again with the
    // Round 16 slice, as the named mechanical consequence
    // of slice 17 standing built and in review, so the count
    // stood at one, the slice 17 status line, until the
    // Round 16 closure landed slice 17 and the count
    // returned to zero. Adjusted again with the
    // Round 17 slice, as the named mechanical consequence
    // of slice 18 standing built and in review, so the count
    // stood at one, the slice 18 status line, until the
    // Round 17 closure landed slice 18 and the count
    // returned to zero. Adjusted again with the
    // Round 18 slice, as the named mechanical consequence
    // of slice 19 standing built and in review, so the count
    // stood at one, the slice 19 status line, until the
    // Round 18 closure landed slice 19 and the count
    // returned to zero. Adjusted again with the
    // Round 19 slice, as the named mechanical consequence
    // of slice 20 standing built and in review, so the count
    // stood at one, the slice 20 status line, until the
    // Round 19 closure landed slice 20 and the count
    // returned to zero. Adjusted again with the
    // Round 20 slice, as the named mechanical consequence
    // of slice 21 standing built and in review, so the count
    // stood at one, the slice 21 status line, until the
    // Round 20 closure landed slice 21 and the count
    // returned to zero. Adjusted again with the
    // Round 21 slice, as the named mechanical consequence
    // of slice 22 standing built and in review, so the count
    // stood at one, the slice 22 status line, until the
    // Round 21 closure landed slice 22 and the count returned
    // to zero. Adjusted again with the
    // Round 22 slice, as the named mechanical consequence
    // of slice 23 standing built and in review, so the count
    // stood at one, the slice 23 status line, until the
    // Round 22 closure landed slice 23 and the count returned
    // to zero.
    assert_eq!(read("README.md").matches("built and in review").count(), 0);
}

#[test]
fn test_readme_reports_the_round_five_status() {
    let lines = readme_lines();
    let standing = "- Contract, PG-11 and PG-12 met. They stand in full form in CONTRACT.md with the Round 5 repair amendment, and Round 5 closed on the merged slice 6 after three reviews, a repair pass, and an approving re-review.";
    assert!(lines.iter().any(|found| found == standing));
    let slice_line = "- Slice 6 has landed. It carries the rematch loader that names every malformed field in a rematch file, the rematch report that reads two field days as one record progression and closes by naming where the record changed hands between the days, and the rematch command, with two synthetic example rematches and their golden reports under examples.";
    assert!(lines.iter().any(|found| found == slice_line));
    assert!(read("README.md").contains("Closed perfection rounds, 22 of 100"));
}

#[test]
fn test_readme_reports_the_round_six_status() {
    let lines = readme_lines();
    let standing = "- Contract, PG-13 and PG-14 met. They stand in full form in CONTRACT.md, and Round 6 closed on the merged slice 7 after three reviews, a repair pass, and an approving re-review.";
    assert!(lines.iter().any(|found| found == standing));
    let slice_line = "- Slice 7 has landed. It carries the meet loader that names every malformed field in a meet file, the meet report that reads three or more field days as one record progression and names the closing holder of each day and the day the standing record was set, and the meet command, with a synthetic example meet and its golden report under examples.";
    assert!(lines.iter().any(|found| found == slice_line));
    assert!(read("README.md").contains("Closed perfection rounds, 22 of 100"));
}

#[test]
fn test_readme_reports_the_round_seven_status() {
    let lines = readme_lines();
    let standing = "- Contract, PG-15 and PG-16 met. They stand in full form in CONTRACT.md, and Round 7 closed on the merged slice 8 after three reviews approving with no findings.";
    assert!(lines.iter().any(|found| found == standing));
    let slice_line = "- Slice 8 has landed. It carries the meet table that reads a meet competitor by competitor over the one record progression of the meet report, naming the lowest standing projection of each competitor and the day and mark at which it was set, the count of marks each competitor held the record after, and their standing projection at the close, and the table command, with the golden meet table of the standing example meet under examples.";
    assert!(lines.iter().any(|found| found == slice_line));
    assert!(read("README.md").contains("Closed perfection rounds, 22 of 100"));
}

#[test]
fn test_readme_reports_the_round_eight_status() {
    let lines = readme_lines();
    let standing = "- Contract, PG-17 and PG-18 met. They stand in full form in CONTRACT.md, and Round 8 closed on the merged slice 9 after three reviews, no repair pass, and the red team condition discharged on executed probes.";
    assert!(lines.iter().any(|found| found == standing));
    let slice_line = "- Slice 9 has landed. It carries the reign reading that reads a meet spell by spell over the one record progression of the meet report, naming each maximal run of consecutive marks one competitor held the record after with the marks it spans and its length, and the count of marks and spells and the longest spell at the close, and the reign command, with the golden reign reading of the standing example meet under examples.";
    assert!(lines.iter().any(|found| found == slice_line));
    assert!(read("README.md").contains("Closed perfection rounds, 22 of 100"));
}

#[test]
fn test_readme_reports_the_round_nine_status() {
    let lines = readme_lines();
    let standing = "- Contract, PG-19 and PG-20 met. They stand in full form in CONTRACT.md, and Round 9 closed on the merged slice 10 after three reviews, no repair pass, and the red team condition discharged on executed probes.";
    assert!(lines.iter().any(|found| found == standing));
    let slice_line = "- Slice 10 has landed. It carries the days reading that reads a meet day by day over the one record progression of the meet report, naming the marks and handovers of each day and the record that stood at its close, and the days command, with the golden days reading of the standing example meet under examples, and the README brought to the fleet README standard, its lede, install, quickstart, and usage sections standing with captured output.";
    assert!(lines.iter().any(|found| found == slice_line));
    assert!(read("README.md").contains("Closed perfection rounds, 22 of 100"));
}

#[test]
fn test_readme_reports_the_round_ten_status() {
    let lines = readme_lines();
    let standing = "- Contract, PG-21 and PG-22 met. They stand in full form in CONTRACT.md under the Round 10 obligations with the FIX-PG-11 repair amendment, and Round 10 closed on the merged slice 11 after three reviews approving with no findings, with the red team's one observation on the field day report carried to a later round as a fencing candidate.";
    assert!(lines.iter().any(|found| found == standing));
    let slice_line = "- Slice 11 has landed. It carries the forks reading that reads a meet competitor by competitor over the one record progression of the meet report, naming every standing each competitor set and the record spells it held and the figures of its close, and the forks command, with the golden forks reading of the standing example meet under examples, and the label fencing closure of the meet readings under FIX-PG-11, and the README opening paragraph repaired of its lede restatement.";
    assert!(lines.iter().any(|found| found == slice_line));
    assert!(read("README.md").contains("Closed perfection rounds, 22 of 100"));
}

#[test]
fn test_readme_reports_the_round_eleven_status() {
    let lines = readme_lines();
    let stated = "- Contract, PG-23 and PG-24 met. They stand in full form in CONTRACT.md under the Round 11 obligations with the FIX-PG-12 repair amendment, and Round 11 closed on the merged slice 12 after three reviews, one standards Minor on a test comment repaired, with the red team's one observation on the task, phase, model, and recorded_at fields carried to a later round as a fencing candidate.";
    assert!(lines.iter().any(|found| found == stated));
    let slice_line = "- Slice 12 has landed. It carries the marks reading that reads a meet mark by mark over the one record progression of the meet report, naming the place of each mark in the whole meet and the competitor who held the record after it and the standing the mark took against the state that stood before it, and the marks command, with the golden marks readings of the standing example meet and of a constructed meet under examples, and the label fencing closure of the field day family and the run readings under FIX-PG-12.";
    assert!(lines.iter().any(|found| found == slice_line));
    assert!(read("README.md").contains("Closed perfection rounds, 22 of 100"));
}

#[test]
fn test_readme_reports_the_round_twelve_status() {
    let lines = readme_lines();
    let stated = "- Contract, PG-25 and PG-26 met. They stand in full form in CONTRACT.md under the Round 12 obligations with the FIX-PG-13 and FIX-PG-14 repair amendments, and Round 12 closed on the merged slice 13 after three reviews, one standards Minor on a test comment repaired.";
    assert!(lines.iter().any(|found| found == stated));
    let slice_line = "- Slice 13 has landed. It carries the standings reading that reads a meet at the close of each day over the one record progression of the meet report, naming every competitor's standing at every day's close and closing each section with the closing line of the meet report for that day, and the standings command, with the golden standings readings of the standing example meet and of a constructed meet under examples, and the file carried fencing closure of the loaders and the summary under FIX-PG-14, and the input forms named at version 1 with the guarded closed stream pin under FIX-PG-13, and the seeded generated fixture family pinned by the closing relations the contract states.";
    assert!(lines.iter().any(|found| found == slice_line));
    assert!(read("README.md").contains("Closed perfection rounds, 22 of 100"));
}

#[test]
fn test_readme_reports_the_round_thirteen_status() {
    let lines = readme_lines();
    let stated = "- Contract, PG-27 and PG-28 met. They stand in full form in CONTRACT.md under the Round 13 obligations with the FIX-PG-15 repair amendment, and Round 13 closed on the merged slice 14 after three reviews approving with no findings, with one stable toolchain lint repair on the traces tests at the landing.";
    assert!(lines.iter().any(|found| found == stated));
    let slice_line = "- Slice 14 has landed. It carries the traces reading that reads a meet competitor by competitor at the close of each day over the one record progression of the meet report, naming every competitor's standing at the close of each day in one section of its own and the closes at which it held a standing, and the traces command, with the golden traces readings of the standing example meet and of a constructed meet under examples, and the summary guard order pins completed under FIX-PG-15, and the seeded generated fixture family extended to the traces reading.";
    assert!(lines.iter().any(|found| found == slice_line));
    assert!(read("README.md").contains("Closed perfection rounds, 22 of 100"));
}

#[test]
fn test_contract_carries_the_round_thirteen_obligations() {
    let text = read("CONTRACT.md");
    for label in ["FIX-PG-15", "PG-27", "PG-28"] {
        assert!(text.contains(label), "the contract carries {}", label);
    }
}

#[test]
fn test_readme_reports_the_round_fourteen_status() {
    let lines = readme_lines();
    let stated = "- Contract, PG-29 and PG-30 met. They stand in full form in CONTRACT.md under the Round 14 obligations, and Round 14 closed on the merged slice 15 after three reviews approving with findings, with the red team's one Minor recorded as a known envelope limit of the shared seconds formatter.";
    assert!(lines.iter().any(|found| found == stated));
    let slice_line = "- Slice 15 has landed. It carries the margins reading that reads a meet mark by mark over the one record progression of the meet report, naming the holder after each mark, the record figure after it, the nearest standing held after it by a competitor other than its holder, and the margin by which the record stands ahead of that nearest standing, and the margins command, with the golden margins readings of the standing example meet and of a constructed meet under examples, and the multi day walk extended in its own pass to record the nearest standing after each mark, and the seeded generated fixture family extended to the margins reading.";
    assert!(lines.iter().any(|found| found == slice_line));
    assert!(read("README.md").contains("Closed perfection rounds, 22 of 100"));
}

#[test]
fn test_contract_carries_the_round_fourteen_obligations() {
    let text = read("CONTRACT.md");
    for label in ["PG-29", "PG-30"] {
        assert!(text.contains(label), "the contract carries {}", label);
    }
}

#[test]
fn test_readme_reports_the_round_fifteen_status() {
    let lines = readme_lines();
    let stated = "- Contract, PG-31 and PG-32 met. They stand in full form in CONTRACT.md under the Round 15 obligations, and Round 15 closed on the merged slice 16 after three reviews approving with findings, with the one Minor standing against the design record and not the slice.";
    assert!(lines.iter().any(|found| found == stated));
    let slice_line = "- Slice 16 has landed. It carries the spread reading that reads a meet mark by mark over the one record progression of the meet report, naming the holder after each mark, the record figure after it, the count of competitors holding a standing after it, the farthest standing held after it by a competitor other than its holder, and the spread by which that farthest standing stands behind the record, and the spread command, with the golden spread readings of the standing example meet and of a constructed meet under examples, and the multi day walk extended in its own pass to record the farthest standing and the field after each mark, and the seeded generated fixture family extended to the spread reading.";
    assert!(lines.iter().any(|found| found == slice_line));
    assert!(read("README.md").contains("Closed perfection rounds, 22 of 100"));
}

#[test]
fn test_contract_carries_the_round_fifteen_obligations() {
    let text = read("CONTRACT.md");
    for label in ["PG-31", "PG-32"] {
        assert!(text.contains(label), "the contract carries {}", label);
    }
}

#[test]
fn test_readme_reports_the_round_sixteen_status() {
    let lines = readme_lines();
    let standing = "- Contract, PG-33 and PG-34 met. They stand in full form in CONTRACT.md under the Round 16 obligations, and Round 16 closed on the merged slice 17 after three reviews, with the standards Minor repaired in the slice and the red team's one Minor standing against the evidence record and not the slice.";
    assert!(lines.iter().any(|found| found == standing));
    let slice_line = "- Slice 17 has landed. It carries the middle reading that reads a meet mark by mark over the one record progression of the meet report, naming the holder after each mark, the record figure after it, the middle standing held after it by a competitor other than its holder, and the middle margin by which the record stands ahead of that middle standing, and the middle command, with the golden middle readings of the standing example meet and of a constructed meet under examples, and the multi day walk extended in its own pass to record the middle standing after each mark, and the seeded generated fixture family extended to the middle reading.";
    assert!(lines.iter().any(|found| found == slice_line));
    assert!(read("README.md").contains("Closed perfection rounds, 22 of 100"));
}

#[test]
fn test_contract_carries_the_round_sixteen_obligations() {
    let text = read("CONTRACT.md");
    for label in ["PG-33", "PG-34"] {
        assert!(text.contains(label), "the contract carries {}", label);
    }
}

#[test]
fn test_readme_reports_the_round_seventeen_status() {
    let lines = readme_lines();
    let standing = "- Contract, PG-35 and PG-36 met. They stand in full form in CONTRACT.md under the Round 17 obligations, and Round 17 closed on the merged slice 18 after three reviews, with the M-6 evidence item closed by execution and its record corrected in the slice, the refuted pin 2 header claim restated in the slice by a disclosed repair commit, and the red team's one Major standing against the build record and not the slice.";
    assert!(lines.iter().any(|found| found == standing));
    let slice_line = "- Slice 18 has landed. It carries the tails reading that reads a meet mark by mark over the one record progression of the meet report, naming the holder after each mark, the record figure after it, the lower tail standing and the upper tail standing held after it by the field behind its holder, and the lower tail margin and the upper tail margin by which the record stands ahead of each tail standing, and the tails command, with the golden tails readings of the standing example meet and of a constructed meet under examples, and the multi day walk extended in its own pass to record the lower tail standing and the upper tail standing after each mark, and the seeded generated fixture family extended to the tails reading.";
    assert!(lines.iter().any(|found| found == slice_line));
    assert!(read("README.md").contains("Closed perfection rounds, 22 of 100"));
}

#[test]
fn test_contract_carries_the_round_seventeen_obligations() {
    let text = read("CONTRACT.md");
    for label in ["PG-35", "PG-36"] {
        assert!(text.contains(label), "the contract carries {}", label);
    }
}

#[test]
fn test_readme_reports_the_round_eighteen_status() {
    let lines = readme_lines();
    let standing = "- Contract, PG-37 and PG-38 met. They stand in full form in CONTRACT.md under the Round 18 obligations, and Round 18 closed on the merged slice 19 after three reviews, with the header postures restated to their executed form and the design plan's README pins landed in the slice.";
    assert!(lines.iter().any(|found| found == standing));
    let slice_line = "- Slice 19 has landed. It carries the mean reading that reads a meet mark by mark over the one record progression of the meet report, naming the holder after each mark, the record figure after it, the mean standing held after it by the field behind its holder, and the mean margin by which the record stands ahead of the mean standing, and the mean command, with the golden mean readings of the standing example meet and of a constructed meet under examples, and the multi day walk extended in its own pass to record the mean standing after each mark, and the seeded generated fixture family extended to the mean reading.";
    assert!(lines.iter().any(|found| found == slice_line));
    assert!(read("README.md").contains("Closed perfection rounds, 22 of 100"));
}

#[test]
fn test_contract_carries_the_round_eighteen_obligations() {
    let text = read("CONTRACT.md");
    for label in ["PG-37", "PG-38"] {
        assert!(text.contains(label), "the contract carries {}", label);
    }
}

#[test]
fn test_readme_reports_the_round_nineteen_status() {
    let lines = readme_lines();
    let standing = "- Contract, PG-39 and PG-40 met. They stand in full form in CONTRACT.md under the Round 19 obligations, and Round 19 closed on the merged slice 20 after three reviews approving with findings, with the pin headers restated to their executed postures and the generated median pin's record comparison repaired in the slice, and the red team's local execution set carried as named debt to the Round 20 design.";
    assert!(lines.iter().any(|found| found == standing));
    let slice_line = "- Slice 20 has landed. It carries the median reading that reads a meet mark by mark over the one record progression of the meet report, naming the holder after each mark, the record figure after it, the median standing held after it by the field behind its holder, and the median margin by which the record stands ahead of the median standing, and the median command, with the golden median readings of the standing example meet and of a constructed meet under examples, and the multi day walk extended in its own pass to record the median standing after each mark, and the seeded generated fixture family extended to the median reading.";
    assert!(lines.iter().any(|found| found == slice_line));
    assert!(read("README.md").contains("Closed perfection rounds, 22 of 100"));
}

#[test]
fn test_contract_carries_the_round_nineteen_obligations() {
    let text = read("CONTRACT.md");
    for label in ["PG-39", "PG-40"] {
        assert!(text.contains(label), "the contract carries {}", label);
    }
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

// The FIX-PG-13 pins, added with the Round 12 slice. The README
// carries the stranger's documents the amendment states, the
// Input files section naming the six forms at version 1 with
// the evolution rule in its short form, the Install sentence
// naming the cargo install route, and the Testing section
// carrying the local gate line and naming the suite's one
// device assumption. Each pin reads the exact line, so a
// README that drops or rewords one fails here.

#[test]
fn test_readme_names_the_input_files_at_version_one() {
    let lines = readme_lines();
    assert!(lines.iter().any(|found| found == "## Input files"));
    let rule = "Paceghost reads six kinds of input file, and CONTRACT.md specifies each one. Every form stands at version 1. Fields the contract does not name are ignored. A new required field, a changed refusal, or a changed defect phrase raises the version of a form, and the defect vocabulary is part of the versioned surface, so a changed defect phrase is a version change and never a silent one. The forms are the recorded run of PG-1 with the Round 2 run extension, the field of PG-5, the field day of the Round 3 section of CONTRACT.md, the ladder of the Round 4 section, the rematch of the Round 5 section, and the meet of the Round 6 section.";
    assert!(lines.iter().any(|found| found == rule));
}

#[test]
fn test_readme_install_names_the_cargo_install_route() {
    let line = "The build leaves the binary at target/release/paceghost. To install the command on your PATH, run cargo install --path . in the clone.";
    assert!(readme_lines().iter().any(|found| found == line));
}

#[test]
fn test_readme_testing_names_the_gate_and_the_device() {
    let lines = readme_lines();
    assert!(lines.iter().any(|found| found == "## Testing"));
    let gate = "The local gate is cargo test, cargo fmt --check, and cargo clippy --all-targets -- -D warnings, run from the repo root. CI runs the same commands.";
    assert!(lines.iter().any(|found| found == gate));
    let device = "One pin in the suite, the closed stream pin in rust/tests/test_cli_widths.rs, opens /dev/full and expects the write to fail. The pin stands skipped on a machine where that path is absent or is not a character device, and it names the state it found in its output when it skips.";
    assert!(lines.iter().any(|found| found == device));
}

#[test]
fn test_contract_carries_the_round_twenty_obligations() {
    let text = read("CONTRACT.md");
    for label in ["PG-41", "PG-42"] {
        assert!(text.contains(label), "the contract carries {}", label);
    }
    let lines: Vec<&str> = text.lines().collect();
    for heading in [
        "## Round 20 obligations, full form",
        "### PG-41 full form, the quartiles reading's inputs",
        "### PG-42 full form, the quartiles reading over a meet",
    ] {
        assert!(
            lines.contains(&heading),
            "the contract carries the heading {}",
            heading
        );
    }
}

#[test]
fn test_readme_reports_the_round_twenty_status() {
    let lines = readme_lines();
    let standing = "- Contract, PG-41 and PG-42 met. They stand in full form in CONTRACT.md under the Round 20 obligations, and Round 20 closed on the merged slice 21 after three reviews, with the red team approving on recomputation of both quartiles goldens byte for byte, the fifth field's fixture restated under the loader rule its design had not modelled, the note's two missing commands pins carried into the slice, and the assertions that passed under their own negation restated to assertions over the binary's own output.";
    assert!(lines.iter().any(|found| found == standing));
    let slice_line = "- Slice 21 has landed. It carries the quartiles reading that reads a meet mark by mark over the one record progression of the meet report, naming the holder after each mark, the record figure after it, the lower quartile standing and the upper quartile standing held after it by the field behind its holder, and the lower quartile margin and the upper quartile margin by which the record stands ahead of each, and the quartiles command, with the golden quartiles readings of the standing example meet and of a constructed meet under examples, and the multi day walk extended in its own pass to record the lower quartile standing and the upper quartile standing after each mark, and the seeded generated fixture family extended to the quartiles reading, and the record prefix repair across the five standing generated pins, and the README Usage sections for the marks, standings, traces, and margins readings and the standing form guard over the Usage block.";
    assert!(lines.iter().any(|found| found == slice_line));
    assert!(read("README.md").contains("Closed perfection rounds, 22 of 100"));
}

#[test]
fn test_the_readme_usage_headings_stand_in_the_commands_order() {
    let text = read("README.md");
    let lines: Vec<&str> = text.lines().collect();
    let usage = lines
        .iter()
        .position(|line| *line == "## Usage")
        .expect("the Usage heading stands");
    let status = lines
        .iter()
        .position(|line| *line == "## Status")
        .expect("the Status heading stands");
    let headings: Vec<String> = lines[usage..status]
        .iter()
        .filter(|line| line.starts_with("### "))
        .map(|line| line.trim_start_matches("### ").to_string())
        .collect();
    let expected: Vec<String> = [
        "show",
        "ghost",
        "race",
        "season",
        "board",
        "fieldday",
        "ladder",
        "rematch",
        "meet",
        "table",
        "reign",
        "days",
        "forks",
        "marks",
        "standings",
        "traces",
        "margins",
        "spread",
        "middle",
        "tails",
        "mean",
        "median",
        "quartiles",
        "range",
    ]
    .iter()
    .map(|name| name.to_string())
    .collect();
    assert_eq!(headings, expected);
    for name in &expected {
        let heading = format!("### {}", name);
        let start = lines
            .iter()
            .position(|line| *line == heading)
            .expect("the command heading stands");
        let fence = lines[start..]
            .iter()
            .position(|line| *line == "```text")
            .expect("the section carries its example block");
        assert!(
            lines[start + fence + 1].starts_with(&format!("$ paceghost {} ", name)),
            "the {} block opens with its invocation",
            name
        );
    }
}

#[test]
fn test_contract_carries_the_round_twenty_one_obligations() {
    let text = read("CONTRACT.md");
    for label in ["PG-43", "PG-44"] {
        assert!(text.contains(label), "the contract carries {}", label);
    }
    let lines: Vec<&str> = text.lines().collect();
    for heading in [
        "## Round 21 obligations, full form",
        "### PG-43 full form, the range reading's inputs",
        "### PG-44 full form, the range reading over a meet",
    ] {
        assert!(
            lines.contains(&heading),
            "the contract carries the heading {}",
            heading
        );
    }
}

#[test]
fn test_readme_reports_the_round_twenty_one_status() {
    let lines = readme_lines();
    let standing = "- Contract, PG-43 and PG-44 met. They stand in full form in CONTRACT.md under the Round 21 obligations, and Round 21 closed on the merged slice 22 after three reviews, with the equality condition of PG-44 restated by the FIX-PG-17 repair amendment and the exact subtraction pin added in the same repair, and both range goldens recomputed byte for byte by the red team.";
    assert!(lines.iter().any(|found| found == standing));
    let slice_line = "- Slice 22 has landed. It carries the range reading that reads a meet mark by mark over the one record progression of the meet report, naming the holder after each mark, the record figure after it, and the interquartile range held after it by the field behind its holder, the upper quartile standing less the lower quartile standing of that field, and the range command, with the golden range readings of the standing example meet and of a constructed meet under examples, and the multi day walk extended in its own pass to record the interquartile range after each mark, and the seeded generated fixture family extended to the range reading, and the README range section, and the README badge set.";
    assert!(lines.iter().any(|found| found == slice_line));
    assert!(read("README.md").contains("Closed perfection rounds, 22 of 100"));
}

#[test]
fn test_readme_carries_the_ci_badge_after_its_opening_paragraph() {
    let text = read("README.md");
    let badge = "[![ci](https://github.com/david-engelmann/paceghost/actions/workflows/ci.yml/badge.svg)](https://github.com/david-engelmann/paceghost/actions/workflows/ci.yml)";
    assert_eq!(text.matches(badge).count(), 1);
    let lines: Vec<&str> = text.lines().collect();
    let opening = lines
        .iter()
        .position(|line| line.starts_with("A recorded run names a task and a runner"))
        .expect("the opening paragraph stands");
    let badge_line = lines
        .iter()
        .position(|line| *line == badge)
        .expect("the badge line stands");
    let block = lines
        .iter()
        .position(|line| *line == "```text")
        .expect("the first captured block stands");
    assert!(
        opening < badge_line && badge_line < block,
        "the badge stands after the opening paragraph and before the first captured block"
    );
    assert!(badge.contains("david-engelmann/paceghost/actions/workflows/ci.yml"));
}

#[test]
fn test_contract_carries_the_round_twenty_two_obligations() {
    let text = read("CONTRACT.md");
    for label in ["PG-45", "PG-46"] {
        assert!(text.contains(label), "the contract carries {}", label);
    }
    let lines: Vec<&str> = text.lines().collect();
    for heading in [
        "## Round 22 obligations, full form",
        "### PG-45 full form, the execution record of the carried mutations and shapes",
        "### PG-46 full form, the probe recompositions and their executions",
    ] {
        assert!(
            lines.contains(&heading),
            "the contract carries the heading {}",
            heading
        );
    }
}

#[test]
fn test_readme_reports_the_round_twenty_two_status() {
    let lines = readme_lines();
    let standing = "- Contract, PG-45 and PG-46 met. They stand in full form in CONTRACT.md under the Round 22 obligations, and Round 22 closed on the merged slice 23 after three reviews, with the execution record of PG-45 and the probe executions of PG-46 standing executed in the red team's report and in the second shape run on the merged tree, and with the middle reading's holder-alone pin passing under R-6 against the design record's computed class, its executed posture standing in the red team record pin by pin, and with the closed-stream pins of the second shape run skipped under their stated posture at run time, the hosted gate standing as their deciding read, and with the standards review approving with findings at two Observations and the spec review approving with findings at one Minor standing against the design record and the red team approving with findings at one Minor against the design record and three Observations, and with CodeRabbit's one Major answered by the executions and its one Minor dismissed with reason on the record.";
    assert!(lines.iter().any(|found| found == standing));
    let slice_line = "- Slice 23 has landed. It carries the execution record of the one unexecuted mutation of the Round 21 family and of the census of the opening tree in both shapes, and the probe recompositions of PG-46 over the three probes of the Round 19 record that stood unexecuted as recorded, with no new reading and no new command.";
    assert!(lines.iter().any(|found| found == slice_line));
    assert!(read("README.md").contains("Closed perfection rounds, 22 of 100"));
}

#[test]
fn test_the_command_surface_keeps_the_twenty_four_commands_in_order_with_midhinge_last() {
    // Restated in place at the Round 23 build commit as a
    // forced restatement, the pin's Round 22 assertion of
    // a surface of twenty four with range last standing
    // falsified by the midhinge landing itself. The pin
    // now asserts the form the landing leaves, the twenty
    // four commands of the earlier rounds standing in
    // their order with range twenty fourth and midhinge
    // appended last of twenty five.
    let text = read("rust/src/cli.rs");
    let lines: Vec<&str> = text.lines().collect();
    let start = lines
        .iter()
        .position(|line| *line == "const COMMANDS: [&str; 25] = [")
        .expect("the command table stands");
    let names: Vec<&str> = lines[start + 1..]
        .iter()
        .take_while(|line| **line != "];")
        .map(|line| line.trim().trim_end_matches(',').trim_matches('"'))
        .collect();
    let expected: Vec<&str> = vec![
        "show",
        "ghost",
        "race",
        "season",
        "board",
        "fieldday",
        "ladder",
        "rematch",
        "meet",
        "table",
        "reign",
        "days",
        "forks",
        "marks",
        "standings",
        "traces",
        "margins",
        "spread",
        "middle",
        "tails",
        "mean",
        "median",
        "quartiles",
        "range",
        "midhinge",
    ];
    assert_eq!(names, expected);
}
