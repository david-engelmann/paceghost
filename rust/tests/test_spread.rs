//! Tests for PG-31 and PG-32, the spread reading over a meet.
//!
//! Written for Round 15. Each reading pin states its whole
//! reading or a named line of it, so a pin fails when the
//! walk, the farthest standing selection, the field count,
//! a phrase form, or a closing count moves. The corners
//! stand pinned at their edges: a holder alone at the first
//! mark of a meet and at every mark of a meet, the whole
//! field standing at the record figure for a spread of
//! zero, a competitor at the record figure while another
//! stands beyond it, two competitors tied at the farthest
//! figure with their standings set at different marks and
//! at one mark, two marks tied at the fullest field, the
//! field growing as standings are set, and a day holding
//! no marks standing with its section empty. The relation
//! pins read the marks reading and the standings reading
//! over the same meet and fail when the spread reading
//! drifts from them, and the guard pins hold the reading
//! inside the fencing closure of FIX-PG-11 and the figure
//! guard of FIX-PG-10 from its birth. Every pin in this
//! file fails before the round by the absence of the
//! reading, the meet_spread function not standing on the
//! pre round tree, and on the built tree each fails
//! against a wrong construction of the reading.

use paceghost::fieldday::{DayRun, FieldDay};
use paceghost::meet::{load_meet_text, meet_marks, meet_report, meet_spread, meet_standings, Meet};
use paceghost::pynum::Num;
use paceghost::pytext::PText;
use paceghost::runs::Read;

fn day(window: i64, operator: &str, runs: &str) -> String {
    format!(
        "{{\"window_seconds\": {}, \"operator\": \"{}\", \"runs\": {}}}",
        window, operator, runs
    )
}

fn run(label: &str, usage: i64, reads: &[(i64, i64)]) -> String {
    if reads.is_empty() {
        return format!("{{\"runner\": \"{}\", \"usage\": {}}}", label, usage);
    }
    let parts: Vec<String> = reads
        .iter()
        .map(|(elapsed, used)| format!("{{\"elapsed_seconds\": {}, \"usage\": {}}}", elapsed, used))
        .collect();
    format!(
        "{{\"runner\": \"{}\", \"usage\": {}, \"reads\": [{}]}}",
        label,
        usage,
        parts.join(", ")
    )
}

fn load_meet(days: &[String]) -> Meet {
    let text = format!("{{\"days\": [{}]}}", days.join(", "));
    let (meet, defects) = load_meet_text(&text);
    assert_eq!(defects, Vec::<String>::new());
    meet.expect("the meet loads")
}

fn quiet_day(window: i64, operator: &str, label: &str) -> String {
    day(window, operator, &format!("[{}]", run(label, 500, &[])))
}

/// A meet over three competitors under one window, where
/// the holder stands alone at the first mark, the farthest
/// standing changes hands on the third day when a standing
/// is replaced by a higher figure, the second day holds no
/// marks at all, and one mark stands at a spread of zero.
fn field_meet() -> Meet {
    load_meet(&[
        day(
            1000,
            "a",
            &format!(
                "[{}, {}, {}]",
                run("a", 500, &[(100, 10), (300, 45)]),
                run("b", 400, &[(200, 30)]),
                run("c", 300, &[])
            ),
        ),
        quiet_day(1000, "a", "a"),
        day(
            1000,
            "a",
            &format!(
                "[{}, {}, {}]",
                run("a", 500, &[]),
                run("c", 200, &[(100, 20)]),
                run("b", 350, &[(400, 110)])
            ),
        ),
    ])
}

