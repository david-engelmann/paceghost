//! Tests for PG-41 and PG-42, the quartiles reading over a meet.
//!
//! Written for Round 20. Each pin's failure mode stands
//! stated here pin by pin, not as one blanket claim. Pins 1
//! to 15 fail before the round by the absence of the reading,
//! the meet_quartiles function not standing on the pre round
//! tree. On the built tree, pin 1 fails under Q-1 at the
//! first mark onward, under Q-2 at its field of five alone,
//! its fields of one to four holding by design where the
//! quartiles are the tails whole, under Q-3 at the second
//! mark onward, its field of one holding by design where the
//! mean is the one figure, under Q-4, the fixture tying its
//! closest lower quartile margin at the third and fourth
//! marks so the loosened scan names the fourth, under Q-5,
//! and under Q-6 as under Q-1. Pin 2 passes under all six
//! mutations by design and claims no mutation failure, the
//! mark line prefix it compares standing rendered from the
//! walk's mark records, which no mutation of this round
//! touches. Pin 3 fails under Q-1 at every close, under Q-3
//! at its closes of three and four, its close of one holding
//! by design, under Q-5 at every close, and under Q-6 at
//! every close, and it passes under Q-2 by design at every
//! close, the closes standing at fields of four or fewer
//! where the quartiles are the tails, and under Q-4 by
//! design, no counts line standing in it. Pin 4 fails under
//! Q-1 and under Q-6 by design, the holder's own figure
//! entering the field at every mark so a quartile standing
//! stands where the pin states none, and it passes under
//! Q-2, Q-3, Q-4, and Q-5 by design. Pin 5 fails under Q-1,
//! the second mark's upper quartile joining the upper zero
//! set and the fourth mark's lower quartile joining the
//! lower, under Q-3, the first two marks' printed margins
//! becoming their mean margins and leaving the lower zero
//! set, under Q-5, and under Q-6 as under Q-1, and it passes
//! under Q-2 by design at every mark, every field counting
//! four or fewer so the quartiles are the tails and the
//! tails' zero sets at these fields are the quartiles' own,
//! and under Q-4 by design, reading margins at marks and
//! never the closest naming. Pin 6 fails under Q-1 at both
//! marks, under Q-2 at its second mark, its first holding by
//! design at its count of four, under Q-3 at its first mark,
//! under Q-5 at its second mark, and under Q-6 as under Q-1,
//! and it passes under Q-4 by design. Pin 7 fails under Q-1
//! at its first mark, its second and third marks holding at
//! their asserted upper quartiles under the enlarged fields,
//! under Q-2 at its third mark, its first two holding by
//! design at their counts of four or fewer, under Q-3 at its
//! first mark, under Q-5 at its second mark, and under Q-6
//! as under Q-1, and it passes under Q-4 by design. Pin 8
//! fails under Q-1, under Q-3, the printed margins moving so
//! both scans name the fifth mark, under Q-4, its designed
//! catcher, the fixture tying its closest lower quartile
//! margin at the second and fifth marks, under Q-5, and
//! under Q-6, and it passes under Q-2 by design, every field
//! of the fixture counting three, where the quartiles are
//! the tails whole. Pin 9 fails under Q-1 at its first step,
//! under Q-2 at its fifth step alone, its first four holding
//! by design at their counts of four or fewer, under Q-3
//! from its second step onward, its first step holding by
//! design where the mean is the one figure, under Q-5, and
//! under Q-6, and it passes under Q-4 by design, its
//! assertions standing on the mark lines. Pin 10 fails under
//! Q-1 at its field of three on its pinned values, under
//! Q-2 at the field of five's pinned values, its lower
//! quartile margin becoming the lower tail margin and its
//! upper the upper tail margin while the order clauses hold
//! by design, under Q-3 at its leaning field, under Q-5 at
//! its growth step, and under Q-6 on its pinned values as
//! under Q-1, and it passes under Q-4 by design, reading
//! mark lines alone. Pin 11 passes under all six mutations
//! by design, no mark standing for any selection, margin,
//! vector, or closing scan to move. Pins 12 to 15 pass under
//! all six mutations by design, the guards standing ahead of
//! any selection or margin computation, and they fail on a
//! built tree whose reading skipped the FIX-PG-10 figure
//! guard, the FIX-PG-11 label fencing, or the day count
//! preconditions of PG-41.

