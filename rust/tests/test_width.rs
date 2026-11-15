//! Tests for PG-51 and PG-52, the width reading over a meet.
//!
//! Written for Round 25. Each pin's failure mode stands
//! stated here pin by pin, not as one blanket claim. Pins 1
//! to 16 fail before the round by the absence of the reading,
//! the meet_width function not standing on the pre round
//! tree. On the built tree, pin 1 fails under S-1 at all five
//! marks, the widths becoming 50, 70, 70, 90, and 120, and
//! under S-6 as under S-1, under S-2 at its third, fourth,
//! and fifth marks, the interquartile ranges standing at 25,
//! 45, and 40 against the widths at 50, 70, and 100, under
//! S-3 at its third, fourth, and fifth marks, the
//! differences of the tails standing at 25, 45, and
//! 46.666667, and under S-5 from the first mark onward, and
//! it passes under S-4 by design, its narrowest width, 0 at
//! the first mark, standing unique. Pin 2 passes under all
//! six mutations by design and claims no mutation failure,
//! the mark line prefix it compares standing rendered from
//! the walk's mark records, which no mutation of this round
//! touches. Pin 3 fails under S-1 at both closes, the widths
//! becoming 70 and 120, and under S-6 as under S-1, under
//! S-5 at both closes, the lagged widths standing at 0 and
//! 70, and under S-2 and S-3 at the third day's close alone,
//! its interquartile range standing at 40 and its difference
//! of the tails at 46.666667 against the width of 100, the
//! first day's close standing at count two where each
//! partner coincides with the width, and it passes under
//! S-4 by design, no counts line standing in it. Pin 4 fails
//! under S-1 and S-6 by design, the holder's own figure
//! entering the field at every mark so a field width stands
//! where the pin states none, and it passes under S-2, S-3,
//! S-4, and S-5 by design. Pin 5 passes under S-1 whole by
//! the extremes' blindness, the margin standing at zero at
//! all five marks so the enlarged field moves neither
//! extreme, a posture stated in the open, and under S-6
//! whole on its width clauses on the same ground, and it
//! fails under S-2 at its first four marks, the
//! interquartile ranges standing at 45, 45, 70, and 57.5
//! against the widths at 90, under S-3 at its first four
//! marks, the differences of the tails standing at the same
//! figures, and under S-5 at its first and fifth marks, and
//! it passes under S-4 by design, its narrowest width, zero
//! at the fifth mark, standing unique. The first edge case
//! holds under S-1 and under S-6, its margin standing at
//! zero, and it fails under S-2, its interquartile range
//! standing at 0 against the width at 90, under S-3, its
//! difference of the tails standing at 30, and under S-5.
//! The second edge case fails under S-1 and under S-6, its
//! width becoming the spread of 50, and under S-5, and it
//! holds under S-2 and S-3, each partner standing at 0 there
//! whole. Pin 6 fails under S-1 at every mark of each meet,
//! the widths becoming the spreads 80, 75, 90, 300, and 300,
//! and under S-6 as under S-1, under S-2 at every mark,
//! under S-3 at every mark, and under S-5 at the one mark of
//! each above meet and from the below meet's first mark, and
//! it passes under S-4 by design, the narrowest width
//! standing unique in each meet. Pin 7 fails under S-1 at
//! every mark of the pin 6 meets, the width becoming the
//! spread and the strict standing failing there, while
//! holding at the zero fixture's marks, and under S-6 as
//! under S-1, under S-2 at the zero fixture's first four
//! marks and at every mark of the pin 6 meets, under S-3 at
//! the same marks, and under S-5 at the zero fixture's first
//! and fifth marks, at the one mark of each above meet, and
//! from the below meet's first mark, and it passes under
//! S-4 by design, the narrowest width standing unique in
//! each meet. Pin 8 fails under S-4, its designed catcher,
//! the loosened scan naming the fourth mark at 400 seconds
//! where the pin states the third at 300, under S-1 at its
//! first to fourth marks, a field width of 0 standing at the
//! first mark where the pin states no other standing and
//! the widths becoming 160, 50, and 50 with the narrowest
//! moving to the first mark, under S-2 and S-3 at its second
//! to fourth marks, the partners standing at 60, 10, and 10,
//! under S-5 from its second mark, and under S-6 as under
//! S-1. Pin 9 fails under S-1 at every step, the widths
//! becoming 50, 110, 140, 230, and 320, and under S-6 as
//! under S-1, under S-2 at its third, fourth, and fifth
//! steps, the interquartile ranges standing at 45, 105, and
//! 120, under S-3 at its third, fourth, and fifth steps, the
//! differences of the tails standing at 45, 105, and 130,
//! and under S-5 from its first step, and it passes under
//! S-4 by design, its narrowest width, 0 at the first step,
//! standing unique. Pin 10 fails under S-1 on the identity
//! at every mark, the identity's two partners consuming no
//! part of the shared helper so the enlarged width parts
//! from their difference wherever the margin stands above
//! zero, which is every mark of the meet, under S-2 and S-3
//! on the identity at its third, fourth, and fifth marks,
//! and under S-5 from its first mark on its pinned values,
//! and under S-6 on its pinned values, its identity clause
//! holding under S-6 by design, the identity standing for
//! any field the shared helper returns, and it passes under
//! S-4 by design. Pin 11 passes under all six mutations by
//! design, no mark standing for any selection, vector, or
//! closing scan to move. Pins 12 to 15 pass under all six
//! mutations by design, the guards standing ahead of any
//! selection or scan, and they fail on a built tree whose
//! reading skipped the FIX-PG-10 figure guard, the
//! FIX-PG-11 label fencing, or the day count preconditions
//! of PG-51. Pin 16, the exact difference pin, fails under
//! the rounding variant that rounds both extreme standings
//! to six decimal places before differencing, that variant
//! printing a field width of 0 at its third mark against
//! the exact difference the pin states in full, under S-1
//! at both figured marks, the widths becoming 2^-22 at the
//! second mark and 2^-21 at the third, and under S-6 as
//! under S-1, and under S-5 at its second and third marks,
//! and it passes under S-2 and S-3 by design, every figured
//! field of its fixture counting two figures or fewer, and
//! under S-4 by design, its narrowest width, 0 at the
//! second mark, standing unique.
//!
//! Pin 17, the separating meet pin, asserts the width
//! reading over the separating meet of PG-53 line by line,
//! the meet whose third mark holds five figures with the
//! two lowest equal and the two highest equal, where the
//! range stands at the field width and the difference of
//! the tails stands below it at two thirds of it. The pin
//! passes under S-2 whole, the range agreeing with the
//! width at every mark of the meet, and fails under S-3
//! at the third mark alone, and it passes under S-4, the
//! meet's one field width of zero standing unique, and
//! fails under S-1, S-5, and S-6. Pin 18, the separation
//! relation pin, asserts the relations of PG-53 over the
//! same meet across the width, range, tails, and
//! quartiles readings read together, the range at the
//! field width at every figured mark, the difference of
//! the tails at the field width at the first and second
//! marks and below it by fifty at the third, and the
//! quartile standings at the third mark standing at one
//! hundred and fifty and three hundred, so a compensating
//! movement of the two quartile standings by one amount,
//! which leaves their difference and every other reading
//! of the meet unmoved, falls at this pin where pin 17,
//! reading the width alone, cannot see it, and its split
//! under the six mutations stands as pin 17's split
//! stands.