#[test]
fn test_the_spread_reading_over_a_hand_built_meet_line_by_line() {
    let lines = meet_spread(&field_meet()).expect("the spread read");
    assert_eq!(
        lines,
        vec![
            "meet spread, 3 days",
            "day, the first day, marks 3",
            "mark 1, at 100 seconds on the first day, a at a projection of 100, field of 1, and no other standing",
            "mark 2, at 200 seconds on the first day, a at a projection of 100, field of 2, farthest standing 150 held by b, spread 50",
            "mark 3, at 300 seconds on the first day, a at a projection of 150, field of 2, farthest standing 150 held by b, spread 0",
            "day, the second day, marks 0",
            "day, the third day, marks 2",
            "mark 4, at 100 seconds on the third day, a at a projection of 150, field of 3, farthest standing 200 held by c, spread 50",
            "mark 5, at 400 seconds on the third day, a at a projection of 150, field of 3, farthest standing 275 held by b, spread 125",
            "spread counts, marks walked 5, marks with another standing 4, days 3, fullest field 3 at 100 seconds on the third day",
        ]
    );
}

// The relation pins of PG-32. The spread reading agrees
// with the marks reading on the record side of every
// mark, line by line, and at the last mark of each day
// holding marks its field and farthest standing are the
// figures the standings reading computes at that day's
// close.

fn mark_lines(lines: &[String]) -> Vec<&String> {
    lines
        .iter()
        .filter(|line| line.starts_with("mark "))
        .collect()
}

fn record_prefix(mark_line: &str) -> String {
    let start = mark_line
        .find("at a projection of ")
        .expect("a mark line names its record figure");
    let figure_start = start + "at a projection of ".len();
    let figure_end = mark_line[figure_start..]
        .find(',')
        .map(|offset| figure_start + offset)
        .expect("a record figure ends at a comma");
    mark_line[..figure_end].to_string()
}

fn count_after(line: &str, phrase: &str) -> usize {
    let rest = line
        .split(phrase)
        .nth(1)
        .unwrap_or_else(|| panic!("the phrase {} stands in {}", phrase, line));
    rest.split(|c: char| !c.is_ascii_digit())
        .next()
        .and_then(|word| word.parse().ok())
        .unwrap_or_else(|| panic!("a count stands after {} in {}", phrase, line))
}

#[test]
fn test_spread_mark_lines_agree_with_the_marks_reading_line_by_line() {
    let meet = field_meet();
    let spread = meet_spread(&meet).expect("the spread read");
    let marks = meet_marks(&meet).expect("the marks read");
    assert_eq!(spread[0], "meet spread, 3 days");
    assert_eq!(marks[0], "meet marks, 3 days");
    let spread_sections: Vec<&String> = spread
        .iter()
        .filter(|line| line.starts_with("day, the "))
        .collect();
    let mark_sections: Vec<&String> = marks
        .iter()
        .filter(|line| line.starts_with("day, the "))
        .collect();
    assert_eq!(spread_sections, mark_sections);
    let spread_marks = mark_lines(&spread);
    let reading_marks = mark_lines(&marks);
    assert_eq!(spread_marks.len(), reading_marks.len());
    for (spread_line, mark_line) in spread_marks.iter().zip(reading_marks.iter()) {
        assert!(
            spread_line.starts_with(&record_prefix(mark_line)),
            "{} against {}",
            spread_line,
            mark_line
        );
    }
    let walked = count_after(spread.last().expect("the spread close"), "marks walked ");
    let reading_walked = count_after(marks.last().expect("the marks close"), "marks walked ");
    assert_eq!(walked, reading_walked);
    assert_eq!(walked, spread_marks.len());
}

