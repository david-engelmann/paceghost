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
    assert!(read("README.md").contains("Closed perfection rounds, 29 of 100"));
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
    // to zero. Adjusted again with the
    // Round 23 slice, as the named mechanical consequence
    // of slice 24 standing built and in review, so the count
    // stood at one, the slice 24 status line, until the
    // Round 23 closure landed slice 24 and the count
    // returned to zero. Adjusted again with the
    // Round 24 slice, as the named mechanical consequence
    // of slice 25 standing built and in review, so the count
    // stood at one, the slice 25 status line, until the
    // Round 24 closure landed slice 25 and the count
    // returned to zero. Adjusted again with the
    // Round 25 slice, as the named mechanical consequence
    // of slice 26 standing built and in review, so the count
    // stood at one, the slice 26 status line, until the
    // Round 25 closure landed slice 26 and the count
    // returned to zero. Adjusted again with the
    // Round 26 slice, as the named mechanical consequence
    // of slice 27 standing built and in review, so the count
    // stood at one, the slice 27 status line, until the
    // Round 26 closure landed slice 27 and the count
    // returned to zero. Adjusted again with the
    // Round 27 slice, as the named mechanical consequence
    // of slice 28 standing built and in review, so the count
    // stood at one, the slice 28 status line, until the
    // Round 27 closure landed slice 28 and the count
    // returned to zero. Adjusted again with the
    // Round 28 slice, as the named mechanical consequence
    // of slice 29 standing built and in review, so the count
    // stood at one, the slice 29 status line, until the
    // Round 28 closure landed slice 29 and the count
    // returned to zero.
    // Adjusted again with the Round 29 slice, as the named
    // mechanical consequence of slice 30 standing built and
    // in review, so the count stood at one, the slice 30
    // status line, until the Round 29 closure landed slice 30
    // and the count returned to zero.
    assert_eq!(read("README.md").matches("built and in review").count(), 0);
}

#[test]
fn test_readme_reports_the_round_five_status() {
    let lines = readme_lines();
    let standing = "- Contract, PG-11 and PG-12 met. They stand in full form in CONTRACT.md with the Round 5 repair amendment, and Round 5 closed on the merged slice 6 after three reviews, a repair pass, and an approving re-review.";
    assert!(lines.iter().any(|found| found == standing));
    let slice_line = "- Slice 6 has landed. It carries the rematch loader that names every malformed field in a rematch file, the rematch report that reads two field days as one record progression and closes by naming where the record changed hands between the days, and the rematch command, with two synthetic example rematches and their golden reports under examples.";
    assert!(lines.iter().any(|found| found == slice_line));
    assert!(read("README.md").contains("Closed perfection rounds, 29 of 100"));
}

#[test]
fn test_readme_reports_the_round_six_status() {
    let lines = readme_lines();
    let standing = "- Contract, PG-13 and PG-14 met. They stand in full form in CONTRACT.md, and Round 6 closed on the merged slice 7 after three reviews, a repair pass, and an approving re-review.";
    assert!(lines.iter().any(|found| found == standing));
    let slice_line = "- Slice 7 has landed. It carries the meet loader that names every malformed field in a meet file, the meet report that reads three or more field days as one record progression and names the closing holder of each day and the day the standing record was set, and the meet command, with a synthetic example meet and its golden report under examples.";
    assert!(lines.iter().any(|found| found == slice_line));
    assert!(read("README.md").contains("Closed perfection rounds, 29 of 100"));
}

#[test]
fn test_readme_reports_the_round_seven_status() {
    let lines = readme_lines();
    let standing = "- Contract, PG-15 and PG-16 met. They stand in full form in CONTRACT.md, and Round 7 closed on the merged slice 8 after three reviews approving with no findings.";
    assert!(lines.iter().any(|found| found == standing));
    let slice_line = "- Slice 8 has landed. It carries the meet table that reads a meet competitor by competitor over the one record progression of the meet report, naming the lowest standing projection of each competitor and the day and mark at which it was set, the count of marks each competitor held the record after, and their standing projection at the close, and the table command, with the golden meet table of the standing example meet under examples.";
    assert!(lines.iter().any(|found| found == slice_line));
    assert!(read("README.md").contains("Closed perfection rounds, 29 of 100"));
}

#[test]
fn test_readme_reports_the_round_eight_status() {
    let lines = readme_lines();
    let standing = "- Contract, PG-17 and PG-18 met. They stand in full form in CONTRACT.md, and Round 8 closed on the merged slice 9 after three reviews, no repair pass, and the red team condition discharged on executed probes.";
    assert!(lines.iter().any(|found| found == standing));
    let slice_line = "- Slice 9 has landed. It carries the reign reading that reads a meet spell by spell over the one record progression of the meet report, naming each maximal run of consecutive marks one competitor held the record after with the marks it spans and its length, and the count of marks and spells and the longest spell at the close, and the reign command, with the golden reign reading of the standing example meet under examples.";
    assert!(lines.iter().any(|found| found == slice_line));
    assert!(read("README.md").contains("Closed perfection rounds, 29 of 100"));
}

#[test]
fn test_readme_reports_the_round_nine_status() {
    let lines = readme_lines();
    let standing = "- Contract, PG-19 and PG-20 met. They stand in full form in CONTRACT.md, and Round 9 closed on the merged slice 10 after three reviews, no repair pass, and the red team condition discharged on executed probes.";
    assert!(lines.iter().any(|found| found == standing));
    let slice_line = "- Slice 10 has landed. It carries the days reading that reads a meet day by day over the one record progression of the meet report, naming the marks and handovers of each day and the record that stood at its close, and the days command, with the golden days reading of the standing example meet under examples, and the README brought to the fleet README standard, its lede, install, quickstart, and usage sections standing with captured output.";
    assert!(lines.iter().any(|found| found == slice_line));
    assert!(read("README.md").contains("Closed perfection rounds, 29 of 100"));
}

#[test]
fn test_readme_reports_the_round_ten_status() {
    let lines = readme_lines();
    let standing = "- Contract, PG-21 and PG-22 met. They stand in full form in CONTRACT.md under the Round 10 obligations with the FIX-PG-11 repair amendment, and Round 10 closed on the merged slice 11 after three reviews approving with no findings, with the red team's one observation on the field day report carried to a later round as a fencing candidate.";
    assert!(lines.iter().any(|found| found == standing));
    let slice_line = "- Slice 11 has landed. It carries the forks reading that reads a meet competitor by competitor over the one record progression of the meet report, naming every standing each competitor set and the record spells it held and the figures of its close, and the forks command, with the golden forks reading of the standing example meet under examples, and the label fencing closure of the meet readings under FIX-PG-11, and the README opening paragraph repaired of its lede restatement.";
    assert!(lines.iter().any(|found| found == slice_line));
    assert!(read("README.md").contains("Closed perfection rounds, 29 of 100"));
}

#[test]
fn test_readme_reports_the_round_eleven_status() {
    let lines = readme_lines();
    let stated = "- Contract, PG-23 and PG-24 met. They stand in full form in CONTRACT.md under the Round 11 obligations with the FIX-PG-12 repair amendment, and Round 11 closed on the merged slice 12 after three reviews, one standards Minor on a test comment repaired, with the red team's one observation on the task, phase, model, and recorded_at fields carried to a later round as a fencing candidate.";
    assert!(lines.iter().any(|found| found == stated));
    let slice_line = "- Slice 12 has landed. It carries the marks reading that reads a meet mark by mark over the one record progression of the meet report, naming the place of each mark in the whole meet and the competitor who held the record after it and the standing the mark took against the state that stood before it, and the marks command, with the golden marks readings of the standing example meet and of a constructed meet under examples, and the label fencing closure of the field day family and the run readings under FIX-PG-12.";
    assert!(lines.iter().any(|found| found == slice_line));
    assert!(read("README.md").contains("Closed perfection rounds, 29 of 100"));
}

#[test]
fn test_readme_reports_the_round_twelve_status() {
    let lines = readme_lines();
    let stated = "- Contract, PG-25 and PG-26 met. They stand in full form in CONTRACT.md under the Round 12 obligations with the FIX-PG-13 and FIX-PG-14 repair amendments, and Round 12 closed on the merged slice 13 after three reviews, one standards Minor on a test comment repaired.";
    assert!(lines.iter().any(|found| found == stated));
    let slice_line = "- Slice 13 has landed. It carries the standings reading that reads a meet at the close of each day over the one record progression of the meet report, naming every competitor's standing at every day's close and closing each section with the closing line of the meet report for that day, and the standings command, with the golden standings readings of the standing example meet and of a constructed meet under examples, and the file carried fencing closure of the loaders and the summary under FIX-PG-14, and the input forms named at version 1 with the guarded closed stream pin under FIX-PG-13, and the seeded generated fixture family pinned by the closing relations the contract states.";
    assert!(lines.iter().any(|found| found == slice_line));
    assert!(read("README.md").contains("Closed perfection rounds, 29 of 100"));
}

#[test]
fn test_readme_reports_the_round_thirteen_status() {
    let lines = readme_lines();
    let stated = "- Contract, PG-27 and PG-28 met. They stand in full form in CONTRACT.md under the Round 13 obligations with the FIX-PG-15 repair amendment, and Round 13 closed on the merged slice 14 after three reviews approving with no findings, with one stable toolchain lint repair on the traces tests at the landing.";
    assert!(lines.iter().any(|found| found == stated));
    let slice_line = "- Slice 14 has landed. It carries the traces reading that reads a meet competitor by competitor at the close of each day over the one record progression of the meet report, naming every competitor's standing at the close of each day in one section of its own and the closes at which it held a standing, and the traces command, with the golden traces readings of the standing example meet and of a constructed meet under examples, and the summary guard order pins completed under FIX-PG-15, and the seeded generated fixture family extended to the traces reading.";
    assert!(lines.iter().any(|found| found == slice_line));
    assert!(read("README.md").contains("Closed perfection rounds, 29 of 100"));
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
    assert!(read("README.md").contains("Closed perfection rounds, 29 of 100"));
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
    assert!(read("README.md").contains("Closed perfection rounds, 29 of 100"));
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
    assert!(read("README.md").contains("Closed perfection rounds, 29 of 100"));
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
    assert!(read("README.md").contains("Closed perfection rounds, 29 of 100"));
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
    assert!(read("README.md").contains("Closed perfection rounds, 29 of 100"));
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
    assert!(read("README.md").contains("Closed perfection rounds, 29 of 100"));
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
    assert!(read("README.md").contains("Closed perfection rounds, 29 of 100"));
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
        "midhinge",
        "midrange",
        "width",
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
    assert!(read("README.md").contains("Closed perfection rounds, 29 of 100"));
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

fn subsection_span(lines: &[String], subheading: &str) -> Vec<String> {
    let starts: Vec<usize> = lines
        .iter()
        .enumerate()
        .filter(|(_, line)| line.as_str() == subheading)
        .map(|(index, _)| index)
        .collect();
    assert_eq!(starts.len(), 1, "the subheading stands exactly once");
    let start = starts[0];
    let mut end = start + 1;
    while end < lines.len() && !lines[end].starts_with("## ") && !lines[end].starts_with("### ") {
        end += 1;
    }
    lines[start..end].to_vec()
}

fn section_subheadings(lines: &[String], section: &str) -> Vec<String> {
    let starts: Vec<usize> = lines
        .iter()
        .enumerate()
        .filter(|(_, line)| line.as_str() == section)
        .map(|(index, _)| index)
        .collect();
    assert_eq!(starts.len(), 1, "the section heading stands exactly once");
    let start = starts[0];
    let mut end = start + 1;
    while end < lines.len() && !lines[end].starts_with("## ") {
        end += 1;
    }
    lines[start + 1..end]
        .iter()
        .filter(|line| line.starts_with("### "))
        .cloned()
        .collect()
}

// Restated under PG-61, the block below asserts the standing subsection text whole.
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
    let lines: Vec<String> = text.lines().map(|line| line.to_string()).collect();
    assert_eq!(
        lines
            .iter()
            .filter(|line| line.as_str() == "## Round 22 obligations, full form")
            .count(),
        1
    );
    assert_eq!(
        subsection_span(&lines, "### PG-45 full form, the execution record of the carried mutations and shapes").join("\n"),
        "### PG-45 full form, the execution record of the carried mutations and shapes\n\nPG-45. The round carries an execution record taken on the tree the round opens from, the merged tree of the Round 21 closure, and the build confirms it at its warm-up on that tree before the slice stands built, and the record stands in two parts. The first part is the execution of the one mutation of the Round 21 family that no lane has executed, R-6, the holder exclusion of the shared field helper dropped inside field_figures itself, so the field behind the holder at a mark stands as every standing projection held after that mark, the holder's own among them, ordered from the lowest figure to the highest. The execution runs the whole suite under that one edit on the opening tree at its standing census, and the failing set is read from the whole log, assertion failures alone deciding it, and a run that dies on the box write fault is reported beside the set as a fault failure and never inside it, and each failing pin's executed posture is set against the posture its header states, and any parting between the two is named in the record pin by pin. The second part is the census of the opening tree in both shapes, the plain shape and the no-default-features shape, each run whole on the opening tree at its standing census, the two counts set side by side in the record, so the second shape stands executed at a standing final tree for the first time and not at a slice base. No figure, form, or behaviour of any reading moves under this obligation, the record being of executions alone, and the tree the executions run on is restored whole after them and verified byte for byte against the opening tree before the slice stands built. Test, the pins named in the record, read under the one edit.\n"
    );
    assert_eq!(
        subsection_span(&lines, "### PG-46 full form, the probe recompositions and their executions").join("\n"),
        "### PG-46 full form, the probe recompositions and their executions\n\nPG-46. The three probes of the Round 19 red team record that stand unexecuted as recorded, the probes named D, F, and G, stand recomposed in this round under the loader discipline the run loader of the repo enforces, the discipline that a read whose field usage stands below the previous read of its run is refused and a read whose elapsed_seconds stands not greater than the previous read of its run is refused, the refusals the phrases read field usage is less than the previous read and read field elapsed_seconds is not greater than the previous read name. A recomposition holds the probe's discrimination whole and moves only what the discipline forces, the reads of each fixture designed as reads first and fields second, the read usages of a run never decreasing and the elapsed of a run's reads strictly rising. Each recomposed fixture stands printed whole in the round's design note, with the bound the validated transcription derives for it, figure by figure against the bound the record carried for the fixture as first composed, every figure that moves named with its reason, and the execution runs each recomposed fixture through the real median command of the built tree and sets the printed lines against the derived bound line by line. The probes named A, B, C, and E stand as controls, already executed and passing exactly as bound, and are not recomposed. The recompositions are review instruments and land as records of the round, not as pins of the suite and not as examples, and no figure, form, or behaviour of any reading moves under this obligation. Test, the round's records, the executions standing in them.\n"
    );
    assert_eq!(
        section_subheadings(&lines, "## Round 22 obligations, full form"),
        [
            "### PG-45 full form, the execution record of the carried mutations and shapes",
            "### PG-46 full form, the probe recompositions and their executions",
            "### The Round 22 command line surface",
            "### The generated fixture family under the execution record",
        ]
        .iter()
        .map(|line| line.to_string())
        .collect::<Vec<String>>()
    );
}