use paceghost::fieldday::{DayRun, FieldDay};
use paceghost::meet::{
    load_meet_text, meet_margins, meet_marks, meet_mean, meet_median, meet_middle, meet_quartiles,
    meet_spread, meet_standings, meet_tails, Meet,
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

struct QuartileMark {
    prefix: String,
    lower: Option<(f64, f64)>,
    upper: Option<(f64, f64)>,
}

fn quartiles_mark_parts(mark_line: &str) -> QuartileMark {
    let prefix = record_prefix(mark_line);
    if mark_line.contains(", and no other standing") {
        return QuartileMark {
            prefix,
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
        .unwrap_or_else(|| panic!("an upper quartile margin stands in {}", mark_line));
    QuartileMark {
        prefix,
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

fn median_margin_of(mark_line: &str) -> Option<f64> {
    if mark_line.contains(", and no other standing") {
        return None;
    }
    let text = mark_line
        .split(", median margin ")
        .nth(1)
        .unwrap_or_else(|| panic!("a median margin stands in {}", mark_line));
    Some(
        text.parse()
            .unwrap_or_else(|_| panic!("the median margin parses in {}", mark_line)),
    )
}

fn mean_margin_of(mark_line: &str) -> Option<f64> {
    if mark_line.contains(", and no other standing") {
        return None;
    }
    let text = mark_line
        .split(", mean margin ")
        .nth(1)
        .unwrap_or_else(|| panic!("a mean margin stands in {}", mark_line));
    Some(
        text.parse()
            .unwrap_or_else(|_| panic!("the mean margin parses in {}", mark_line)),
    )
}

fn middle_margin_of(mark_line: &str) -> Option<f64> {
    if mark_line.contains(", and no other standing") {
        return None;
    }
    let text = mark_line
        .split(", middle margin ")
        .nth(1)
        .unwrap_or_else(|| panic!("a middle margin stands in {}", mark_line));
    Some(
        text.parse()
            .unwrap_or_else(|_| panic!("the middle margin parses in {}", mark_line)),
    )
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

fn quartile_meet() -> Meet {
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
fn test_the_quartiles_reading_over_a_hand_built_meet_line_by_line() {
    let meet = quartile_meet();
    let expected: Vec<String> = [
        "meet quartiles, 3 days",
        "day, the first day, marks 2",
        "mark 1, at 100 seconds on the first day, a at a projection of 100, lower quartile standing 150, lower quartile margin 50, upper quartile standing 150, upper quartile margin 50",
        "mark 2, at 200 seconds on the first day, a at a projection of 100, lower quartile standing 150, lower quartile margin 50, upper quartile standing 170, upper quartile margin 70",
        "day, the second day, marks 0",
        "day, the third day, marks 3",
        "mark 3, at 100 seconds on the third day, a at a projection of 100, lower quartile standing 135, lower quartile margin 35, upper quartile standing 160, upper quartile margin 60",
        "mark 4, at 200 seconds on the third day, a at a projection of 100, lower quartile standing 135, lower quartile margin 35, upper quartile standing 180, upper quartile margin 80",
        "mark 5, at 300 seconds on the third day, a at a projection of 100, lower quartile standing 150, lower quartile margin 50, upper quartile standing 190, upper quartile margin 90",
        "quartiles counts, marks walked 5, marks with another standing 5, days 3, closest lower quartile margin 35 at 100 seconds on the third day, closest upper quartile margin 50 at 100 seconds on the first day",
    ]
    .iter()
    .map(|line| line.to_string())
    .collect();
    assert_eq!(meet_quartiles(&meet), Ok(expected));
}

#[test]
fn test_quartiles_mark_lines_agree_with_the_marks_reading_line_by_line() {
    let meet = quartile_meet();
    let marks = meet_marks(&meet).expect("the marks reading stands");
    let quartiles = meet_quartiles(&meet).expect("the quartiles reading stands");
    assert_eq!(
        quartiles.first().map(String::as_str),
        Some("meet quartiles, 3 days")
    );
    assert_eq!(
        marks.first().map(String::as_str),
        Some("meet marks, 3 days")
    );
    let marks_days: Vec<&String> = marks
        .iter()
        .filter(|line| line.starts_with("day,"))
        .collect();
    let quartiles_days: Vec<&String> = quartiles
        .iter()
        .filter(|line| line.starts_with("day,"))
        .collect();
    assert_eq!(marks_days, quartiles_days);
    let marks_marks = mark_lines(&marks);
    let quartiles_marks = mark_lines(&quartiles);
    assert_eq!(marks_marks.len(), quartiles_marks.len());
    for (place, (marks_line, quartiles_line)) in
        marks_marks.iter().zip(quartiles_marks.iter()).enumerate()
    {
        let parts = quartiles_mark_parts(quartiles_line);
        assert_eq!(
            parts.prefix,
            record_prefix(marks_line),
            "mark {}",
            place + 1
        );
        assert!(
            quartiles_line.starts_with(&format!("mark {}, ", place + 1)),
            "mark {} carries its place in order",
            place + 1
        );
    }
    let marks_counts = marks.last().expect("the marks counts line stands");
    let quartiles_counts = quartiles.last().expect("the quartiles counts line stands");
    assert_eq!(
        count_after(marks_counts, "marks walked "),
        count_after(quartiles_counts, "marks walked ")
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
    let quartiles = meet_quartiles(&meet).expect("the quartiles reading stands");
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
    let mut quartiles_sections: Vec<Vec<&String>> = Vec::new();
    for line in &quartiles {
        if line.starts_with("day, the ") {
            quartiles_sections.push(Vec::new());
        }
        if let Some(section) = quartiles_sections.last_mut() {
            section.push(line);
        }
    }
    assert_eq!(sections.len(), quartiles_sections.len());
    let expected_counts = [1usize, 3, 4];
    let expected_quartiles = [(150.0f64, 150.0f64), (135.0, 160.0), (135.0, 180.0)];
    for (day_index, (section, quartiles_section)) in
        sections.iter().zip(quartiles_sections.iter()).enumerate()
    {
        let day_marks: Vec<&&String> = quartiles_section
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
        let record: f64 = closing
            .rsplit("at a projection of ")
            .next()
            .and_then(|word| word.parse().ok())
            .expect("a closing line names its record figure");
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
        let median_of = |part: &[f64]| {
            if part.len() % 2 == 1 {
                part[part.len() / 2]
            } else {
                (part[part.len() / 2 - 1] + part[part.len() / 2]) / 2.0
            }
        };
        let expected_lower = median_of(&others[..count]);
        let expected_upper = median_of(&others[others.len() - count..]);
        let (named_lower, named_upper) = expected_quartiles[day_index];
        assert!(
            (expected_lower - named_lower).abs() <= 0.000002,
            "day {} closes at the named lower quartile",
            day_index + 1
        );
        assert!(
            (expected_upper - named_upper).abs() <= 0.000002,
            "day {} closes at the named upper quartile",
            day_index + 1
        );
        let last = day_marks.last().expect("the day holds a mark");
        let parts = quartiles_mark_parts(last);
        match (others.is_empty(), parts.lower, parts.upper) {
            (true, None, None) => {}
            (false, Some((lower_figure, lower_margin)), Some((upper_figure, upper_margin))) => {
                assert!(
                    (lower_figure - expected_lower).abs() <= 0.000002,
                    "day {} last mark {}",
                    day_index,
                    last
                );
                assert!(
                    (upper_figure - expected_upper).abs() <= 0.000002,
                    "day {} last mark {}",
                    day_index,
                    last
                );
                assert!(
                    (lower_figure - record - lower_margin).abs() <= 0.000002,
                    "day {} last mark {}",
                    day_index,
                    last
                );
                assert!(
                    (upper_figure - record - upper_margin).abs() <= 0.000002,
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
    let lines = meet_quartiles(&meet).expect("the quartiles read");
    assert_eq!(
        lines,
        vec![
            "meet quartiles, 3 days",
            "day, the first day, marks 1",
            "mark 1, at 100 seconds on the first day, solo at a projection of 100, and no other standing",
            "day, the second day, marks 1",
            "mark 2, at 200 seconds on the second day, solo at a projection of 150, and no other standing",
            "day, the third day, marks 0",
            "quartiles counts, marks walked 2, marks with another standing 0, days 3, and, no other standing at any mark",
        ]
    );
}

#[test]
fn test_the_quartile_margins_stand_at_zero_exactly_where_their_sub_fields_middle_places_stand_at_the_record(
) {
    // At the first mark the field behind the holder stands
    // at 100, 100, and 190, its lower quartile at the mean
    // of its two lowest figures, 100, its margin zero, both
    // middle places of the lowest two figures standing at
    // the record, while its upper quartile stands at 145.
    // At the second mark the field stands at 100, 100, 100,
    // and 190 and the lower quartile stands at zero again.
    // At the third mark the field stands at 100, 100, 150,
    // and 190 and its lower quartile stands at zero while
    // its median margin stands at 25, separating the
    // quartile's zero set from the median's in one fixture.
    // At the fourth mark the field stands at 100, 130, 150,
    // and 190 and its lower quartile stands at 112.5, outside
    // the lower zero set, while the margin of PG-30 stands
    // at zero at every mark of the meet through the
    // nearest figure. At the fifth mark every standing
    // figure of the field stands at the record and both
    // quartile margins stand at zero. The lower zero set
    // of the meet is exactly the first, second, third, and
    // fifth marks, and the upper zero set is exactly the
    // fifth.
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
    let lines = meet_quartiles(&meet).expect("the quartiles read");
    let marks = mark_lines(&lines);
    assert_eq!(marks.len(), 5);
    let lower_margins: Vec<f64> = marks
        .iter()
        .map(|line| {
            quartiles_mark_parts(line)
                .lower
                .expect("a lower quartile stands")
                .1
        })
        .collect();
    assert_eq!(lower_margins, vec![0.0, 0.0, 0.0, 12.5, 0.0]);
    let upper_margins: Vec<f64> = marks
        .iter()
        .map(|line| {
            quartiles_mark_parts(line)
                .upper
                .expect("an upper quartile stands")
                .1
        })
        .collect();
    assert_eq!(upper_margins, vec![45.0, 45.0, 70.0, 70.0, 0.0]);
    let lower_zero: Vec<usize> = marks
        .iter()
        .enumerate()
        .filter(|(_, line)| quartiles_mark_parts(line).lower.map(|(_, margin)| margin) == Some(0.0))
        .map(|(index, _)| index + 1)
        .collect();
    assert_eq!(lower_zero, vec![1, 2, 3, 5]);
    let upper_zero: Vec<usize> = marks
        .iter()
        .enumerate()
        .filter(|(_, line)| quartiles_mark_parts(line).upper.map(|(_, margin)| margin) == Some(0.0))
        .map(|(index, _)| index + 1)
        .collect();
    assert_eq!(upper_zero, vec![5]);
    let median = meet_median(&meet).expect("the median read");
    let median_marks = mark_lines(&median);
    assert_eq!(median_margin_of(median_marks[2]), Some(25.0));
    let margins = meet_margins(&meet).expect("the margins read");
    let margin_marks = mark_lines(&margins);
    assert_eq!(margin_marks.len(), 5);
    for line in &margin_marks {
        assert_eq!(margin_of(line), Some(0.0));
    }
    let counts = lines.last().expect("the quartiles close");
    assert_eq!(count_after(counts, "marks walked "), 5);
    assert_eq!(count_after(counts, "marks with another standing "), 5);
    assert_eq!(count_after(counts, "days "), 3);
}

#[test]
fn test_the_lower_quartile_separates_from_below_against_the_margin_the_lower_tail_and_the_middle() {
    // The meet stands at two marks behind a record of 100.
    // At the first mark the field stands at 110, 150, 170,
    // and 180, the lower quartile the mean of 110 and 150,
    // 130, and its margin 30, the margin of PG-30 standing
    // at 10 and the middle margin at 50, the lower quartile
    // margin above the one and below the other. At the
    // second mark the field stands at 110, 120, 150, 170,
    // and 400, the lower quartile the middle figure of its
    // three lowest, 120, and its margin 20, the lower tail
    // margin standing at 26.666667 over the mean of 110,
    // 120, and 150, and the middle margin at 50, the lower
    // quartile margin above the margin of PG-30 and below
    // the lower tail margin and the middle margin both,
    // the below direction of the no fixed order relation
    // of PG-42 in pin form.
    let meet = load_meet(&[
        day(
            1000,
            "a",
            &format!(
                "[{}, {}, {}, {}, {}, {}]",
                run("a", 500, &[(100, 10)]),
                run("b", 400, &[(100, 11)]),
                run("c", 300, &[(100, 15)]),
                run("d", 250, &[(100, 17)]),
                run("e", 200, &[(100, 18), (200, 80)]),
                run("f", 150, &[(200, 24)])
            ),
        ),
        quiet_day(1000, "a", "a"),
        quiet_day(1000, "a", "a"),
    ]);
    let quartiles = meet_quartiles(&meet).expect("the quartiles read");
    let margins = meet_margins(&meet).expect("the margins read");
    let tails = meet_tails(&meet).expect("the tails read");
    let middle = meet_middle(&meet).expect("the middle read");
    let marks = mark_lines(&quartiles);
    let margin_marks = mark_lines(&margins);
    let tails_marks = mark_lines(&tails);
    let middle_marks = mark_lines(&middle);
    assert_eq!(marks.len(), 2);
    let first = quartiles_mark_parts(marks[0]);
    assert_eq!(first.lower, Some((130.0, 30.0)));
    let margin = margin_of(margin_marks[0]).expect("a margin stands");
    assert_eq!(margin, 10.0);
    let middle_margin = middle_margin_of(middle_marks[0]).expect("a middle margin stands");
    assert_eq!(middle_margin, 50.0);
    assert!(
        30.0 > margin && 30.0 < middle_margin,
        "lower quartile 30, margin {}, middle {}",
        margin,
        middle_margin
    );
    let second = quartiles_mark_parts(marks[1]);
    assert_eq!(second.lower, Some((120.0, 20.0)));
    let (lower_tail_margin, _) = tail_margins_of(tails_marks[1]).expect("tail margins stand");
    assert!(
        (lower_tail_margin - 26.666667).abs() <= 0.000002,
        "lower tail margin {}",
        lower_tail_margin
    );
    let middle_margin = middle_margin_of(middle_marks[1]).expect("a middle margin stands");
    assert_eq!(middle_margin, 50.0);
    let margin = margin_of(margin_marks[1]).expect("a margin stands");
    assert_eq!(margin, 10.0);
    assert!(
        20.0 > margin && 20.0 < lower_tail_margin && 20.0 < middle_margin,
        "lower quartile 20, margin {}, lower tail {}, middle {}",
        margin,
        lower_tail_margin,
        middle_margin
    );
}

#[test]
fn test_the_upper_quartile_separates_from_above_against_the_spread_the_mean_and_the_upper_tail() {
    // The first meet stands at one mark whose field behind
    // the holder stands at 120, 150, 170, and 190, its
    // upper quartile the mean of 170 and 190, 180, and its
    // margin 80, the upper tail margin standing at 80 by
    // the count four coincidence and the spread at 90 and
    // the median margin at 60, the pin asserting the upper
    // quartile margin below the spread and above the
    // median margin. The second meet stands at one mark
    // whose field stands at 120, 130, and 400, its upper
    // quartile the mean of 130 and 400, 265, and its margin
    // 165, and the mean margin standing at 116.666667, the
    // pin asserting the upper quartile margin above the
    // mean margin. The third meet stands at one mark whose
    // field stands at 120, 130, 140, 380, and 400, its
    // upper quartile the middle figure of its three
    // highest, 380, and its margin 280, the upper tail
    // margin standing at 206.666667 over the mean of 140,
    // 380, and 400, the pin asserting the upper quartile
    // margin above the upper tail margin, the above
    // direction of the no fixed order relation of PG-42
    // standing in this pin where pin 6 states the below
    // direction on the lower side.
    let first = load_meet(&[
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
    ]);
    let quartiles = meet_quartiles(&first).expect("the quartiles read");
    let spread = meet_spread(&first).expect("the spread read");
    let median = meet_median(&first).expect("the median read");
    let marks = mark_lines(&quartiles);
    assert_eq!(marks.len(), 1);
    let parts = quartiles_mark_parts(marks[0]);
    assert_eq!(parts.upper, Some((180.0, 80.0)));
    let spread = spread_of(mark_lines(&spread)[0]).expect("a spread stands");
    assert_eq!(spread, 90.0);
    let median_margin = median_margin_of(mark_lines(&median)[0]).expect("a median margin stands");
    assert_eq!(median_margin, 60.0);
    assert!(
        80.0 < spread && 80.0 > median_margin,
        "upper quartile 80, spread {}, median {}",
        spread,
        median_margin
    );
    let second = load_meet(&[
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
    ]);
    let quartiles = meet_quartiles(&second).expect("the quartiles read");
    let mean = meet_mean(&second).expect("the mean read");
    let marks = mark_lines(&quartiles);
    assert_eq!(marks.len(), 1);
    let parts = quartiles_mark_parts(marks[0]);
    assert_eq!(parts.upper, Some((265.0, 165.0)));
    let mean_margin = mean_margin_of(mark_lines(&mean)[0]).expect("a mean margin stands");
    assert!(
        (mean_margin - 116.666667).abs() <= 0.000002,
        "mean margin {}",
        mean_margin
    );
    assert!(
        165.0 > mean_margin,
        "upper quartile 165, mean {}",
        mean_margin
    );
    let third = load_meet(&[
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
    ]);
    let quartiles = meet_quartiles(&third).expect("the quartiles read");
    let tails = meet_tails(&third).expect("the tails read");
    let marks = mark_lines(&quartiles);
    assert_eq!(marks.len(), 1);
    let parts = quartiles_mark_parts(marks[0]);
    assert_eq!(parts.upper, Some((380.0, 280.0)));
    let (_, upper_tail_margin) =
        tail_margins_of(mark_lines(&tails)[0]).expect("tail margins stand");
    assert!(
        (upper_tail_margin - 206.666667).abs() <= 0.000002,
        "upper tail margin {}",
        upper_tail_margin
    );
    assert!(
        280.0 > upper_tail_margin,
        "upper quartile 280, upper tail {}",
        upper_tail_margin
    );
}

#[test]
fn test_the_closest_quartile_margins_tie_names_the_earliest_mark() {
    // The second mark's field stands at 120, 150, and 400
    // behind the record of 100, its lower quartile 135 and
    // its margin 35 and its upper quartile 275 and its
    // margin 175. The fifth mark's field stands at 120,
    // 150, and 160, its lower quartile again 135 and its
    // margin again 35, the tie real in the printed lines,
    // and its upper quartile 155 and its margin 55, the
    // only mark standing there. The counting names the
    // second mark on the lower scan, the earliest at the
    // smallest lower quartile margin, and the fifth mark
    // on the upper scan. The fourth mark is the holder's
    // own re-read at its standing figure, so the field
    // stands at the third mark's field of 120, 190, and
    // 400 there and the fourth mark carries the third
    // mark's quartiles.
    let meet = load_meet(&[
        day(
            1000,
            "a",
            &format!(
                "[{}, {}, {}, {}]",
                run("a", 500, &[(100, 10), (400, 40)]),
                run("b", 400, &[(200, 24), (500, 60)]),
                run("c", 300, &[(200, 30), (300, 57), (500, 75)]),
                run("d", 250, &[(200, 80), (500, 80)])
            ),
        ),
        quiet_day(1000, "a", "a"),
        quiet_day(1000, "a", "a"),
    ]);
    let lines = meet_quartiles(&meet).expect("the quartiles read");
    assert_eq!(
        lines,
        vec![
            "meet quartiles, 3 days",
            "day, the first day, marks 5",
            "mark 1, at 100 seconds on the first day, a at a projection of 100, and no other standing",
            "mark 2, at 200 seconds on the first day, a at a projection of 100, lower quartile standing 135, lower quartile margin 35, upper quartile standing 275, upper quartile margin 175",
            "mark 3, at 300 seconds on the first day, a at a projection of 100, lower quartile standing 155, lower quartile margin 55, upper quartile standing 295, upper quartile margin 195",
            "mark 4, at 400 seconds on the first day, a at a projection of 100, lower quartile standing 155, lower quartile margin 55, upper quartile standing 295, upper quartile margin 195",
            "mark 5, at 500 seconds on the first day, a at a projection of 100, lower quartile standing 135, lower quartile margin 35, upper quartile standing 155, upper quartile margin 55",
            "day, the second day, marks 0",
            "day, the third day, marks 0",
            "quartiles counts, marks walked 5, marks with another standing 4, days 3, closest lower quartile margin 35 at 200 seconds on the first day, closest upper quartile margin 55 at 500 seconds on the first day",
        ]
    );
}

/// The lower quartile figure text a quartiles mark line
/// states, where the line carries another standing.
fn lower_figure_text(mark_line: &str) -> Option<String> {
    if mark_line.contains(", and no other standing") {
        return None;
    }
    mark_line
        .split(", lower quartile standing ")
        .nth(1)
        .and_then(|rest| rest.split(',').next())
        .map(|word| word.to_string())
}

/// The upper quartile figure text a quartiles mark line
/// states, where the line carries another standing.
fn upper_figure_text(mark_line: &str) -> Option<String> {
    if mark_line.contains(", and no other standing") {
        return None;
    }
    mark_line
        .split(", upper quartile standing ")
        .nth(1)
        .and_then(|rest| rest.split(',').next())
        .map(|word| word.to_string())
}

#[test]
fn test_the_quartiles_step_as_the_field_grows() {
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
                run("f", 550, &[(500, 195)])
            ),
        ),
        quiet_day(1000, "a", "a"),
        quiet_day(1000, "a", "a"),
    ]);
    let lines = meet_quartiles(&meet).expect("the quartiles read");
    let marks = mark_lines(&lines);
    let lowers: Vec<String> = marks
        .iter()
        .filter_map(|line| lower_figure_text(line))
        .collect();
    assert_eq!(lowers, vec!["150", "150", "180", "180", "210"]);
    let uppers: Vec<String> = marks
        .iter()
        .filter_map(|line| upper_figure_text(line))
        .collect();
    assert_eq!(uppers, vec!["150", "210", "225", "285", "330"]);
    assert_eq!(
        marks[0].as_str(),
        "mark 1, at 100 seconds on the first day, a at a projection of 100, lower quartile standing 150, lower quartile margin 50, upper quartile standing 150, upper quartile margin 50"
    );
    assert_eq!(
        marks[4].as_str(),
        "mark 5, at 500 seconds on the first day, a at a projection of 100, lower quartile standing 210, lower quartile margin 110, upper quartile standing 330, upper quartile margin 230"
    );
}

#[test]
fn test_the_bracketing_chain_holds_line_by_line() {
    // The chain of PG-42 at every figured mark: the margin
    // to the nearest standing stands at or below the lower
    // quartile margin, which stands at or below the middle
    // margin, which stands at or below the median margin,
    // which stands at or below the upper quartile margin,
    // which stands at or below the spread. At the third
    // mark the field stands at 120, 150, and 170 behind
    // the record of 100 and the chain stands at its named
    // values there, the margin at 20, the lower quartile
    // margin at 35, the middle margin and the median
    // margin both at 50, the upper quartile margin at 60,
    // and the spread at 70. At the fifth mark the field
    // stands at 120, 380, 390, and 400, its mean below its
    // middle figure, and the chain stands at 20, 150, 280,
    // 285, 295, and 300 in chain order. A second meet
    // stands at one mark whose field stands at 120, 150,
    // 170, 190, and 220, the field at which the quartiles
    // first stand apart from the tails, its quartile
    // margins pinned as values at 50 and 90.
    let meet = load_meet(&[
        day(
            1000,
            "a",
            &format!(
                "[{}, {}, {}, {}, {}]",
                run("a", 500, &[(100, 10)]),
                run("b", 400, &[(100, 12), (200, 30), (300, 36)]),
                run("c", 300, &[(200, 34), (400, 60), (500, 190)]),
                run("d", 250, &[(200, 38), (300, 45), (400, 68), (500, 195)]),
                run("e", 200, &[(400, 76), (500, 200)])
            ),
        ),
        quiet_day(1000, "a", "a"),
        quiet_day(1000, "a", "a"),
    ]);
    let quartiles = meet_quartiles(&meet).expect("the quartiles read");
    let margins = meet_margins(&meet).expect("the margins read");
    let middle = meet_middle(&meet).expect("the middle read");
    let median = meet_median(&meet).expect("the median read");
    let spread = meet_spread(&meet).expect("the spread read");
    let quartiles_marks = mark_lines(&quartiles);
    let margin_marks = mark_lines(&margins);
    let middle_marks = mark_lines(&middle);
    let median_marks = mark_lines(&median);
    let spread_marks = mark_lines(&spread);
    assert_eq!(quartiles_marks.len(), margin_marks.len());
    assert_eq!(quartiles_marks.len(), middle_marks.len());
    assert_eq!(quartiles_marks.len(), median_marks.len());
    assert_eq!(quartiles_marks.len(), spread_marks.len());
    let expected = [
        (20.0, 20.0, 20.0, 20.0, 20.0, 20.0),
        (50.0, 60.0, 70.0, 70.0, 80.0, 90.0),
        (20.0, 35.0, 50.0, 50.0, 60.0, 70.0),
        (20.0, 35.0, 50.0, 60.0, 80.0, 90.0),
        (20.0, 150.0, 280.0, 285.0, 295.0, 300.0),
    ];
    for (((((quartile_line, margin_line), middle_line), median_line), spread_line), place) in
        quartiles_marks
            .iter()
            .zip(margin_marks.iter())
            .zip(middle_marks.iter())
            .zip(median_marks.iter())
            .zip(spread_marks.iter())
            .zip(1..)
    {
        let parts = quartiles_mark_parts(quartile_line);
        let Some((lower_margin, upper_margin)) = (match (parts.lower, parts.upper) {
            (Some((_, lower_margin)), Some((_, upper_margin))) => {
                Some((lower_margin, upper_margin))
            }
            _ => None,
        }) else {
            assert!(margin_of(margin_line).is_none(), "mark {}", place);
            assert!(middle_margin_of(middle_line).is_none(), "mark {}", place);
            assert!(median_margin_of(median_line).is_none(), "mark {}", place);
            assert!(spread_of(spread_line).is_none(), "mark {}", place);
            continue;
        };
        let margin = margin_of(margin_line).expect("a margin stands");
        let middle_margin = middle_margin_of(middle_line).expect("a middle margin stands");
        let median_margin = median_margin_of(median_line).expect("a median margin stands");
        let spread = spread_of(spread_line).expect("a spread stands");
        assert!(margin <= lower_margin + 0.000002, "mark {}", place);
        assert!(lower_margin <= middle_margin + 0.000002, "mark {}", place);
        assert!(middle_margin <= median_margin + 0.000002, "mark {}", place);
        assert!(median_margin <= upper_margin + 0.000002, "mark {}", place);
        assert!(upper_margin <= spread + 0.000002, "mark {}", place);
        assert_eq!(
            (
                margin,
                lower_margin,
                middle_margin,
                median_margin,
                upper_margin,
                spread
            ),
            expected[place - 1],
            "mark {}",
            place
        );
    }
    let wide = load_meet(&[
        day(
            1000,
            "a",
            &format!(
                "[{}, {}, {}, {}, {}, {}]",
                run("a", 500, &[(100, 10)]),
                run("b", 400, &[(100, 12)]),
                run("c", 300, &[(100, 15)]),
                run("d", 250, &[(100, 17)]),
                run("e", 200, &[(100, 19)]),
                run("f", 150, &[(100, 22)])
            ),
        ),
        quiet_day(1000, "a", "a"),
        quiet_day(1000, "a", "a"),
    ]);
    let quartiles = meet_quartiles(&wide).expect("the quartiles read");
    let margins = meet_margins(&wide).expect("the margins read");
    let middle = meet_middle(&wide).expect("the middle read");
    let median = meet_median(&wide).expect("the median read");
    let spread = meet_spread(&wide).expect("the spread read");
    let quartile_line = mark_lines(&quartiles)[0];
    let parts = quartiles_mark_parts(quartile_line);
    assert_eq!(parts.lower, Some((150.0, 50.0)));
    assert_eq!(parts.upper, Some((190.0, 90.0)));
    let margin = margin_of(mark_lines(&margins)[0]).expect("a margin stands");
    let middle_margin = middle_margin_of(mark_lines(&middle)[0]).expect("a middle margin stands");
    let median_margin = median_margin_of(mark_lines(&median)[0]).expect("a median margin stands");
    let spread = spread_of(mark_lines(&spread)[0]).expect("a spread stands");
    assert_eq!(
        (margin, middle_margin, median_margin, spread),
        (20.0, 70.0, 70.0, 120.0)
    );
    assert!(margin <= 50.0 && 50.0 <= middle_margin && middle_margin <= median_margin);
    assert!(median_margin <= 90.0 && 90.0 <= spread);
}

#[test]
fn test_a_meet_in_which_no_read_projects_reads_with_marks_walked_zero() {
    let meet = load_meet(&[
        quiet_day(600, "a", "a"),
        quiet_day(600, "a", "a"),
        quiet_day(600, "a", "a"),
    ]);
    let lines = meet_quartiles(&meet).expect("the quartiles read");
    assert_eq!(
        lines,
        vec![
            "meet quartiles, 3 days",
            "day, the first day, marks 0",
            "day, the second day, marks 0",
            "day, the third day, marks 0",
            "quartiles counts, marks walked 0, marks with another standing 0, days 3, and, no other standing at any mark",
        ]
    );
}

// The guard pins. The quartiles reading stands inside the
// fencing closure of FIX-PG-11 from its birth, and under
// the figure guard of FIX-PG-10 and the day count errors
// of PG-41, in the one walk it shares with the fourteen
// readings before it.

#[test]
fn test_a_hand_built_meet_with_no_days_draws_the_named_error_through_quartiles() {
    let meet = Meet { days: vec![] };
    let error = meet_quartiles(&meet).expect_err("the reading names the precondition");
    assert_eq!(error, "the meet holds no days");
}

#[test]
fn test_a_hand_built_meet_with_too_many_days_draws_the_named_error_through_quartiles() {
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
    let error = meet_quartiles(&meet).expect_err("the reading names the precondition");
    assert_eq!(
        error,
        "the meet holds more days than the ordinal table names"
    );
}

#[test]
fn test_a_hand_built_non_finite_figure_draws_the_named_error_through_quartiles() {
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
    let error = meet_quartiles(&meet).expect_err("the reading names the precondition");
    assert_eq!(error, "the window is not a finite number");
}

#[test]
fn test_a_hand_built_break_bearing_label_draws_the_named_error_through_quartiles() {
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
    let error = meet_quartiles(&meet).expect_err("the reading names the precondition");
    assert_eq!(error, "a runner label holds a line break");
}