#[test]
fn test_the_last_mark_of_a_day_agrees_with_the_standings_reading() {
    // The last mark of each day here is not a record line
    // of the meet report, the second day's marks changing
    // no holder and no record figure at all.
    let meet = load_meet(&[
        day(
            1000,
            "a",
            &format!(
                "[{}, {}]",
                run("a", 500, &[(100, 10)]),
                run("b", 400, &[(200, 30)])
            ),
        ),
        day(
            1000,
            "a",
            &format!(
                "[{}, {}, {}]",
                run("a", 500, &[]),
                run("c", 300, &[(100, 20)]),
                run("b", 400, &[(300, 45)])
            ),
        ),
        quiet_day(1000, "a", "a"),
    ]);
    let report = meet_report(&meet).expect("the report reads");
    assert!(
        !report
            .iter()
            .any(|line| line.starts_with("record at 200 seconds")),
        "the first day's last mark stands in no record line"
    );
    let spread = meet_spread(&meet).expect("the spread read");
    let standings = meet_standings(&meet).expect("the standings read");
    let mut sections: Vec<Vec<String>> = Vec::new();
    for line in &standings {
        if line.starts_with("day, the ") {
            sections.push(Vec::new());
        }
        if let Some(section) = sections.last_mut() {
            section.push(line.clone());
        }
    }
    let mut spread_sections: Vec<Vec<&String>> = Vec::new();
    for line in &spread {
        if line.starts_with("day, the ") {
            spread_sections.push(Vec::new());
        }
        if let Some(section) = spread_sections.last_mut() {
            section.push(line);
        }
    }
    assert_eq!(sections.len(), spread_sections.len());
    for (day_index, (section, spread_section)) in
        sections.iter().zip(spread_sections.iter()).enumerate()
    {
        let day_marks: Vec<&&String> = spread_section
            .iter()
            .filter(|line| line.starts_with("mark "))
            .collect();
        if day_marks.is_empty() {
            continue;
        }
        let closing = section
            .iter()
            .find(|line| line.starts_with("close of the "))
            .expect("a standings section closes");
        let holder = closing
            .split("record held by ")
            .nth(1)
            .and_then(|rest| rest.split(" at a projection of ").next())
            .expect("a closing line names its holder");
        let record: i64 = closing
            .rsplit("at a projection of ")
            .next()
            .and_then(|word| word.parse().ok())
            .expect("a closing line names its record figure");
        let others: Vec<i64> = section
            .iter()
            .filter(|line| line.starts_with("competitor "))
            .filter(|line| !line.starts_with(&format!("competitor {},", holder)))
            .filter_map(|line| {
                line.split("standing projection ")
                    .nth(1)
                    .and_then(|rest| rest.split(' ').next())
                    .and_then(|word| word.parse::<i64>().ok())
            })
            .collect();
        let standing_count = section
            .iter()
            .filter(|line| line.starts_with("competitor "))
            .filter(|line| !line.contains("no standing projection"))
            .count();
        let last = day_marks.last().expect("the day holds a mark");
        assert!(
            last.contains(&format!(", field of {}", standing_count)),
            "day {} last mark {}",
            day_index,
            last
        );
        match others.iter().max() {
            Some(figure) => {
                assert!(
                    last.contains(&format!("farthest standing {} held by", figure)),
                    "day {} last mark {}",
                    day_index,
                    last
                );
                assert!(
                    last.ends_with(&format!(", spread {}", figure - record)),
                    "day {} last mark {}",
                    day_index,
                    last
                );
            }
            None => assert!(last.ends_with(", and no other standing"), "{}", last),
        }
    }
}

#[test]
fn test_a_holder_alone_at_every_mark_reads_in_the_no_other_standing_form() {
    let meet = load_meet(&[
        day(
            1000,
            "solo",
            &format!("[{}]", run("solo", 500, &[(100, 10)])),
        ),
        day(
            1000,
            "solo",
            &format!("[{}]", run("solo", 500, &[(200, 30)])),
        ),
        quiet_day(1000, "solo", "solo"),
    ]);
    let lines = meet_spread(&meet).expect("the spread read");
    assert_eq!(
        lines,
        vec![
            "meet spread, 3 days",
            "day, the first day, marks 1",
            "mark 1, at 100 seconds on the first day, solo at a projection of 100, field of 1, and no other standing",
            "day, the second day, marks 1",
            "mark 2, at 200 seconds on the second day, solo at a projection of 150, field of 1, and no other standing",
            "day, the third day, marks 0",
            "spread counts, marks walked 2, marks with another standing 0, days 3, and, no other standing at any mark",
        ]
    );
}