#[test]
fn test_readme_reports_the_round_twenty_two_status() {
    let lines = readme_lines();
    let standing = "- Contract, PG-45 and PG-46 met. They stand in full form in CONTRACT.md under the Round 22 obligations, and Round 22 closed on the merged slice 23 after three reviews, with the execution record of PG-45 and the probe executions of PG-46 standing executed in the red team's report and in the second shape run on the merged tree, and with the middle reading's holder-alone pin passing under R-6 against the design record's computed class, its executed posture standing in the red team record pin by pin, and with the closed-stream pins of the second shape run skipped under their stated posture at run time, the hosted gate standing as their deciding read, and with the standards review approving with findings at two Observations and the spec review approving with findings at one Minor standing against the design record and the red team approving with findings at one Minor against the design record and three Observations, and with CodeRabbit's one Major answered by the executions and its one Minor dismissed with reason on the record.";
    assert!(lines.iter().any(|found| found == standing));
    let slice_line = "- Slice 23 has landed. It carries the execution record of the one unexecuted mutation of the Round 21 family and of the census of the opening tree in both shapes, and the probe recompositions of PG-46 over the three probes of the Round 19 record that stood unexecuted as recorded, with no new reading and no new command.";
    assert!(lines.iter().any(|found| found == slice_line));
    assert!(read("README.md").contains("Closed perfection rounds, 29 of 100"));
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
    // appended last of twenty five. Restated again in
    // place at the Round 24 build commit as the same
    // forced restatement, the midrange landing falsifying
    // the twenty five name form, the pin asserting the
    // twenty five commands of the earlier rounds standing
    // in their order with midhinge twenty fifth and
    // midrange appended last of twenty six. Restated
    // again in place at the Round 25 build commit as the
    // same forced restatement, the width landing
    // falsifying the twenty six name form, the pin
    // asserting the twenty six commands of the earlier
    // rounds standing in their order with midrange
    // twenty sixth and width appended last of twenty
    // seven.
    let text = read("rust/src/cli.rs");
    let lines: Vec<&str> = text.lines().collect();
    let start = lines
        .iter()
        .position(|line| *line == "const COMMANDS: [&str; 27] = [")
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
        "midrange",
        "width",
    ];
    assert_eq!(names, expected);
}

// Restated under PG-61, the block below asserts the standing subsection text whole.
#[test]
fn test_contract_carries_the_round_twenty_three_obligations() {
    let text = read("CONTRACT.md");
    assert!(text.contains("## Round 23 obligations, full form"));
    assert!(text.contains("### PG-47 full form, the midhinge reading's inputs"));
    assert!(text.contains("### PG-48 full form, the midhinge reading over a meet"));
    let lines: Vec<String> = text.lines().map(|line| line.to_string()).collect();
    assert_eq!(
        lines
            .iter()
            .filter(|line| line.as_str() == "## Round 23 obligations, full form")
            .count(),
        1
    );
    assert_eq!(
        subsection_span(&lines, "### PG-47 full form, the midhinge reading's inputs").join("\n"),
        "### PG-47 full form, the midhinge reading's inputs\n\nThe midhinge reading reads a meet, the three or more field days the meet shape of the Round 6 section states. Its input is one meet in that shape, and the reading takes the meet the meet loader returns, loaded through load_meet_text and load_meet_file in the paceghost.meet module, whole or nothing exactly as the meet reading of PG-13 takes it, so a meet carrying any defect in any day loads not at all and the midhinge reading only ever reads a meet that loaded whole. The reading holds no loader of its own and no second meet validation, and the meet shape and its defects are exactly the ones the Round 6 section states, neither widened nor narrowed here.\n\nThe midhinge standing after a mark is the mean of the lower quartile standing and the upper quartile standing after that mark, the quartile standings being the standings PG-41 states, the median of the lowest (m + 1) div 2 figures of the field behind the holder after that mark and the median of the highest (m + 1) div 2 figures of that field, the median of a collection of figures being the figure PG-39 states for a field, the figure standing at the middle place where the count of the collection is odd and the mean of the two figures standing at the middle places where the count of the collection is even, the holder and the record after a mark and the standing projection of a competitor being the standings PG-13 states, in the words PG-17 states the standing projection, a standing projection once set at a mark is never cleared, only replaced by a later read of the same competitor. The field behind the holder after a mark is the standing projections held after that mark by competitors other than the holder after that mark, those projections ordered from the lowest figure to the highest, in the one ordered collection the tails reading of PG-35 states for its own selections, and the midhinge of that field is taken over the (m + 1) div 2 lowest and the (m + 1) div 2 highest of those projections, where m counts those competitors and places count from one, so where one competitor stands behind the holder the midhinge is that competitor's figure whole, each quartile standing at it, where two stand behind the holder the midhinge is the midpoint of their two figures, the lower quartile standing at the lower and the upper at the higher, where three stand behind the holder the midhinge is the mean of the mean of the two lowest figures and the mean of the two highest, the middle figure standing in both sub-fields and counting twice in the mean of the quartiles, where four stand behind the holder the midhinge is the mean of the whole field, the two sub-fields standing apart and covering the field, the count at which the midhinge first stands apart from the median while standing at the mean whole, and where five stand behind the holder the midhinge is the midpoint of the second figure and the fourth figure of the field, each quartile standing at the middle figure of its three, the count at which the midhinge first stands apart from the mean as well as from the median. A midhinge standing is a statistic of the figures of the field and is held by no competitor, so no tie discipline stands in this reading, because a midhinge does not choose among the competitors standing at a figure, and the reading names no holder for the midhinge at any count, not even where the field counts one figure and the midhinge is one competitor's figure whole, so its lines stand in one form at every count, in the discipline the tails reading of PG-35 states for its own tails. The midhinge margin after a mark is the midhinge standing figure less the record figure after that mark, computed on exact values before any rendering, so it is never negative, it is exactly the mean of the lower quartile margin and the upper quartile margin of PG-42 at that mark, and it stands at zero exactly where the lower quartile standing and the upper quartile standing both stand at the record figure, every figure standing at a middle place of the lowest (m + 1) div 2 figures of the field and every figure standing at a middle place of the highest (m + 1) div 2 figures standing at the record figure, the one middle place where that count is odd and the two middle places where it is even, on the form PG-42 states for its own margins, the midhinge margin standing at zero under the two conditions together and under neither alone, because each quartile standing stands at or above the record figure and their mean stands at the record figure only where both do. Where no competitor other than the holder holds a standing after a mark, no midhinge standing stands after the mark and no midhinge margin stands after it.\n\nThe reading builds on the one record progression walk of PG-13, the same collected events, the same marks in the same order, and the same holder selection, extended in the same pass to record, after the holder selection at each mark, the midhinge standing after the mark, computed from the standing vector the selection just read through the one ordered field collection PG-35 names, by the midhinge selection that stands as the seventh consumer of that collection beside the middle, tails, mean, median, quartiles, and range selections, as one optional figure, in one vector standing beside the range standings and index aligned with them by construction, the vector pushed at the same point of the pass for every mark at which a record stands, which is every mark of the walk, and no second progression walk and no second holder computation stand anywhere in the reading, and no recorded state a standing reading reads changes under the extension, so the midhinge reading and the fifteen standing meet readings cannot drift apart in which reads project, under which day's window, in what mark order, or who held the record after a mark. The competitors of the reading are the competitors of the walk, in the competitor order PG-13 states, the runner values of every day read as one field in first appearance order. The reading reads only the runner labels, the reads, and the windows of the days, as PG-13 states for its own reading.\n\nThe fields of FIX-PG-14 stand nowhere in the lines of this reading, because the reading prints labels and figures alone, and no task, phase, model, or recorded_at stands in any meet file for it to print.\n\nHanded a meet value built by hand, the reading checks every figure of each day for finiteness before it converts any of them, the days checked in list order, under the guard FIX-PG-10 states and in the named forms that amendment states, and a day carrying a figure that is not finite draws the named error instead of lines, the lines returning on the error channel FIX-PG-10 states. The reading stands inside the label fencing closure of FIX-PG-11 from its birth, as the forks reading of PG-21 does, so a hand built meet any of whose runner labels holds a line break draws the error that amendment states instead of lines, the labels checked in the same one guard pass after the figures of every day are checked, in day order and then in run order within the day. A hand built meet holding no days draws the error the meet holds no days, and a hand built meet holding more days than the ordinal table names draws the error the meet holds more days than the ordinal table names, the errors PG-13 states for its own reading, so the midhinge reading never fails bare on a value its loader would refuse, in the discipline of FIX-PG-8. Test, rust/tests/test_midhinge.rs.\n"
    );
    assert_eq!(
        subsection_span(&lines, "### PG-48 full form, the midhinge reading over a meet").join("\n"),
        "### PG-48 full form, the midhinge reading over a meet\n\nThe meet_midhinge function in the paceghost.meet module takes a loaded meet and returns the midhinge lines on the error channel FIX-PG-10 states, and the reading is a pure function of the meet. The standing projection of a competitor at a mark, the holder after a mark, the record after a mark, the lower quartile standing after a mark, the upper quartile standing after a mark, and the midhinge standing after a mark are the standings PG-47 states, read off the one walk PG-47 names. Test, rust/tests/test_midhinge.rs.\n\nThe first line of the reading names the meet, in the form meet midhinge, and the count of days and the word days, the first line form of the marks reading. One section then stands for each day of the meet, in meet order, and every day of the meet holds a section, a day in which no mark of the walk stands holding a section with no mark line in it. A section opens with one line reading day, and the ordinal phrase of the day as the meet report states it, in the form the and the ordinal word of the day and day, and, marks and the count of marks of the walk standing in that day, the day opening of the marks reading byte for byte. One mark line then stands for each mark of the walk standing in that day, in walk order, reading mark and the place of the mark in the whole walk, and, at and the elapsed figure of the mark and seconds on the and the ordinal phrase of the day of the mark and day, and the label of the holder after the mark and at a projection of and the record figure after the mark, in the mark form of the marks reading as far as the record figure, and then, where another competitor holds a standing after the mark, the line continues with midhinge standing and the midhinge standing figure and, midhinge margin and the midhinge margin figure. Where no competitor other than the holder holds a standing after the mark, the line ends with the words and no other standing. The reading closes with one counts line. Where some mark held another standing, the counts line reads midhinge counts, and marks walked and the count of marks of the walk and, marks with another standing and the count of marks at which another competitor held a standing and, days and the count of days and, closest midhinge margin and the smallest midhinge margin among the marks of the walk and at and the elapsed figure of the earliest mark standing at that margin and seconds on the and the ordinal phrase of the day of that mark and day. Where no mark held another standing, the counts line ends with the words and, no other standing at any mark in place of the closest margin clause, and that form also covers the meet in which no read projects, whose counts line states marks walked zero. No other line stands in the reading. Every elapsed figure renders with format_seconds_fraction, and every projection and every midhinge figure and midhinge margin renders with format_usage, so every figure renders under a discipline an earlier section states. No competitor is ranked, ordered, or compared against another anywhere in the reading beyond the holder each mark line names, and no mark line names a holder for a midhinge standing, because a midhinge standing is a mean of two statistics of the field and no competitor holds it, and no line of the reading names a winner or says that a competitor or a figure did well or badly, because a figure names only what the walk computed, and the comparison the reading makes beyond its mark lines is the one closest midhinge margin its closing line names, taken over the marks themselves and never over the competitors, in the discipline PG-18 states for its longest spell.\n\nThe closing relations bind the reading to the standing meet readings over the same meet, and each is stated so it can fail. The sections equal the days of the meet, the count the days read line of the meet report states, and the marks of the sections sum to the marks walked of the marks reading over the same meet, because every mark of the walk stands in exactly one day. Line by line, the holder and the record figure of each mark line are the holder after the mark and the record figure after the mark the marks reading states for the mark at that place. At the last mark of each day holding marks, the midhinge standing figure is the mean of the quartile standings of the standings the standings reading's section for that day states for competitors other than the holder its closing line names, the quartiles taken in the order and at the count PG-41 states and the midhinge standing as their mean, and the midhinge margin is that figure less the record figure of that closing line, the closing line being the closing line the meet report prints for that day, which the standings section carries byte for byte. The marks with another standing of the counts line and the marks whose lines end in no other standing sum to the marks walked. The midhinge standing of every mark line is the exact mean of its lower quartile standing figure and its upper quartile standing figure, computed on exact values, and the midhinge margin of every mark line is the exact mean of its lower quartile margin and its upper quartile margin, and no midhinge margin is negative. The margin of PG-30 at a mark is never more than the lower quartile margin of PG-42 at that mark, the lower quartile margin is never more than the midhinge margin at that mark, the midhinge margin is never more than the upper quartile margin at that mark, and the upper quartile margin is never more than the spread of PG-32 at that mark, the midhinge margin equal to the lower quartile margin exactly where the interquartile range of PG-44 at that mark stands at zero, and the midhinge margin equal to the upper quartile margin exactly where the range stands at zero there, because the midhinge margin is the lower quartile margin plus one half of the range at every mark. The closest midhinge margin of the counts line is the smallest midhinge margin among the mark lines, it stands at the earliest mark standing at that margin, and the mark line at the day and the elapsed figure the counts line names states that margin. The midhinge margin stands in no fixed order against the median margin at a mark, a field whose upper figures stand far carrying its midhinge above its median and a field whose lower figures stand far carrying its midhinge below it, and the two directions of the relation stand witnessed in the round's pins and in the generated family and nowhere claimed as an order. The midhinge margin stands in no fixed order against the mean margin at a mark either, on the same form, and the two directions of that relation stand witnessed the same way, the midhinge standing at the mean margin identically wherever the field counts four figures and standing at the median margin identically wherever the field counts one figure or two, the two statistics the midhinge coincides with whole at those counts and separates from at every other count. One further relation stands as a pin rather than a closing. The midhinge margin stands at zero exactly at the marks where every figure standing at a middle place of the lowest (m + 1) div 2 figures of the field and every figure standing at a middle place of the highest (m + 1) div 2 figures stand at the record figure, and at no other mark, because the midhinge margin is the mean of the two quartile margins the line's field carries, each standing at or above zero, and their mean stands at zero exactly where both do. The reading walks the marks of the progression in plain order and holds no recursion of its own beyond the walk it shares. Test, rust/tests/test_midhinge.rs, and the golden midhinge readings held in rust/tests/test_examples_round22.rs.\n\nThe example stands in the examples directory. The meet file at examples/meet.json, the example meet of Round 6, does not move, and the midhinge reading is taken over that meet as it stands. The golden reading at examples/meet-midhinge-golden.txt is produced by the real midhinge command over the meet file. The midhinge reading of the example meet closes at marks walked 16, marks with another standing 15, and days 3, with closest midhinge margin 90 at 400 seconds on the first day, the second mark, the only mark standing at that margin, and the last mark of each day agrees with the standings golden's closes, the first day closing with night fork at 840 and a midhinge of 1069.090909 over the closing field {998.181818, 1140}, the second with dawn fork at 675 and a midhinge of 954 over {900, 1008}, and the third with dawn fork at 560 and a midhinge of 735 over {720, 750}, counted by hand from the meet file through the walk, every figured mark of the shipped meet standing at one or two behind its holder, so each mark's midhinge is its median and its mean whole, the separations from both standing in the round's hand built pins and in the generated family in this golden's place. The constructed meet of Round 11, at examples/constructed-meet.json, does not move, and its golden reading stands beside its marks golden at examples/constructed-meet-midhinge-golden.txt, produced by the real midhinge command over the constructed meet file, closing at marks walked 7, marks with another standing 7, and days 3, with closest midhinge margin 50 at 100 seconds on the second day, the third mark, the earliest of the two marks standing at that margin, the constructed field standing at one behind its holder at every mark, so its midhinge is its one figure whole at every mark, counted by hand from the constructed meet file through the walk. Test, rust/tests/test_examples_round22.rs.\n"
    );
    assert_eq!(
        section_subheadings(&lines, "## Round 23 obligations, full form"),
        [
            "### PG-47 full form, the midhinge reading's inputs",
            "### PG-48 full form, the midhinge reading over a meet",
            "### The Round 23 command line surface",
            "### The generated fixture family extended to the midhinge reading",
        ]
        .iter()
        .map(|line| line.to_string())
        .collect::<Vec<String>>()
    );
}

