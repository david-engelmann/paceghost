//! Tests for PG-39 and PG-40, the median reading over a meet.
//!
//! Written for Round 19. Each pin's failure mode stands
//! stated here pin by pin, not as one blanket claim. Pins 1
//! to 15 fail before the round by the absence of the reading,
//! the meet_median function not standing on the pre round
//! tree. On the built tree, pin 1 fails under P-1, under
//! P-2 at its fields of two and four, its fields of one and
//! three holding by design where the median stands at the
//! middle figure, under P-3 at its fields of three and four,
//! its fields of one and two holding by design where the
//! mean stands at the median, under P-4, the fixture tying
//! its closest margin at the first and third marks so the
//! loosened scan names the third, under P-5, and under P-6.
//! Pin 2 passes under all six mutations by design and claims
//! no mutation failure, the mark line prefix it compares
//! standing rendered from the walk's mark records, which no
//! mutation of this round touches, the lag of P-5 living
//! only in the median figures behind the prefix. Pin 3
//! fails under P-1 at every close, under P-2 at its close
//! of four, its closes of one and three holding by design,
//! under P-3 at its closes of three and four, its close of
//! one holding by design, under P-5 at every close, and
//! under P-6 at every close, and it passes under P-4 by
//! design, no counts line standing in it. Pin 4 fails under
//! P-1 and under P-6 by design, the holder's own figure
//! entering the field at every mark so a median standing
//! stands where the pin states none, and it passes under
//! P-2, P-3, P-4, and P-5 by design. Pin 5 fails under P-1,
//! the third mark's field gaining the record so its median
//! joins the zero set, under P-2 the same way through the
//! middle figure, under P-3, the first two marks' printed
//! margins becoming their mean margins and leaving the zero
//! set, under P-5, and under P-6 as under P-1, and it
//! passes under P-4 by design, reading margins at marks and
//! never the closest naming. Pin 6 fails under P-1 at both
//! marks, under P-2 at its first mark, the second holding
//! by design at its odd count, under P-3 at its second
//! mark's equality clause, its first mark passing by
//! design, under P-5 at its second mark, and under P-6 at
//! its first mark, and it passes under P-4 by design.
//! Pin 7 fails under P-1 at its third mark, its first two
//! marks holding by design at their moved figures, under
//! P-3 at its second and third marks, under P-5 at its
//! third mark, and under P-6 at its third mark, and it
//! passes under P-2 and P-4 by design. Pin 8 fails under
//! P-1, under P-3, the printed margins moving so the
//! smallest stands at the fifth mark, under P-4, its
//! designed catcher, the fixture tying its closest margin
//! at the second and fifth marks, under P-5, and under
//! P-6, and it passes under P-2 by design, every field of
//! the fixture standing at an odd count. Pin 9 fails under
//! P-1, under P-2 at its fields of two and four, under P-3
//! at its fields of three and four, under P-5, and under
//! P-6, and it passes under P-4 by design. Pin 10 fails
//! under P-1 at its field of three, under P-3 at its
//! leaning mark, under P-5 at its growth step, and under
//! P-6 at the value clauses of its field of three, and it
//! passes under P-2 by design, the substituted figure being
//! itself a member of the chain the pin asserts, and under
//! P-4 by design, reading mark lines alone. Pin 11 passes
//! under all six mutations by design, no mark standing for
//! any selection, margin, vector, or closing scan to move.
//! Pins 12 to 15 pass under all six mutations by design,
//! the guards standing ahead of any selection or margin
//! computation, and they fail on a built tree whose reading
//! skipped the FIX-PG-10 figure guard, the FIX-PG-11 label
//! fencing, or the day count preconditions of PG-39.

