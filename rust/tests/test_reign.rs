//! Tests for PG-17 and PG-18, the reign reading over a meet.
//!
//! Written for Round 8 in the shape of the Round 7 table tests.
//! Each reign pin states its whole reading, so a pin fails when
//! the shared walk, the holder after a mark, a spell boundary,
//! a spell length, or a closing count moves, and the
//! reconciliation pins read the meet table and the reign
//! reading off the same meet and fail when the two readings of
//! the one walk disagree.

use std::path::PathBuf;

use num_bigint::BigInt;

use paceghost::fieldday::{DayRun, FieldDay};
use paceghost::meet::{load_meet_text, meet_reign, meet_table, Meet};
use paceghost::pynum::Num;
use paceghost::pytext::PText;
use paceghost::runs::Read;

fn day_json(window: i64, operator: &str, runs: &str) -> String {
    format!(
        "{{\"window_seconds\": {}, \"operator\": \"{}\", \"runs\": {}}}",
        window, operator, runs
    )
}

fn run_json(label: &str, usage: i64, reads: &[(i64, i64)]) -> String {
    if reads.is_empty() {
        return format!("{{\"runner\": \"{}\", \"usage\": {}}}", label, usage);
    }
    let reads_text = reads
        .iter()
        .map(|(elapsed, usage)| {
            format!("{{\"elapsed_seconds\": {}, \"usage\": {}}}", elapsed, usage)
        })
        .collect::<Vec<String>>()
        .join(", ");
    format!(
        "{{\"runner\": \"{}\", \"usage\": {}, \"reads\": [{}]}}",
        label, usage, reads_text
    )
}

fn runs_json(runs: &[String]) -> String {
    format!("[{}]", runs.join(", "))
}

fn meet_text(days: &[String]) -> String {
    format!("{{\"days\": [{}]}}", days.join(", "))
}

fn day(window: i64, operator: &str, runs: &[String]) -> String {
    day_json(window, operator, &runs_json(runs))
}

fn loaded(days: &[String]) -> Meet {
    let (meet, defects) = load_meet_text(&meet_text(days));
    assert_eq!(defects, Vec::<String>::new());
    meet.expect("the meet loads")
}

fn reign(days: &[String]) -> Vec<String> {
    meet_reign(&loaded(days)).expect("the reign reads")
}

/// The meet of the line for line and reconciliation pins, the
/// hand meet of the Round 7 table tests.
///
/// Window 600 on every day. Alpha holds the record after the
/// only mark of the first day and both marks of the second,
/// and aardvark takes it for both marks of the third, so the
/// reign is one spell of three marks for alpha spanning the
/// close of the first day and one spell of two for aardvark.
fn hand_meet() -> Vec<String> {
    vec![
        day(
            600,
            "alpha",
            &[
                run_json("alpha", 500, &[(300, 150)]),
                run_json("beta", 900, &[]),
                run_json("gamma", 600, &[(300, 200)]),
            ],
        ),
        day(
            600,
            "alpha",
            &[
                run_json("alpha", 500, &[(200, 80)]),
                run_json("beta", 900, &[(100, 90)]),
            ],
        ),
        day(
            600,
            "beta",
            &[
                run_json("beta", 900, &[(300, 60)]),
                run_json("aardvark", 100, &[(100, 10)]),
            ],
        ),
    ]
}

#[test]
fn test_the_reign_of_the_hand_meet_is_stated_line_for_line() {
    assert_eq!(
        reign(&hand_meet()),
        vec![
            "meet reign, 3 days",
            "spell, alpha, first mark at 300 seconds on the first day, last mark at 200 seconds on the second day, 3 marks",
            "spell, aardvark, first mark at 100 seconds on the third day, last mark at 300 seconds on the third day, 2 marks",
            "reign counts, marks walked 5, spells 2, holders 2, longest spell 3 marks held by alpha",
        ]
    );
}

fn spell_lengths_by_holder(lines: &[String]) -> Vec<(String, i64)> {
    lines
        .iter()
        .filter(|line| line.starts_with("spell, "))
        .map(|line| {
            let rest = line.trim_start_matches("spell, ");
            let label = rest
                .split(',')
                .next()
                .expect("the spell line names its holder")
                .to_string();
            let length = line
                .rsplit(", ")
                .next()
                .expect("the spell line names its length")
                .trim_end_matches(" marks")
                .parse::<i64>()
                .expect("the spell length is a whole number");
            (label, length)
        })
        .collect()
}