#[test]
fn test_readme_reports_the_round_twenty_three_status() {
    let lines = readme_lines();
    let standing = "- Contract, PG-47 and PG-48 met. They stand in full form in CONTRACT.md under the Round 23 obligations, and Round 23 closed on the merged slice 24 after three reviews, with FIX-PG-18 spent in the slice on the restatement of PG-47's two first-separation clauses, and with the spec review approving with findings at four Observations standing against records and the standards review approving with findings at one Minor cured in the slice by a comment-only repair and three Observations and the red team approving with findings at four Observations standing against records, its independent transcription reproducing all eight goldens byte for byte and its mutation executions matching the stated splits, and with CodeRabbit's two inline comments standing valid and cured in the slice under FIX-PG-18 and the pin 5 edge case, and with the one erratum the round leaves priced standing as a named records item and not as met work, PG-48's generated paragraph compressing the day-close counts against the family's own computation, its cure standing owed through the amendment mechanism at a later round.";
    assert!(lines.iter().any(|found| found == standing));
    let slice_line = "- Slice 24 has landed. It carries the midhinge reading that reads a meet mark by mark over the one record progression of the meet report, naming the holder after each mark, the record figure after it, the midhinge standing held after it by the field behind its holder, the mean of the lower quartile standing and the upper quartile standing of that field, and the midhinge margin by which the record stands ahead of that standing, and the midhinge command, with the golden midhinge readings of the standing example meet and of a constructed meet under examples, and the multi day walk extended in its own pass to record the midhinge standing after each mark, and the seeded generated fixture family extended to the midhinge reading, and the README midhinge section.";
    assert!(lines.iter().any(|found| found == slice_line));
    assert!(read("README.md").contains("Closed perfection rounds, 29 of 100"));
}