#[test]
fn test_the_spread_stands_at_zero_only_where_the_whole_field_stands_at_the_record() {
    let meet = load_meet(&[
        day(
            1000,
            "a",
            &format!(
                "[{}, {}, {}]",
                run("a", 500, &[(100, 10)]),
                run("b", 400, &[(200, 20)]),
                run("c", 300, &[(300, 45)])
            ),
        ),
        quiet_day(1000, "a", "a"),
        quiet_day(1000, "a", "a"),
    ]);
    let lines = meet_spread(&meet).expect("the spread read");
    assert_eq!(
        lines,
        vec![
            "meet spread, 3 days",
            "day, the first day, marks 3",
            "mark 1, at 100 seconds on the first day, a at a projection of 100, field of 1, and no other standing",
            "mark 2, at 200 seconds on the first day, a at a projection of 100, field of 2, farthest standing 100 held by b, spread 0",
            "mark 3, at 300 seconds on the first day, a at a projection of 100, field of 3, farthest standing 150 held by c, spread 50",
            "day, the second day, marks 0",
            "day, the third day, marks 0",
            "spread counts, marks walked 3, marks with another standing 2, days 3, fullest field 3 at 300 seconds on the first day",
        ]
    );
    let zero_lines: Vec<&String> = lines
        .iter()
        .filter(|line| line.starts_with("mark ") && line.ends_with(", spread 0"))
        .collect();
    assert_eq!(zero_lines.len(), 1);
    assert!(zero_lines[0].starts_with("mark 2,"));
}

#[test]
fn test_the_farthest_tie_breaks_to_the_standing_set_at_the_earliest_mark() {
    let meet = load_meet(&[
        day(
            1000,
            "a",
            &format!(
                "[{}, {}, {}]",
                run("a", 500, &[(100, 10)]),
                run("b", 400, &[(200, 30)]),
                run("c", 300, &[(300, 45)])
            ),
        ),
        quiet_day(1000, "a", "a"),
        quiet_day(1000, "a", "a"),
    ]);
    let lines = meet_spread(&meet).expect("the spread read");
    let third = lines
        .iter()
        .find(|line| line.starts_with("mark 3,"))
        .expect("the third mark stands");
    assert_eq!(
        third,
        "mark 3, at 300 seconds on the first day, a at a projection of 100, field of 3, farthest standing 150 held by b, spread 50"
    );
}

#[test]
fn test_the_farthest_tie_at_one_mark_breaks_to_the_earlier_competitor() {
    let meet = load_meet(&[
        day(
            1000,
            "a",
            &format!(
                "[{}, {}, {}]",
                run("a", 500, &[(100, 10)]),
                run("b", 400, &[(200, 30)]),
                run("c", 300, &[(200, 30)])
            ),
        ),
        quiet_day(1000, "a", "a"),
        quiet_day(1000, "a", "a"),
    ]);
    let lines = meet_spread(&meet).expect("the spread read");
    let second = lines
        .iter()
        .find(|line| line.starts_with("mark 2,"))
        .expect("the second mark stands");
    assert_eq!(
        second,
        "mark 2, at 200 seconds on the first day, a at a projection of 100, field of 3, farthest standing 150 held by b, spread 50"
    );
}

/// A meet whose field reaches three at its third mark
/// and stands at three again at its fourth, so the
/// fullest field is tied and the counts line names the
/// earliest of the two marks standing at it.
fn growing_meet() -> Meet {
    load_meet(&[
        day(
            1000,
            "a",
            &format!(
                "[{}, {}, {}]",
                run("a", 500, &[(100, 10)]),
                run("b", 400, &[(200, 30)]),
                run("c", 300, &[(300, 60)])
            ),
        ),
        day(1000, "a", &format!("[{}]", run("a", 500, &[(100, 15)]))),
        quiet_day(1000, "a", "a"),
    ])
}