use paceghost::fieldday::{DayRun, FieldDay};
use paceghost::meet::{
    load_meet_text, meet_margins, meet_marks, meet_mean, meet_median, meet_middle, meet_spread,
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

struct MedianMark {
    prefix: String,
    figure: Option<f64>,
    margin: Option<f64>,
}

fn median_mark_parts(mark_line: &str) -> MedianMark {
    let prefix = record_prefix(mark_line);
    if mark_line.contains(", and no other standing") {
        return MedianMark {
            prefix,
            figure: None,
            margin: None,
        };
    }
    let figure_text = mark_line
        .split(", median standing ")
        .nth(1)
        .and_then(|rest| rest.split(',').next())
        .unwrap_or_else(|| panic!("a median standing stands in {}", mark_line));
    let margin_text = mark_line
        .split(", median margin ")
        .nth(1)
        .unwrap_or_else(|| panic!("a median margin stands in {}", mark_line));
    MedianMark {
        prefix,
        figure: Some(
            figure_text
                .parse()
                .unwrap_or_else(|_| panic!("the median standing parses in {}", mark_line)),
        ),
        margin: Some(
            margin_text
                .parse()
                .unwrap_or_else(|_| panic!("the median margin parses in {}", mark_line)),
        ),
    }
}

fn median_margin_of(mark_line: &str) -> Option<f64> {
    median_mark_parts(mark_line).margin
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

fn median_meet() -> Meet {
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
                "[{}, {}, {}, {}, {}]",
                run("a", 500, &[]),
                run("b", 400, &[]),
                run("c", 300, &[]),
                run("d", 250, &[(100, 12)]),
                run("e", 200, &[(200, 38)])
            ),
        ),
    ])
}

#[test]
fn test_the_median_reading_over_a_hand_built_meet_line_by_line() {
    let meet = median_meet();
    let expected: Vec<String> = [
        "meet median, 3 days",
        "day, the first day, marks 2",
        "mark 1, at 100 seconds on the first day, a at a projection of 100, median standing 150, median margin 50",
        "mark 2, at 200 seconds on the first day, a at a projection of 100, median standing 160, median margin 60",
        "day, the second day, marks 0",
        "day, the third day, marks 2",
        "mark 3, at 100 seconds on the third day, a at a projection of 100, median standing 150, median margin 50",
        "mark 4, at 200 seconds on the third day, a at a projection of 100, median standing 160, median margin 60",
        "median counts, marks walked 4, marks with another standing 4, days 3, closest median margin 50 at 100 seconds on the first day",
    ]
    .iter()
    .map(|line| line.to_string())
    .collect();
    assert_eq!(meet_median(&meet), Ok(expected));
}