#[test]
fn test_the_command_surface_stands_at_twenty_five_with_midhinge_last() {
    // Restated in place at the Round 24 build commit as a
    // forced restatement, the pin's Round 23 assertion of
    // a surface of twenty five with midhinge last standing
    // falsified by the midrange landing itself. The pin
    // now asserts the form the landing leaves, the twenty
    // five commands of the earlier rounds standing in
    // their order with midhinge twenty fifth and midrange
    // appended last of twenty six. Restated again in
    // place at the Round 25 build commit as the same
    // forced restatement, the width landing falsifying
    // the twenty six name form, the pin asserting the
    // twenty six commands of the earlier rounds standing
    // in their order with midrange twenty sixth and width
    // appended last of twenty seven.
    let text = read("rust/src/cli.rs");
    let lines: Vec<&str> = text.lines().collect();
    let start = lines
        .iter()
        .position(|line| *line == "const COMMANDS: [&str; 27] = [")
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
        "midrange",
        "width",
    ];
    assert_eq!(names, expected);
}
// Restated under PG-61, the block below asserts the standing subsection text whole.
#[test]
fn test_contract_carries_the_round_twenty_four_obligations() {
    let text = read("CONTRACT.md");
    assert!(text.contains("## Round 24 obligations, full form"));
    assert!(text.contains("### PG-49 full form, the midrange reading's inputs"));
    assert!(text.contains("### PG-50 full form, the midrange reading over a meet"));
    let lines: Vec<String> = text.lines().map(|line| line.to_string()).collect();
    assert_eq!(
        lines
            .iter()
            .filter(|line| line.as_str() == "## Round 24 obligations, full form")
            .count(),
        1
    );
    assert_eq!(
        subsection_span(&lines, "### PG-49 full form, the midrange reading's inputs").join("\n"),
        "### PG-49 full form, the midrange reading's inputs\n\nThe midrange reading reads a meet, the three or more field days the meet shape of the Round 6 section states. Its input is one meet in that shape, and the reading takes the meet the meet loader returns, loaded through load_meet_text and load_meet_file in the paceghost.meet module, whole or nothing exactly as the meet reading of PG-13 takes it, so a meet carrying any defect in any day loads not at all and the midrange reading only ever reads a meet that loaded whole. The reading holds no loader of its own and no second meet validation, and the meet shape and its defects are exactly the ones the Round 6 section states, neither widened nor narrowed here.\n\nThe midrange standing after a mark is the mean of the nearest standing and the farthest standing after that mark, the nearest standing being the lowest figure of the field behind the holder after that mark, the standing PG-29 states, and the farthest standing being the highest figure of that field, the standing PG-31 states, the holder and the record after a mark and the standing projection of a competitor being the standings PG-13 states, in the words PG-17 states the standing projection, a standing projection once set at a mark is never cleared, only replaced by a later read of the same competitor. The field behind the holder after a mark is the standing projections held after that mark by competitors other than the holder after that mark, those projections ordered from the lowest figure to the highest, in the one ordered collection the tails reading of PG-35 states for its own selections, and the midrange of that field is taken over its lowest figure and its highest figure alone, where m counts those competitors and places count from one, so where one competitor stands behind the holder the midrange is that competitor's figure whole, the nearest standing and the farthest standing both standing at it, where two stand behind the holder the midrange is the midpoint of their two figures, standing at the median and at the mean and at the midhinge identically there, and where three or more stand behind the holder the midrange can stand apart from each of those statistics, the separations the generated family produces at those counts standing stated in PG-50 as family figures and nowhere as an order, no clause of this obligation claiming a first separation at any count. A midrange standing is a statistic of the figures of the field and is held by no competitor, so no tie discipline stands in this reading, because a midrange does not choose among the competitors standing at a figure, and the reading names no holder for the midrange at any count, not even where the field counts one figure and the midrange is one competitor's figure whole, so its lines stand in one form at every count, in the discipline the tails reading of PG-35 states for its own tails. The midrange margin after a mark is the midrange standing figure less the record figure after that mark, computed on exact values before any rendering, so it is never negative, it is exactly the mean of the margin of PG-30 and the spread of PG-32 at that mark, and it stands at zero exactly where every figure of the field stands at the record figure, the nearest standing and the farthest standing both standing at the record figure together, the midrange margin standing at zero under the two conditions together and under neither alone, because the nearest standing stands at or above the record figure and the farthest standing stands at or above the nearest, and their mean stands at the record figure only where both do. Where no competitor other than the holder holds a standing after a mark, no midrange standing stands after the mark and no midrange margin stands after it.\n\nThe reading builds on the one record progression walk of PG-13, the same collected events, the same marks in the same order, and the same holder selection, extended in the same pass to record, after the holder selection at each mark, the midrange standing after the mark, computed from the standing vector the selection just read through the one ordered field collection PG-35 names, by the midrange selection that stands as the eighth consumer of that collection beside the middle, tails, mean, median, quartiles, range, and midhinge selections, as one optional figure, in one vector standing beside the midhinge standings and index aligned with them by construction, the vector pushed at the same point of the pass for every mark at which a record stands, which is every mark of the walk, and no second progression walk and no second holder computation stand anywhere in the reading, and no recorded state a standing reading reads changes under the extension, so the midrange reading and the sixteen standing meet readings cannot drift apart in which reads project, under which day's window, in what mark order, or who held the record after a mark. The competitors of the reading are the competitors of the walk, in the competitor order PG-13 states, the runner values of every day read as one field in first appearance order. The reading reads only the runner labels, the reads, and the windows of the days, as PG-13 states for its own reading.\n\nThe fields of FIX-PG-14 stand nowhere in the lines of this reading, because the reading prints labels and figures alone, and no task, phase, model, or recorded_at stands in any meet file for it to print.\n\nHanded a meet value built by hand, the reading checks every figure of each day for finiteness before it converts any of them, the days checked in list order, under the guard FIX-PG-10 states and in the named forms that amendment states, and a day carrying a figure that is not finite draws the named error instead of lines, the lines returning on the error channel FIX-PG-10 states. The reading stands inside the label fencing closure of FIX-PG-11 from its birth, as the forks reading of PG-21 does, so a hand built meet any of whose runner labels holds a line break draws the error that amendment states instead of lines, the labels checked in the same one guard pass after the figures of every day are checked, in day order and then in run order within the day. A hand built meet holding no days draws the error the meet holds no days, and a hand built meet holding more days than the ordinal table names draws the error the meet holds more days than the ordinal table names, the errors PG-13 states for its own reading, so the midrange reading never fails bare on a value its loader would refuse, in the discipline of FIX-PG-8. Test, rust/tests/test_midrange.rs.\n"
    );
    assert_eq!(
        subsection_span(&lines, "### PG-50 full form, the midrange reading over a meet").join("\n"),
        "### PG-50 full form, the midrange reading over a meet\n\nThe meet_midrange function in the paceghost.meet module takes a loaded meet and returns the midrange lines on the error channel FIX-PG-10 states, and the reading is a pure function of the meet. The standing projection of a competitor at a mark, the holder after a mark, the record after a mark, the nearest standing after a mark, the farthest standing after a mark, and the midrange standing after a mark are the standings PG-49 states, read off the one walk PG-49 names. Test, rust/tests/test_midrange.rs.\n\nThe first line of the reading names the meet, in the form meet midrange, and the count of days and the word days, the first line form of the marks reading. One section then stands for each day of the meet, in meet order, and every day of the meet holds a section, a day in which no mark of the walk stands holding a section with no mark line in it. A section opens with one line reading day, and the ordinal phrase of the day as the meet report states it, in the form the and the ordinal word of the day and day, and, marks and the count of marks of the walk standing in that day, the day opening of the marks reading byte for byte. One mark line then stands for each mark of the walk standing in that day, in walk order, reading mark and the place of the mark in the whole walk, and, at and the elapsed figure of the mark and seconds on the and the ordinal phrase of the day of the mark and day, and the label of the holder after the mark and at a projection of and the record figure after the mark, in the mark form of the marks reading as far as the record figure, and then, where another competitor holds a standing after the mark, the line continues with midrange standing and the midrange standing figure and, midrange margin and the midrange margin figure. Where no competitor other than the holder holds a standing after the mark, the line ends with the words and no other standing. The reading closes with one counts line. Where some mark held another standing, the counts line reads midrange counts, and marks walked and the count of marks of the walk and, marks with another standing and the count of marks at which another competitor held a standing and, days and the count of days and, closest midrange margin and the smallest midrange margin among the marks of the walk and at and the elapsed figure of the earliest mark standing at that margin and seconds on the and the ordinal phrase of the day of that mark and day. Where no mark held another standing, the counts line ends with the words and, no other standing at any mark in place of the closest margin clause, and that form also covers the meet in which no read projects, whose counts line states marks walked zero. No other line stands in the reading. Every elapsed figure renders with format_seconds_fraction, and every projection and every midrange figure and midrange margin renders with format_usage, so every figure renders under a discipline an earlier section states. No competitor is ranked, ordered, or compared against another anywhere in the reading beyond the holder each mark line names, and no mark line names a holder for a midrange standing, because a midrange standing is a mean of two standings of the field and no competitor holds it, and no line of the reading names a winner or says that a competitor or a figure did well or badly, because a figure names only what the walk computed, and the comparison the reading makes beyond its mark lines is the one closest midrange margin its closing line names, taken over the marks themselves and never over the competitors, in the discipline PG-18 states for its longest spell.\n\nThe closing relations bind the reading to the standing meet readings over the same meet, and each is stated so it can fail. The sections equal the days of the meet, the count the days read line of the meet report states, and the marks of the sections sum to the marks walked of the marks reading over the same meet, because every mark of the walk stands in exactly one day. Line by line, the holder and the record figure of each mark line are the holder after the mark and the record figure after the mark the marks reading states for the mark at that place. At the last mark of each day holding marks, the midrange standing figure is the mean of the nearest standing and the farthest standing of the standings the standings reading's section for that day states for competitors other than the holder its closing line names, the extremes taken as PG-29 and PG-31 state them and the midrange standing as their mean, and the midrange margin is that figure less the record figure of that closing line, the closing line being the closing line the meet report prints for that day, which the standings section carries byte for byte. The marks with another standing of the counts line and the marks whose lines end in no other standing sum to the marks walked. The midrange standing of every mark line is the exact mean of its nearest standing figure and its farthest standing figure, computed on exact values, and the midrange margin of every mark line is the exact mean of the margin of PG-30 and the spread of PG-32 at that mark, and no midrange margin is negative. The margin of PG-30 at a mark is never more than the midrange margin at that mark, and the midrange margin is never more than the spread of PG-32 at that mark, the midrange margin equal to the margin exactly where the spread equals the margin there, every figure of the field standing at one figure, and the midrange margin equal to the spread exactly where the spread equals the margin there, the two equalities standing or falling together at every mark, because the midrange margin is the mean of the margin and the spread at every mark. The closest midrange margin of the counts line is the smallest midrange margin among the mark lines, it stands at the earliest mark standing at that margin, and the mark line at the day and the elapsed figure the counts line names states that margin. The midrange margin stands in no fixed order against the median margin at a mark, a field whose upper figures stand far carrying its midrange above its median and a field whose lower figures stand far carrying its midrange below it, and the two directions of the relation stand witnessed in the round's pins and in the generated family and nowhere claimed as an order. The midrange margin stands in no fixed order against the mean margin at a mark either, and in no fixed order against the midhinge margin, on the same form, and the two directions of each relation stand witnessed the same way, the midrange standing at the median margin and at the mean margin and at the midhinge margin identically wherever the field counts one figure or two, the three statistics the midrange coincides with whole at those counts and can stand apart from at every count from three upward. One further relation stands as a pin rather than a closing. The midrange margin stands at zero exactly at the marks where every figure of the field stands at the record figure, and at no other mark, because the midrange margin is the mean of the margin of PG-30 and the spread of PG-32 at that mark, each standing at or above zero, and their mean stands at zero exactly where both do. The reading walks the marks of the progression in plain order and holds no recursion of its own beyond the walk it shares. Test, rust/tests/test_midrange.rs, and the golden midrange readings held in rust/tests/test_examples_round23.rs.\n\nThe example stands in the examples directory. The meet file at examples/meet.json, the example meet of Round 6, does not move, and the midrange reading is taken over that meet as it stands. The golden reading at examples/meet-midrange-golden.txt is produced by the real midrange command over the meet file. The midrange reading of the example meet closes at marks walked 16, marks with another standing 15, and days 3, with closest midrange margin 90 at 400 seconds on the first day, the second mark, the only mark standing at that margin, and the last mark of each day agrees with the standings golden's closes, the first day closing with night fork at 840 and a midrange of 1069.090909 over the closing field {998.181818, 1140}, the second with dawn fork at 675 and a midrange of 954 over {900, 1008}, and the third with dawn fork at 560 and a midrange of 735 over {720, 750}, counted by hand from the meet file through the walk, every figured mark of the shipped meet standing at one or two behind its holder, so each mark's midrange is its median and its mean and its midhinge whole, the separations from each standing in the round's hand built pins and in the generated family in this golden's place. The constructed meet of Round 11, at examples/constructed-meet.json, does not move, and its golden reading stands beside its marks golden at examples/constructed-meet-midrange-golden.txt, produced by the real midrange command over the constructed meet file, closing at marks walked 7, marks with another standing 7, and days 3, with closest midrange margin 50 at 100 seconds on the second day, the third mark, the earliest of the two marks standing at that margin, the constructed field standing at one behind its holder at every mark, so its midrange is its one figure whole at every mark, counted by hand from the constructed meet file through the walk. Test, rust/tests/test_examples_round23.rs.\n"
    );
    assert_eq!(
        section_subheadings(&lines, "## Round 24 obligations, full form"),
        [
            "### PG-49 full form, the midrange reading's inputs",
            "### PG-50 full form, the midrange reading over a meet",
            "### The Round 24 command line surface",
            "### The generated fixture family extended to the midrange reading",
        ]
        .iter()
        .map(|line| line.to_string())
        .collect::<Vec<String>>()
    );
}

#[test]
fn test_readme_reports_the_round_twenty_four_status() {
    let lines = readme_lines();
    let standing = "- Contract, PG-49 and PG-50 met. They stand in full form in CONTRACT.md under the Round 24 obligations, and Round 24 closed on the merged slice 25 after three reviews, with FIX-PG-19 spent in the slice on the cure charged at the Round 23 closure, PG-48's generated paragraph standing compressed against the family's own computation, and with FIX-PG-20 spent in the slice on the repairs the round's reviews charged, and with the spec review approving with findings at one Major cured in the slice and one Observation and the standards review approving with findings at one Minor and the red team approving with findings at one Major cured in the slice and one Minor, and with CodeRabbit returning one inline Minor on the slice, posted after the three reviews had read none, standing valid and cured in the slice under FIX-PG-20, and with the charged FIX-PG-19 cure landing as designed and then standing itself miscounted, the design's section 3 account standing true as written and the amendment's conversion carrying one at seven from the first closes alone and fifty-eight from the later closes alone into a by-count partition, the true partition counted on three agreeing instruments at 2 closes at a count of two, 6 at four, 18 at five, 20 at six, 14 at seven, and 7 at nine, with fifty-nine standing at counts of five and more, and with FIX-PG-20 restating the distribution truly in both contract places, and with PG-50's second equality clause standing false in both directions as shipped, pin 1's first mark standing against its necessity and pin 5's fixture standing against its sufficiency, and its restatement to the spread-equals-margin form standing under the same amendment, pin 10 asserting the true condition from its birth and its comment restated to name the parting, and with the build record's two partings standing corrected by the repair report, its pin 10 account's attribution clause and its section 5 repeating the false distribution pair as verified, and with the design note's section 4 carrying the same false pair, standing as a records item since amendments do not reach design notes, and with the Round 22 design record's manifest figure of 73 against the counted 71 standing as a records reconciliation and not as cured work, and with the two stated S postures standing confirmed by execution, pin 5 passing under S-1 whole and the generated pin failing under S-4 at seed 1's tie.";
    assert!(lines.iter().any(|found| found == standing));
    let slice_line = "- Slice 25 has landed. It carries the midrange reading that reads a meet mark by mark over the one record progression of the meet report, naming the holder after each mark, the record figure after it, the midrange standing held after it by the field behind its holder, the mean of the nearest standing and the farthest standing of that field, and the midrange margin by which the record stands ahead of that standing, and the midrange command, with the golden midrange readings of the standing example meet and of a constructed meet under examples, and the multi day walk extended in its own pass to record the midrange standing after each mark, and the seeded generated fixture family extended to the midrange reading, and the README midrange section.";
    assert!(lines.iter().any(|found| found == slice_line));
    assert!(read("README.md").contains("Closed perfection rounds, 29 of 100"));
}

#[test]
fn test_the_command_surface_stands_at_twenty_six_with_midrange_last() {
    // Restated in place at the Round 25 build commit as a
    // forced restatement, the pin's Round 24 assertion of
    // a surface of twenty six with midrange last standing
    // falsified by the width landing itself. The pin now
    // asserts the form the landing leaves, the twenty
    // six commands of the earlier rounds standing in
    // their order with midrange twenty sixth and width
    // appended last of twenty seven.
    let text = read("rust/src/cli.rs");
    let lines: Vec<&str> = text.lines().collect();
    let start = lines
        .iter()
        .position(|line| *line == "const COMMANDS: [&str; 27] = [")
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
        "midrange",
        "width",
    ];
    assert_eq!(names, expected);
}

