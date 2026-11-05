//! Tests for PG-13 and PG-14, the meet loader and reading.
//!
//! Written for Round 6. Each reading pin states its whole report,
//! so a pin fails when the walk, the tie discipline, the day
//! ordering, a closing form, or the record set line moves, and the
//! loader pins state the whole defect list, so a pin fails when a
//! phrase or its day prefix moves.

use std::path::PathBuf;

use num_bigint::BigInt;

use paceghost::fieldday::{DayRun, FieldDay};
use paceghost::meet::{load_meet_file, load_meet_text, meet_report, Meet};
use paceghost::pynum::Num;
use paceghost::pytext::PText;
use paceghost::rematch::{load_rematch_text, rematch_report};
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

fn std_day(runs: &[String]) -> String {
    day_json(1800, "operator", &runs_json(runs))
}

fn loaded(days: &[String]) -> Meet {
    let (meet, defects) = load_meet_text(&meet_text(days));
    assert_eq!(defects, Vec::<String>::new());
    meet.expect("the meet loads")
}

fn report(days: &[String]) -> Vec<String> {
    meet_report(&loaded(days)).expect("the report reads")
}

fn three_std_days() -> Vec<String> {
    vec![
        std_day(&[run_json("operator", 900, &[(300, 150)])]),
        std_day(&[run_json("operator", 800, &[(300, 100)])]),
        std_day(&[run_json("operator", 700, &[(300, 50)])]),
    ]
}

fn temp_path(name: &str) -> PathBuf {
    std::env::temp_dir().join(format!("paceghost-meet-{}-{}", std::process::id(), name))
}

#[test]
fn test_a_clean_meet_loads_carrying_every_day_in_list_order() {
    let days = vec![
        day_json(
            100,
            "alpha",
            &runs_json(&[run_json("alpha", 900, &[(10, 50)])]),
        ),
        day_json(
            200,
            "beta",
            &runs_json(&[run_json("beta", 800, &[(20, 60)])]),
        ),
        day_json(
            300,
            "gamma",
            &runs_json(&[run_json("gamma", 700, &[(30, 70)])]),
        ),
    ];
    let meet = loaded(&days);
    assert_eq!(meet.days.len(), 3);
    assert_eq!(meet.days[0].window_seconds, Num::from_i64(100));
    assert_eq!(meet.days[1].window_seconds, Num::from_i64(200));
    assert_eq!(meet.days[2].window_seconds, Num::from_i64(300));
    assert!(meet.days[1].operator.eq_str("beta"));
    assert!(meet.days[2].runs[0].runner.eq_str("gamma"));
}

#[test]
fn test_extra_fields_are_ignored() {
    let mut days = three_std_days();
    days[0] = format!(
        "{}, \"standing\": \"ignored\"}}",
        &days[0][..days[0].len() - 1]
    );
    let text = format!("{{\"days\": [{}], \"meet\": \"ignored\"}}", days.join(", "));
    let (meet, defects) = load_meet_text(&text);
    assert_eq!(defects, Vec::<String>::new());
    assert!(meet.is_some());
}

#[test]
fn test_text_that_is_not_json_is_refused() {
    let (meet, defects) = load_meet_text("not json");
    assert!(meet.is_none());
    assert_eq!(defects, vec!["input is not valid JSON"]);
}

#[test]
fn test_input_nested_too_deeply_is_refused() {
    let text = format!("{}{}", "[".repeat(100000), "]".repeat(100000));
    let (meet, defects) = load_meet_text(&text);
    assert!(meet.is_none());
    assert_eq!(defects, vec!["input is not valid JSON"]);
}

#[test]
fn test_json_that_is_not_an_object_is_refused() {
    let (meet, defects) = load_meet_text("[1, 2]");
    assert!(meet.is_none());
    assert_eq!(defects, vec!["input is not a JSON object"]);
}

#[test]
fn test_a_missing_days_field_is_named() {
    let (meet, defects) = load_meet_text("{}");
    assert!(meet.is_none());
    assert_eq!(defects, vec!["missing field days"]);
}

#[test]
fn test_a_days_field_of_the_wrong_type_is_named() {
    for text in ["{\"days\": {}}", "{\"days\": 3}", "{\"days\": \"three\"}"] {
        let (meet, defects) = load_meet_text(text);
        assert!(meet.is_none());
        assert_eq!(defects, vec!["field days has the wrong type"]);
    }
}

#[test]
fn test_an_empty_days_list_is_named() {
    let (meet, defects) = load_meet_text("{\"days\": []}");
    assert!(meet.is_none());
    assert_eq!(defects, vec!["field days is empty"]);
}