#[test]
fn test_the_fullest_field_tie_names_the_earliest_mark() {
    let lines = meet_spread(&growing_meet()).expect("the spread read");
    assert_eq!(
        lines.last().map(String::as_str),
        Some("spread counts, marks walked 4, marks with another standing 3, days 3, fullest field 3 at 300 seconds on the first day")
    );
}

#[test]
fn test_the_field_grows_as_standings_are_set() {
    let lines = meet_spread(&growing_meet()).expect("the spread read");
    let fields: Vec<usize> = mark_lines(&lines)
        .iter()
        .map(|line| count_after(line, "field of "))
        .collect();
    assert_eq!(fields, vec![1, 2, 3, 3]);
}

#[test]
fn test_a_meet_in_which_no_read_projects_reads_with_marks_walked_zero() {
    let meet = load_meet(&[
        quiet_day(600, "solo", "solo"),
        quiet_day(600, "solo", "solo"),
        quiet_day(600, "solo", "solo"),
    ]);
    let lines = meet_spread(&meet).expect("the spread read");
    assert_eq!(
        lines,
        vec![
            "meet spread, 3 days",
            "day, the first day, marks 0",
            "day, the second day, marks 0",
            "day, the third day, marks 0",
            "spread counts, marks walked 0, marks with another standing 0, days 3, and, no other standing at any mark",
        ]
    );
}

// The guard pins. The spread reading stands inside the
// fencing closure of FIX-PG-11 from its birth, and under
// the figure guard of FIX-PG-10 and the day count errors
// of PG-13, in the one walk it shares with the nine
// readings before it.

#[test]
fn test_a_non_finite_figure_draws_the_named_error() {
    let clean_day = |window: Num| FieldDay {
        window_seconds: window,
        operator: PText::from_text("a"),
        runs: vec![DayRun {
            runner: PText::from_text("a"),
            usage: None,
            reads: Some(vec![Read {
                elapsed_seconds: Num::from_i64(100),
                usage: Num::from_i64(40),
            }]),
        }],
    };
    let meet = Meet {
        days: vec![
            clean_day(Num::from_i64(1000)),
            clean_day(Num::from_f64(f64::INFINITY)),
            clean_day(Num::from_i64(1000)),
        ],
    };
    let error = meet_spread(&meet).expect_err("the reading names the precondition");
    assert_eq!(error, "the window is not a finite number");
}

#[test]
fn test_a_break_bearing_runner_label_draws_the_named_error() {
    let clean_day = |label: &str| FieldDay {
        window_seconds: Num::from_i64(1000),
        operator: PText::from_text(label),
        runs: vec![DayRun {
            runner: PText::from_text(label),
            usage: None,
            reads: Some(vec![Read {
                elapsed_seconds: Num::from_i64(100),
                usage: Num::from_i64(40),
            }]),
        }],
    };
    let mut broken_day = clean_day("a");
    broken_day.runs.push(DayRun {
        runner: PText::from_text("b\nb"),
        usage: None,
        reads: None,
    });
    let meet = Meet {
        days: vec![clean_day("a"), broken_day, clean_day("a")],
    };
    let error = meet_spread(&meet).expect_err("the reading names the precondition");
    assert_eq!(error, "a runner label holds a line break");
}

#[test]
fn test_a_meet_holding_no_days_draws_the_named_error() {
    let meet = Meet { days: vec![] };
    let error = meet_spread(&meet).expect_err("the reading names the precondition");
    assert_eq!(error, "the meet holds no days");
}

#[test]
fn test_a_meet_holding_more_days_than_the_ordinal_table_names_draws_the_named_error() {
    let clean_day = || FieldDay {
        window_seconds: Num::from_i64(1000),
        operator: PText::from_text("a"),
        runs: vec![DayRun {
            runner: PText::from_text("a"),
            usage: None,
            reads: None,
        }],
    };
    let meet = Meet {
        days: (0..11).map(|_| clean_day()).collect(),
    };
    let error = meet_spread(&meet).expect_err("the reading names the precondition");
    assert_eq!(
        error,
        "the meet holds more days than the ordinal table names"
    );
}