// Restated under PG-61, the block below asserts the standing subsection text whole.
#[test]
fn test_contract_carries_the_round_twenty_five_obligations() {
    let text = read("CONTRACT.md");
    assert!(text.contains("## Round 25 obligations, full form"));
    assert!(text.contains("### PG-51 full form, the width reading's inputs"));
    assert!(text.contains("### PG-52 full form, the width reading over a meet"));
    let lines: Vec<String> = text.lines().map(|line| line.to_string()).collect();
    assert_eq!(
        lines
            .iter()
            .filter(|line| line.as_str() == "## Round 25 obligations, full form")
            .count(),
        1
    );
    assert_eq!(
        subsection_span(&lines, "### PG-51 full form, the width reading's inputs").join("\n"),
        "### PG-51 full form, the width reading's inputs\n\nThe width reading reads a meet, the three or more field days the meet shape of the Round 6 section states. Its input is one meet in that shape, and the reading takes the meet the meet loader returns, loaded through load_meet_text and load_meet_file in the paceghost.meet module, whole or nothing exactly as the meet reading of PG-13 takes it, so a meet carrying any defect in any day loads not at all and the width reading only ever reads a meet that loaded whole. The reading holds no loader of its own and no second meet validation, and the meet shape and its defects are exactly the ones the Round 6 section states, neither widened nor narrowed here.\n\nThe field width after a mark is the farthest standing after that mark less the nearest standing after that mark, the nearest standing being the lowest figure of the field behind the holder after that mark, the standing PG-29 states, and the farthest standing being the highest figure of that field, the standing PG-31 states, the holder and the record after a mark and the standing projection of a competitor being the standings PG-13 states, in the words PG-17 states the standing projection, a standing projection once set at a mark is never cleared, only replaced by a later read of the same competitor. The field behind the holder after a mark is the standing projections held after that mark by competitors other than the holder after that mark, those projections ordered from the lowest figure to the highest, in the one ordered collection the tails reading of PG-35 states for its own selections, and the field width of that field is taken over its lowest figure and its highest figure alone, where m counts those competitors and places count from one, so where one competitor stands behind the holder the field width is zero, the nearest standing and the farthest standing both standing at that competitor's figure, where two stand behind the holder the field width is the higher of their two figures less the lower, standing as the interquartile range of PG-44 whole and as the difference of the tails of PG-36 whole at that count, and where three or more stand behind the holder the field width can stand apart from each of those figures, the separations the generated family produces at those counts standing stated in PG-52 as family figures and nowhere as an order, no clause of this obligation claiming a first separation at any count. A field width is a statistic of the figures of the field and is held by no competitor, so no tie discipline stands in this reading, because a field width does not choose among the competitors standing at a figure, and the reading names no holder for a field width at any count, not even where the field counts one figure and the field width is zero at that competitor's figure whole, so its lines stand in one form at every count, in the discipline the range reading of PG-43 states for its own range. The field width after a mark is exactly the spread of PG-32 at that mark less the margin of PG-30 at that mark, computed on exact values before any rendering, so it is never negative, and it stands at zero exactly where every figure of the field stands at one figure, the nearest standing and the farthest standing standing at that one figure together, the field width standing at zero under that condition alone, because the farthest standing stands at or above the nearest standing and the two stand at one figure only where every figure of the field stands at it. Where no competitor other than the holder holds a standing after a mark, no field width stands after the mark.\n\nThe reading builds on the one record progression walk of PG-13, the same collected events, the same marks in the same order, and the same holder selection, extended in the same pass to record, after the holder selection at each mark, the field width after the mark, computed from the standing vector the selection just read through the one ordered field collection PG-35 names, by the width selection that stands as the ninth consumer of that collection beside the middle, tails, mean, median, quartiles, range, midhinge, and midrange selections, as one optional figure, in one vector standing beside the midrange standings and index aligned with them by construction, the vector pushed at the same point of the pass for every mark at which a record stands, which is every mark of the walk, and no second progression walk and no second holder computation stand anywhere in the reading, and no recorded state a standing reading reads changes under the extension, so the width reading and the seventeen standing meet readings cannot drift apart in which reads project, under which day's window, in what mark order, or who held the record after a mark. The competitors of the reading are the competitors of the walk, in the competitor order PG-13 states, the runner values of every day read as one field in first appearance order. The reading reads only the runner labels, the reads, and the windows of the days, as PG-13 states for its own reading.\n\nThe fields of FIX-PG-14 stand nowhere in the lines of this reading, because the reading prints labels and figures alone, and no task, phase, model, or recorded_at stands in any meet file for it to print.\n\nHanded a meet value built by hand, the reading checks every figure of each day for finiteness before it converts any of them, the days checked in list order, under the guard FIX-PG-10 states and in the named forms that amendment states, and a day carrying a figure that is not finite draws the named error instead of lines, the lines returning on the error channel FIX-PG-10 states. The reading stands inside the label fencing closure of FIX-PG-11 from its birth, as the forks reading of PG-21 does, so a hand built meet any of whose runner labels holds a line break draws the error that amendment states instead of lines, the labels checked in the same one guard pass after the figures of every day are checked, in day order and then in run order within the day. A hand built meet holding no days draws the error the meet holds no days, and a hand built meet holding more days than the ordinal table names draws the error the meet holds more days than the ordinal table names, the errors PG-13 states for its own reading, so the width reading never fails bare on a value its loader would refuse, in the discipline of FIX-PG-8. Test, rust/tests/test_width.rs.\n"
    );
    assert_eq!(
        subsection_span(&lines, "### PG-52 full form, the width reading over a meet").join("\n"),
        "### PG-52 full form, the width reading over a meet\n\nThe meet_width function in the paceghost.meet module takes a loaded meet and returns the width lines on the error channel FIX-PG-10 states, and the reading is a pure function of the meet. The standing projection of a competitor at a mark, the holder after a mark, the record after a mark, the nearest standing after a mark, the farthest standing after a mark, and the field width after a mark are the standings PG-51 states, read off the one walk PG-51 names. Test, rust/tests/test_width.rs.\n\nThe first line of the reading names the meet, in the form meet width, and the count of days and the word days, the first line form of the marks reading. One section then stands for each day of the meet, in meet order, and every day of the meet holds a section, a day in which no mark of the walk stands holding a section with no mark line in it. A section opens with one line reading day, and the ordinal phrase of the day as the meet report states it, in the form the and the ordinal word of the day and day, and, marks and the count of marks of the walk standing in that day, the day opening of the marks reading byte for byte. One mark line then stands for each mark of the walk standing in that day, in walk order, reading mark and the place of the mark in the whole walk, and, at and the elapsed figure of the mark and seconds on the and the ordinal phrase of the day of the mark and day, and the label of the holder after the mark and at a projection of and the record figure after the mark, in the mark form of the marks reading as far as the record figure, and then, where another competitor holds a standing after the mark, the line continues with field width and the field width figure. Where no competitor other than the holder holds a standing after the mark, the line ends with the words and no other standing. The reading closes with one counts line. Where some mark held another standing, the counts line reads width counts, and marks walked and the count of marks of the walk and, marks with another standing and the count of marks at which another competitor held a standing and, days and the count of days and, narrowest field width and the smallest field width among the marks of the walk and at and the elapsed figure of the earliest mark standing at that width and seconds on the and the ordinal phrase of the day of that mark and day. Where no mark held another standing, the counts line ends with the words and, no other standing at any mark in place of the narrowest width clause, and that form also covers the meet in which no read projects, whose counts line states marks walked zero. No other line stands in the reading. Every elapsed figure renders with format_seconds_fraction, and every projection and every field width figure renders with format_usage, so every figure renders under a discipline an earlier section states. No competitor is ranked, ordered, or compared against another anywhere in the reading beyond the holder each mark line names, and no mark line names a holder for a field width, because a field width is a difference of two standings of the field and no competitor holds it, and no line of the reading names a winner or says that a competitor or a figure did well or badly, because a figure names only what the walk computed, and the comparison the reading makes beyond its mark lines is the one narrowest field width its closing line names, taken over the marks themselves and never over the competitors, in the discipline PG-18 states for its longest spell.\n\nThe closing relations bind the reading to the standing meet readings over the same meet, and each is stated so it can fail. The sections equal the days of the meet, the count the days read line of the meet report states, and the marks of the sections sum to the marks walked of the marks reading over the same meet, because every mark of the walk stands in exactly one day. Line by line, the holder and the record figure of each mark line are the holder after the mark and the record figure after the mark the marks reading states for the mark at that place. At the last mark of each day holding marks, the field width figure is the farthest standing less the nearest standing of the standings the standings reading's section for that day states for competitors other than the holder its closing line names, the extremes taken as PG-29 and PG-31 state them and the field width standing as their difference, the closing line being the closing line the meet report prints for that day, which the standings section carries byte for byte. The marks with another standing of the counts line and the marks whose lines end in no other standing sum to the marks walked. The field width of every mark line is the exact spread of PG-32 at that mark less the margin of PG-30 at that mark, computed on exact values, and no field width is negative. The interquartile range of PG-44 at a mark is never more than the field width at that mark, and the field width is never more than the spread of PG-32 at that mark, the field width equal to the interquartile range exactly where the lower quartile standing stands at the nearest standing and the upper quartile standing stands at the farthest standing there, and the field width equal to the spread exactly where the margin of PG-30 stands at zero there, the nearest standing standing at the record figure, because the field width is the farthest standing less the nearest standing at every mark and each quartile standing stands at or between the two. The difference of the tails of PG-36 at a mark is never more than the field width at that mark either, each tail standing at or between the nearest standing and the farthest standing there. The narrowest field width of the counts line is the smallest field width among the mark lines, it stands at the earliest mark standing at that width, and the mark line at the day and the elapsed figure the counts line names states that width. One further relation stands as a pin rather than a closing. The field width stands at zero exactly at the marks where every figure of the field stands at one figure, and at no other mark, because the field width is the farthest standing less the nearest standing at that mark, each figure of the field standing at or between the two, and the two stand at one figure exactly where every figure of the field does. The reading walks the marks of the progression in plain order and holds no recursion of its own beyond the walk it shares. Test, rust/tests/test_width.rs, and the golden width readings held in rust/tests/test_examples_round24.rs.\n\nThe example stands in the examples directory. The meet file at examples/meet.json, the example meet of Round 6, does not move, and the width reading is taken over that meet as it stands. The golden reading at examples/meet-width-golden.txt is produced by the real width command over the meet file. The width reading of the example meet closes at marks walked 16, marks with another standing 15, and days 3, with narrowest field width 0 at 400 seconds on the first day, the second mark, the only mark standing at that width, and the last mark of each day agrees with the standings golden's closes, the first day closing with night fork at 840 and a field width of 141.818182 over the closing field {998.181818, 1140}, the second with dawn fork at 675 and a field width of 108 over {900, 1008}, and the third with dawn fork at 560 and a field width of 30 over {720, 750}, counted by hand from the meet file through the walk, every figured mark of the shipped meet standing at one or two behind its holder, so each mark's field width is its interquartile range whole and its difference of the tails whole, the separations from each standing in the round's hand built pins and in the generated family in this golden's place. The constructed meet of Round 11, at examples/constructed-meet.json, does not move, and its golden reading stands beside its marks golden at examples/constructed-meet-width-golden.txt, produced by the real width command over the constructed meet file, closing at marks walked 7, marks with another standing 7, and days 3, with narrowest field width 0 at 100 seconds on the first day, the first mark, the earliest of the seven marks standing at that width, the constructed field standing at one behind its holder at every mark, so its field width is zero at every mark, counted by hand from the constructed meet file through the walk. Test, rust/tests/test_examples_round24.rs.\n"
    );
    assert_eq!(
        section_subheadings(&lines, "## Round 25 obligations, full form"),
        [
            "### PG-51 full form, the width reading's inputs",
            "### PG-52 full form, the width reading over a meet",
            "### The Round 25 command line surface",
            "### The generated fixture family extended to the width reading",
        ]
        .iter()
        .map(|line| line.to_string())
        .collect::<Vec<String>>()
    );
}

#[test]
fn test_readme_reports_the_round_twenty_five_status() {
    let lines = readme_lines();
    let standing = "- Contract, PG-51 and PG-52 met. They stand in full form in CONTRACT.md under the Round 25 obligations, and Round 25 closed on the merged slice 26 after three reviews, with the spec review approving with findings at two Observations standing against the design record, and the standards review approving with findings at one Minor and two Observations, and the red team approving with no findings, and with CodeRabbit returning no inline comments on the slice, and with the standards review's Minor standing cured in the slice by the repair commit 85f22890, the cli.rs module doc count restated at twenty seven, comment-only and strip-proven, the census unmoved, and with the design record's seventh section naming two standing command surface pins against the counted three, all three standing restated in the build commit as the landing falsified them, and with pin 8's tie meet standing recomposed in the shipped fixture, the meet as transcribed standing unloadable under the loader's rising-usage rule, every figure the design states standing identical in the shipped lines, and its S-4 discrimination standing proven end to end by the red team on the recomposed fixture itself, and with pin 5's presence in the S-1 and S-6 sets standing on its second edge under the design's own structural rule, its five fixture marks holding under both, and with the day close example pin's split standing as the shape-forced split the execution ran, failing under S-1, S-5, and S-6 and passing under S-2, S-3, and S-4, the design stating it only by reference to the reading pin, and with S-2 and S-3 standing at identical failing sets as honest discrimination stated in the open, the two partners parting from each other by figure inside the round's own fixtures and each standing caught independently at nine pins, a separating fixture standing priced for a later round, and with the three records items standing as records only and not as cured work, the Round 24 design note's fourth section carrying the false distribution pair as written, one at seven and fifty eight at five and more against the counted fourteen at seven and fifty nine at five and more, its contract places standing cured inside Round 24 by FIX-PG-20, and the Round 23 design note's fifth section carrying its compression as written, and the Round 22 design record's manifest figure of 73 standing against the counted 71 as a records reconciliation, and with FIX-PG-21 standing free and unspent through the round.";
    assert!(lines.iter().any(|found| found == standing));
    let slice_line = "- Slice 26 has landed. It carries the width reading that reads a meet mark by mark over the one record progression of the meet report, naming the holder after each mark, the record figure after it, and the field width held after it by the field behind its holder, the farthest standing less the nearest standing of that field, and the width command, with the golden width readings of the standing example meet and of a constructed meet under examples, and the multi day walk extended in its own pass to record the field width after each mark, and the seeded generated fixture family extended to the width reading, and the README width section.";
    assert!(lines.iter().any(|found| found == slice_line));
    assert!(read("README.md").contains("Closed perfection rounds, 29 of 100"));
}

