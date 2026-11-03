"""Setup pins. Each test can fail for a real reason against a repo that
lost a setup file or let its status drift from its contract."""
import pathlib, re
ROOT = pathlib.Path(__file__).resolve().parent.parent

def test_agents_carries_the_canonical_marker():
    text = (ROOT / "AGENTS.md").read_text(encoding="utf-8")
    assert "<!-- repo annex below this line -->" in text
    assert "39 repo storefront perfection program" in text

def test_readme_states_the_closed_round_count():
    text = (ROOT / "README.md").read_text(encoding="utf-8")
    assert re.search(r"Closed perfection rounds, 4 of 100", text)

def test_contract_carries_the_setup_obligations():
    text = (ROOT / "CONTRACT.md").read_text(encoding="utf-8")
    for oid in ("SETUP-1", "SETUP-2", "SETUP-3", "SETUP-4", "PG-1"):
        assert oid in text, oid

def test_readme_reports_the_contract_status():
    text = (ROOT / "README.md").read_text(encoding="utf-8")
    expected = (
        "- Contract, SETUP-1 to SETUP-4 and PG-1 to PG-8 met. "
        "PG-1 to PG-4 stand in full form in CONTRACT.md with the Round 1 "
        "repair amendments, and Round 1 closed on the merged slice 2 after "
        "three reviews, a repair pass, and an approving re-review. "
        "PG-5 and PG-6 stand in full form with the Round 2 repair amendment."
    )
    assert expected in text.splitlines()

def test_readme_reports_the_round_three_status():
    text = (ROOT / "README.md").read_text(encoding="utf-8")
    lines = text.splitlines()
    standing = (
        "- Contract, PG-7 and PG-8 met. They stand in full form in "
        "CONTRACT.md, and Round 3 closed on the merged slice 4 after "
        "three reviews, a repair pass, and an approving re-review."
    )
    assert standing in lines
    slice_line = (
        "- Slice 4 has landed. It carries the field day "
        "loader that names every malformed field, the stolen record "
        "report that walks the record progression of one window across "
        "a field of recorded runs, and the fieldday command, with a "
        "synthetic example field day and its golden report under examples."
    )
    assert slice_line in lines

def test_readme_reports_the_round_four_status():
    text = (ROOT / "README.md").read_text(encoding="utf-8")
    lines = text.splitlines()
    standing = (
        "- Contract, PG-9 and PG-10 met. They stand in full form in "
        "CONTRACT.md with the Round 4 repair amendments, and Round 4 "
        "closed on the merged slice 5 after three reviews, a repair "
        "pass, and an approving re-review."
    )
    assert standing in lines
    slice_line = (
        "- Slice 5 has landed. It carries the ladder "
        "loader that names every malformed field in a ladder file, the "
        "ladder report that reads one field day as it stood at each "
        "window in a list and closes with a summary line over the "
        "closing holders, and the ladder command, with a synthetic "
        "example ladder and its golden report under examples."
    )
    assert slice_line in lines
    assert text.count("built and in review") == 0

def test_ci_runs_the_same_gate_as_the_local_gate():
    text = (ROOT / ".github" / "workflows" / "ci.yml").read_text(encoding="utf-8")
    live = "\n".join(line.split("#", 1)[0] for line in text.splitlines())
    assert "python3 scripts/codepoint_scan.py" in live
    assert "python3 -m pytest tests -q" in live
    # Added with the port review repairs, by addition only. The port
    # gate of PORT-7 stands in CI beside the cargo test step.
    assert "cargo test" in live
    assert "cargo fmt --check" in live
    assert "cargo clippy --all-targets -- -D warnings" in live
