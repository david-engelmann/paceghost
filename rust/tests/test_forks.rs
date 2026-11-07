//! Tests for PG-21 and PG-22, the forks reading over a meet.
//!
//! Written for Round 10 in the shape of the Round 9 days tests.
//! Each forks pin states its whole reading, so a pin fails when
//! the shared walk, a standing event, the marks a competitor
//! stood at, the days it stood in, a spell cut by holder, or a
//! closing count moves, and the reconciliation pins read the
//! meet report, the meet table, the reign reading, and the
//! forks reading off the same meet and fail when the four
//! readings of the one walk disagree.

use num_bigint::BigInt;

use paceghost::fieldday::{DayRun, FieldDay};
use paceghost::meet::{load_meet_text, meet_forks, meet_reign, meet_report, meet_table, Meet};
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

fn forks_reading(days: &[String]) -> Vec<String> {
    meet_forks(&loaded(days)).expect("the forks read")
}

/// The meet of the line for line and reconciliation pins, the
/// hand meet of the Round 7 table tests, the Round 8 reign
/// tests, and the Round 9 days tests.
///
/// Window 600 on every day. Alpha sets a standing of 300 at
/// the only mark of the first day and of 240 on the second
/// day, beta sets 540 on the second day and 120 on the third,
/// gamma sets 400 on the first day and holds it to the close
/// without ever holding the record, and aardvark enters on
/// the third day at 60 and takes the record there, so the
/// walk holds five marks, alpha holds a spell of three, and
/// aardvark holds a spell of two.
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
fn test_the_forks_of_the_hand_meet_are_stated_line_for_line() {
    assert_eq!(
        forks_reading(&hand_meet()),
        vec![
            "meet forks, 3 days, 4 competitors",
            "competitor alpha",
            "standing, at a projection of 300, set at 300 seconds on the first day",
            "standing, at a projection of 240, set at 200 seconds on the second day",
            "spell, first mark at 300 seconds on the first day, last mark at 200 seconds on the second day, 3 marks",
            "close, days stood in 2, marks stood at 5, spells held 1, standing projection 240 at the close",
            "competitor beta",
            "standing, at a projection of 540, set at 100 seconds on the second day",
            "standing, at a projection of 120, set at 300 seconds on the third day",
            "close, days stood in 3, marks stood at 4, spells held 0, standing projection 120 at the close",
            "competitor gamma",
            "standing, at a projection of 400, set at 300 seconds on the first day",
            "close, days stood in 1, marks stood at 5, spells held 0, standing projection 400 at the close",
            "competitor aardvark",
            "standing, at a projection of 60, set at 100 seconds on the third day",
            "spell, first mark at 100 seconds on the third day, last mark at 300 seconds on the third day, 2 marks",
            "close, days stood in 1, marks stood at 2, spells held 1, standing projection 60 at the close",
            "forks counts, competitors 4, marks stood at 16, spells 2",
        ]
    );
}

#[test]
fn test_the_section_spells_are_the_reign_spells_cut_by_holder() {
    let meet = loaded(&hand_meet());
    let lines = meet_forks(&meet).expect("the forks read");
    let reign = meet_reign(&meet).expect("the reign reads");
    let reign_spells: Vec<&String> = reign
        .iter()
        .filter(|line| line.starts_with("spell, "))
        .collect();
    // The forks spell lines, gathered section by section in
    // competitor order, are the reign spell lines in walk
    // order with the holder label read off each one, so the
    // two readings state one list of spells cut two ways.
    let mut forks_spells: Vec<String> = Vec::new();
    let mut holder = "";
    for line in &lines {
        if let Some(label) = line.strip_prefix("competitor ") {
            holder = label;
        }
        if let Some(rest) = line.strip_prefix("spell, ") {
            forks_spells.push(format!("spell, {}, {}", holder, rest));
        }
    }
    let reign_owned: Vec<String> = reign_spells.iter().map(|line| line.to_string()).collect();
    assert_eq!(forks_spells.len(), reign_owned.len());
    let mut forks_sorted = forks_spells.clone();
    let mut reign_sorted = reign_owned.clone();
    forks_sorted.sort();
    reign_sorted.sort();
    assert_eq!(forks_sorted, reign_sorted);
    let counts = lines.last().expect("the counts line stands");
    assert!(counts.ends_with(", spells 2"));
}

