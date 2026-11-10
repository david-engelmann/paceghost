//! Tests for PG-33 and PG-34, the middle reading over a meet.
//!
//! Written for Round 16. Each pin's failure mode stands
//! stated here pin by pin, not as one blanket claim. Pins 1
//! to 15 fail before the round by the absence of the reading,
//! the meet_middle function not standing on the pre round
//! tree. On the built tree, pin 1 fails under M-1 at its even
//! field marks, under M-2 at every figured mark, under M-4 at
//! the marks where its middle separates from its nearest, and
//! under M-6 at every mark after the first. Pin 2 fails under
//! M-6 wherever a mark moves the record. Pin 3 fails under
//! M-1 and M-2 at the closing marks whose fields separate.
//! Pin 4 fails under M-2, a field taken whole figuring the
//! holder itself. Pin 5 fails under M-1 at its second figured
//! mark, where the place above the middle place stands above
//! the record, and under M-4 at its fourth mark, where the
//! nearest figure stands at the record itself and the margin
//! would read zero against a middle standing above it.
//! Pin 6 fails under M-3 while every other tie
//! pin stands. Pin 7 fails under a same mark reversal of the
//! tie key. Pin 8 fails under M-5 while the closing's figure
//! stands, and under M-1 at its even field marks. Pin 9
//! fails under M-4 at once, under M-1 at the even fields,
//! and under the farthest degeneration symmetrically.
//! Pin 10 fails under M-1 at its two challenger mark, and
//! under M-4. Pin 11 pins the no read form of the counts
//! line.
//! Pins 12 to 15 fail on a built tree whose reading skipped
//! the FIX-PG-10 figure guard, the FIX-PG-11 label fencing,
//! or the day count preconditions of PG-33.

use paceghost::fieldday::{DayRun, FieldDay};
use paceghost::meet::{
    load_meet_text, meet_margins, meet_marks, meet_middle, meet_spread, meet_standings, Meet,
};
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

/// The parts of one middle mark line the pins read, its
/// place in the walk and its record figure, and where
/// another standing stands, the middle standing figure,
/// the label of its holder, and the middle margin. The
/// parse walks the whole line, its seconds, its day, and
/// its holder included, so a line malformed at any part
/// of it fails the parse.
struct MiddleMark {
    place: usize,
    record: String,
    middle: Option<(String, String, String)>,
}

fn middle_mark_parts(line: &str) -> MiddleMark {
    let rest = line
        .strip_prefix("mark ")
        .expect("a middle mark line opens with its place");
    let (place, rest) = rest
        .split_once(", at ")
        .expect("a middle mark line names its mark");
    let (_seconds, rest) = rest
        .split_once(" seconds on the ")
        .expect("a middle mark line names its seconds");
    let (_day, rest) = rest
        .split_once(" day, ")
        .expect("a middle mark line names its day");
    let (_holder, rest) = rest
        .split_once(" at a projection of ")
        .expect("a middle mark line names its holder");
    if let Some(record) = rest.strip_suffix(", and no other standing") {
        return MiddleMark {
            place: place.parse().expect("a place parses"),
            record: record.to_string(),
            middle: None,
        };
    }
    let (record, rest) = rest
        .split_once(", middle standing ")
        .expect("a figured middle mark line names its middle standing");
    let (figure, rest) = rest
        .split_once(" held by ")
        .expect("a middle standing names its holder");
    let (label, margin) = rest
        .split_once(", middle margin ")
        .expect("a middle standing names its margin");
    MiddleMark {
        place: place.parse().expect("a place parses"),
        record: record.to_string(),
        middle: Some((figure.to_string(), label.to_string(), margin.to_string())),
    }
}