fn table_held_counts(lines: &[String]) -> Vec<(String, i64)> {
    lines
        .iter()
        .filter(|line| line.starts_with("competitor "))
        .map(|line| {
            let label = line
                .trim_start_matches("competitor ")
                .split(',')
                .next()
                .expect("the table line names its competitor")
                .to_string();
            let held = line
                .split("record held after ")
                .nth(1)
                .expect("the table line names a held count")
                .split(' ')
                .next()
                .expect("the held count stands")
                .parse::<i64>()
                .expect("the held count is a whole number");
            (label, held)
        })
        .collect()
}

#[test]
fn test_spell_lengths_reconcile_with_the_table_held_counts() {
    let meet = loaded(&hand_meet());
    let lines = meet_reign(&meet).expect("the reign reads");
    let table = meet_table(&meet).expect("the table reads");
    let mut spells = spell_lengths_by_holder(&lines);
    spells.sort();
    let mut summed: Vec<(String, i64)> = Vec::new();
    for (label, length) in spells {
        match summed
            .iter_mut()
            .find(|(held_label, _)| *held_label == label)
        {
            Some((_, total)) => *total += length,
            None => summed.push((label, length)),
        }
    }
    let mut held = table_held_counts(&table);
    held.retain(|(_, count)| *count > 0);
    held.sort();
    assert_eq!(summed, held);
    let walked: i64 = lines
        .last()
        .expect("the counts line stands")
        .split("marks walked ")
        .nth(1)
        .expect("the counts line names the marks")
        .split(',')
        .next()
        .expect("the marks count stands")
        .parse()
        .expect("the marks count is a whole number");
    let total: i64 = spell_lengths_by_holder(&lines)
        .iter()
        .map(|(_, length)| *length)
        .sum();
    assert_eq!(total, walked);
    assert_eq!(walked, 5);
}

#[test]
fn test_one_competitor_holding_every_mark_reads_one_spell() {
    // Ace projects the lowest figure at every mark of the
    // meet, so the whole walk is one spell of ace, spanning
    // both day closes.
    let days = vec![
        day(
            600,
            "ace",
            &[
                run_json("ace", 500, &[(100, 50)]),
                run_json("bee", 600, &[(100, 100)]),
            ],
        ),
        day(600, "ace", &[run_json("ace", 500, &[(200, 50)])]),
        day(
            600,
            "ace",
            &[
                run_json("ace", 500, &[]),
                run_json("bee", 700, &[(100, 200)]),
            ],
        ),
    ];
    assert_eq!(
        reign(&days),
        vec![
            "meet reign, 3 days",
            "spell, ace, first mark at 100 seconds on the first day, last mark at 100 seconds on the third day, 3 marks",
            "reign counts, marks walked 3, spells 1, holders 1, longest spell 3 marks held by ace",
        ]
    );
}

#[test]
fn test_a_lead_changing_at_every_mark_reads_one_spell_per_mark() {
    // Every mark of the walk changes the holder, so each spell
    // is one mark long, and the longest spell is the earliest
    // of them, the first spell of ace. Read usages rise within
    // each run, as the loader's cumulative usage rule requires,
    // while the projections still trade the lead at every mark.
    let days = vec![
        day(
            600,
            "ace",
            &[
                run_json("ace", 500, &[(100, 50), (300, 60)]),
                run_json("bee", 600, &[(200, 50)]),
            ],
        ),
        day(600, "bee", &[run_json("bee", 600, &[(100, 15)])]),
        day(600, "ace", &[run_json("ace", 500, &[(100, 10)])]),
    ];
    assert_eq!(
        reign(&days),
        vec![
            "meet reign, 3 days",
            "spell, ace, first mark at 100 seconds on the first day, last mark at 100 seconds on the first day, 1 marks",
            "spell, bee, first mark at 200 seconds on the first day, last mark at 200 seconds on the first day, 1 marks",
            "spell, ace, first mark at 300 seconds on the first day, last mark at 300 seconds on the first day, 1 marks",
            "spell, bee, first mark at 100 seconds on the second day, last mark at 100 seconds on the second day, 1 marks",
            "spell, ace, first mark at 100 seconds on the third day, last mark at 100 seconds on the third day, 1 marks",
            "reign counts, marks walked 5, spells 5, holders 2, longest spell 1 marks held by ace",
        ]
    );
}