#[test]
fn test_a_one_day_meet_is_refused() {
    let (meet, defects) = load_meet_text(&meet_text(&three_std_days()[..1]));
    assert!(meet.is_none());
    assert_eq!(defects, vec!["field days holds one day"]);
}

#[test]
fn test_a_two_day_meet_is_refused_as_the_count_of_the_rematch() {
    let (meet, defects) = load_meet_text(&meet_text(&three_std_days()[..2]));
    assert!(meet.is_none());
    assert_eq!(
        defects,
        vec!["field days holds two days, the count of the rematch"]
    );
}

#[test]
fn test_a_meet_past_the_ordinal_bound_is_refused_at_load() {
    let days: Vec<String> = (0..11)
        .map(|_| std_day(&[run_json("operator", 900, &[(300, 150)])]))
        .collect();
    let (meet, defects) = load_meet_text(&meet_text(&days));
    assert!(meet.is_none());
    assert_eq!(
        defects,
        vec!["field days holds more days than the ordinal table names"]
    );
}

#[test]
fn test_a_meet_at_the_ordinal_bound_loads() {
    let days: Vec<String> = (0..10)
        .map(|_| std_day(&[run_json("operator", 900, &[(300, 150)])]))
        .collect();
    let meet = loaded(&days);
    assert_eq!(meet.days.len(), 10);
}

#[test]
fn test_a_day_that_is_not_an_object_is_named_by_its_index() {
    let mut days = three_std_days();
    days[1] = "42".to_string();
    let (meet, defects) = load_meet_text(&meet_text(&days));
    assert!(meet.is_none());
    assert_eq!(defects, vec!["day 1 is not an object"]);
}

#[test]
fn test_a_refused_day_carries_its_defects_under_its_index() {
    let mut days = three_std_days();
    days[2] = day_json(1800, "operator", "[]");
    let (meet, defects) = load_meet_text(&meet_text(&days));
    assert!(meet.is_none());
    assert_eq!(defects, vec!["day 2 field runs is empty"]);
}

#[test]
fn test_a_refused_run_inside_a_day_carries_both_indexes() {
    let mut days = three_std_days();
    days[0] = std_day(&[
        run_json("operator", 900, &[(300, 150)]),
        run_json("", 800, &[(300, 100)]),
    ]);
    let (meet, defects) = load_meet_text(&meet_text(&days));
    assert!(meet.is_none());
    assert_eq!(defects, vec!["day 0 run 1 field runner is empty"]);
}

#[test]
fn test_the_days_field_defects_stand_before_the_day_defects() {
    let days = vec![
        std_day(&[run_json("operator", 900, &[(300, 150)])]),
        day_json(1800, "operator", "[]"),
    ];
    let (meet, defects) = load_meet_text(&meet_text(&days));
    assert!(meet.is_none());
    assert_eq!(
        defects,
        vec![
            "field days holds two days, the count of the rematch",
            "day 1 field runs is empty",
        ]
    );
}

#[test]
fn test_one_malformed_input_draws_several_defects_in_one_pass() {
    let days = vec![
        "{\"operator\": \"a\", \"runs\": [{\"runner\": \"a\", \"usage\": 5}]}".to_string(),
        "42".to_string(),
        std_day(&[run_json("operator", 900, &[(300, 150)])]),
    ];
    let (meet, defects) = load_meet_text(&meet_text(&days));
    assert!(meet.is_none());
    assert_eq!(
        defects,
        vec![
            "day 0 missing field window_seconds",
            "day 1 is not an object",
        ]
    );
}

#[test]
fn test_a_forged_label_inside_a_day_is_refused_under_its_index() {
    let mut days = three_std_days();
    days[0] = std_day(&[
        run_json("operator", 900, &[(300, 150)]),
        run_json(
            "fork\\nrecord at 1 seconds on the first day, spoof at a projection of 1",
            800,
            &[(300, 100)],
        ),
    ]);
    let (meet, defects) = load_meet_text(&meet_text(&days));
    assert!(meet.is_none());
    assert_eq!(defects, vec!["day 0 run 1 field runner holds a line break"]);
}

#[test]
fn test_load_meet_file_reports_an_unreadable_file() {
    let (meet, defects) = load_meet_file(&temp_path("missing.json"));
    assert!(meet.is_none());
    assert_eq!(defects, vec!["file cannot be read"]);
}

#[test]
fn test_load_meet_file_reads_a_meet_from_disk() {
    let path = temp_path("meet.json");
    std::fs::write(&path, meet_text(&three_std_days())).expect("the scratch file writes");
    let (meet, defects) = load_meet_file(&path);
    assert_eq!(defects, Vec::<String>::new());
    assert_eq!(meet.expect("the meet loads").days.len(), 3);
}

