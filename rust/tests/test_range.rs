//! Tests for PG-43 and PG-44, the range reading over a meet.
//!
//! Written for Round 21. Each pin's failure mode stands
//! stated here pin by pin, not as one blanket claim. Pins 1
//! to 15 fail before the round by the absence of the reading,
//! the meet_range function not standing on the pre round
//! tree. On the built tree, pin 1 fails under R-1 from the
//! first mark onward, the ranges becoming 50, 35, 50, 50,
//! and 70 over the enlarged fields, under R-2 at its field
//! of five alone, the difference of the tails standing at
//! 46.666667 there against the range's 40, under R-3 from
//! its field of three onward in value at the third, fourth,
//! and fifth marks, under R-5 from the first mark onward,
//! and under R-6 as under R-1, and it passes under R-4 by
//! design, its narrowest range standing unique. Pin 2
//! passes under all six mutations by design and claims no
//! mutation failure, the mark line prefix it compares
//! standing rendered from the walk's mark records, which no
//! mutation of this round touches. Pin 3 fails under R-1 at
//! every close, under R-3 at its closes of three and four,
//! its close of one holding by design, under R-5 at every
//! close, and under R-6 at every close, and it passes under
//! R-2 by design at every close, the closes standing at
//! fields of four or fewer where the quartiles are the
//! tails, and under R-4 by design, no counts line standing
//! in it. Pin 4 fails under R-1 and under R-6 by design, the
//! holder's own figure entering the field at every mark so
//! a range stands where the pin states none, and it passes
//! under R-2, R-3, R-4, and R-5 by design. Pin 5 fails under
//! R-1 from its second mark onward in its zero set, the
//! ranges becoming 45, 0, 50, 50, and 0, under R-3 in value
//! at every mark but the fifth, the spreads standing at 90
//! against the ranges, under R-5 at its third mark, the
//! lagged vector printing 45 against 70 there, and under
//! R-6 as under R-1, and it passes under R-2 by design at
//! every mark, every field counting four or fewer so the
//! range is the difference of the tails whole, and under
//! R-4 by design, reading ranges at marks and never the
//! narrowest naming. Pin 6 fails under R-1 at both marks,
//! the ranges becoming 190 and 70, under R-2 at both marks,
//! its designed catcher in dedicated form, under R-3 at
//! both marks, the spreads standing at 200 and 100, under
//! R-5 at its second mark, and under R-6 as under R-1, and
//! it passes under R-4 by design. Pin 7 fails under R-1 at
//! every mark, the ranges becoming 35, 50, and 70, under
//! R-2 at its third mark alone, under R-3 at its second and
//! third marks, its designed catcher in dedicated form, the
//! first mark holding where the range is the spread whole,
//! under R-5 at its second mark, and under R-6 as under
//! R-1, and it passes under R-4 by design. Pin 8 fails
//! under R-1, the ranges becoming 65, 80, and 40 and the
//! counting naming the third mark at 40, under R-3, the
//! printed ranges becoming the spreads 70, 100, and 50 and
//! the counting naming the third mark, under R-4, its
//! designed catcher, the fixture tying its range at the
//! first and third marks so the loosened scan names the
//! third, under R-5, and under R-6 as under R-1, and it
//! passes under R-2 by design, every field of the fixture
//! counting four or fewer where the range is the difference
//! of the tails whole. Pin 9 fails under R-1 at its first
//! step, under R-2 at its fifth step alone, the difference
//! of the tails standing at 130 against the range's 120
//! there, under R-3 from its third step onward, under R-5,
//! and under R-6 as under R-1, and it passes under R-4 by
//! design, its assertions standing on the mark lines. Pin
//! 10 fails under R-1 at its field of one's pinned value,
//! the identity itself standing, under R-2 at the field of
//! five's pinned value, the difference of the tails
//! standing at 46.666667 against the range's 40 while the
//! identity and bound clauses hold by design, under R-3 at
//! its field of three on the identity, the lower quartile
//! margin and the spread summing past the upper quartile
//! margin there, under R-5 at its second mark, and under
//! R-6 on its pinned values as under R-1, and it passes
//! under R-4 by design, reading mark lines alone. Pin 11
//! passes under all six mutations by design, no mark
//! standing for any selection, vector, or closing scan to
//! move. Pins 12 to 15 pass under all six mutations by
//! design, the guards standing ahead of any selection or
//! scan, and they fail on a built tree whose reading
//! skipped the FIX-PG-10 figure guard, the FIX-PG-11 label
//! fencing, or the day count preconditions of PG-43.