#[test]
fn test_the_longest_spell_tie_goes_to_the_earliest_spell() {
    // Ace holds the first two marks and bee the next two, so
    // two spells share the greatest length and the closing
    // line names the earliest of them, the spell of ace.
    let days = vec![
        day(
            600,
            "ace",
            &[
                run_json("ace", 500, &[(100, 50), (200, 50)]),
                run_json("bee", 600, &[(300, 60)]),
            ],
        ),
        day(
            600,
            "bee",
            &[
                run_json("bee", 600, &[(100, 15)]),
                run_json("ace", 500, &[(200, 10)]),
            ],
        ),
        day(600, "ace", &[run_json("ace", 500, &[])]),
    ];
    assert_eq!(
        reign(&days),
        vec![
            "meet reign, 3 days",
            "spell, ace, first mark at 100 seconds on the first day, last mark at 200 seconds on the first day, 2 marks",
            "spell, bee, first mark at 300 seconds on the first day, last mark at 100 seconds on the second day, 2 marks",
            "spell, ace, first mark at 200 seconds on the second day, last mark at 200 seconds on the second day, 1 marks",
            "reign counts, marks walked 5, spells 3, holders 2, longest spell 2 marks held by ace",
        ]
    );
}

#[test]
fn test_a_day_with_no_projecting_read_neither_joins_nor_breaks_a_spell() {
    // The first day carries reads only at elapsed zero, which
    // project no figure, so the day closes with no record
    // stood and holds no mark, and the reign opens on the
    // second day with the one spell of ace unbroken.
    let days = vec![
        day(
            600,
            "ace",
            &[
                run_json("ace", 100, &[(0, 10)]),
                run_json("bee", 100, &[(0, 20)]),
            ],
        ),
        day(600, "ace", &[run_json("ace", 500, &[(100, 50)])]),
        day(
            600,
            "ace",
            &[
                run_json("ace", 500, &[(100, 25)]),
                run_json("bee", 900, &[(200, 300)]),
            ],
        ),
    ];
    assert_eq!(
        reign(&days),
        vec![
            "meet reign, 3 days",
            "spell, ace, first mark at 100 seconds on the second day, last mark at 200 seconds on the third day, 3 marks",
            "reign counts, marks walked 3, spells 1, holders 1, longest spell 3 marks held by ace",
        ]
    );
}

#[test]
fn test_a_meet_where_nothing_projects_reads_no_spell() {
    // Still reads only at elapsed zero, which projects no
    // figure, and mute holds no reads at all, so no mark
    // stands anywhere in the walk and no spell stands.
    let days = vec![
        day(
            600,
            "still",
            &[
                run_json("still", 100, &[(0, 10)]),
                run_json("mute", 100, &[]),
            ],
        ),
        day(600, "still", &[run_json("still", 100, &[(0, 20)])]),
        day(600, "still", &[run_json("still", 100, &[(0, 30)])]),
    ];
    assert_eq!(
        reign(&days),
        vec![
            "meet reign, 3 days",
            "reign counts, marks walked 0, spells 0, holders 0, no spell stood",
        ]
    );
}

#[test]
fn test_the_reign_is_a_pure_function_of_the_meet() {
    let meet = loaded(&hand_meet());
    let first = meet_reign(&meet).expect("the reign reads");
    let second = meet_reign(&meet).expect("the reign reads again");
    assert_eq!(first, second);
    let table = meet_table(&meet).expect("the table reads");
    assert_ne!(first, table);
}

fn hand_day(window: Num, runs: Vec<DayRun>) -> FieldDay {
    FieldDay {
        window_seconds: window,
        operator: PText::from_text("a"),
        runs,
    }
}

fn ordinary_run() -> DayRun {
    DayRun {
        runner: PText::from_text("a"),
        usage: None,
        reads: Some(vec![Read {
            elapsed_seconds: Num::from_i64(300),
            usage: Num::from_i64(100),
        }]),
    }
}

fn ordinary_day() -> FieldDay {
    hand_day(Num::from_i64(1800), vec![ordinary_run()])
}