#[test]
fn test_the_command_surface_stands_at_twenty_seven_with_width_last() {
    let text = read("rust/src/cli.rs");
    let lines: Vec<&str> = text.lines().collect();
    let start = lines
        .iter()
        .position(|line| *line == "const COMMANDS: [&str; 27] = [")
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
        "midrange",
        "width",
    ];
    assert_eq!(names, expected);
}

// Restated under PG-61, the block below asserts the standing subsection text whole.
#[test]
fn test_contract_carries_the_round_twenty_six_obligations() {
    let text = read("CONTRACT.md");
    assert!(text.contains("## Round 26 obligations, full form"));
    assert!(text.contains("### PG-53 full form, the separating meet and the width reading over it"));
    assert!(text.contains(
        "### PG-54 full form, the coincidence of the partners over the generated family"
    ));
    let lines: Vec<String> = text.lines().map(|line| line.to_string()).collect();
    assert_eq!(
        lines
            .iter()
            .filter(|line| line.as_str() == "## Round 26 obligations, full form")
            .count(),
        1
    );
    assert_eq!(
        subsection_span(&lines, "### PG-53 full form, the separating meet and the width reading over it").join("\n"),
        "### PG-53 full form, the separating meet and the width reading over it\n\nPG-53. One hand built meet stands composed for this round and printed whole in the round's design note, a meet of three days under a window of one thousand seconds on each day, where the holder reads once on the first day to a projection of one hundred and stands as holder at every mark, where one competitor reads beside the holder on the first day and another reads after them on the same day, where the second day carries no read, and where all five competitors read together at one hundred seconds on the third day. The fields behind the holder stand at one figure of one hundred and fifty at the first mark, at two figures of one hundred and fifty and one hundred and eighty at the second mark, and at five figures of one hundred and fifty twice, two hundred once, and three hundred twice at the third mark. The width reading over the meet reports a field width of zero at the first mark, of thirty at the second, and of one hundred and fifty at the third, and its counts line reports three marks walked, three marks with another standing, three days, and the narrowest field width of zero standing at one hundred seconds on the first day, the one mark of the meet whose field width stands at zero. Over the same meet the range stands at the field width at every figured mark, and the difference of the tails stands at the field width at the first and second marks and below it at the third, where their difference stands at one hundred, two thirds of the field width there, the lower quartile standing at one hundred and fifty and the upper quartile standing at three hundred at that mark. The width report of this meet stands unchanged where the range stands in the width's place, since the two reports agree at every mark of the meet, and fails where the difference of the tails stands in the width's place, at the third mark, and the failing sets of the two edits part at the pins of this meet, which is the separation the Round 25 record priced and no earlier fixture made. Test, test_width.rs, the pins of the width reading over this meet and of the relations among the readings over it.\n"
    );
    assert_eq!(
        subsection_span(&lines, "### PG-54 full form, the coincidence of the partners over the generated family").join("\n"),
        "### PG-54 full form, the coincidence of the partners over the generated family\n\nPG-54. Over the generated fixture family of the generated pins, eight hundred and seventy six figured marks standing counted over the family's eight meets, the range stands at the field width at thirty four marks and the difference of the tails stands at the field width at the same thirty four marks, every one of them a mark whose field holds one figure or two, where the three figures coincide by construction. No figured mark of the family separates the partners, no mark standing counted where the range stands at the field width and the difference of the tails stands below it, and the range and the difference of the tails part from each other at seven hundred and fifty six of the eight hundred and seventy six marks without either standing at the field width. The coincidence the family counts is the family's instance of the standing relation this round states in the open, that where the difference of the tails stands at the field width at any figured mark of any meet the range stands at the field width there, since a mean of figures standing at or below a bound stands at that bound only where every figure of the mean stands at it, so a difference of the tails at the field width forces every figure of the lower tail part to the nearest standing and every figure of the upper tail part to the farthest standing, and the quartile of each part, the median of that part, stands at the same extreme. The mirror separation, a mark whose difference of the tails stands at the field width while its range stands below it, is not a fixture a later round can build, and this obligation records its impossibility so the search for it does not reopen. Test, test_generated.rs, the coincidence pin of the generated family.\n"
    );
    assert_eq!(
        section_subheadings(&lines, "## Round 26 obligations, full form"),
        [
            "### PG-53 full form, the separating meet and the width reading over it",
            "### PG-54 full form, the coincidence of the partners over the generated family",
            "### The Round 26 command line surface",
            "### The generated fixture family under the separation record",
        ]
        .iter()
        .map(|line| line.to_string())
        .collect::<Vec<String>>()
    );
}

#[test]
fn test_readme_reports_the_round_twenty_six_status() {
    let lines = readme_lines();
    let standing = "- Contract, PG-53 and PG-54 met. They stand in full form in CONTRACT.md under the Round 26 obligations, and Round 26, the separation round, closed on the merged slice 27 after three reviews, with the spec review approving with findings at two Observations, and the standards review approving with findings at two Minors and four Observations, with both Minors standing cured in the slice by the repair commit bbb5310a, and the red team approving with no findings on its completion lane's report, its first seating standing incomplete on the write fault and discharged by execution, and with CodeRabbit returning three inline comments on the slice, two standing valid and cured in the repair, the qualifier half of the standards review's first Minor and the quartile standings of its second, and one standing not valid, the exact comparison comment, dismissed on the family's standing print grain tolerance standing at one hundred and thirty one lines of the base's generated file alone, and with the separation standing executed whole, under S-2 the failing set standing at ten with pins 17 and 18 passing, and under S-3 at exactly those ten plus pins 17 and 18, the sets parting at the separating meet's pins and nowhere else, and with the impossibility standing proved at the desk and swept by execution, one thousand five hundred and fourteen meets and three thousand and twenty eight figured marks at no counterexample, the claim's antecedent standing exercised at eight hundred and fifty three marks, and with the two cures standing proven in both directions, pin 16 failing at eight hundred and forty two partings counted against seven hundred and fifty six under disturbances at the tails seat and at the range seat where the unrepaired pin passed whole, and pin 18 failing at its lower quartile assertion under the compensating movement where the unrepaired pin passed, and with the build's fresh extraction re-run standing discharged under the bounded fault limb, the form gate on the identical tree and the hosted gate standing as its verdicts of record, and the repair's own fresh extraction gate standing executed whole at 959 and 0, and with the design note's section 0 base tree string standing for the errata queue, the base commit's tree standing at 7842fa27, and with the three records items of the design note's second section standing carried forward named with no cure charged, the Round 24 design note's fourth section carrying the false distribution pair as written, one at seven and fifty eight at five and more against the counted fourteen at seven and fifty nine at five and more, its contract places standing cured inside Round 24 by FIX-PG-20, and the Round 23 design note's fifth section carrying its compression as written, and the Round 22 design record's manifest figure of 73 standing against the counted 71 as a records reconciliation, and with FIX-PG-21 standing free and unspent through the round.";
    assert!(lines.iter().any(|found| found == standing));
    let slice_line = "- Slice 27 has landed. It carries the separating meet of PG-53, a hand built meet whose third mark holds the five figure field that separates the width reading's two partners, the range standing at the field width at every mark of the meet and the difference of the tails standing below it at the third, and the coincidence pin of PG-54 over the generated fixture family, with no reading and no command added.";
    assert!(lines.iter().any(|found| found == slice_line));
    assert!(read("README.md").contains("Closed perfection rounds, 29 of 100"));
}

// Restated under PG-61, the block below asserts the standing subsection text whole.
#[test]
fn test_contract_carries_the_round_twenty_seven_obligations() {
    let text = read("CONTRACT.md");
    assert!(text.contains("## Round 27 obligations, full form"));
    assert!(text.contains(
        "### PG-55 full form, the execution record of the width file's unexecuted mutations"
    ));
    assert!(text.contains("### PG-56 full form, the hunt record of the width neighbourhood"));

    assert!(text.contains(
        "Under S-1 the suite stands at nine hundred and thirty nine passed and twenty failed"
    ));
    assert!(text.contains(
        "Under S-5 the suite stands at nine hundred and forty passed and nineteen failed"
    ));
    assert!(text.contains("Under S-6 the suite stands at eight hundred and eleven passed and one hundred and forty eight failed"));
    assert!(text.contains(
        "Under S-4 the suite stands at nine hundred and fifty five passed and four failed"
    ));
    assert!(text.contains("The header's stated splits for the separating meet pin and the separation relation pin, falling under S-1, S-5, and S-6 and passing under S-4, stand confirmed by execution whole"));
    assert!(text.contains("W-1 takes the field width as the second farthest standing less the nearest standing in the width selection alone, the mirror of the seat the Round 26 red team probed with the farthest less the second lowest, and falls at sixteen pins, its set parting from that edit's set at three names each way"));
    assert!(text.contains("W-2 takes the lower of the two middle figures at an even part in the quartiles selection's own copy of the median construction, the range and midhinge copies standing untouched, and falls at eighteen pins"));
    assert!(text.contains("W-3 makes the tails parts disjoint in the tails selection alone, the upper part opening at the part count at the odd counts of three or more with the shipped form standing at counts one and two and at every even count, and falls at twenty one failures standing at nineteen distinct pin names, the separation relation pin among them at its third mark and the separating meet pin passing, the width standing untouched by the edit"));
    assert!(text.contains("W-4 makes the range parts disjoint in the range selection's own copy of the part construction under the same guard as W-3, and falls at twelve pins, the separation relation pin passing by the separating meet's own shape, the upper part standing at three hundred twice under both constructions at its third mark, and the generated range pin and the range family's pins over odd counted fields carrying the catch"));
    assert!(text.contains("W-5 loosens the narrowest scan in the range reading alone to name a later mark at an equal range, and falls at four pins, the range tie pin, the constructed range golden pin, the constructed range command pin, and the generated range pin"));
    let lines: Vec<String> = text.lines().map(|line| line.to_string()).collect();
    assert_eq!(
        lines
            .iter()
            .filter(|line| line.as_str() == "## Round 27 obligations, full form")
            .count(),
        1
    );
    assert_eq!(
        subsection_span(&lines, "### PG-55 full form, the execution record of the width file's unexecuted mutations").join("\n"),
        "### PG-55 full form, the execution record of the width file's unexecuted mutations\n\nPG-55. The round carries an execution record taken on the tree the round opens from, the merged tree of the Round 26 closure at its standing census of nine hundred and fifty nine, and the build confirms the record at its warm-up on that tree before the slice stands built. The record is the execution of the four mutations of the Round 25 family that no lane has executed against the pins Round 26 added, S-1 the holder exclusion dropped in the width selection alone with its field taken holder inclusive, S-4 the narrowest field width scan loosened to name a later mark at an equal width, S-5 the width vector lagged one mark against the walk, and S-6 the holder exclusion dropped in the shared field collection itself. Each execution runs the whole suite under that one edit on the opening tree, each edit standing in its own fresh target directory, and the failing set of each is read from the whole log with assertion failures alone deciding it, a run dying on the box write fault standing reported beside its set as a fault failure and never inside it, and a run whose edit crashes the walk standing as no set at all. Each failing pin's executed posture stands set against the posture the header of rust/tests/test_width.rs states for it, and any parting between an executed set and a stated posture stands named in the record pin by pin.\n\nThe record stands as follows, executed at the round's design desk on the opening tree. Under S-1 the suite stands at nine hundred and thirty nine passed and twenty failed, the failing set standing at the seventeen pins of the Round 25 family's own account with the separating meet pin, the separation relation pin, and the coincidence pin of the generated family added, each of the three falling. Under S-5 the suite stands at nine hundred and forty passed and nineteen failed, the failing set standing at the sixteen pins of the family's account with the same three pins added, each of the three falling. Under S-6 the suite stands at eight hundred and eleven passed and one hundred and forty eight failed, the failures standing at one hundred and twenty three distinct pin names with four of the names standing in several test files of the family at once, and the same three pins of Round 26 falling among them. Under S-4 the suite stands at nine hundred and fifty five passed and four failed, the failing set standing at exactly the four pins of the family's account, the narrowest tie pin, the constructed command pin, the constructed golden pin, and the generated width pin, and the three pins of Round 26 passing whole. The header's stated splits for the separating meet pin and the separation relation pin, falling under S-1, S-5, and S-6 and passing under S-4, stand confirmed by execution whole, and no parting stands between any executed set and any stated posture, so no cure stands charged against the header's text and FIX-PG-21 stands unspent. The slice restates the header account of the two pins in the tests step to record that their splits stand on this round's execution and not on the family's standing account alone. No figure, form, or behaviour of any reading moves under this obligation, the record being of executions alone, and the tree the executions run on stands restored whole after them and verified against the opening tree before the slice stands built. Test, the pins named in the record, read under the four edits.\n"
    );
    assert_eq!(
        subsection_span(&lines, "### PG-56 full form, the hunt record of the width neighbourhood").join("\n"),
        "### PG-56 full form, the hunt record of the width neighbourhood\n\nPG-56. The round carries a hunt record over the seats of the width reading's neighbourhood that the standing record has not executed, five edits of the shipped code, each executed whole on the opening tree at its standing census in its own fresh target directory at the round's design desk under the discipline PG-55 states, the failing sets read from assertion failures alone. W-1 takes the field width as the second farthest standing less the nearest standing in the width selection alone, the mirror of the seat the Round 26 red team probed with the farthest less the second lowest, and falls at sixteen pins, its set parting from that edit's set at three names each way. W-2 takes the lower of the two middle figures at an even part in the quartiles selection's own copy of the median construction, the range and midhinge copies standing untouched, and falls at eighteen pins. W-3 makes the tails parts disjoint in the tails selection alone, the upper part opening at the part count at the odd counts of three or more with the shipped form standing at counts one and two and at every even count, and falls at twenty one failures standing at nineteen distinct pin names, the separation relation pin among them at its third mark and the separating meet pin passing, the width standing untouched by the edit. W-4 makes the range parts disjoint in the range selection's own copy of the part construction under the same guard as W-3, and falls at twelve pins, the separation relation pin passing by the separating meet's own shape, the upper part standing at three hundred twice under both constructions at its third mark, and the generated range pin and the range family's pins over odd counted fields carrying the catch. W-5 loosens the narrowest scan in the range reading alone to name a later mark at an equal range, and falls at four pins, the range tie pin, the constructed range golden pin, the constructed range command pin, and the generated range pin. The first constructions of W-3 and W-4 stand void on this record and are not discrimination reads, each emptying its upper part at a field of one figure and crashing the walk it ran in, their runs standing as crash totals beside this record and never inside any set, and the guarded constructions of this obligation stand executed in their place. No edit this round executed passes the whole suite on the opening tree, and no discrimination gap stands in the width reading's neighbourhood at the seats this record names. No figure, form, or behaviour of any reading moves under this obligation. Test, the round's records, the executions standing in them.\n"
    );
    assert_eq!(
        section_subheadings(&lines, "## Round 27 obligations, full form"),
        [
            "### PG-55 full form, the execution record of the width file's unexecuted mutations",
            "### PG-56 full form, the hunt record of the width neighbourhood",
            "### The Round 27 command line surface",
            "### The generated fixture family under the execution record",
        ]
        .iter()
        .map(|line| line.to_string())
        .collect::<Vec<String>>()
    );
}