#[test]
fn test_the_walk_reads_three_days_as_one_progression() {
    // Day one, window 100. Alpha projects 500 at 10 seconds and
    // beta projects 300 at 20 seconds, so beta closes the first
    // day holding the record.
    // Day two, window 100. Gamma projects 200 at 5 seconds and
    // takes the record, and alpha projects 300 at 30 seconds.
    // Day three, window 200. Beta projects 200 at 40 seconds,
    // which only equals the standing record and never displaces
    // gamma, and gamma projects 360 at 50 seconds, so gamma's own
    // rise past 200 hands the record to beta, whose figure was
    // set at 40 seconds on the third day.
    let days = vec![
        day_json(
            100,
            "alpha",
            &runs_json(&[
                run_json("alpha", 900, &[(10, 50)]),
                run_json("beta", 800, &[(20, 60)]),
            ]),
        ),
        day_json(
            100,
            "gamma",
            &runs_json(&[
                run_json("gamma", 700, &[(5, 10)]),
                run_json("alpha", 950, &[(30, 90)]),
            ]),
        ),
        day_json(
            200,
            "beta",
            &runs_json(&[
                run_json("beta", 600, &[(40, 40)]),
                run_json("gamma", 650, &[(50, 90)]),
            ]),
        ),
    ];
    assert_eq!(
        report(&days),
        vec![
            "meet, 3 days",
            "record at 10 seconds on the first day, alpha at a projection of 500",
            "record at 20 seconds on the first day, beta at a projection of 300",
            "close of the first day, record held by beta at a projection of 300",
            "record at 5 seconds on the second day, gamma at a projection of 200",
            "close of the second day, record held by gamma at a projection of 200",
            "record at 50 seconds on the third day, beta at a projection of 200",
            "close of the third day, record held by beta at a projection of 200",
            "finish, record held by beta at a projection of 200",
            "record set on the third day at 40 seconds",
            "days read, 3",
        ]
    );
}

#[test]
fn test_the_first_two_days_agree_with_the_rematch_reading() {
    let first = day_json(
        100,
        "alpha",
        &runs_json(&[
            run_json("alpha", 900, &[(10, 50)]),
            run_json("beta", 800, &[(20, 60)]),
        ]),
    );
    let second = day_json(
        100,
        "gamma",
        &runs_json(&[
            run_json("gamma", 700, &[(5, 10)]),
            run_json("alpha", 950, &[(30, 90)]),
        ]),
    );
    let third = day_json(100, "delta", &runs_json(&[run_json("delta", 500, &[])]));
    let (rematch, defects) =
        load_rematch_text(&format!("{{\"first\": {}, \"second\": {}}}", first, second));
    assert_eq!(defects, Vec::<String>::new());
    let rematch_lines =
        rematch_report(&rematch.expect("the rematch loads")).expect("the rematch report reads");
    let rematch_records: Vec<String> = rematch_lines
        .iter()
        .filter(|line| line.starts_with("record at "))
        .cloned()
        .collect();
    let meet_lines = report(&[first, second, third]);
    let meet_records: Vec<String> = meet_lines
        .iter()
        .filter(|line| line.starts_with("record at "))
        .cloned()
        .collect();
    assert_eq!(meet_records, rematch_records);
    assert_eq!(
        meet_lines,
        vec![
            "meet, 3 days",
            "record at 10 seconds on the first day, alpha at a projection of 500",
            "record at 20 seconds on the first day, beta at a projection of 300",
            "close of the first day, record held by beta at a projection of 300",
            "record at 5 seconds on the second day, gamma at a projection of 200",
            "close of the second day, record held by gamma at a projection of 200",
            "close of the third day, record held by gamma at a projection of 200",
            "finish, record held by gamma at a projection of 200",
            "record set on the second day at 5 seconds",
            "days read, 3",
        ]
    );
}

#[test]
fn test_a_record_carried_over_a_readless_day_still_stands_at_its_close() {
    let days = vec![
        day_json(
            100,
            "alpha",
            &runs_json(&[
                run_json("alpha", 900, &[(10, 50)]),
                run_json("beta", 800, &[(20, 60)]),
            ]),
        ),
        day_json(
            100,
            "alpha",
            &runs_json(&[run_json("alpha", 900, &[]), run_json("beta", 800, &[])]),
        ),
        day_json(
            100,
            "gamma",
            &runs_json(&[run_json("gamma", 700, &[(5, 10)])]),
        ),
    ];
    assert_eq!(
        report(&days),
        vec![
            "meet, 3 days",
            "record at 10 seconds on the first day, alpha at a projection of 500",
            "record at 20 seconds on the first day, beta at a projection of 300",
            "close of the first day, record held by beta at a projection of 300",
            "close of the second day, record held by beta at a projection of 300",
            "record at 5 seconds on the third day, gamma at a projection of 200",
            "close of the third day, record held by gamma at a projection of 200",
            "finish, record held by gamma at a projection of 200",
            "record set on the third day at 5 seconds",
            "days read, 3",
        ]
    );
}