/// A meet over six competitors where the holder stands alone
/// at the first mark, the field behind the holder grows to
/// four at distinct figures on the first day and to five on
/// the third, the second day holds no marks at all, and the
/// holder's own later read re-prices the record on the third
/// day while the field behind it stands unmoved.
fn middle_meet() -> Meet {
    load_meet(&[
        day(
            1000,
            "a",
            &format!(
                "[{}, {}, {}, {}, {}]",
                run("a", 500, &[(100, 10)]),
                run("b", 400, &[(200, 60)]),
                run("c", 300, &[(300, 120)]),
                run("d", 250, &[(400, 200)]),
                run("e", 200, &[(500, 300)])
            ),
        ),
        quiet_day(1000, "a", "a"),
        day(
            1000,
            "a",
            &format!(
                "[{}, {}, {}, {}, {}, {}]",
                run("a", 500, &[(200, 24)]),
                run("b", 400, &[]),
                run("c", 300, &[]),
                run("d", 250, &[]),
                run("e", 200, &[]),
                run("f", 600, &[(100, 66)])
            ),
        ),
    ])
}

#[test]
fn test_the_middle_reading_over_a_hand_built_meet_line_by_line() {
    let lines = meet_middle(&middle_meet()).expect("the middle read");
    assert_eq!(
        lines,
        vec![
            "meet middle, 3 days",
            "day, the first day, marks 5",
            "mark 1, at 100 seconds on the first day, a at a projection of 100, and no other standing",
            "mark 2, at 200 seconds on the first day, a at a projection of 100, middle standing 300 held by b, middle margin 200",
            "mark 3, at 300 seconds on the first day, a at a projection of 100, middle standing 300 held by b, middle margin 200",
            "mark 4, at 400 seconds on the first day, a at a projection of 100, middle standing 400 held by c, middle margin 300",
            "mark 5, at 500 seconds on the first day, a at a projection of 100, middle standing 400 held by c, middle margin 300",
            "day, the second day, marks 0",
            "day, the third day, marks 2",
            "mark 6, at 100 seconds on the third day, a at a projection of 100, middle standing 500 held by d, middle margin 400",
            "mark 7, at 200 seconds on the third day, a at a projection of 120, middle standing 500 held by d, middle margin 380",
            "middle counts, marks walked 7, marks with another standing 6, days 3, closest middle margin 200 at 200 seconds on the first day",
        ]
    );
}

// The relation pins of PG-34. The middle reading agrees
// with the marks reading on the record side of every
// mark, line by line, and at the last mark of each day
// holding marks its middle standing is the middle figure
// the standings reading computes at that day's close.

#[test]
fn test_middle_mark_lines_agree_with_the_marks_reading_line_by_line() {
    let meet = middle_meet();
    let middle = meet_middle(&meet).expect("the middle read");
    let marks = meet_marks(&meet).expect("the marks read");
    assert_eq!(middle[0], "meet middle, 3 days");
    assert_eq!(marks[0], "meet marks, 3 days");
    let middle_sections: Vec<&String> = middle
        .iter()
        .filter(|line| line.starts_with("day, the "))
        .collect();
    let mark_sections: Vec<&String> = marks
        .iter()
        .filter(|line| line.starts_with("day, the "))
        .collect();
    assert_eq!(middle_sections, mark_sections);
    let middle_marks = mark_lines(&middle);
    let reading_marks = mark_lines(&marks);
    assert_eq!(middle_marks.len(), reading_marks.len());
    for (middle_line, mark_line) in middle_marks.iter().zip(reading_marks.iter()) {
        assert!(
            middle_line.starts_with(&record_prefix(mark_line)),
            "{} against {}",
            middle_line,
            mark_line
        );
    }
    let places: Vec<usize> = middle_marks
        .iter()
        .map(|line| middle_mark_parts(line).place)
        .collect();
    assert_eq!(places, (1..=middle_marks.len()).collect::<Vec<usize>>());
    let walked = count_after(middle.last().expect("the middle close"), "marks walked ");
    let reading_walked = count_after(marks.last().expect("the marks close"), "marks walked ");
    assert_eq!(walked, reading_walked);
    assert_eq!(walked, middle_marks.len());
}