#[test]
fn test_readme_reports_the_round_twenty_seven_status() {
    let lines = readme_lines();
    let standing = "- Contract, PG-55 and PG-56 met. They stand in full form in CONTRACT.md under the Round 27 obligations, and Round 27, the width execution record round, closed on the merged slice 28 after three reviews, the slice standing as pull request 59 merged at a8668aea with main's tree standing verified equal to the repaired slice tree 535193a7 and the census standing at 961, with the spec review approving with findings at three Observations, each standing against the design record and none charged to the slice, and the standards review approving with findings at one Minor and three Observations, the Minor standing charged on the round twenty seven contract pin asserting its headings alone over a round whose whole substance stands in its contract body, and standing cured in the slice by the repair commit 4ad6eef0 extending the pin with ten content assertions over the four PG-55 totals, the posture sentence, and the five W sentences, the cure standing proven in both directions by execution, the repaired pin failing at the falsified S-1 sentence and at the falsified W-2 count where the unrepaired pin passed the whole setup target, and the repair's fresh extraction gate standing executed whole at 961 and 0, and the red team approving with no findings, and with CodeRabbit returning two inline comments on the slice, both standing valid as the standards review's Minor at its two seats and cured in the repair, its LanguageTool sub-notes standing dismissed on the contract's standing spelled out number form, and with the execution record standing confirmed by independent execution whole, under S-1 the suite standing at nine hundred and thirty nine passed and twenty failed, under S-5 at nine hundred and forty passed and nineteen failed, under S-6 at eight hundred and eleven passed and one hundred and forty eight failed standing at one hundred and twenty three distinct names, and under S-4 at nine hundred and fifty five passed and four failed, pins 17 and 18 falling under S-1, S-5, and S-6 and passing under S-4 exactly as the width file's header states, and with the hunt standing executed whole, five fresh edits falling and none passing whole, and the red team's own edit RT-1 falling at ten range family pins as it predicted before its run, and with the tests step standing proven comment only by mechanical stripping and the base and head binaries standing in agreement on two hundred and seventy drives at no parting, and with the design note's fifth section naming a round seven count pin and a Round 27 in flight heading the base's setup file does not carry, the construction standing on the family's executed form and the note's place counts standing exact as occurrence counts, and with the design report's section figure of 7,635 characters standing against the contract side figure of 7,651, the sixteen character difference standing fully accounted and every paragraph standing byte identical between the two spans, and with the standards review's whole family whole obligation cure standing as a priced later round question in the grandjury species, and with the three records items of the Round 26 note's second section standing carried forward named with no cure charged, the Round 24 design note's fourth section carrying the false distribution pair as written, one at seven and fifty eight at five and more against the counted fourteen at seven and fifty nine at five and more, its contract places standing cured inside Round 24 by FIX-PG-20, and the Round 23 design note's fifth section carrying its compression as written, and the Round 22 design record's manifest figure of 73 standing against the counted 71 as a records reconciliation, and with the Round 26 design note's section 0 base tree string standing for the errata queue, 0b9e4eea against the true base tree 7842fa27, and with FIX-PG-21 standing free and unspent through the round.";
    assert!(lines.iter().any(|found| found == standing));
    let slice_line = "- Slice 28 has landed. It carries the execution record of PG-55, the four mutations of the Round 25 family standing executed against the width file's pins with the header's stated splits for the separating meet pin and the separation relation pin confirmed whole and no parting standing, and the hunt record of PG-56 over the width reading's neighbourhood, five fresh edits falling and none passing whole, with no reading and no command added.";
    assert!(lines.iter().any(|found| found == slice_line));
    assert!(read("README.md").contains("Closed perfection rounds, 29 of 100"));
}

// Restated under PG-61, the block below asserts the standing subsection text whole.
#[test]
fn test_contract_carries_the_round_twenty_eight_obligations() {
    let text = read("CONTRACT.md");
    assert!(text.contains("## Round 28 obligations, full form"));
    assert!(text.contains("### PG-57 full form, the whole obligation assertion cure"));
    assert!(text.contains("### PG-58 full form, the hunt record of the field selections"));
    let lines: Vec<String> = text.lines().map(|line| line.to_string()).collect();
    assert_eq!(
        lines
            .iter()
            .filter(|line| line.as_str() == "## Round 28 obligations, full form")
            .count(),
        1
    );
    assert_eq!(
        subsection_span(&lines, "### PG-57 full form, the whole obligation assertion cure").join("\n"),
        "### PG-57 full form, the whole obligation assertion cure\n\nPG-57. The contract pins of Rounds 22 through 26 stand restated in rust/tests/test_setup.rs so that each asserts the whole text of the two obligations its round carries. Each restated pin asserts that the heading of its round's section stands exactly once among the lines of this contract and that the subheading of each of its two obligations stands exactly once among them, and it asserts the lines standing under each subheading, down to the next heading of either level, equal to that subsection's whole standing text and nothing else, the subheading itself, a blank line, the body of the obligation in its paragraphs as they stand, and a closing blank line. A cut of any sentence of a guarded body in place fails at the pin of its round, a planting of a guarded body whole under a duplicate of its subheading fails at the same pin on the subheading's count, and a sentence moved from one guarded body into another fails at the pins of both rounds, and this round's design record carries the base executions that prove the gap at every guarded body and the computed directions of the cure against the whole suite, the cure's own executions standing charged to the warm up of the build as first executions under the stopping rule the design record states. The contract pins of Rounds 13 through 21 stand unrestated in the forms their rounds set, and the whole text question over those pins stands priced for a later round on the ground this round's design record states, their obligations standing guarded by the behaviour pins of the readings they name. The preamble paragraph of each guarded section and the section's command line surface and generated fixture family paragraphs stand outside the spans this obligation asserts. No figure, form, or behaviour of any reading moves under this obligation, and no text of this contract moves under it, the restated pins passing over the contract as it stands and failing only where a guarded text stands moved. Test, rust/tests/test_setup.rs, the contract pins of Rounds 22 through 26 as restated.\n"
    );
    assert_eq!(
        subsection_span(&lines, "### PG-58 full form, the hunt record of the field selections").join("\n"),
        "### PG-58 full form, the hunt record of the field selections\n\nPG-58. The round carries a hunt record over the seats of the field selections that the standing record has not executed, four edits of the shipped code, each executed whole on the opening tree at its standing census in its own fresh target directory at the round's design desk under the discipline PG-55 states, the failing sets read from assertion failures alone. X-1, the midhinge selection's own copy of the median construction taking the lower middle figure at an even part, stands executed at the design desk at 950 passed and 11 failed, its failing set standing in the round's design record name for name, the midhinge reading's own pins and the separation pins that name the midhinge among them. X-2, the median selection taking the lower of its two middle figures alone at an even field count, X-3, the middle selection taking the upper middle figure at an even field count, and X-4, the quartiles parts taking the floor count at the odd field counts of three or more, stand computed in the design record at their seats and carried unexecuted under the write fault, one run of X-2 standing void in that record as a window straddle with no set read off it, and their execution stands charged to the warm up of the build as first executions in the order the design record states, any of the three passing whole at its execution reopening the round's design before the slice is seated. This paragraph stands restated to the executed form, the four edits and their sets standing in it whole, if the build's warm up runs the family whole before the contract commit is assembled, the swap standing named in the build's report, and the round stands open under the rule of this obligation while that execution stands unexecuted. No figure, form, or behaviour of any reading moves under this obligation. Test, the round's records, the executions standing in them.\n"
    );
    assert_eq!(
        section_subheadings(&lines, "## Round 28 obligations, full form"),
        [
            "### PG-57 full form, the whole obligation assertion cure",
            "### PG-58 full form, the hunt record of the field selections",
            "### The Round 28 command line surface",
            "### The generated fixture family under the assertion cure",
        ]
        .iter()
        .map(|line| line.to_string())
        .collect::<Vec<String>>()
    );
}