#[test]
fn test_a_record_set_on_the_first_day_names_the_first_day() {
    let days = vec![
        day_json(
            100,
            "alpha",
            &runs_json(&[run_json("alpha", 900, &[(10, 20)])]),
        ),
        day_json(
            100,
            "beta",
            &runs_json(&[run_json("beta", 800, &[(10, 50)])]),
        ),
        day_json(
            100,
            "beta",
            &runs_json(&[run_json("beta", 800, &[(10, 50)])]),
        ),
    ];
    assert_eq!(
        report(&days),
        vec![
            "meet, 3 days",
            "record at 10 seconds on the first day, alpha at a projection of 200",
            "close of the first day, record held by alpha at a projection of 200",
            "close of the second day, record held by alpha at a projection of 200",
            "close of the third day, record held by alpha at a projection of 200",
            "finish, record held by alpha at a projection of 200",
            "record set on the first day at 10 seconds",
            "days read, 3",
        ]
    );
}

#[test]
fn test_a_meet_with_no_reads_sets_no_record() {
    let days = vec![
        std_day(&[run_json("operator", 900, &[])]),
        std_day(&[run_json("operator", 800, &[])]),
        std_day(&[run_json("operator", 700, &[])]),
    ];
    assert_eq!(
        report(&days),
        vec![
            "meet, 3 days",
            "close of the first day, no record stood",
            "close of the second day, no record stood",
            "close of the third day, no record stood",
            "finish, no record was set",
            "days read, 3",
        ]
    );
}

#[test]
fn test_the_later_ordinal_words_name_the_later_days() {
    let days: Vec<String> = (0..5)
        .map(|_| std_day(&[run_json("operator", 900, &[])]))
        .collect();
    let lines = report(&days);
    assert_eq!(lines[0], "meet, 5 days");
    assert_eq!(lines[4], "close of the fourth day, no record stood");
    assert_eq!(lines[5], "close of the fifth day, no record stood");
    assert_eq!(lines[7], "days read, 5");
}

#[test]
fn test_a_ten_day_meet_names_the_tenth_day() {
    let days: Vec<String> = (0..10)
        .map(|_| std_day(&[run_json("operator", 900, &[])]))
        .collect();
    let lines = report(&days);
    assert_eq!(lines[0], "meet, 10 days");
    assert_eq!(lines[10], "close of the tenth day, no record stood");
    assert_eq!(lines[12], "days read, 10");
}

// The hand built pins, in the discipline of FIX-PG-8 and
// FIX-PG-10. Each one hands the reading a value its loader would
// refuse and states the error or the report the reading must
// return, so a pin fails if the reading ever fails bare on such a
// value or converts a figure that is not finite.

fn hand_day(window: Num, runs: Vec<DayRun>) -> FieldDay {
    FieldDay {
        window_seconds: window,
        operator: PText::from_text("a"),
        runs,
    }
}

fn hand_run(reads: Vec<Read>) -> DayRun {
    DayRun {
        runner: PText::from_text("a"),
        usage: None,
        reads: Some(reads),
    }
}

fn ordinary_run() -> DayRun {
    hand_run(vec![Read {
        elapsed_seconds: Num::from_i64(300),
        usage: Num::from_i64(100),
    }])
}

fn ordinary_day() -> FieldDay {
    hand_day(Num::from_i64(1800), vec![ordinary_run()])
}

fn non_finite_figures() -> Vec<Num> {
    vec![
        Num::from_f64(f64::NAN),
        Num::from_f64(f64::INFINITY),
        Num::from_f64(f64::NEG_INFINITY),
    ]
}

fn oversized_integer() -> Num {
    let mut value = BigInt::from(1);
    for _ in 0..400 {
        value *= 10;
    }
    Num::Int(value)
}

#[test]
fn test_a_hand_built_meet_holding_no_days_names_the_precondition() {
    let error =
        meet_report(&Meet { days: Vec::new() }).expect_err("the report names the precondition");
    assert_eq!(error, "the meet holds no days");
}