#[test]
fn test_median_mark_lines_agree_with_the_marks_reading_line_by_line() {
    let meet = median_meet();
    let marks = meet_marks(&meet).expect("the marks reading stands");
    let median = meet_median(&meet).expect("the median reading stands");
    assert_eq!(
        median.first().map(String::as_str),
        Some("meet median, 3 days")
    );
    assert_eq!(
        marks.first().map(String::as_str),
        Some("meet marks, 3 days")
    );
    let marks_days: Vec<&String> = marks
        .iter()
        .filter(|line| line.starts_with("day,"))
        .collect();
    let median_days: Vec<&String> = median
        .iter()
        .filter(|line| line.starts_with("day,"))
        .collect();
    assert_eq!(marks_days, median_days);
    let marks_marks = mark_lines(&marks);
    let median_marks = mark_lines(&median);
    assert_eq!(marks_marks.len(), median_marks.len());
    for (place, (marks_line, median_line)) in
        marks_marks.iter().zip(median_marks.iter()).enumerate()
    {
        let parts = median_mark_parts(median_line);
        assert_eq!(
            parts.prefix,
            record_prefix(marks_line),
            "mark {}",
            place + 1
        );
        assert!(
            median_line.starts_with(&format!("mark {}, ", place + 1)),
            "mark {} carries its place in order",
            place + 1
        );
    }
    let marks_counts = marks.last().expect("the marks counts line stands");
    let median_counts = median.last().expect("the median counts line stands");
    assert_eq!(
        count_after(marks_counts, "marks walked "),
        count_after(median_counts, "marks walked ")
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
    let median = meet_median(&meet).expect("the median reading stands");
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
    let mut median_sections: Vec<Vec<&String>> = Vec::new();
    for line in &median {
        if line.starts_with("day, the ") {
            median_sections.push(Vec::new());
        }
        if let Some(section) = median_sections.last_mut() {
            section.push(line);
        }
    }
    assert_eq!(sections.len(), median_sections.len());
    let expected_counts = [1usize, 3, 4];
    let expected_medians = [150.0f64, 150.0, 160.0];
    for (day_index, (section, median_section)) in
        sections.iter().zip(median_sections.iter()).enumerate()
    {
        let day_marks: Vec<&&String> = median_section
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
        let expected = if others.len() % 2 == 1 {
            others[others.len() / 2]
        } else {
            (others[others.len() / 2 - 1] + others[others.len() / 2]) / 2.0
        };
        assert!(
            (expected - expected_medians[day_index]).abs() <= 0.000002,
            "day {} closes at the named median",
            day_index + 1
        );
        let last = day_marks.last().expect("the day holds a mark");
        let parts = median_mark_parts(last);
        match (others.is_empty(), parts.figure) {
            (true, None) => {}
            (false, Some(figure)) => {
                let stated_margin = parts.margin.expect("a figured mark states its margin");
                assert!(
                    (figure - expected).abs() <= 0.000002,
                    "day {} last mark {}",
                    day_index,
                    last
                );
                assert!(
                    (figure - record - stated_margin).abs() <= 0.000002,
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
    let lines = meet_median(&meet).expect("the median read");
    assert_eq!(
        lines,
        vec![
            "meet median, 3 days",
            "day, the first day, marks 1",
            "mark 1, at 100 seconds on the first day, solo at a projection of 100, and no other standing",
            "day, the second day, marks 1",
            "mark 2, at 200 seconds on the second day, solo at a projection of 150, and no other standing",
            "day, the third day, marks 0",
            "median counts, marks walked 2, marks with another standing 0, days 3, and, no other standing at any mark",
        ]
    );
}

#[test]
fn test_the_median_margin_stands_at_zero_exactly_where_the_middle_places_stand_at_the_record() {
    // At the first mark the field behind the holder stands
    // at 100, 100, and 190, its middle place at the record
    // figure itself, and at the second mark the field stands
    // at 100, 100, 100, and 190, its two middle places both
    // at the record figure, so the median margin stands at
    // zero at exactly those two marks. At the third mark the
    // field stands at 100, 100, 150, and 190 and its median
    // margin stands at 25, and at the fourth mark the field
    // stands at 100, 130, 150, and 190 and its median margin
    // stands at 40. The pin reads the margins at the marks
    // and the counts, and never the closest naming.
    let meet = load_meet(&[
        day(
            1000,
            "a",
            &format!(
                "[{}, {}, {}, {}, {}]",
                run("a", 500, &[(100, 10)]),
                run("b", 400, &[(100, 10), (400, 52)]),
                run("c", 300, &[(100, 10)]),
                run("d", 250, &[(100, 19)]),
                run("e", 200, &[(200, 20), (300, 45)])
            ),
        ),
        quiet_day(1000, "a", "a"),
        quiet_day(1000, "a", "a"),
    ]);
    let lines = meet_median(&meet).expect("the median read");
    let marks = mark_lines(&lines);
    assert_eq!(marks.len(), 4);
    let zero_marks: Vec<usize> = marks
        .iter()
        .enumerate()
        .filter(|(_, line)| median_margin_of(line) == Some(0.0))
        .map(|(index, _)| index + 1)
        .collect();
    assert_eq!(zero_marks, vec![1, 2]);
    assert_eq!(median_margin_of(marks[2]), Some(25.0));
    assert_eq!(median_margin_of(marks[3]), Some(40.0));
    let counts = lines.last().expect("the median close");
    assert_eq!(count_after(counts, "marks walked "), 4);
    assert_eq!(count_after(counts, "marks with another standing "), 4);
    assert_eq!(count_after(counts, "days "), 3);
}

#[test]
fn test_the_median_separates_from_below_against_the_lower_tail_and_the_middle() {
    // The first meet stands at one mark whose field behind
    // the holder stands at 110, 150, 170, and 180, its
    // median margin at 60, its lower tail margin at 30, and
    // its middle margin at 50, the median standing strictly
    // above both. The second meet stands at one mark whose
    // field stands at 110, 150, and 400, its median margin
    // at 50, its lower tail margin at 30, and its middle
    // margin at 50, the median standing above the lower
    // tail and level with the middle at the odd count.
    let first = load_meet(&[
        day(
            1000,
            "a",
            &format!(
                "[{}, {}, {}, {}, {}]",
                run("a", 500, &[(100, 10)]),
                run("b", 400, &[(100, 11)]),
                run("c", 300, &[(100, 15)]),
                run("d", 250, &[(100, 17)]),
                run("e", 200, &[(100, 18)])
            ),
        ),
        quiet_day(1000, "a", "a"),
        quiet_day(1000, "a", "a"),
    ]);
    let second = load_meet(&[
        day(
            1000,
            "a",
            &format!(
                "[{}, {}, {}, {}]",
                run("a", 500, &[(100, 10)]),
                run("b", 400, &[(100, 11)]),
                run("c", 300, &[(100, 15)]),
                run("d", 250, &[(100, 40)])
            ),
        ),
        quiet_day(1000, "a", "a"),
        quiet_day(1000, "a", "a"),
    ]);
    for (meet, strict) in [(&first, true), (&second, false)] {
        let median = meet_median(meet).expect("the median read");
        let middle = meet_middle(meet).expect("the middle read");
        let tails = meet_tails(meet).expect("the tails read");
        let median_marks = mark_lines(&median);
        let middle_marks = mark_lines(&middle);
        let tails_marks = mark_lines(&tails);
        assert_eq!(median_marks.len(), 1);
        let median_margin = median_margin_of(median_marks[0]).expect("a median margin stands");
        let middle_margin = middle_margin_of(middle_marks[0]).expect("a middle margin stands");
        let (lower_margin, _) = tail_margins_of(tails_marks[0]).expect("tail margins stand");
        assert!(
            median_margin > lower_margin,
            "median {} lower {}",
            median_margin,
            lower_margin
        );
        if strict {
            assert!(
                median_margin > middle_margin,
                "median {} middle {}",
                median_margin,
                middle_margin
            );
        } else {
            assert_eq!(median_margin, middle_margin);
        }
    }
}

#[test]
fn test_the_median_separates_from_above_against_the_upper_tail_the_spread_and_the_mean() {
    // The first meet stands at one mark whose field behind
    // the holder stands at 120, 150, 170, and 190, its
    // median margin at 60, standing below its upper tail
    // margin at 80 and below its spread at 90. The second
    // meet stands at one mark whose field stands at 120,
    // 130, and 400, its median margin at 30 standing below
    // its mean margin. The third meet stands at one mark
    // whose field stands at 120, 380, and 400, its median
    // margin at 280 standing above its mean margin at 200.
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
    let median = meet_median(&first).expect("the median read");
    let tails = meet_tails(&first).expect("the tails read");
    let spread = meet_spread(&first).expect("the spread read");
    let median_marks = mark_lines(&median);
    let tails_marks = mark_lines(&tails);
    let spread_marks = mark_lines(&spread);
    assert_eq!(median_marks.len(), 1);
    let median_margin = median_margin_of(median_marks[0]).expect("a median margin stands");
    let (_, upper_margin) = tail_margins_of(tails_marks[0]).expect("tail margins stand");
    let spread = spread_of(spread_marks[0]).expect("a spread stands");
    assert_eq!(median_margin, 60.0);
    assert_eq!(upper_margin, 80.0);
    assert_eq!(spread, 90.0);
    assert!(median_margin < upper_margin);
    assert!(median_margin < spread);
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
    let third = load_meet(&[
        day(
            1000,
            "a",
            &format!(
                "[{}, {}, {}, {}]",
                run("a", 500, &[(100, 10)]),
                run("b", 400, &[(100, 12)]),
                run("c", 300, &[(100, 38)]),
                run("d", 250, &[(100, 40)])
            ),
        ),
        quiet_day(1000, "a", "a"),
        quiet_day(1000, "a", "a"),
    ]);
    for (meet, below) in [(&second, true), (&third, false)] {
        let median = meet_median(meet).expect("the median read");
        let mean = meet_mean(meet).expect("the mean read");
        let median_marks = mark_lines(&median);
        let mean_marks = mark_lines(&mean);
        assert_eq!(median_marks.len(), 1);
        let median_margin = median_margin_of(median_marks[0]).expect("a median margin stands");
        let mean_margin = mean_margin_of(mean_marks[0]).expect("a mean margin stands");
        if below {
            assert_eq!(median_margin, 30.0);
            assert!(
                median_margin < mean_margin,
                "median {} mean {}",
                median_margin,
                mean_margin
            );
        } else {
            assert_eq!(median_margin, 280.0);
            assert_eq!(mean_margin, 200.0);
            assert!(
                median_margin > mean_margin,
                "median {} mean {}",
                median_margin,
                mean_margin
            );
        }
    }
}

#[test]
fn test_the_closest_median_margin_tie_names_the_earliest_mark() {
    // The second mark's field stands at 120, 150, and 400
    // behind the record of 100, its median 150 and its
    // median margin 50. The fifth mark's field stands at
    // 130, 150, and 160 behind the same record, its median
    // again 150 and its median margin again 50, the tie
    // real in the printed lines. The counting names the
    // second mark, the earliest at the smallest median
    // margin. The fourth mark is the holder's own re-read
    // at its standing figure, so the field stands at the
    // third mark's field of 120, 190, and 400 there and
    // the fourth mark carries the third mark's median and
    // median margin, a standing projection once set never
    // standing cleared for a later mark to stand unfigured
    // behind figured marks.
    let meet = load_meet(&[
        day(
            1000,
            "a",
            &format!(
                "[{}, {}, {}, {}]",
                run("a", 500, &[(100, 10), (400, 40)]),
                run("b", 400, &[(200, 24), (500, 65)]),
                run("c", 300, &[(200, 30), (300, 57), (500, 75)]),
                run("d", 250, &[(200, 80), (500, 80)])
            ),
        ),
        quiet_day(1000, "a", "a"),
        quiet_day(1000, "a", "a"),
    ]);
    let lines = meet_median(&meet).expect("the median read");
    assert_eq!(
        lines,
        vec![
            "meet median, 3 days",
            "day, the first day, marks 5",
            "mark 1, at 100 seconds on the first day, a at a projection of 100, and no other standing",
            "mark 2, at 200 seconds on the first day, a at a projection of 100, median standing 150, median margin 50",
            "mark 3, at 300 seconds on the first day, a at a projection of 100, median standing 190, median margin 90",
            "mark 4, at 400 seconds on the first day, a at a projection of 100, median standing 190, median margin 90",
            "mark 5, at 500 seconds on the first day, a at a projection of 100, median standing 150, median margin 50",
            "day, the second day, marks 0",
            "day, the third day, marks 0",
            "median counts, marks walked 5, marks with another standing 4, days 3, closest median margin 50 at 200 seconds on the first day",
        ]
    );
}

/// The median figure text a median mark line states, where
/// the line carries another standing.
fn median_figure_text(mark_line: &str) -> Option<String> {
    if mark_line.contains(", and no other standing") {
        return None;
    }
    mark_line
        .split(", median standing ")
        .nth(1)
        .and_then(|rest| rest.split(',').next())
        .map(|word| word.to_string())
}

#[test]
fn test_the_median_steps_as_the_field_grows() {
    let meet = load_meet(&[
        day(
            1000,
            "a",
            &format!(
                "[{}, {}, {}, {}, {}]",
                run("a", 500, &[(100, 10)]),
                run("b", 400, &[(100, 15)]),
                run("c", 300, &[(200, 42)]),
                run("d", 250, &[(300, 72)]),
                run("e", 600, &[(400, 132)])
            ),
        ),
        quiet_day(1000, "a", "a"),
        quiet_day(1000, "a", "a"),
    ]);
    let lines = meet_median(&meet).expect("the median read");
    let marks = mark_lines(&lines);
    let figures: Vec<String> = marks
        .iter()
        .filter_map(|line| median_figure_text(line))
        .collect();
    assert_eq!(figures, vec!["150", "180", "210", "225"]);
    assert_eq!(
        marks[0].as_str(),
        "mark 1, at 100 seconds on the first day, a at a projection of 100, median standing 150, median margin 50"
    );
    assert_eq!(
        marks[3].as_str(),
        "mark 4, at 400 seconds on the first day, a at a projection of 100, median standing 225, median margin 125"
    );
}

#[test]
fn test_the_bracketing_chain_holds_line_by_line() {
    // The chain of PG-40 at every figured mark: the margin
    // to the nearest standing stands at or below the lower
    // tail margin, which stands at or below the median
    // margin, which stands at or below the upper tail
    // margin, which stands at or below the spread, and the
    // middle margin stands at or below the median margin.
    // At the third mark the field stands at 120, 150, and
    // 170 behind the record of 100 and the chain stands at
    // its named values there, the margin at 20, the lower
    // tail margin at 35, the median margin and the middle
    // margin both at 50, the upper tail margin at 60, and
    // the spread at 70.
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
    let median = meet_median(&meet).expect("the median read");
    let margins = meet_margins(&meet).expect("the margins read");
    let tails = meet_tails(&meet).expect("the tails read");
    let spread = meet_spread(&meet).expect("the spread read");
    let middle = meet_middle(&meet).expect("the middle read");
    let median_marks = mark_lines(&median);
    let margin_marks = mark_lines(&margins);
    let tails_marks = mark_lines(&tails);
    let spread_marks = mark_lines(&spread);
    let middle_marks = mark_lines(&middle);
    assert_eq!(median_marks.len(), margin_marks.len());
    assert_eq!(median_marks.len(), tails_marks.len());
    assert_eq!(median_marks.len(), spread_marks.len());
    assert_eq!(median_marks.len(), middle_marks.len());
    for (((((median_line, margin_line), tails_line), spread_line), middle_line), place) in
        median_marks
            .iter()
            .zip(margin_marks.iter())
            .zip(tails_marks.iter())
            .zip(spread_marks.iter())
            .zip(middle_marks.iter())
            .zip(1..)
    {
        let Some(median_margin) = median_margin_of(median_line) else {
            assert!(margin_of(margin_line).is_none(), "mark {}", place);
            assert!(tail_margins_of(tails_line).is_none(), "mark {}", place);
            assert!(spread_of(spread_line).is_none(), "mark {}", place);
            assert!(middle_margin_of(middle_line).is_none(), "mark {}", place);
            continue;
        };
        let margin = margin_of(margin_line).expect("a margin stands");
        let (lower_margin, upper_margin) = tail_margins_of(tails_line).expect("tail margins stand");
        let spread = spread_of(spread_line).expect("a spread stands");
        let middle_margin = middle_margin_of(middle_line).expect("a middle margin stands");
        assert!(margin <= lower_margin + 0.000002, "mark {}", place);
        assert!(lower_margin <= median_margin + 0.000002, "mark {}", place);
        assert!(median_margin <= upper_margin + 0.000002, "mark {}", place);
        assert!(upper_margin <= spread + 0.000002, "mark {}", place);
        assert!(middle_margin <= median_margin + 0.000002, "mark {}", place);
        if place == 3 {
            assert_eq!(
                (margin, lower_margin, median_margin, upper_margin, spread),
                (20.0, 35.0, 50.0, 60.0, 70.0),
                "mark {}",
                place
            );
            assert_eq!(middle_margin, 50.0, "mark {}", place);
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
    let lines = meet_median(&meet).expect("the median read");
    assert_eq!(
        lines,
        vec![
            "meet median, 3 days",
            "day, the first day, marks 0",
            "day, the second day, marks 0",
            "day, the third day, marks 0",
            "median counts, marks walked 0, marks with another standing 0, days 3, and, no other standing at any mark",
        ]
    );
}

// The guard pins. The median reading stands inside the
// fencing closure of FIX-PG-11 from its birth, and under
// the figure guard of FIX-PG-10 and the day count errors
// of PG-39, in the one walk it shares with the thirteen
// readings before it.

#[test]
fn test_a_hand_built_meet_with_no_days_draws_the_named_error_through_median() {
    let meet = Meet { days: vec![] };
    let error = meet_median(&meet).expect_err("the reading names the precondition");
    assert_eq!(error, "the meet holds no days");
}

#[test]
fn test_a_hand_built_meet_with_too_many_days_draws_the_named_error_through_median() {
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
    let error = meet_median(&meet).expect_err("the reading names the precondition");
    assert_eq!(
        error,
        "the meet holds more days than the ordinal table names"
    );
}

#[test]
fn test_a_hand_built_non_finite_figure_draws_the_named_error_through_median() {
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
    let error = meet_median(&meet).expect_err("the reading names the precondition");
    assert_eq!(error, "the window is not a finite number");
}

#[test]
fn test_a_hand_built_break_bearing_label_draws_the_named_error_through_median() {
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
    let error = meet_median(&meet).expect_err("the reading names the precondition");
    assert_eq!(error, "a runner label holds a line break");
}