#[test]
fn test_the_section_closes_agree_with_the_meet_table_closes() {
    let meet = loaded(&hand_meet());
    let lines = meet_forks(&meet).expect("the forks read");
    let table = meet_table(&meet).expect("the table reads");
    for table_line in table.iter().filter(|line| line.starts_with("competitor ")) {
        let rest = table_line
            .strip_prefix("competitor ")
            .expect("the table line names its competitor");
        let (label, _) = rest
            .split_once(',')
            .expect("the table line names its competitor");
        let section_head = format!("competitor {}", label);
        let start = lines
            .iter()
            .position(|line| line == &section_head)
            .expect("the forks hold the section");
        let close = lines[start..]
            .iter()
            .find(|line| line.starts_with("close, "))
            .expect("the section closes");
        if table_line.contains("no standing projection at the close") {
            assert!(
                close.ends_with("no standing projection at the close"),
                "the forks close for {} names no standing",
                label
            );
        } else {
            let figure = table_line
                .rsplit("standing projection ")
                .next()
                .expect("the table close names its figure")
                .strip_suffix(" at the close")
                .expect("the table close ends at the close");
            assert!(
                close.ends_with(&format!("standing projection {} at the close", figure)),
                "the forks close for {} names the table close figure",
                label
            );
        }
    }
}

#[test]
fn test_the_marks_stood_at_sum_over_the_sections_to_the_counts_line() {
    let meet = loaded(&hand_meet());
    let lines = meet_forks(&meet).expect("the forks read");
    let report = meet_report(&meet).expect("the report reads");
    // The closing marks stood at counts the five marks of the
    // walk once for each competitor holding a standing after
    // the mark, two after the first mark, three after the
    // second and third, and four after the fourth and fifth,
    // which sums to sixteen, counted here off the section
    // closes alone and set against the counts line.
    let summed: i64 = lines
        .iter()
        .filter(|line| line.starts_with("close, "))
        .map(|line| {
            line.split("marks stood at ")
                .nth(1)
                .expect("the close names the marks stood at")
                .split(',')
                .next()
                .expect("the count stands")
                .parse::<i64>()
                .expect("the count is a whole number")
        })
        .sum();
    let counts = lines.last().expect("the counts line stands");
    let closing: i64 = counts
        .split("marks stood at ")
        .nth(1)
        .expect("the counts line names the marks stood at")
        .split(',')
        .next()
        .expect("the count stands")
        .parse()
        .expect("the count is a whole number");
    assert_eq!(summed, closing);
    assert_eq!(closing, 16);
    assert!(report.contains(&"days read, 3".to_string()));
}

#[test]
fn test_a_standing_event_stands_when_the_projection_rises() {
    // Solo is the only competitor. Its second read projects
    // higher than its first, and the standing line for that
    // read stands all the same, because a standing event is a
    // read applying and not a figure falling.
    let days = vec![
        day(600, "solo", &[run_json("solo", 500, &[(300, 150)])]),
        day(600, "solo", &[run_json("solo", 500, &[(200, 250)])]),
        day(600, "solo", &[run_json("solo", 500, &[(100, 10)])]),
    ];
    assert_eq!(
        forks_reading(&days),
        vec![
            "meet forks, 3 days, 1 competitors",
            "competitor solo",
            "standing, at a projection of 300, set at 300 seconds on the first day",
            "standing, at a projection of 750, set at 200 seconds on the second day",
            "standing, at a projection of 60, set at 100 seconds on the third day",
            "spell, first mark at 300 seconds on the first day, last mark at 100 seconds on the third day, 3 marks",
            "close, days stood in 3, marks stood at 3, spells held 1, standing projection 60 at the close",
            "forks counts, competitors 1, marks stood at 3, spells 1",
        ]
    );
}