#[test]
fn test_a_hand_built_meet_past_the_bound_names_the_precondition() {
    let meet = Meet {
        days: (0..11).map(|_| ordinary_day()).collect(),
    };
    let error = meet_report(&meet).expect_err("the report names the precondition");
    assert_eq!(
        error,
        "the meet holds more days than the ordinal table names"
    );
}

#[test]
fn test_a_hand_built_meet_of_one_or_two_days_still_renders() {
    // The count discipline is the loader's. A hand built meet of
    // one day renders under the first day word, and a hand built
    // meet of two days renders the rematch span of the walk.
    let one = meet_report(&Meet {
        days: vec![hand_day(Num::from_i64(100), vec![ordinary_run()])],
    })
    .expect("the report reads");
    assert_eq!(
        one,
        vec![
            "meet, 1 days",
            "record at 300 seconds on the first day, a at a projection of 33.333333",
            "close of the first day, record held by a at a projection of 33.333333",
            "finish, record held by a at a projection of 33.333333",
            "record set on the first day at 300 seconds",
            "days read, 1",
        ]
    );
    let two = meet_report(&Meet {
        days: vec![ordinary_day(), ordinary_day()],
    })
    .expect("the report reads");
    assert_eq!(two[0], "meet, 2 days");
    assert_eq!(two[two.len() - 1], "days read, 2");
}

#[test]
fn test_a_non_finite_window_on_a_later_day_names_the_precondition() {
    for figure in non_finite_figures() {
        let meet = Meet {
            days: vec![
                ordinary_day(),
                ordinary_day(),
                hand_day(figure, vec![ordinary_run()]),
            ],
        };
        let error = meet_report(&meet).expect_err("the report names the precondition");
        assert_eq!(error, "the window is not a finite number");
    }
    let meet = Meet {
        days: vec![
            ordinary_day(),
            ordinary_day(),
            hand_day(oversized_integer(), vec![ordinary_run()]),
        ],
    };
    let error = meet_report(&meet).expect_err("the report names the precondition");
    assert_eq!(error, "the window is not a finite number");
}

#[test]
fn test_a_non_finite_run_usage_on_a_later_day_names_the_precondition() {
    let bad = DayRun {
        runner: PText::from_text("a"),
        usage: Some(Num::from_f64(f64::NAN)),
        reads: Some(vec![Read {
            elapsed_seconds: Num::from_i64(300),
            usage: Num::from_i64(100),
        }]),
    };
    let meet = Meet {
        days: vec![
            ordinary_day(),
            ordinary_day(),
            hand_day(Num::from_i64(1800), vec![bad]),
        ],
    };
    let error = meet_report(&meet).expect_err("the report names the precondition");
    assert_eq!(error, "a run usage value is not a finite number");
}

#[test]
fn test_a_non_finite_read_elapsed_on_a_later_day_names_the_precondition() {
    for figure in non_finite_figures() {
        let bad = hand_run(vec![Read {
            elapsed_seconds: figure,
            usage: Num::from_i64(100),
        }]);
        let meet = Meet {
            days: vec![
                ordinary_day(),
                ordinary_day(),
                hand_day(Num::from_i64(1800), vec![bad]),
            ],
        };
        let error = meet_report(&meet).expect_err("the report names the precondition");
        assert_eq!(error, "a read elapsed value is not a finite number");
    }
}

#[test]
fn test_a_non_finite_read_usage_on_a_later_day_names_the_precondition() {
    for figure in non_finite_figures() {
        let bad = hand_run(vec![Read {
            elapsed_seconds: Num::from_i64(300),
            usage: figure,
        }]);
        let meet = Meet {
            days: vec![
                ordinary_day(),
                ordinary_day(),
                hand_day(Num::from_i64(1800), vec![bad]),
            ],
        };
        let error = meet_report(&meet).expect_err("the report names the precondition");
        assert_eq!(error, "a read usage value is not a finite number");
    }
}

#[test]
fn test_the_finite_guard_checks_the_days_in_list_order() {
    // The second day carries a read usage that is not finite and
    // the third day carries a window that is not finite. The
    // guard walks the days in list order, so the error names the
    // second day's broken figure.
    let bad_usage = hand_run(vec![Read {
        elapsed_seconds: Num::from_i64(300),
        usage: Num::from_f64(f64::INFINITY),
    }]);
    let meet = Meet {
        days: vec![
            ordinary_day(),
            hand_day(Num::from_i64(1800), vec![bad_usage]),
            hand_day(Num::from_f64(f64::NAN), vec![ordinary_run()]),
        ],
    };
    let error = meet_report(&meet).expect_err("the report names the precondition");
    assert_eq!(error, "a read usage value is not a finite number");
}