#[test]
fn test_a_hand_built_meet_holding_no_days_names_the_precondition() {
    let error =
        meet_reign(&Meet { days: Vec::new() }).expect_err("the reign names the precondition");
    assert_eq!(error, "the meet holds no days");
}

#[test]
fn test_a_hand_built_meet_past_the_ordinal_bound_names_the_precondition() {
    let days: Vec<FieldDay> = (0..11).map(|_| ordinary_day()).collect();
    let error = meet_reign(&Meet { days }).expect_err("the reign names the precondition");
    assert_eq!(
        error,
        "the meet holds more days than the ordinal table names"
    );
}

#[test]
fn test_a_hand_built_meet_with_a_non_finite_window_names_it_in_day_order() {
    let bad_first = Meet {
        days: vec![
            ordinary_day(),
            hand_day(Num::from_f64(f64::NAN), vec![ordinary_run()]),
            ordinary_day(),
        ],
    };
    let error = meet_reign(&bad_first).expect_err("the reign names the precondition");
    assert_eq!(error, "the window is not a finite number");
    let oversized = {
        let mut value = BigInt::from(1);
        for _ in 0..400 {
            value *= 10;
        }
        Num::Int(value)
    };
    let bad_last = Meet {
        days: vec![
            ordinary_day(),
            ordinary_day(),
            hand_day(oversized, vec![ordinary_run()]),
        ],
    };
    let error = meet_reign(&bad_last).expect_err("the reign names the precondition");
    assert_eq!(error, "the window is not a finite number");
}

#[test]
fn test_a_hand_built_meet_with_a_non_finite_read_figure_names_it() {
    let bad_run = DayRun {
        runner: PText::from_text("a"),
        usage: None,
        reads: Some(vec![Read {
            elapsed_seconds: Num::from_i64(300),
            usage: Num::from_f64(f64::INFINITY),
        }]),
    };
    let meet = Meet {
        days: vec![
            hand_day(Num::from_i64(1800), vec![bad_run]),
            ordinary_day(),
            ordinary_day(),
        ],
    };
    let error = meet_reign(&meet).expect_err("the reign names the precondition");
    assert_eq!(error, "a read usage value is not a finite number");
}

#[test]
fn test_a_hand_built_meet_with_no_runs_reads_no_spell() {
    let days: Vec<FieldDay> = (0..3)
        .map(|_| hand_day(Num::from_i64(1800), Vec::new()))
        .collect();
    let lines = meet_reign(&Meet { days }).expect("the reign reads");
    assert_eq!(
        lines,
        vec![
            "meet reign, 3 days",
            "reign counts, marks walked 0, spells 0, holders 0, no spell stood",
        ]
    );
}

#[test]
fn test_the_example_reign_stands_as_hand_computed() {
    let examples = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples");
    let (meet, defects) = paceghost::meet::load_meet_file(&examples.join("meet.json"));
    assert_eq!(defects, Vec::<String>::new());
    let meet = meet.expect("the meet loads");
    let lines = meet_reign(&meet).expect("the reign reads");
    assert_eq!(
        lines,
        vec![
            "meet reign, 3 days",
            "spell, operator, first mark at 300 seconds on the first day, last mark at 400 seconds on the first day, 2 marks",
            "spell, dawn fork, first mark at 600 seconds on the first day, last mark at 600 seconds on the first day, 1 marks",
            "spell, operator, first mark at 700 seconds on the first day, last mark at 700 seconds on the first day, 1 marks",
            "spell, night fork, first mark at 900 seconds on the first day, last mark at 300 seconds on the second day, 5 marks",
            "spell, dawn fork, first mark at 400 seconds on the second day, last mark at 600 seconds on the third day, 7 marks",
            "reign counts, marks walked 16, spells 5, holders 3, longest spell 7 marks held by dawn fork",
        ]
    );
    let table = meet_table(&meet).expect("the table reads");
    let mut spells = spell_lengths_by_holder(&lines);
    spells.sort();
    let mut summed: Vec<(String, i64)> = Vec::new();
    for (label, length) in spells {
        match summed
            .iter_mut()
            .find(|(held_label, _)| *held_label == label)
        {
            Some((_, total)) => *total += length,
            None => summed.push((label, length)),
        }
    }
    let mut held = table_held_counts(&table);
    held.retain(|(_, count)| *count > 0);
    held.sort();
    assert_eq!(summed, held);
}