use paceghost::fieldday::{DayRun, FieldDay};
use paceghost::meet::{
    load_meet_text, meet_margins, meet_marks, meet_quartiles, meet_range, meet_spread,
    meet_standings, meet_tails, meet_width, Meet,
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

struct WidthMark {
    prefix: String,
    width: Option<f64>,
}

fn width_mark_parts(mark_line: &str) -> WidthMark {
    let prefix = record_prefix(mark_line);
    if mark_line.contains(", and no other standing") {
        return WidthMark {
            prefix,
            width: None,
        };
    }
    let text = mark_line
        .split(", field width ")
        .nth(1)
        .unwrap_or_else(|| panic!("a field width stands in {}", mark_line));
    WidthMark {
        prefix,
        width: Some(
            text.parse()
                .unwrap_or_else(|_| panic!("the field width parses in {}", mark_line)),
        ),
    }
}

fn width_of(mark_line: &str) -> Option<f64> {
    width_mark_parts(mark_line).width
}

fn range_of(mark_line: &str) -> Option<f64> {
    if mark_line.contains(", and no other standing") {
        return None;
    }
    let text = mark_line
        .split(", interquartile range ")
        .nth(1)
        .unwrap_or_else(|| panic!("an interquartile range stands in {}", mark_line));
    Some(
        text.parse()
            .unwrap_or_else(|_| panic!("the interquartile range parses in {}", mark_line)),
    )
}

fn tails_standings_of(mark_line: &str) -> Option<(f64, f64)> {
    if mark_line.contains(", and no other standing") {
        return None;
    }
    let lower_text = mark_line
        .split(", lower tail standing ")
        .nth(1)
        .and_then(|rest| rest.split(',').next())
        .unwrap_or_else(|| panic!("a lower tail standing stands in {}", mark_line));
    let upper_text = mark_line
        .split(", upper tail standing ")
        .nth(1)
        .and_then(|rest| rest.split(',').next())
        .unwrap_or_else(|| panic!("an upper tail standing stands in {}", mark_line));
    Some((
        lower_text
            .parse()
            .unwrap_or_else(|_| panic!("the lower tail standing parses in {}", mark_line)),
        upper_text
            .parse()
            .unwrap_or_else(|_| panic!("the upper tail standing parses in {}", mark_line)),
    ))
}

fn tails_difference_of(mark_line: &str) -> Option<f64> {
    tails_standings_of(mark_line).map(|(lower, upper)| upper - lower)
}

fn quartile_standings_of(mark_line: &str) -> Option<(f64, f64)> {
    if mark_line.contains(", and no other standing") {
        return None;
    }
    let lower_text = mark_line
        .split(", lower quartile standing ")
        .nth(1)
        .and_then(|rest| rest.split(',').next())
        .unwrap_or_else(|| panic!("a lower quartile standing stands in {}", mark_line));
    let upper_text = mark_line
        .split(", upper quartile standing ")
        .nth(1)
        .and_then(|rest| rest.split(',').next())
        .unwrap_or_else(|| panic!("an upper quartile standing stands in {}", mark_line));
    Some((
        lower_text
            .parse()
            .unwrap_or_else(|_| panic!("the lower quartile standing parses in {}", mark_line)),
        upper_text
            .parse()
            .unwrap_or_else(|_| panic!("the upper quartile standing parses in {}", mark_line)),
    ))
}

fn nearest_of(mark_line: &str) -> Option<f64> {
    if mark_line.contains(", and no other standing") {
        return None;
    }
    let text = mark_line
        .split(", nearest standing ")
        .nth(1)
        .and_then(|rest| rest.split(' ').next())
        .unwrap_or_else(|| panic!("a nearest standing stands in {}", mark_line));
    Some(
        text.parse()
            .unwrap_or_else(|_| panic!("the nearest standing parses in {}", mark_line)),
    )
}

fn farthest_of(mark_line: &str) -> Option<f64> {
    if mark_line.contains(", and no other standing") {
        return None;
    }
    let text = mark_line
        .split(", farthest standing ")
        .nth(1)
        .and_then(|rest| rest.split(' ').next())
        .unwrap_or_else(|| panic!("a farthest standing stands in {}", mark_line));
    Some(
        text.parse()
            .unwrap_or_else(|_| panic!("the farthest standing parses in {}", mark_line)),
    )
}

fn margin_of(mark_line: &str) -> Option<f64> {
    if mark_line.contains(", and no other standing") {
        return None;
    }
    let text = mark_line
        .split(", margin ")
        .nth(1)
        .unwrap_or_else(|| panic!("a margin stands in {}", mark_line));
    Some(
        text.parse()
            .unwrap_or_else(|_| panic!("the margin parses in {}", mark_line)),
    )
}

fn spread_of(mark_line: &str) -> Option<f64> {
    if mark_line.contains(", and no other standing") {
        return None;
    }
    let text = mark_line
        .split(", spread ")
        .nth(1)
        .unwrap_or_else(|| panic!("a spread stands in {}", mark_line));
    Some(
        text.parse()
            .unwrap_or_else(|_| panic!("the spread parses in {}", mark_line)),
    )
}

fn width_meet() -> Meet {
    load_meet(&[
        day(
            1000,
            "a",
            &format!(
                "[{}, {}, {}]",
                run("a", 500, &[(100, 10)]),
                run("b", 400, &[(100, 15)]),
                run("c", 300, &[(200, 34)])
            ),
        ),
        quiet_day(1000, "a", "a"),
        day(
            1000,
            "a",
            &format!(
                "[{}, {}, {}, {}, {}, {}]",
                run("a", 500, &[]),
                run("b", 400, &[]),
                run("c", 300, &[]),
                run("d", 250, &[(100, 12)]),
                run("e", 200, &[(200, 38)]),
                run("f", 150, &[(300, 66)])
            ),
        ),
    ])
}

fn separating_meet() -> Meet {
    load_meet(&[
        day(
            1000,
            "a",
            &format!(
                "[{}, {}, {}]",
                run("a", 500, &[(100, 10)]),
                run("b", 400, &[(100, 15)]),
                run("c", 300, &[(200, 36)])
            ),
        ),
        quiet_day(1000, "a", "a"),
        day(
            1000,
            "a",
            &format!(
                "[{}, {}, {}, {}, {}, {}]",
                run("a", 500, &[]),
                run("b", 400, &[(100, 15)]),
                run("c", 300, &[(100, 15)]),
                run("d", 250, &[(100, 20)]),
                run("e", 200, &[(100, 30)]),
                run("f", 150, &[(100, 30)])
            ),
        ),
    ])
}

#[test]
fn test_the_width_reading_over_a_hand_built_meet_line_by_line() {
    let meet = width_meet();
    let expected: Vec<String> = [
        "meet width, 3 days",
        "day, the first day, marks 2",
        "mark 1, at 100 seconds on the first day, a at a projection of 100, field width 0",
        "mark 2, at 200 seconds on the first day, a at a projection of 100, field width 20",
        "day, the second day, marks 0",
        "day, the third day, marks 3",
        "mark 3, at 100 seconds on the third day, a at a projection of 100, field width 50",
        "mark 4, at 200 seconds on the third day, a at a projection of 100, field width 70",
        "mark 5, at 300 seconds on the third day, a at a projection of 100, field width 100",
        "width counts, marks walked 5, marks with another standing 5, days 3, narrowest field width 0 at 100 seconds on the first day",
    ]
    .iter()
    .map(|line| line.to_string())
    .collect();
    assert_eq!(meet_width(&meet), Ok(expected));
}

#[test]
fn test_width_mark_lines_agree_with_the_marks_reading_line_by_line() {
    let meet = width_meet();
    let marks = meet_marks(&meet).expect("the marks reading stands");
    let width = meet_width(&meet).expect("the width reading stands");
    assert_eq!(
        width.first().map(String::as_str),
        Some("meet width, 3 days")
    );
    assert_eq!(
        marks.first().map(String::as_str),
        Some("meet marks, 3 days")
    );
    let marks_days: Vec<&String> = marks
        .iter()
        .filter(|line| line.starts_with("day,"))
        .collect();
    let width_days: Vec<&String> = width
        .iter()
        .filter(|line| line.starts_with("day,"))
        .collect();
    assert_eq!(marks_days, width_days);
    let marks_marks = mark_lines(&marks);
    let width_marks = mark_lines(&width);
    assert_eq!(marks_marks.len(), width_marks.len());
    for (place, (marks_line, width_line)) in marks_marks.iter().zip(width_marks.iter()).enumerate()
    {
        let parts = width_mark_parts(width_line);
        assert_eq!(
            parts.prefix,
            record_prefix(marks_line),
            "mark {}",
            place + 1
        );
        assert!(
            width_line.starts_with(&format!("mark {}, ", place + 1)),
            "mark {} carries its place in order",
            place + 1
        );
    }
    let marks_counts = marks.last().expect("the marks counts line stands");
    let width_counts = width.last().expect("the width counts line stands");
    assert_eq!(
        count_after(marks_counts, "marks walked "),
        count_after(width_counts, "marks walked ")
    );
}

#[test]
fn test_the_last_mark_of_a_day_agrees_with_the_standings_reading() {
    let meet = width_meet();
    let width = meet_width(&meet).expect("the width reading stands");
    let standings = meet_standings(&meet).expect("the standings reading stands");
    let mut sections: Vec<Vec<String>> = Vec::new();
    for line in &standings {
        if line.starts_with("day, the ") {
            sections.push(Vec::new());
        }
        if let Some(section) = sections.last_mut() {
            section.push(line.clone());
        }
    }
    let mut width_sections: Vec<Vec<&String>> = Vec::new();
    for line in &width {
        if line.starts_with("day, the ") {
            width_sections.push(Vec::new());
        }
        if let Some(section) = width_sections.last_mut() {
            section.push(line);
        }
    }
    assert_eq!(sections.len(), width_sections.len());
    let expected_counts = [2usize, 0, 5];
    let expected_widths = [20.0f64, 0.0, 100.0];
    for (day_index, (section, width_section)) in
        sections.iter().zip(width_sections.iter()).enumerate()
    {
        let day_marks: Vec<&&String> = width_section
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
        let mut others: Vec<f64> = section
            .iter()
            .filter(|line| line.starts_with("competitor "))
            .filter(|line| !line.starts_with(&format!("competitor {},", holder)))
            .filter_map(|line| {
                line.split("standing projection ")
                    .nth(1)
                    .and_then(|rest| rest.split(' ').next())
                    .and_then(|word| word.parse::<f64>().ok())
            })
            .collect();
        assert_eq!(
            others.len(),
            expected_counts[day_index],
            "day {}",
            day_index + 1
        );
        others.sort_by(|a, b| a.partial_cmp(b).expect("figures order"));
        let expected = others[others.len() - 1] - others[0];
        let named = expected_widths[day_index];
        assert!(
            (expected - named).abs() <= 0.000002,
            "day {} closes at the named width",
            day_index + 1
        );
        let last = day_marks.last().expect("the day holds a mark");
        assert_eq!(
            width_of(last),
            Some(named),
            "day {} last mark {}",
            day_index,
            last
        );
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
    let lines = meet_width(&meet).expect("the width read");
    assert_eq!(
        lines,
        vec![
            "meet width, 3 days",
            "day, the first day, marks 1",
            "mark 1, at 100 seconds on the first day, solo at a projection of 100, and no other standing",
            "day, the second day, marks 1",
            "mark 2, at 200 seconds on the second day, solo at a projection of 150, and no other standing",
            "day, the third day, marks 0",
            "width counts, marks walked 2, marks with another standing 0, days 3, and, no other standing at any mark",
        ]
    );
}

fn width_zero_condition_holds(field: &[f64]) -> bool {
    field.iter().all(|figure| *figure == field[0])
}

fn zero_fixture() -> Meet {
    load_meet(&[
        day(
            1000,
            "a",
            &format!(
                "[{}, {}, {}, {}, {}]",
                run("a", 500, &[(100, 10)]),
                run("b", 400, &[(100, 10), (400, 50), (500, 50)]),
                run("c", 300, &[(100, 10)]),
                run("d", 250, &[(100, 19), (500, 50)]),
                run("e", 200, &[(200, 20), (300, 45), (500, 50)])
            ),
        ),
        quiet_day(1000, "a", "a"),
        quiet_day(1000, "a", "a"),
    ])
}

#[test]
fn test_the_field_width_stands_at_zero_exactly_where_every_figure_of_the_field_stands_at_one_figure(
) {
    // The zero fixture of the quartiles file standing
    // whole. At the first mark the field behind the holder
    // stands at 100, 100, and 190 and the field width
    // stands at 90. At the second mark the field stands at
    // 100, 100, 100, and 190 and the width stands at 90.
    // At the third mark the field stands at 100, 100, 150,
    // and 190 and the width stands at 90. At the fourth
    // mark the field stands at 100, 125, 150, and 190 and
    // the width stands at 90. At the fifth mark every
    // figure of the field stands at 100 and the width
    // stands at zero. The width zero set is exactly the
    // fifth mark, the set of the spread of PG-32, while
    // the margin of PG-30 stands at zero at all five marks,
    // and the zero condition of PG-52, every figure of the
    // field standing at one figure, holds at the fifth
    // mark alone, checked in both directions at every mark
    // from the fields the fixture states. The first edge
    // case stands in a meet of its own behind the same
    // holder, its field standing at 100, 100, 100, 100,
    // and 190, its field width standing at 90 exactly
    // where four figures of the field stand at the record,
    // the zero condition failing there, its interquartile
    // range standing at 0 against the width of 90 and its
    // difference of the tails at 30. The second edge case
    // stands in a meet of its own, its field standing at
    // 150, 150, and 150 behind a record of 100, its field
    // width standing at 0 with no figure of the field
    // standing at the record, the zero condition holding
    // there with the record playing no part, each partner
    // standing at 0 there whole.
    let meet = zero_fixture();
    let lines = meet_width(&meet).expect("the width read");
    let marks = mark_lines(&lines);
    assert_eq!(marks.len(), 5);
    let widths: Vec<f64> = marks
        .iter()
        .map(|line| width_of(line).expect("a field width stands"))
        .collect();
    assert_eq!(widths, vec![90.0, 90.0, 90.0, 90.0, 0.0]);
    let fields: [Vec<f64>; 5] = [
        vec![100.0, 100.0, 190.0],
        vec![100.0, 100.0, 100.0, 190.0],
        vec![100.0, 100.0, 150.0, 190.0],
        vec![100.0, 125.0, 150.0, 190.0],
        vec![100.0, 100.0, 100.0, 100.0],
    ];
    for (place, (width, field)) in widths.iter().zip(fields.iter()).enumerate() {
        assert_eq!(
            *width == 0.0,
            width_zero_condition_holds(field),
            "mark {}",
            place + 1
        );
    }
    let width_zero: Vec<usize> = widths
        .iter()
        .enumerate()
        .filter(|(_, width)| **width == 0.0)
        .map(|(index, _)| index + 1)
        .collect();
    assert_eq!(width_zero, vec![5]);
    let spread = meet_spread(&meet).expect("the spread read");
    let spread_marks = mark_lines(&spread);
    let spread_zero: Vec<usize> = spread_marks
        .iter()
        .enumerate()
        .filter(|(_, line)| spread_of(line) == Some(0.0))
        .map(|(index, _)| index + 1)
        .collect();
    assert_eq!(spread_zero, vec![5]);
    assert_eq!(width_zero, spread_zero);
    let margins_reading = meet_margins(&meet).expect("the margins read");
    let margin_marks = mark_lines(&margins_reading);
    let margin_zero: Vec<usize> = margin_marks
        .iter()
        .enumerate()
        .filter(|(_, line)| margin_of(line) == Some(0.0))
        .map(|(index, _)| index + 1)
        .collect();
    assert_eq!(margin_zero, vec![1, 2, 3, 4, 5]);
    assert_ne!(width_zero, margin_zero);
    let range = meet_range(&meet).expect("the range read");
    let range_marks = mark_lines(&range);
    let range_zero: Vec<usize> = range_marks
        .iter()
        .enumerate()
        .filter(|(_, line)| range_of(line) == Some(0.0))
        .map(|(index, _)| index + 1)
        .collect();
    assert_eq!(range_zero, vec![5]);
    let tails = meet_tails(&meet).expect("the tails read");
    let tails_marks = mark_lines(&tails);
    let tails_zero: Vec<usize> = tails_marks
        .iter()
        .enumerate()
        .filter(|(_, line)| tails_difference_of(line) == Some(0.0))
        .map(|(index, _)| index + 1)
        .collect();
    assert_eq!(tails_zero, vec![5]);
    let counts = lines.last().expect("the width closes");
    assert_eq!(count_after(counts, "marks walked "), 5);
    assert_eq!(count_after(counts, "marks with another standing "), 5);
    assert_eq!(count_after(counts, "days "), 3);
    let edge = load_meet(&[
        day(
            1000,
            "a",
            &format!(
                "[{}, {}, {}, {}, {}, {}]",
                run("a", 500, &[(100, 10)]),
                run("b", 400, &[(100, 10)]),
                run("c", 300, &[(100, 10)]),
                run("d", 250, &[(100, 10)]),
                run("e", 200, &[(100, 10)]),
                run("f", 150, &[(100, 19)])
            ),
        ),
        quiet_day(1000, "a", "a"),
        quiet_day(1000, "a", "a"),
    ]);
    let edge_lines = meet_width(&edge).expect("the width read");
    let edge_marks = mark_lines(&edge_lines);
    assert_eq!(edge_marks.len(), 1);
    assert_eq!(width_of(edge_marks[0]), Some(90.0));
    assert_eq!(
        width_of(edge_marks[0]) == Some(0.0),
        width_zero_condition_holds(&[100.0, 100.0, 100.0, 100.0, 190.0])
    );
    let edge_range = meet_range(&edge).expect("the range read");
    let edge_range_marks = mark_lines(&edge_range);
    assert_eq!(range_of(edge_range_marks[0]), Some(0.0));
    let edge_tails = meet_tails(&edge).expect("the tails read");
    let edge_tails_marks = mark_lines(&edge_tails);
    assert_eq!(tails_difference_of(edge_tails_marks[0]), Some(30.0));
    let second_edge = load_meet(&[
        day(
            1000,
            "a",
            &format!(
                "[{}, {}, {}, {}]",
                run("a", 500, &[(100, 10)]),
                run("b", 400, &[(100, 15)]),
                run("c", 300, &[(100, 15)]),
                run("d", 250, &[(100, 15)])
            ),
        ),
        quiet_day(1000, "a", "a"),
        quiet_day(1000, "a", "a"),
    ]);
    let second_lines = meet_width(&second_edge).expect("the width read");
    let second_marks = mark_lines(&second_lines);
    assert_eq!(second_marks.len(), 1);
    assert_eq!(width_of(second_marks[0]), Some(0.0));
    assert_eq!(
        width_of(second_marks[0]) == Some(0.0),
        width_zero_condition_holds(&[150.0, 150.0, 150.0])
    );
    let second_range = meet_range(&second_edge).expect("the range read");
    let second_range_marks = mark_lines(&second_range);
    assert_eq!(range_of(second_range_marks[0]), Some(0.0));
    let second_tails = meet_tails(&second_edge).expect("the tails read");
    let second_tails_marks = mark_lines(&second_tails);
    assert_eq!(tails_difference_of(second_tails_marks[0]), Some(0.0));
}

fn below_meet() -> Meet {
    load_meet(&[
        day(
            1000,
            "a",
            &format!(
                "[{}, {}, {}, {}, {}]",
                run("a", 500, &[(100, 10)]),
                run("b", 400, &[(100, 11)]),
                run("c", 300, &[(100, 15)]),
                run("d", 250, &[(100, 17)]),
                run("e", 200, &[(100, 18), (200, 35)])
            ),
        ),
        quiet_day(1000, "a", "a"),
        quiet_day(1000, "a", "a"),
    ])
}

fn above_meets() -> [Meet; 3] {
    [
        load_meet(&[
            day(
                1000,
                "a",
                &format!(
                    "[{}, {}, {}, {}, {}]",
                    run("a", 500, &[(100, 10)]),
                    run("b", 400, &[(100, 12)]),
                    run("c", 300, &[(100, 15)]),
                    run("d", 250, &[(100, 17)]),
                    run("e", 200, &[(100, 19)])
                ),
            ),
            quiet_day(1000, "a", "a"),
            quiet_day(1000, "a", "a"),
        ]),
        load_meet(&[
            day(
                1000,
                "a",
                &format!(
                    "[{}, {}, {}, {}]",
                    run("a", 500, &[(100, 10)]),
                    run("b", 400, &[(100, 12)]),
                    run("c", 300, &[(100, 13)]),
                    run("d", 250, &[(100, 40)])
                ),
            ),
            quiet_day(1000, "a", "a"),
            quiet_day(1000, "a", "a"),
        ]),
        load_meet(&[
            day(
                1000,
                "a",
                &format!(
                    "[{}, {}, {}, {}, {}, {}]",
                    run("a", 500, &[(100, 10)]),
                    run("b", 400, &[(100, 12)]),
                    run("c", 300, &[(100, 13)]),
                    run("d", 250, &[(100, 14)]),
                    run("e", 200, &[(100, 38)]),
                    run("f", 150, &[(100, 40)])
                ),
            ),
            quiet_day(1000, "a", "a"),
            quiet_day(1000, "a", "a"),
        ]),
    ]
}

#[test]
fn test_the_field_width_stands_above_the_interquartile_range_and_the_difference_of_the_tails() {
    // The below meet of the quartiles file recomposed for
    // the midrange in the PG-46 discipline standing whole,
    // and the three above meets of the quartiles file
    // standing whole. The below meet's marks carry fields
    // of 110, 150, 170, and 180 and of 110, 150, 170, and
    // 175, their widths at 70 and 65 above each partner at
    // 45 and 45 and at 42.5 and 42.5. The field of four,
    // 120, 150, 170, and 190, carries its width at 70 above
    // each partner at 45. The field of three, 120, 130,
    // and 400, carries its width at 280 above each partner
    // at 140, each partner standing at exactly half the
    // width at count three. The field of five, 120, 130,
    // 140, 380, and 400, carries its width at 280 above
    // the interquartile range at 250 and the difference of
    // the tails at 176.666667.
    let below = below_meet();
    let width = meet_width(&below).expect("the width read");
    let range = meet_range(&below).expect("the range read");
    let tails = meet_tails(&below).expect("the tails read");
    let marks = mark_lines(&width);
    assert_eq!(marks.len(), 2);
    let range_marks = mark_lines(&range);
    let tails_marks = mark_lines(&tails);
    let expected = [(70.0, 45.0, 45.0), (65.0, 42.5, 42.5)];
    for (place, (mark, wants)) in marks.iter().zip(expected.iter()).enumerate() {
        let (want_width, want_range, want_tails) = wants;
        assert_eq!(width_of(mark), Some(*want_width));
        assert_eq!(range_of(range_marks[place]), Some(*want_range));
        let difference = tails_difference_of(tails_marks[place]).expect("a tails difference");
        assert!(
            (difference - want_tails).abs() <= 0.000002,
            "mark {} tails difference {}",
            place + 1,
            difference
        );
        assert!(
            want_width > want_range && *want_width > difference,
            "mark {} width above its partners",
            place + 1
        );
    }
    let above = above_meets();
    let expected_above = [
        (70.0, 45.0, 45.0),
        (280.0, 140.0, 140.0),
        (280.0, 250.0, 176.666667),
    ];
    for (meet, (want_width, want_range, want_tails)) in above.iter().zip(expected_above.iter()) {
        let width = meet_width(meet).expect("the width read");
        let range = meet_range(meet).expect("the range read");
        let tails = meet_tails(meet).expect("the tails read");
        let marks = mark_lines(&width);
        assert_eq!(marks.len(), 1);
        assert_eq!(width_of(marks[0]), Some(*want_width));
        assert_eq!(range_of(mark_lines(&range)[0]), Some(*want_range));
        let difference =
            tails_difference_of(mark_lines(&tails)[0]).expect("a tails difference stands");
        assert!(
            (difference - want_tails).abs() <= 0.000002,
            "tails difference {}",
            difference
        );
        assert!(
            want_width > want_range && *want_width > difference,
            "width above its partners"
        );
    }
}

#[test]
fn test_the_field_width_stands_at_the_spread_exactly_where_the_margin_stands_at_zero() {
    // The zero fixture of pin 5 standing whole, its widths
    // standing at its spreads at all five marks with the
    // margin at zero throughout, and the meets of pin 6,
    // their widths standing below their spreads at every
    // mark, 70 below 80 and 65 below 75 in the below meet
    // and 70 below 90, 280 below 300, and 280 below 300 in
    // the above meets, their margins standing at 10, 10,
    // 20, 20, and 20. At every mark the field width stands
    // at the spread exactly where the margin stands at
    // zero, checked as a biconditional, and below the
    // spread where the margin stands above zero.
    assert_width_spread_margin(
        &zero_fixture(),
        &[
            (90.0, 90.0, 0.0),
            (90.0, 90.0, 0.0),
            (90.0, 90.0, 0.0),
            (90.0, 90.0, 0.0),
            (0.0, 0.0, 0.0),
        ],
    );
    assert_width_spread_margin(&below_meet(), &[(70.0, 80.0, 10.0), (65.0, 75.0, 10.0)]);
    let above = above_meets();
    let above_expected = [
        (70.0, 90.0, 20.0),
        (280.0, 300.0, 20.0),
        (280.0, 300.0, 20.0),
    ];
    for (meet, expected) in above.iter().zip(above_expected.iter()) {
        assert_width_spread_margin(meet, &[*expected]);
    }
}

fn assert_width_spread_margin(meet: &Meet, expected: &[(f64, f64, f64)]) {
    let width = meet_width(meet).expect("the width read");
    let spread = meet_spread(meet).expect("the spread read");
    let margins = meet_margins(meet).expect("the margins read");
    let width_marks = mark_lines(&width);
    let spread_marks = mark_lines(&spread);
    let margin_marks = mark_lines(&margins);
    assert_eq!(width_marks.len(), expected.len());
    for (place, (width_line, wants)) in width_marks.iter().zip(expected.iter()).enumerate() {
        let (want_width, want_spread, want_margin) = wants;
        let width = width_of(width_line).expect("a field width stands");
        let spread = spread_of(spread_marks[place]).expect("a spread stands");
        let margin = margin_of(margin_marks[place]).expect("a margin stands");
        assert_eq!(width, *want_width, "mark {}", place + 1);
        assert_eq!(spread, *want_spread, "mark {}", place + 1);
        assert_eq!(margin, *want_margin, "mark {}", place + 1);
        assert_eq!(width == spread, margin == 0.0, "mark {}", place + 1);
        assert!(width <= spread, "mark {}", place + 1);
        if margin > 0.0 {
            assert!(width < spread, "mark {}", place + 1);
        }
    }
}

#[test]
fn test_the_narrowest_field_width_tie_names_the_earliest_mark() {
    // Four marks stand on the first day, the first with no
    // other standing, the second carrying a field of 140,
    // 150, and 260 with a field width of 120, the third
    // and fourth carrying a field of 130, 140, and 150
    // with a field width of 20, the fourth standing as the
    // holder's own re-read at its standing figure. The tie
    // meet stands built for the width in the family's
    // recomposition discipline, the 260 standing set at
    // the first mark by the fourth run's first read, where
    // that run holds the record alone, and loosened at
    // the third mark by its re-read at 130, its usages
    // rising 26 and 39 under the loader's rule. The
    // counting names the third mark, the earliest at the
    // narrowest width, the tie real in the printed lines
    // at two marks whose day and place differ whole.
    let meet = load_meet(&[
        day(
            1000,
            "a",
            &format!(
                "[{}, {}, {}, {}]",
                run("a", 500, &[(200, 20), (400, 40)]),
                run("b", 400, &[(200, 28)]),
                run("c", 300, &[(200, 30)]),
                run("d", 250, &[(100, 26), (300, 39)])
            ),
        ),
        quiet_day(1000, "a", "a"),
        quiet_day(1000, "a", "a"),
    ]);
    let lines = meet_width(&meet).expect("the width read");
    assert_eq!(
        lines,
        vec![
            "meet width, 3 days",
            "day, the first day, marks 4",
            "mark 1, at 100 seconds on the first day, d at a projection of 260, and no other standing",
            "mark 2, at 200 seconds on the first day, a at a projection of 100, field width 120",
            "mark 3, at 300 seconds on the first day, a at a projection of 100, field width 20",
            "mark 4, at 400 seconds on the first day, a at a projection of 100, field width 20",
            "day, the second day, marks 0",
            "day, the third day, marks 0",
            "width counts, marks walked 4, marks with another standing 3, days 3, narrowest field width 20 at 300 seconds on the first day",
        ]
    );
}

#[test]
fn test_the_field_width_steps_as_the_field_grows() {
    let meet = load_meet(&[
        day(
            1000,
            "a",
            &format!(
                "[{}, {}, {}, {}, {}, {}]",
                run("a", 500, &[(100, 10)]),
                run("b", 400, &[(100, 15)]),
                run("c", 300, &[(200, 42)]),
                run("d", 250, &[(300, 72)]),
                run("e", 600, &[(400, 132)]),
                run("f", 550, &[(500, 210)])
            ),
        ),
        quiet_day(1000, "a", "a"),
        quiet_day(1000, "a", "a"),
    ]);
    let lines = meet_width(&meet).expect("the width read");
    let marks = mark_lines(&lines);
    let widths: Vec<f64> = marks
        .iter()
        .map(|line| width_of(line).expect("a field width stands"))
        .collect();
    assert_eq!(widths, vec![0.0, 60.0, 90.0, 180.0, 270.0]);
    assert_eq!(
        marks[0].as_str(),
        "mark 1, at 100 seconds on the first day, a at a projection of 100, field width 0"
    );
    assert_eq!(
        marks[4].as_str(),
        "mark 5, at 500 seconds on the first day, a at a projection of 100, field width 270"
    );
}

#[test]
fn test_the_width_identity_and_the_bound_chain_hold_line_by_line() {
    // The identity and the bound chain of PG-52 over the
    // hand built meet of pin 1. At every mark the field
    // width is the exact spread of PG-32 less the exact
    // margin of PG-30, and the chain holds line by line,
    // the interquartile range of PG-44 never more than the
    // field width and the field width never more than the
    // spread, the five widths pinned at 0, 20, 50, 70, and
    // 100. The width stands equal to the interquartile
    // range exactly where the lower quartile standing
    // stands at the nearest standing and the upper
    // quartile standing stands at the farthest, which over
    // this meet holds at the first and second marks and at
    // no other, and equal to the spread exactly where the
    // margin stands at zero, which over this meet holds at
    // no mark, the margin standing above zero at every
    // mark, each equality checked as a biconditional at
    // every mark against the standings the partner
    // readings state.
    let meet = width_meet();
    let width = meet_width(&meet).expect("the width read");
    let margins = meet_margins(&meet).expect("the margins read");
    let spread = meet_spread(&meet).expect("the spread read");
    let range = meet_range(&meet).expect("the range read");
    let quartiles = meet_quartiles(&meet).expect("the quartiles read");
    let width_marks = mark_lines(&width);
    let margin_marks = mark_lines(&margins);
    let spread_marks = mark_lines(&spread);
    let range_marks = mark_lines(&range);
    let quartile_marks = mark_lines(&quartiles);
    assert_eq!(width_marks.len(), margin_marks.len());
    assert_eq!(width_marks.len(), spread_marks.len());
    assert_eq!(width_marks.len(), range_marks.len());
    assert_eq!(width_marks.len(), quartile_marks.len());
    let expected_widths = [0.0, 20.0, 50.0, 70.0, 100.0];
    for (place, width_line) in width_marks.iter().enumerate() {
        let width = width_of(width_line).expect("a field width stands");
        let margin = margin_of(margin_marks[place]).expect("a margin stands");
        let spread = spread_of(spread_marks[place]).expect("a spread stands");
        let range = range_of(range_marks[place]).expect("an interquartile range stands");
        let (lower_quartile, upper_quartile) =
            quartile_standings_of(quartile_marks[place]).expect("quartile standings stand");
        let nearest = nearest_of(margin_marks[place]).expect("a nearest standing stands");
        let farthest = farthest_of(spread_marks[place]).expect("a farthest standing stands");
        assert_eq!(width, expected_widths[place], "mark {}", place + 1);
        assert!(
            (width - (spread - margin)).abs() <= 0.000002,
            "mark {}",
            place + 1
        );
        assert!(range <= width + 0.000002, "mark {}", place + 1);
        assert!(width <= spread + 0.000002, "mark {}", place + 1);
        assert_eq!(
            width == range,
            lower_quartile == nearest && upper_quartile == farthest,
            "mark {}",
            place + 1
        );
        assert_eq!(width == spread, margin == 0.0, "mark {}", place + 1);
        assert!(width >= 0.0, "mark {}", place + 1);
    }
}

#[test]
fn test_a_meet_in_which_no_read_projects_reads_with_marks_walked_zero() {
    let meet = load_meet(&[
        quiet_day(600, "a", "a"),
        quiet_day(600, "a", "a"),
        quiet_day(600, "a", "a"),
    ]);
    let lines = meet_width(&meet).expect("the width read");
    assert_eq!(
        lines,
        vec![
            "meet width, 3 days",
            "day, the first day, marks 0",
            "day, the second day, marks 0",
            "day, the third day, marks 0",
            "width counts, marks walked 0, marks with another standing 0, days 3, and, no other standing at any mark",
        ]
    );
}

// The guard pins. The width reading stands inside the
// fencing closure of FIX-PG-11 from its birth, and under
// the figure guard of FIX-PG-10 and the day count errors
// of PG-51, in the one walk it shares with the eighteen
// readings before it.

#[test]
fn test_a_hand_built_meet_with_no_days_draws_the_named_error_through_width() {
    let meet = Meet { days: vec![] };
    let error = meet_width(&meet).expect_err("the reading names the precondition");
    assert_eq!(error, "the meet holds no days");
}

#[test]
fn test_a_hand_built_meet_with_too_many_days_draws_the_named_error_through_width() {
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
    let error = meet_width(&meet).expect_err("the reading names the precondition");
    assert_eq!(
        error,
        "the meet holds more days than the ordinal table names"
    );
}

#[test]
fn test_a_hand_built_non_finite_figure_draws_the_named_error_through_width() {
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
    let error = meet_width(&meet).expect_err("the reading names the precondition");
    assert_eq!(error, "the window is not a finite number");
}

#[test]
fn test_a_hand_built_break_bearing_label_draws_the_named_error_through_width() {
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
    let error = meet_width(&meet).expect_err("the reading names the precondition");
    assert_eq!(error, "a runner label holds a line break");
}

// The exact difference pin. The field width of PG-52 is
// the difference of the farthest standing and the
// nearest standing computed on exact values, and this
// pin states marks whose two extreme standings render
// alike at six decimal places while their exact
// difference does not stand at zero.

#[test]
fn test_the_width_is_the_exact_difference_of_extremes_that_render_alike() {
    // The field of the second mark counts one figure at
    // 1 + 2^-22, the standing of the one competitor behind
    // the holder, rendering as 1, its field width standing
    // at zero whole. The field of the third mark counts
    // two figures at 1 + 2^-22 and 1 + 2^-21, both
    // rendering as 1, its field width standing at 2^-22
    // rendered in full. A width computed by differencing
    // the rendered extreme standings would stand at 0 at
    // the third mark. The holder stands at a projection of
    // 1 whole, read under the first day's window at the
    // window itself.
    let meet = load_meet(&[
        day(
            4194305,
            "a",
            &format!(
                "[{}, {}]",
                run("a", 500, &[(4194305, 1)]),
                run("b", 400, &[(4194304, 1)])
            ),
        ),
        day(
            2097153,
            "a",
            &format!(
                "[{}, {}, {}]",
                run("a", 500, &[]),
                run("b", 400, &[]),
                run("c", 300, &[(2097152, 1)])
            ),
        ),
        quiet_day(1000, "a", "a"),
    ]);
    let lines = meet_width(&meet).expect("the width read");
    assert_eq!(
        lines,
        vec![
            "meet width, 3 days",
            "day, the first day, marks 2",
            "mark 1, at 4194304 seconds on the first day, b at a projection of 1, and no other standing",
            "mark 2, at 4194305 seconds on the first day, a at a projection of 1, field width 0",
            "day, the second day, marks 1",
            "mark 3, at 2097152 seconds on the second day, a at a projection of 1, field width 0.0000002384185791015625",
            "day, the third day, marks 0",
            "width counts, marks walked 3, marks with another standing 2, days 3, narrowest field width 0 at 4194305 seconds on the first day",
        ]
    );
}

#[test]
fn test_the_width_reading_over_the_separating_meet_line_by_line() {
    let meet = separating_meet();
    let expected: Vec<String> = [
        "meet width, 3 days",
        "day, the first day, marks 2",
        "mark 1, at 100 seconds on the first day, a at a projection of 100, field width 0",
        "mark 2, at 200 seconds on the first day, a at a projection of 100, field width 30",
        "day, the second day, marks 0",
        "day, the third day, marks 1",
        "mark 3, at 100 seconds on the third day, a at a projection of 100, field width 150",
        "width counts, marks walked 3, marks with another standing 3, days 3, narrowest field width 0 at 100 seconds on the first day",
    ]
    .iter()
    .map(|line| line.to_string())
    .collect();
    assert_eq!(meet_width(&meet), Ok(expected));
}

#[test]
fn test_the_separation_relation_over_the_separating_meet() {
    let meet = separating_meet();
    let width = meet_width(&meet).expect("the width reading stands");
    let range = meet_range(&meet).expect("the range reading stands");
    let tails = meet_tails(&meet).expect("the tails reading stands");
    let quartiles = meet_quartiles(&meet).expect("the quartiles reading stands");
    let width_marks = mark_lines(&width);
    let range_marks = mark_lines(&range);
    let tails_marks = mark_lines(&tails);
    let quartiles_marks = mark_lines(&quartiles);
    assert_eq!(width_marks.len(), 3);
    assert_eq!(range_marks.len(), 3);
    assert_eq!(tails_marks.len(), 3);
    assert_eq!(quartiles_marks.len(), 3);
    for (place, (((width_line, range_line), tails_line), quartiles_line)) in width_marks
        .iter()
        .zip(range_marks.iter())
        .zip(tails_marks.iter())
        .zip(quartiles_marks.iter())
        .enumerate()
    {
        let width_figure = width_of(width_line).expect("a field width stands");
        let range_figure = range_of(range_line).expect("an interquartile range stands");
        assert_eq!(range_figure, width_figure, "mark {}", place + 1);
        let difference = tails_difference_of(tails_line).expect("the tail standings stand");
        if place < 2 {
            assert_eq!(difference, width_figure, "mark {}", place + 1);
        } else {
            assert!(difference < width_figure, "mark {}", place + 1);
            assert!(
                (width_figure - difference - 50.0).abs() < 0.001,
                "mark {}",
                place + 1
            );
            let (lower_quartile, upper_quartile) =
                quartile_standings_of(quartiles_line).expect("the quartile standings stand");
            assert!((lower_quartile - 150.0).abs() < 0.001, "mark {}", place + 1);
            assert!((upper_quartile - 300.0).abs() < 0.001, "mark {}", place + 1);
        }
    }
}