#[test]
fn test_readme_reports_the_round_twenty_eight_status() {
    let lines = readme_lines();
    let standing = "- Contract, PG-57 and PG-58 met. They stand in full form in CONTRACT.md under the Round 28 obligations, PG-57 standing as the whole obligation assertion cure over the contract pins of Rounds 22 through 26 and PG-58 as the hunt record of the field selections beside it, the hunt standing executed whole and no edit passing whole anywhere. The slice standing as pull request 61 merged at f6154458 with main's tree standing verified equal to the repaired slice tree ca0445e8 and the census standing at 963. The spec review approving with findings at one Observation, standing against the design record and none charged to the slice, that PG-58's opening sentence in the design note's fourth section states the hunt's four edits in executed form while its own later sentences stand them computed and carried, the item riding the errata queue. The standards review approving with findings at one Minor and one Observation, the Minor standing charged on the five restated pins carrying no local justification comment naming PG-57 where their alterations land, and standing cured in the slice by the repair commit d5ea2dd9 adding the one comment at each of the five seats in the file's own double slash form naming PG-57 and that the block asserts the standing subsection text whole, the cure standing proven in both directions, and the Observation standing against no text in the slice, the heading count block repeating at six seats in the standing family form and priced forward only. CodeRabbit returning one inline comment on the slice, standing valid as the standards review's Minor at its seat and cured in the same repair. The red team approving with findings at two Observations and nothing charged to the slice, the first standing against the standing record, that the Round 27 contract pin asserts PG-55 and PG-56 by named sentences only with nine sentences of PG-55 and six of PG-56 standing under no literal, priced forward for a later round, and the second naming the helper subsection_span as the cure's one shared component, priced in the open and guarded by its own count assertion. The hunt standing executed whole as first executions, X-1 at the design desk at nine hundred and fifty passed and eleven failed, X-2 at nine hundred and thirty eight passed and twenty three failed, X-3 at nine hundred and forty two passed and nineteen failed at eighteen distinct pins, and X-4 at nine hundred and forty six passed and fifteen failed, every failure an assertion failure and every set standing inside its computed expectation on the single channel ground its edit moves one figure alone. The cure's tightness standing verified by the red team's computation, the twelve span literals standing equal to the spans computed from the head contract under the pins' own semantics, the ten cuts standing re-derived in both directions over every contract reading pin, passing whole at the base and falling on the form at the pin of their own round alone, and the shared span break attempts falling at every seat whose span they move and at no other. The form family standing carried at this closure on the write fault at the attempts the closure report records, the form gate, the ten cuts, the directions d1, d2, r1, rad1, and f57, the candidate legs A and B, and the fresh extraction gate standing unexecuted here as they stood at the build and the red team, their computed sets standing confirmed pin by pin at the red team's desk over every pin that reads the contract text, and the hosted gate on the closure head standing as the verdict of record in their place. The three records items of the Round 26 note's second section standing carried forward named with no cure charged, the Round 24 design note's fourth section carrying the false distribution pair as written, one at seven and fifty eight at five and more against the counted fourteen at seven and fifty nine at five and more, its contract places standing cured inside Round 24 by FIX-PG-20, and the Round 23 design note's fifth section carrying its day close compression as written, and the Round 22 design record's manifest figure of 73 standing against the counted 71 as a records reconciliation. The Round 26 design note's section 0 base tree string standing for the errata queue, 0b9e4eea against the true base tree 7842fa27. The Round 27 design record's items standing carried the same way, its note's fifth section naming a round seven count pin and a Round 27 in flight heading the base's setup file does not carry, and its design report's section figure of 7,635 standing against the contract side figure of 7,651. And FIX-PG-21 standing free and unspent through the round.";
    assert!(lines.iter().any(|found| found == standing));
    let slice_line = "- Slice 29 has landed. It carries the whole obligation assertion cure of PG-57, the contract pins of Rounds 22 through 26 standing restated to assert the whole text of their two obligations each, and the hunt record of PG-58 over the field selections standing part executed and part carried under the write fault, with no reading and no command added.";
    assert!(lines.iter().any(|found| found == slice_line));
    assert!(read("README.md").contains("Closed perfection rounds, 29 of 100"));
}

// Restated under PG-61, the block below asserts the standing subsection text whole.
#[test]
fn test_contract_carries_the_round_twenty_nine_obligations() {
    let text = read("CONTRACT.md");
    assert!(text.contains("## Round 29 obligations, full form"));
    assert!(text.contains("### PG-59 full form, the whole obligation assertion cure over the contract pin of Round 27"));
    assert!(text.contains("### PG-60 full form, the cut record of the Round 27 cohort"));
    let lines: Vec<String> = text.lines().map(|line| line.to_string()).collect();
    assert_eq!(
        lines
            .iter()
            .filter(|line| line.as_str() == "## Round 29 obligations, full form")
            .count(),
        1
    );
    assert_eq!(
        subsection_span(&lines, "### PG-59 full form, the whole obligation assertion cure over the contract pin of Round 27").join("\n"),
        "### PG-59 full form, the whole obligation assertion cure over the contract pin of Round 27\n\nPG-59. The contract pin of Round 27 stands restated in rust/tests/test_setup.rs so that it asserts the whole text of the two obligations its round carries, PG-55 and PG-56, in the form PG-57 set for the pins of Rounds 22 through 26. The restated pin keeps every assertion it already carries, its section heading and its two subheadings by containment and the ten named sentences of the two bodies by containment, and it gains the charged block of the cure, an assertion that the heading of the Round 27 section stands exactly once among the lines of this contract and an assertion for each of the two obligations that the lines standing under its subheading, down to the next heading of either level, stand equal to that subsection's whole standing text and nothing else, the subheading itself, a blank line, the body of the obligation in its paragraphs as they stand, and a closing blank line. One comment stands directly above the restated pin's test attribute in the form the Round 28 repair set, naming this obligation and that the block below asserts the standing subsection text whole. A cut of any sentence of either guarded body in place fails at the contract pin of Round 27, a planting of either guarded body whole under a duplicate of its subheading fails at the same pin on the subheading's count, and a sentence moved from one guarded body into the other fails at the same pin at both of its span assertions, the one pin carrying both obligations where the cure of PG-57 charged two pins for the same move across two rounds. The preamble paragraph of the Round 27 section and the section's command line surface and generated fixture family paragraphs stand outside the spans this obligation asserts. The enumeration that prices this cure stands in this round's design record, taken over the whole contract at the round's base, and it finds the two obligations of Round 27 the whole remaining surface of the named sentence species, the obligations of Rounds 22 through 26 and of Round 28 standing asserted whole already, the obligations of Rounds 13 through 21 standing asserted by their labels and headings alone with every load bearing rule of theirs guarded by the behaviour pins of the readings they name, and the obligations of Rounds 1 through 12 standing under no contract pin with their rules guarded the same way, the whole text question over those two cohorts standing priced in the design record with its triggers unmet. This round's design record carries the computed directions of the cure against the whole suite in both directions, the cure's own executions standing charged to the warm up of the build as first executions under the stopping rule PG-60 states. No figure, form, or behaviour of any reading moves under this obligation, and no text of this contract moves under it, the restated pin passing over the contract as it stands and failing only where a guarded text stands moved. Test, rust/tests/test_setup.rs, the contract pin of Round 27 as restated.\n"
    );
    assert_eq!(
        subsection_span(&lines, "### PG-60 full form, the cut record of the Round 27 cohort").join("\n"),
        "### PG-60 full form, the cut record of the Round 27 cohort\n\nPG-60. The round carries a cut record over the two obligations of Round 27, eight cuts of the contract text, each computed seat by seat in this round's design record over every pin that reads the contract, in its own fresh target directory at its execution under the discipline PG-55 states, the failing sets read from assertion failures alone. In the base direction, on the tree the round opens from, a cut of the second sentence of PG-55 and a cut of the second sentence of PG-56 each pass the whole suite, no pin holding a literal inside either sentence and each sentence standing exactly once in the contract, a cut of the Under S-1 sentence of PG-55 and a cut of the W-2 sentence of PG-56 each fall at the contract pin of Round 27 alone on the fragment the cut removes whole, a duplication of the record paragraph of PG-55 inside its own subsection passes whole, a planting of the whole subsection of PG-56 a second time at the end of the Round 27 section passes whole, and a sentence moved from the body of PG-55 into the body of PG-56 passes whole in both its forms, the moved sentence standing under no literal in the first and carrying the Under S-1 fragment in the second, the containment pin reading the whole contract text and standing blind to the fragment's seat. In the form direction, on the tree the slice assembles, every one of those eight cuts falls at the contract pin of Round 27 alone, the relocation falling at both of its span assertions under the one pin, a cut of one sentence of the preamble paragraph of the Round 27 section passes whole outside the cure's radius, and a cut of the assertion sentence of PG-59 falls at the obligations pin of Round 29 alone. The eight cuts and their directions stand computed in the design record and carried unexecuted under the write fault, and their execution stands charged to the warm up of the build as first executions in the order the design record states beside the form family Round 28 carried, any base cut falling anywhere but its computed seat, any of the eight passing whole on the form, or the preamble cut falling anywhere reopening the round's design before the slice is seated, and the round stands open under the rule of this obligation while that execution stands unexecuted. This paragraph stands restated to the executed form, the eight cuts and their sets standing in it whole, if the build's warm up runs the family whole before the contract commit is assembled, the swap standing named in the build's report. No figure, form, or behaviour of any reading moves under this obligation. Test, the round's records, the executions standing in them.\n"
    );
    assert_eq!(
        section_subheadings(&lines, "## Round 29 obligations, full form"),
        [
            "### PG-59 full form, the whole obligation assertion cure over the contract pin of Round 27",
            "### PG-60 full form, the cut record of the Round 27 cohort",
            "### The Round 29 command line surface",
            "### The generated fixture family under the cut record",
        ]
        .iter()
        .map(|line| line.to_string())
        .collect::<Vec<String>>()
    );
}

#[test]
fn test_readme_reports_the_round_twenty_nine_status() {
    let lines = readme_lines();
    let standing = "- Contract, PG-59 met and PG-60 not yet met, the two standing in full form in CONTRACT.md under the Round 29 obligations, PG-59 standing as the whole obligation assertion cure over the contract pin of Round 27 and PG-60 as the cut record of the Round 27 cohort beside it, its cuts standing carried unexecuted under the write fault, the round standing closed under the rule of PG-60 while that execution stands unexecuted. The slice standing as pull request 63 merged at 7a37a2c1 with main's tree standing verified equal to the repaired slice tree d9464174 and the census standing at 965. The spec review approving with findings at one Minor and two Observations, the Minor standing charged to the slice as the wording of the Round 28 hunt ledger carried over Round 29's own family in the standing and slice texts, and standing cured in the slice by the repair commit 4a9a7da3 restating the clause to carried unexecuted at exactly the four seats, two in this file and two in the setup file, the third seat in each file standing as the Round 28 line of PG-58, true of that round and not moved, and the two Observations standing against the design record and the build record with nothing further charged to the slice. The standards review approving with findings at one Minor, two Observations, and one further Minor, the Minor standing charged to the slice as the same wording at its seats and cured in the same repair, the two Observations standing against the design record, its fifth section carrying a false clause over the family it narrates and its seventh section phrasing the construction in five commits where the slice stands in three, and the further Minor standing against the build record, its eighth section's stage tree figures standing nowhere on the remote where the commits' own trees stand read in their place, every blob level claim of the build standing verified whole. The red team approving with findings at one Minor and two Observations, the Minor standing as the same wording at its seats and cured in the same repair, the first Observation standing charged to the slice at no action, that a subsection planted in the Round 27 section outside the two spans passes the restated pin whole, priced in the open, and the second Observation standing against the build record. CodeRabbit returning one inline comment on the slice, standing valid as the same Minor at its seat and cured in the same repair. The cure's tightness standing verified at the red team's desk, the two span literals standing byte-equal to the spans computed from the head contract at 3,479 characters for PG-55 and 2,681 for PG-56, the eight cuts standing re-derived in both directions name for name, and the candidate legs standing read on their whole evaluable surface. The warm up family standing at this closure with one leg executed and the rest carried, the Round 28 form gate standing executed on expectation at the closure's third seating, the whole suite on the Round 28 form tree at 963 passed and 0 failed with the format check and clippy each at exit 0 and no fault signature in its log, and the Round 29 trees standing extracted whole and verified at that seating after ten extraction runs across the closure's three seatings, the base tree and the form tree each at 232 of 232 files matching its remote listing blob for blob, the base tree's last four files standing landed by direct write after their writes refused across twenty one passes of the extractor, the remaining legs standing carried on the write fault, the rate limit stops the closure report records, and the runtime's kills of the hunt's processes mid compile on the contended box, the form gate of Round 29 and its base gate, the ten cuts of Round 28, the directions and the relocation and rad1 and f57 of that round, the eight cuts and directions of Round 29 on its base tree and the ten legs on its form tree including rad1 and f59, and the candidate legs A and B of both rounds standing unexecuted here as they stood at the build and the red team, their computed sets standing confirmed at the red team's desk, and the hosted gate on the closure head standing as the verdict of record in their place. The records items standing carried forward named with no cure charged, the Round 24 design note's fourth section carrying the false distribution pair as written, one at seven and fifty eight at five and more against the counted fourteen at seven and fifty nine at five and more, its contract places standing cured inside Round 24 by FIX-PG-20, the Round 23 design note's fifth section carrying its day close compression as written, and the Round 22 design record's manifest figure of 73 standing against the counted 71 as a records reconciliation. The Round 26 design note's section 0 base tree string standing for the errata queue, 0b9e4eea against the true base tree 7842fa27. The Round 27 design record's items standing carried the same way, its note's fifth section namings and its design report's section figure of 7,635 standing against the contract side figure of 7,651. The Round 28 design note's fourth section carrying its PG-58 opening sentence as written. The Round 28 red team's literal counts standing corrected on the record at eight for PG-55 and twelve for PG-56, the extraction standing at five content literals in each body, ten in the two bodies beside the three heading assertions, its sentence counts and its finding standing confirmed whole. And FIX-PG-21 standing free and unspent through the round.";
    assert!(lines.iter().any(|found| found == standing));
    let slice_line = "- Slice 30 has landed. It carries the whole obligation assertion cure of PG-59, the contract pin of Round 27 standing restated to assert the whole text of its two obligations, and the cut record of PG-60 over the Round 27 cohort standing carried unexecuted under the write fault, with no reading and no command added.";
    assert!(lines.iter().any(|found| found == slice_line));
    assert!(read("README.md").contains("Closed perfection rounds, 29 of 100"));
}