#[test]
fn test_two_reads_at_one_elapsed_hold_one_standing_event() {
    // Handed a meet built by hand whose first day holds two
    // runs naming one competitor with reads at one elapsed
    // value, a shape the loader refuses, the collected events
    // of the walk key by the day and the elapsed value with
    // the later run's read standing, so the competitor holds
    // one standing line at that mark naming the later read's
    // projection and not the earlier one's.
    let read = |usage: i64| Read {
        elapsed_seconds: Num::from_i64(300),
        usage: Num::from_i64(usage),
    };
    let run = |usage: i64| DayRun {
        runner: PText::from_text("a"),
        usage: None,
        reads: Some(vec![read(usage)]),
    };
    let meet = Meet {
        days: vec![
            hand_day(Num::from_i64(1800), vec![run(100), run(50)]),
            ordinary_day(),
            ordinary_day(),
        ],
    };
    let lines = meet_forks(&meet).expect("the forks read");
    let standing_lines: Vec<&String> = lines
        .iter()
        .filter(|line| line.starts_with("standing, "))
        .collect();
    assert_eq!(
        standing_lines,
        vec![
            "standing, at a projection of 300, set at 300 seconds on the first day",
            "standing, at a projection of 600, set at 300 seconds on the second day",
            "standing, at a projection of 600, set at 300 seconds on the third day",
        ]
    );
}

#[test]
fn test_a_meet_where_nothing_projects_reads_every_section_empty() {
    // Reads only at elapsed zero project no figure and mute
    // holds no reads at all, so no standing event and no spell
    // stands anywhere in the walk and every section closes in
    // the none form.
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
        forks_reading(&days),
        vec![
            "meet forks, 3 days, 2 competitors",
            "competitor still",
            "close, days stood in 3, marks stood at 0, spells held 0, no standing projection at the close",
            "competitor mute",
            "close, days stood in 1, marks stood at 0, spells held 0, no standing projection at the close",
            "forks counts, competitors 2, marks stood at 0, spells 0",
        ]
    );
}

#[test]
fn test_the_forks_are_a_pure_function_of_the_meet() {
    let meet = loaded(&hand_meet());
    let first = meet_forks(&meet).expect("the forks read");
    let second = meet_forks(&meet).expect("the forks read again");
    assert_eq!(first, second);
    let reign = meet_reign(&meet).expect("the reign reads");
    assert_ne!(first, reign);
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
        meet_forks(&Meet { days: Vec::new() }).expect_err("the forks name the precondition");
    assert_eq!(error, "the meet holds no days");
}

#[test]
fn test_a_hand_built_meet_past_the_ordinal_bound_names_the_precondition() {
    let days: Vec<FieldDay> = (0..11).map(|_| ordinary_day()).collect();
    let error = meet_forks(&Meet { days }).expect_err("the forks name the precondition");
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
    let error = meet_forks(&bad_first).expect_err("the forks name the precondition");
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
    let error = meet_forks(&bad_last).expect_err("the forks name the precondition");
    assert_eq!(error, "the window is not a finite number");
}

#[test]
fn test_a_hand_built_meet_with_a_line_break_in_a_runner_label_names_it() {
    // The forks reading is born inside the FIX-PG-11 closure,
    // so a hand built meet carrying a runner label its loader
    // would refuse draws the named error and not lines.
    let bad_run = DayRun {
        runner: PText::from_text("a\nb"),
        usage: None,
        reads: Some(vec![Read {
            elapsed_seconds: Num::from_i64(300),
            usage: Num::from_i64(100),
        }]),
    };
    let meet = Meet {
        days: vec![
            ordinary_day(),
            hand_day(Num::from_i64(1800), vec![bad_run]),
            ordinary_day(),
        ],
    };
    let error = meet_forks(&meet).expect_err("the forks name the precondition");
    assert_eq!(error, "a runner label holds a line break");
}