#[test]
fn test_the_last_mark_of_a_day_agrees_with_the_standings_reading() {
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
    let middle = meet_middle(&meet).expect("the middle read");
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
    let mut middle_sections: Vec<Vec<&String>> = Vec::new();
    for line in &middle {
        if line.starts_with("day, the ") {
            middle_sections.push(Vec::new());
        }
        if let Some(section) = middle_sections.last_mut() {
            section.push(line);
        }
    }
    assert_eq!(sections.len(), middle_sections.len());
    for (day_index, (section, middle_section)) in
        sections.iter().zip(middle_sections.iter()).enumerate()
    {
        let day_marks: Vec<&&String> = middle_section
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
        let mut others: Vec<i64> = section
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
        others.sort_unstable();
        let last = day_marks.last().expect("the day holds a mark");
        let parts = middle_mark_parts(last);
        match (others.is_empty(), parts.middle) {
            (true, None) => {}
            (false, Some((figure, _label, margin))) => {
                let middle_value = others[others.len().div_ceil(2) - 1];
                assert_eq!(
                    figure,
                    middle_value.to_string(),
                    "day {} last mark {}",
                    day_index,
                    last
                );
                assert_eq!(
                    margin,
                    (middle_value - record).to_string(),
                    "day {} last mark {}",
                    day_index,
                    last
                );
            }
            _ => panic!("the day close relation fails on day {}", day_index),
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
    let lines = meet_middle(&meet).expect("the middle read");
    assert_eq!(
        lines,
        vec![
            "meet middle, 3 days",
            "day, the first day, marks 1",
            "mark 1, at 100 seconds on the first day, solo at a projection of 100, and no other standing",
            "day, the second day, marks 1",
            "mark 2, at 200 seconds on the second day, solo at a projection of 150, and no other standing",
            "day, the third day, marks 0",
            "middle counts, marks walked 2, marks with another standing 0, days 3, and, no other standing at any mark",
        ]
    );
}

#[test]
fn test_the_middle_margin_stands_at_zero_only_where_the_middle_standing_stands_at_the_record() {
    let meet = load_meet(&[
        day(
            1000,
            "a",
            &format!(
                "[{}, {}, {}, {}]",
                run("a", 500, &[(100, 10)]),
                run("b", 400, &[(200, 20)]),
                run("c", 300, &[(250, 30)]),
                run("d", 250, &[(300, 45)])
            ),
        ),
        quiet_day(1000, "a", "a"),
        quiet_day(1000, "a", "a"),
    ]);
    let lines = meet_middle(&meet).expect("the middle read");
    assert_eq!(
        lines,
        vec![
            "meet middle, 3 days",
            "day, the first day, marks 4",
            "mark 1, at 100 seconds on the first day, a at a projection of 100, and no other standing",
            "mark 2, at 200 seconds on the first day, a at a projection of 100, middle standing 100 held by b, middle margin 0",
            "mark 3, at 250 seconds on the first day, a at a projection of 100, middle standing 100 held by b, middle margin 0",
            "mark 4, at 300 seconds on the first day, a at a projection of 100, middle standing 120 held by c, middle margin 20",
            "day, the second day, marks 0",
            "day, the third day, marks 0",
            "middle counts, marks walked 4, marks with another standing 3, days 3, closest middle margin 0 at 200 seconds on the first day",
        ]
    );
    let zero_lines: Vec<&String> = lines
        .iter()
        .filter(|line| line.starts_with("mark ") && line.ends_with(", middle margin 0"))
        .collect();
    assert_eq!(zero_lines.len(), 2);
    for line in &zero_lines {
        let parts = middle_mark_parts(line);
        assert_eq!(parts.middle.expect("a figured line").0, parts.record);
    }
    let at_record: Vec<&String> = lines
        .iter()
        .filter(|line| line.starts_with("mark "))
        .filter(|line| {
            let parts = middle_mark_parts(line);
            matches!(parts.middle, Some((ref figure, _, _)) if *figure == parts.record)
        })
        .collect();
    assert_eq!(at_record.len(), zero_lines.len());
}

#[test]
fn test_the_middle_tie_breaks_to_the_standing_set_at_the_earliest_mark() {
    let meet = load_meet(&[
        day(
            1000,
            "a",
            &format!(
                "[{}, {}, {}]",
                run("a", 500, &[(100, 10)]),
                run("b", 400, &[(150, 30)]),
                run("c", 300, &[(200, 60)])
            ),
        ),
        day(
            1000,
            "a",
            &format!(
                "[{}, {}, {}, {}]",
                run("a", 500, &[]),
                run("b", 400, &[]),
                run("c", 300, &[]),
                run("d", 250, &[(100, 20)])
            ),
        ),
        quiet_day(1000, "a", "a"),
    ]);
    let lines = meet_middle(&meet).expect("the middle read");
    let fourth = lines
        .iter()
        .find(|line| line.starts_with("mark 4,"))
        .expect("the fourth mark stands");
    assert_eq!(
        fourth,
        "mark 4, at 100 seconds on the second day, a at a projection of 100, middle standing 200 held by b, middle margin 100"
    );
}

#[test]
fn test_the_middle_tie_at_one_mark_breaks_to_the_earlier_competitor() {
    let meet = load_meet(&[
        day(
            1000,
            "a",
            &format!(
                "[{}, {}, {}]",
                run("a", 500, &[(100, 10)]),
                run("b", 400, &[(200, 40)]),
                run("c", 300, &[(200, 40)])
            ),
        ),
        quiet_day(1000, "a", "a"),
        quiet_day(1000, "a", "a"),
    ]);
    let lines = meet_middle(&meet).expect("the middle read");
    let second = lines
        .iter()
        .find(|line| line.starts_with("mark 2,"))
        .expect("the second mark stands");
    assert_eq!(
        second,
        "mark 2, at 200 seconds on the first day, a at a projection of 100, middle standing 200 held by b, middle margin 100"
    );
}

#[test]
fn test_the_closest_middle_margin_tie_names_the_earliest_mark() {
    let meet = load_meet(&[
        day(
            1000,
            "a",
            &format!(
                "[{}, {}, {}]",
                run("a", 500, &[(100, 10)]),
                run("b", 400, &[(200, 30)]),
                run("c", 300, &[(300, 75)])
            ),
        ),
        day(
            1000,
            "a",
            &format!(
                "[{}, {}, {}, {}]",
                run("a", 500, &[]),
                run("b", 400, &[]),
                run("c", 300, &[]),
                run("d", 250, &[(100, 28)])
            ),
        ),
        quiet_day(1000, "a", "a"),
    ]);
    let lines = meet_middle(&meet).expect("the middle read");
    let third = lines
        .iter()
        .find(|line| line.starts_with("mark 3,"))
        .expect("the third mark stands");
    assert!(
        third.ends_with(", middle margin 50"),
        "the tie is real at the third mark, {}",
        third
    );
    assert_eq!(
        lines.last().map(String::as_str),
        Some("middle counts, marks walked 4, marks with another standing 3, days 3, closest middle margin 50 at 200 seconds on the first day")
    );
}

#[test]
fn test_the_middle_separates_from_the_nearest_and_the_farthest() {
    let meet = load_meet(&[
        day(
            1000,
            "a",
            &format!(
                "[{}, {}, {}, {}, {}]",
                run("a", 500, &[(100, 10)]),
                run("b", 400, &[(150, 15)]),
                run("c", 300, &[(200, 50)]),
                run("d", 250, &[(250, 90)]),
                run("e", 200, &[(300, 150)])
            ),
        ),
        quiet_day(1000, "a", "a"),
        quiet_day(1000, "a", "a"),
    ]);
    let middle = meet_middle(&meet).expect("the middle read");
    let margins = meet_margins(&meet).expect("the margins read");
    let spread = meet_spread(&meet).expect("the spread read");
    let middle_line = middle
        .iter()
        .find(|line| line.starts_with("mark 5,"))
        .expect("the fifth mark stands");
    assert_eq!(
        middle_line,
        "mark 5, at 300 seconds on the first day, a at a projection of 100, middle standing 250 held by c, middle margin 150"
    );
    let margins_line = margins
        .iter()
        .find(|line| line.starts_with("mark 5,"))
        .expect("the fifth mark stands");
    assert_eq!(
        margins_line,
        "mark 5, at 300 seconds on the first day, a at a projection of 100, nearest standing 100 held by b, margin 0"
    );
    let spread_line = spread
        .iter()
        .find(|line| line.starts_with("mark 5,"))
        .expect("the fifth mark stands");
    assert_eq!(
        spread_line,
        "mark 5, at 300 seconds on the first day, a at a projection of 100, field of 5, farthest standing 500 held by e, spread 400"
    );
    let margin_value: i64 = margins_line
        .rsplit(", margin ")
        .next()
        .and_then(|word| word.parse().ok())
        .expect("the margins line names its margin");
    let middle_value: i64 = middle_line
        .rsplit(", middle margin ")
        .next()
        .and_then(|word| word.parse().ok())
        .expect("the middle line names its middle margin");
    let spread_value: i64 = spread_line
        .rsplit(", spread ")
        .next()
        .and_then(|word| word.parse().ok())
        .expect("the spread line names its spread");
    assert!(margin_value < middle_value && middle_value < spread_value);
}

#[test]
fn test_the_middle_place_moves_as_the_field_grows() {
    let meet = load_meet(&[
        day(
            1000,
            "a",
            &format!(
                "[{}, {}, {}]",
                run("a", 500, &[(100, 10)]),
                run("b", 400, &[(200, 60)]),
                run("c", 300, &[(300, 150)])
            ),
        ),
        day(
            1000,
            "a",
            &format!(
                "[{}, {}, {}, {}]",
                run("a", 500, &[]),
                run("b", 400, &[]),
                run("c", 300, &[]),
                run("d", 250, &[(100, 70)])
            ),
        ),
        quiet_day(1000, "a", "a"),
    ]);
    let lines = meet_middle(&meet).expect("the middle read");
    let figured: Vec<String> = mark_lines(&lines)
        .iter()
        .filter(|line| line.contains(", middle standing "))
        .map(|line| middle_mark_parts(line).middle.expect("a figured line").0)
        .collect();
    assert_eq!(figured, vec!["300", "300", "500"]);
    let second = lines
        .iter()
        .find(|line| line.starts_with("mark 2,"))
        .expect("the second mark stands");
    assert_eq!(
        second,
        "mark 2, at 200 seconds on the first day, a at a projection of 100, middle standing 300 held by b, middle margin 200"
    );
    let fourth = lines
        .iter()
        .find(|line| line.starts_with("mark 4,"))
        .expect("the fourth mark stands");
    assert_eq!(
        fourth,
        "mark 4, at 100 seconds on the second day, a at a projection of 100, middle standing 500 held by c, middle margin 400"
    );
}

#[test]
fn test_a_meet_in_which_no_read_projects_reads_with_marks_walked_zero() {
    let meet = load_meet(&[
        quiet_day(600, "solo", "solo"),
        quiet_day(600, "solo", "solo"),
        quiet_day(600, "solo", "solo"),
    ]);
    let lines = meet_middle(&meet).expect("the middle read");
    assert_eq!(
        lines,
        vec![
            "meet middle, 3 days",
            "day, the first day, marks 0",
            "day, the second day, marks 0",
            "day, the third day, marks 0",
            "middle counts, marks walked 0, marks with another standing 0, days 3, and, no other standing at any mark",
        ]
    );
}

// The guard pins. The middle reading stands inside the
// fencing closure of FIX-PG-11 from its birth, and under
// the figure guard of FIX-PG-10 and the day count errors
// of PG-33, in the one walk it shares with the ten
// readings before it.

#[test]
fn test_a_hand_built_non_finite_figure_draws_the_named_error_through_middle() {
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
    let error = meet_middle(&meet).expect_err("the reading names the precondition");
    assert_eq!(error, "the window is not a finite number");
}

#[test]
fn test_a_hand_built_break_bearing_label_draws_the_named_error_through_middle() {
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
    let error = meet_middle(&meet).expect_err("the reading names the precondition");
    assert_eq!(error, "a runner label holds a line break");
}

#[test]
fn test_a_hand_built_meet_with_no_days_draws_the_named_error_through_middle() {
    let meet = Meet { days: vec![] };
    let error = meet_middle(&meet).expect_err("the reading names the precondition");
    assert_eq!(error, "the meet holds no days");
}

#[test]
fn test_a_hand_built_meet_with_too_many_days_draws_the_named_error_through_middle() {
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
    let error = meet_middle(&meet).expect_err("the reading names the precondition");
    assert_eq!(
        error,
        "the meet holds more days than the ordinal table names"
    );
}