use paceghost::fieldday::{DayRun, FieldDay};
use paceghost::meet::{
    load_meet_text, meet_margins, meet_marks, meet_quartiles, meet_range, meet_spread,
    meet_standings, meet_tails, Meet,
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

struct RangeMark {
    prefix: String,
    range: Option<f64>,
}

fn range_mark_parts(mark_line: &str) -> RangeMark {
    let prefix = record_prefix(mark_line);
    if mark_line.contains(", and no other standing") {
        return RangeMark {
            prefix,
            range: None,
        };
    }
    let text = mark_line
        .split(", interquartile range ")
        .nth(1)
        .unwrap_or_else(|| panic!("an interquartile range stands in {}", mark_line));
    RangeMark {
        prefix,
        range: Some(
            text.parse()
                .unwrap_or_else(|_| panic!("the interquartile range parses in {}", mark_line)),
        ),
    }
}

fn range_of(mark_line: &str) -> Option<f64> {
    range_mark_parts(mark_line).range
}

struct QuartileMark {
    lower: Option<(f64, f64)>,
    upper: Option<(f64, f64)>,
}

fn quartiles_mark_parts(mark_line: &str) -> QuartileMark {
    if mark_line.contains(", and no other standing") {
        return QuartileMark {
            lower: None,
            upper: None,
        };
    }
    let lower_figure_text = mark_line
        .split(", lower quartile standing ")
        .nth(1)
        .and_then(|rest| rest.split(',').next())
        .unwrap_or_else(|| panic!("a lower quartile standing stands in {}", mark_line));
    let lower_margin_text = mark_line
        .split(", lower quartile margin ")
        .nth(1)
        .and_then(|rest| rest.split(',').next())
        .unwrap_or_else(|| panic!("a lower quartile margin stands in {}", mark_line));
    let upper_figure_text = mark_line
        .split(", upper quartile standing ")
        .nth(1)
        .and_then(|rest| rest.split(',').next())
        .unwrap_or_else(|| panic!("an upper quartile standing stands in {}", mark_line));
    let upper_margin_text = mark_line
        .split(", upper quartile margin ")
        .nth(1)
        .and_then(|rest| rest.split(',').next())
        .unwrap_or_else(|| panic!("an upper quartile margin stands in {}", mark_line));
    QuartileMark {
        lower: Some((
            lower_figure_text
                .parse()
                .unwrap_or_else(|_| panic!("the lower quartile standing parses in {}", mark_line)),
            lower_margin_text
                .parse()
                .unwrap_or_else(|_| panic!("the lower quartile margin parses in {}", mark_line)),
        )),
        upper: Some((
            upper_figure_text
                .parse()
                .unwrap_or_else(|_| panic!("the upper quartile standing parses in {}", mark_line)),
            upper_margin_text
                .parse()
                .unwrap_or_else(|_| panic!("the upper quartile margin parses in {}", mark_line)),
        )),
    }
}

fn tail_margins_of(mark_line: &str) -> Option<(f64, f64)> {
    if mark_line.contains(", and no other standing") {
        return None;
    }
    let lower_text = mark_line
        .split(", lower tail margin ")
        .nth(1)
        .and_then(|rest| rest.split(',').next())
        .unwrap_or_else(|| panic!("a lower tail margin stands in {}", mark_line));
    let upper_text = mark_line
        .split(", upper tail margin ")
        .nth(1)
        .and_then(|rest| rest.split(',').next())
        .unwrap_or_else(|| panic!("an upper tail margin stands in {}", mark_line));
    Some((
        lower_text
            .parse()
            .unwrap_or_else(|_| panic!("the lower tail margin parses in {}", mark_line)),
        upper_text
            .parse()
            .unwrap_or_else(|_| panic!("the upper tail margin parses in {}", mark_line)),
    ))
}

fn nearest_figure_of(mark_line: &str) -> Option<f64> {
    if mark_line.contains(", and no other standing") {
        return None;
    }
    let text = mark_line
        .split(", nearest standing ")
        .nth(1)
        .and_then(|rest| rest.split(" held by ").next())
        .unwrap_or_else(|| panic!("a nearest standing stands in {}", mark_line));
    Some(
        text.parse()
            .unwrap_or_else(|_| panic!("the nearest standing parses in {}", mark_line)),
    )
}

fn farthest_figure_of(mark_line: &str) -> Option<f64> {
    if mark_line.contains(", and no other standing") {
        return None;
    }
    let text = mark_line
        .split(", farthest standing ")
        .nth(1)
        .and_then(|rest| rest.split(" held by ").next())
        .unwrap_or_else(|| panic!("a farthest standing stands in {}", mark_line));
    Some(
        text.parse()
            .unwrap_or_else(|_| panic!("the farthest standing parses in {}", mark_line)),
    )
}

/// The spread of the field itself at a mark, the
/// farthest standing figure less the nearest standing
/// figure, read across the spread and margins readings.
fn field_spread(spread_line: &str, margin_line: &str) -> Option<f64> {
    match (
        farthest_figure_of(spread_line),
        nearest_figure_of(margin_line),
    ) {
        (Some(farthest), Some(nearest)) => Some(farthest - nearest),
        _ => None,
    }
}

fn median_of(figures: &[f64]) -> f64 {
    let count = figures.len();
    if count % 2 == 1 {
        figures[count / 2]
    } else {
        (figures[count / 2 - 1] + figures[count / 2]) / 2.0
    }
}

fn range_meet() -> Meet {
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

#[test]
fn test_the_range_reading_over_a_hand_built_meet_line_by_line() {
    let meet = range_meet();
    let expected: Vec<String> = [
        "meet range, 3 days",
        "day, the first day, marks 2",
        "mark 1, at 100 seconds on the first day, a at a projection of 100, interquartile range 0",
        "mark 2, at 200 seconds on the first day, a at a projection of 100, interquartile range 20",
        "day, the second day, marks 0",
        "day, the third day, marks 3",
        "mark 3, at 100 seconds on the third day, a at a projection of 100, interquartile range 25",
        "mark 4, at 200 seconds on the third day, a at a projection of 100, interquartile range 45",
        "mark 5, at 300 seconds on the third day, a at a projection of 100, interquartile range 40",
        "range counts, marks walked 5, marks with another standing 5, days 3, narrowest interquartile range 0 at 100 seconds on the first day",
    ]
    .iter()
    .map(|line| line.to_string())
    .collect();
    assert_eq!(meet_range(&meet), Ok(expected));
    let tails = meet_tails(&meet).expect("the tails read");
    let (lower_tail_margin, upper_tail_margin) =
        tail_margins_of(mark_lines(&tails)[4]).expect("tail margins stand");
    let difference = upper_tail_margin - lower_tail_margin;
    assert!(
        (difference - 46.666667).abs() <= 0.000002,
        "difference of the tails {}",
        difference
    );
    assert!(
        (40.0 - difference).abs() > 0.000002,
        "the range separates from the difference of the tails at the field of five"
    );
    let spread = meet_spread(&meet).expect("the spread read");
    let margins = meet_margins(&meet).expect("the margins read");
    assert_eq!(
        field_spread(mark_lines(&spread)[4], mark_lines(&margins)[4]),
        Some(100.0)
    );
}

#[test]
fn test_range_mark_lines_agree_with_the_marks_reading_line_by_line() {
    let meet = range_meet();
    let marks = meet_marks(&meet).expect("the marks reading stands");
    let range = meet_range(&meet).expect("the range reading stands");
    assert_eq!(
        range.first().map(String::as_str),
        Some("meet range, 3 days")
    );
    assert_eq!(
        marks.first().map(String::as_str),
        Some("meet marks, 3 days")
    );
    let marks_days: Vec<&String> = marks
        .iter()
        .filter(|line| line.starts_with("day,"))
        .collect();
    let range_days: Vec<&String> = range
        .iter()
        .filter(|line| line.starts_with("day,"))
        .collect();
    assert_eq!(marks_days, range_days);
    let marks_marks = mark_lines(&marks);
    let range_marks = mark_lines(&range);
    assert_eq!(marks_marks.len(), range_marks.len());
    for (place, (marks_line, range_line)) in marks_marks.iter().zip(range_marks.iter()).enumerate()
    {
        let parts = range_mark_parts(range_line);
        assert_eq!(
            parts.prefix,
            record_prefix(marks_line),
            "mark {}",
            place + 1
        );
        assert!(
            range_line.starts_with(&format!("mark {}, ", place + 1)),
            "mark {} carries its place in order",
            place + 1
        );
    }
    let marks_counts = marks.last().expect("the marks counts line stands");
    let range_counts = range.last().expect("the range counts line stands");
    assert_eq!(
        count_after(marks_counts, "marks walked "),
        count_after(range_counts, "marks walked ")
    );
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
                run("b", 400, &[(100, 15)])
            ),
        ),
        day(
            1000,
            "a",
            &format!(
                "[{}, {}, {}, {}]",
                run("a", 500, &[]),
                run("b", 400, &[]),
                run("c", 300, &[(100, 12)]),
                run("d", 250, &[(100, 17)])
            ),
        ),
        day(
            1000,
            "a",
            &format!(
                "[{}, {}, {}, {}, {}]",
                run("a", 500, &[]),
                run("b", 400, &[]),
                run("c", 300, &[]),
                run("d", 250, &[]),
                run("e", 200, &[(100, 19)])
            ),
        ),
    ]);
    let range = meet_range(&meet).expect("the range reading stands");
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
    let mut range_sections: Vec<Vec<&String>> = Vec::new();
    for line in &range {
        if line.starts_with("day, the ") {
            range_sections.push(Vec::new());
        }
        if let Some(section) = range_sections.last_mut() {
            section.push(line);
        }
    }
    assert_eq!(sections.len(), range_sections.len());
    let expected_counts = [1usize, 3, 4];
    let expected_ranges = [0.0f64, 25.0, 45.0];
    for (day_index, (section, range_section)) in
        sections.iter().zip(range_sections.iter()).enumerate()
    {
        let day_marks: Vec<&&String> = range_section
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
        let count = others.len().div_ceil(2);
        let expected = median_of(&others[others.len() - count..]) - median_of(&others[..count]);
        let named = expected_ranges[day_index];
        assert!(
            (expected - named).abs() <= 0.000002,
            "day {} closes at the named range",
            day_index + 1
        );
        let last = day_marks.last().expect("the day holds a mark");
        assert_eq!(
            range_of(last),
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
    let lines = meet_range(&meet).expect("the range read");
    assert_eq!(
        lines,
        vec![
            "meet range, 3 days",
            "day, the first day, marks 1",
            "mark 1, at 100 seconds on the first day, solo at a projection of 100, and no other standing",
            "day, the second day, marks 1",
            "mark 2, at 200 seconds on the second day, solo at a projection of 150, and no other standing",
            "day, the third day, marks 0",
            "range counts, marks walked 2, marks with another standing 0, days 3, and, no other standing at any mark",
        ]
    );
}

fn middle_place_figures(part: &[f64]) -> Vec<f64> {
    if part.len() % 2 == 1 {
        vec![part[part.len() / 2]]
    } else {
        vec![part[part.len() / 2 - 1], part[part.len() / 2]]
    }
}

fn zero_condition_holds(field: &[f64]) -> bool {
    let count = field.len().div_ceil(2);
    let mut figures = middle_place_figures(&field[..count]);
    figures.extend(middle_place_figures(&field[field.len() - count..]));
    figures.iter().all(|figure| *figure == figures[0])
}

#[test]
fn test_the_range_stands_at_zero_exactly_where_the_sub_fields_middle_places_stand_at_one_figure() {
    // The shipped repaired fixture of the quartiles pin on
    // the zero sets. At the first mark the field behind the
    // holder stands at 100, 100, and 190 and the range
    // stands at 45. At the second mark the field stands at
    // 100, 100, 100, and 190 and the range stands at 45. At
    // the third mark the field stands at 100, 100, 150, and
    // 190 and the range stands at 70. At the fourth mark the
    // field stands at 100, 125, 150, and 190 and the range
    // stands at 57.5. At the fifth mark every standing
    // figure of the field stands at 100 and the range stands
    // at zero. The range's zero set is exactly the fifth
    // mark, while the lower quartile margin stands at zero
    // at the first, second, third, and fifth marks and the
    // upper quartile margin at the fifth alone, and the zero
    // condition of PG-43, every middle place figure of the
    // two sub-fields standing at one figure, holds at the
    // fifth mark alone, checked in both directions at every
    // mark from the fields the fixture states.
    let meet = load_meet(&[
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
    ]);
    let lines = meet_range(&meet).expect("the range read");
    let marks = mark_lines(&lines);
    assert_eq!(marks.len(), 5);
    let ranges: Vec<f64> = marks
        .iter()
        .map(|line| range_of(line).expect("a range stands"))
        .collect();
    assert_eq!(ranges, vec![45.0, 45.0, 70.0, 57.5, 0.0]);
    let fields: [Vec<f64>; 5] = [
        vec![100.0, 100.0, 190.0],
        vec![100.0, 100.0, 100.0, 190.0],
        vec![100.0, 100.0, 150.0, 190.0],
        vec![100.0, 125.0, 150.0, 190.0],
        vec![100.0, 100.0, 100.0, 100.0],
    ];
    for (place, (range, field)) in ranges.iter().zip(fields.iter()).enumerate() {
        assert_eq!(
            *range == 0.0,
            zero_condition_holds(field),
            "mark {}",
            place + 1
        );
    }
    let range_zero: Vec<usize> = ranges
        .iter()
        .enumerate()
        .filter(|(_, range)| **range == 0.0)
        .map(|(index, _)| index + 1)
        .collect();
    assert_eq!(range_zero, vec![5]);
    let quartiles = meet_quartiles(&meet).expect("the quartiles read");
    let quartile_marks = mark_lines(&quartiles);
    let lower_zero: Vec<usize> = quartile_marks
        .iter()
        .enumerate()
        .filter(|(_, line)| quartiles_mark_parts(line).lower.map(|(_, margin)| margin) == Some(0.0))
        .map(|(index, _)| index + 1)
        .collect();
    assert_eq!(lower_zero, vec![1, 2, 3, 5]);
    let upper_zero: Vec<usize> = quartile_marks
        .iter()
        .enumerate()
        .filter(|(_, line)| quartiles_mark_parts(line).upper.map(|(_, margin)| margin) == Some(0.0))
        .map(|(index, _)| index + 1)
        .collect();
    assert_eq!(upper_zero, vec![5]);
    let counts = lines.last().expect("the range closes");
    assert_eq!(count_after(counts, "marks walked "), 5);
    assert_eq!(count_after(counts, "marks with another standing "), 5);
    assert_eq!(count_after(counts, "days "), 3);
}

#[test]
fn test_the_range_separates_from_the_difference_of_the_tails_in_both_directions() {
    // At the first mark five competitors stand behind the
    // record of 100 at 120, 130, 300, 310, and 320, the
    // range standing at 180, the middle figure of the three
    // highest less the middle figure of the three lowest,
    // and the difference of the tails standing at
    // 126.666667, the range above it. At the second mark
    // the four higher competitors read again and the field
    // stands at 120, 150, 170, 190, and 220, the range
    // standing at 40 and the difference of the tails at
    // 46.666667, the range below it, the two directions of
    // the no fixed order relation of PG-44 in one fixture.
    let meet = load_meet(&[
        day(
            1000,
            "a",
            &format!(
                "[{}, {}, {}, {}, {}, {}]",
                run("a", 500, &[(100, 10)]),
                run("b", 400, &[(100, 12)]),
                run("c", 300, &[(100, 13), (200, 30)]),
                run("d", 250, &[(100, 30), (200, 34)]),
                run("e", 200, &[(100, 31), (200, 38)]),
                run("f", 150, &[(100, 32), (200, 44)])
            ),
        ),
        quiet_day(1000, "a", "a"),
        quiet_day(1000, "a", "a"),
    ]);
    let range = meet_range(&meet).expect("the range read");
    let tails = meet_tails(&meet).expect("the tails read");
    let marks = mark_lines(&range);
    let tails_marks = mark_lines(&tails);
    assert_eq!(marks.len(), 2);
    assert_eq!(range_of(marks[0]), Some(180.0));
    let (lower_tail_margin, upper_tail_margin) =
        tail_margins_of(tails_marks[0]).expect("tail margins stand");
    let difference = upper_tail_margin - lower_tail_margin;
    assert!(
        (difference - 126.666667).abs() <= 0.000002,
        "difference of the tails {}",
        difference
    );
    assert!(
        180.0 > difference,
        "range 180 above the difference {}",
        difference
    );
    assert_eq!(range_of(marks[1]), Some(40.0));
    let (lower_tail_margin, upper_tail_margin) =
        tail_margins_of(tails_marks[1]).expect("tail margins stand");
    let difference = upper_tail_margin - lower_tail_margin;
    assert!(
        (difference - 46.666667).abs() <= 0.000002,
        "difference of the tails {}",
        difference
    );
    assert!(
        40.0 < difference,
        "range 40 below the difference {}",
        difference
    );
}

#[test]
fn test_the_range_separates_from_the_spread() {
    // At the first mark the field stands at 150 and 170 and
    // the range stands at 20, equal to the spread there by
    // the count two coincidence, asserted as equality. At
    // the second mark the field stands at 120, 150, and 170
    // and the range stands at 25 against a spread of 50. At
    // the third mark the field stands at 120, 150, 170,
    // 190, and 220 and the range stands at 40 against a
    // spread of 100, the separation standing from the field
    // of three onward, where a sub-field median first stands
    // inside the field's extremes.
    let meet = load_meet(&[
        day(
            1000,
            "a",
            &format!(
                "[{}, {}, {}, {}, {}, {}]",
                run("a", 500, &[(100, 10)]),
                run("b", 400, &[(100, 15)]),
                run("c", 300, &[(100, 17)]),
                run("d", 250, &[(200, 24)]),
                run("e", 200, &[(300, 57)]),
                run("f", 150, &[(300, 66)])
            ),
        ),
        quiet_day(1000, "a", "a"),
        quiet_day(1000, "a", "a"),
    ]);
    let range = meet_range(&meet).expect("the range read");
    let spread = meet_spread(&meet).expect("the spread read");
    let margins = meet_margins(&meet).expect("the margins read");
    let marks = mark_lines(&range);
    let spread_marks = mark_lines(&spread);
    let margin_marks = mark_lines(&margins);
    assert_eq!(marks.len(), 3);
    assert_eq!(range_of(marks[0]), Some(20.0));
    assert_eq!(field_spread(spread_marks[0], margin_marks[0]), Some(20.0));
    assert_eq!(range_of(marks[1]), Some(25.0));
    assert_eq!(field_spread(spread_marks[1], margin_marks[1]), Some(50.0));
    assert_eq!(range_of(marks[2]), Some(40.0));
    assert_eq!(field_spread(spread_marks[2], margin_marks[2]), Some(100.0));
}

#[test]
fn test_the_narrowest_range_tie_names_the_earliest_mark() {
    // The first mark's field stands at 120, 160, and 190
    // behind the record of 100 and its range stands at 35.
    // The second mark's field stands at 120, 140, 200, and
    // 220 and its range stands at 80. The third mark's
    // field stands at 150, 170, 190, and 200 and its range
    // stands at 35 again, the tie real in the printed
    // lines. The counting names the first mark, the
    // earliest standing at the smallest range.
    let meet = load_meet(&[
        day(
            1000,
            "a",
            &format!(
                "[{}, {}, {}, {}, {}]",
                run("a", 500, &[(100, 10)]),
                run("b", 400, &[(100, 12), (300, 45)]),
                run("c", 300, &[(100, 16), (200, 28), (300, 51)]),
                run("d", 250, &[(100, 19), (200, 40), (300, 57)]),
                run("e", 200, &[(200, 44), (300, 60)])
            ),
        ),
        quiet_day(1000, "a", "a"),
        quiet_day(1000, "a", "a"),
    ]);
    let lines = meet_range(&meet).expect("the range read");
    assert_eq!(
        lines,
        vec![
            "meet range, 3 days",
            "day, the first day, marks 3",
            "mark 1, at 100 seconds on the first day, a at a projection of 100, interquartile range 35",
            "mark 2, at 200 seconds on the first day, a at a projection of 100, interquartile range 80",
            "mark 3, at 300 seconds on the first day, a at a projection of 100, interquartile range 35",
            "day, the second day, marks 0",
            "day, the third day, marks 0",
            "range counts, marks walked 3, marks with another standing 3, days 3, narrowest interquartile range 35 at 100 seconds on the first day",
        ]
    );
}

#[test]
fn test_the_range_steps_as_the_field_grows() {
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
    let lines = meet_range(&meet).expect("the range read");
    let marks = mark_lines(&lines);
    let ranges: Vec<f64> = marks
        .iter()
        .map(|line| range_of(line).expect("a range stands"))
        .collect();
    assert_eq!(ranges, vec![0.0, 60.0, 45.0, 105.0, 120.0]);
    assert_eq!(
        marks[0].as_str(),
        "mark 1, at 100 seconds on the first day, a at a projection of 100, interquartile range 0"
    );
    assert_eq!(
        marks[4].as_str(),
        "mark 5, at 500 seconds on the first day, a at a projection of 100, interquartile range 120"
    );
}

#[test]
fn test_the_quartile_margin_identity_and_the_spread_bound_hold_line_by_line() {
    // The identity and the bound of PG-44 over the hand
    // built meet of pin 1. At every mark the lower quartile
    // margin and the interquartile range sum to the upper
    // quartile margin, the range is the upper quartile
    // standing less the lower quartile standing, and the
    // range stands at or below the spread, the five ranges
    // pinned at 0, 20, 25, 45, and 40 against the five
    // spreads 0, 20, 50, 70, and 100, and at the first mark
    // the identity holds as 50 and 0 summing to 50.
    let meet = range_meet();
    let range = meet_range(&meet).expect("the range read");
    let quartiles = meet_quartiles(&meet).expect("the quartiles read");
    let spread = meet_spread(&meet).expect("the spread read");
    let margins = meet_margins(&meet).expect("the margins read");
    let range_marks = mark_lines(&range);
    let quartile_marks = mark_lines(&quartiles);
    let spread_marks = mark_lines(&spread);
    let margin_marks = mark_lines(&margins);
    assert_eq!(range_marks.len(), quartile_marks.len());
    assert_eq!(range_marks.len(), spread_marks.len());
    let expected_ranges = [0.0, 20.0, 25.0, 45.0, 40.0];
    let expected_spreads = [0.0, 20.0, 50.0, 70.0, 100.0];
    for (place, (((range_line, quartile_line), spread_line), margin_line)) in range_marks
        .iter()
        .zip(quartile_marks.iter())
        .zip(spread_marks.iter())
        .zip(margin_marks.iter())
        .enumerate()
    {
        let range = range_of(range_line).expect("a range stands");
        let parts = quartiles_mark_parts(quartile_line);
        let (lower_figure, lower_margin) = parts.lower.expect("a lower quartile stands");
        let (upper_figure, upper_margin) = parts.upper.expect("an upper quartile stands");
        let spread = field_spread(spread_line, margin_line).expect("a field spread stands");
        assert_eq!(range, expected_ranges[place], "mark {}", place + 1);
        assert_eq!(spread, expected_spreads[place], "mark {}", place + 1);
        assert!(
            (range - (upper_figure - lower_figure)).abs() <= 0.000002,
            "mark {}",
            place + 1
        );
        assert!(
            (lower_margin + range - upper_margin).abs() <= 0.000002,
            "mark {}",
            place + 1
        );
        assert!(range <= spread + 0.000002, "mark {}", place + 1);
        assert!(range >= 0.0, "mark {}", place + 1);
        if place == 0 {
            assert_eq!((lower_margin, range, upper_margin), (50.0, 0.0, 50.0));
        }
    }
}

#[test]
fn test_a_meet_in_which_no_read_projects_reads_with_marks_walked_zero() {
    let meet = load_meet(&[
        quiet_day(600, "a", "a"),
        quiet_day(600, "a", "a"),
        quiet_day(600, "a", "a"),
    ]);
    let lines = meet_range(&meet).expect("the range read");
    assert_eq!(
        lines,
        vec![
            "meet range, 3 days",
            "day, the first day, marks 0",
            "day, the second day, marks 0",
            "day, the third day, marks 0",
            "range counts, marks walked 0, marks with another standing 0, days 3, and, no other standing at any mark",
        ]
    );
}

// The guard pins. The range reading stands inside the
// fencing closure of FIX-PG-11 from its birth, and under
// the figure guard of FIX-PG-10 and the day count errors
// of PG-43, in the one walk it shares with the fifteen
// readings before it.

#[test]
fn test_a_hand_built_meet_with_no_days_draws_the_named_error_through_range() {
    let meet = Meet { days: vec![] };
    let error = meet_range(&meet).expect_err("the reading names the precondition");
    assert_eq!(error, "the meet holds no days");
}

#[test]
fn test_a_hand_built_meet_with_too_many_days_draws_the_named_error_through_range() {
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
    let error = meet_range(&meet).expect_err("the reading names the precondition");
    assert_eq!(
        error,
        "the meet holds more days than the ordinal table names"
    );
}

#[test]
fn test_a_hand_built_non_finite_figure_draws_the_named_error_through_range() {
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
    let error = meet_range(&meet).expect_err("the reading names the precondition");
    assert_eq!(error, "the window is not a finite number");
}

#[test]
fn test_a_hand_built_break_bearing_label_draws_the_named_error_through_range() {
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
    let error = meet_range(&meet).expect_err("the reading names the precondition");
    assert_eq!(error, "a runner label holds a line break");
}
