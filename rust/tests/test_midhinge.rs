//! Tests for PG-47 and PG-48, the midhinge reading over a meet.
//!
//! Written for Round 23. Each pin's failure mode stands
//! stated here pin by pin, not as one blanket claim. Pins 1
//! to 16 fail before the round by the absence of the reading,
//! the meet_midhinge function not standing on the pre round
//! tree. On the built tree, pin 1 fails under S-1 from the
//! first mark onward, the midhinges becoming 125, 142.5, 135,
//! 145, and 155 over the enlarged fields, and under S-6 as
//! under S-1, under S-2 at its third and fourth marks, the
//! medians standing at 150 and 160 against the midhinges at
//! 147.5 and 157.5, under S-3 at its third mark alone, the
//! mean standing at 146.666667 against 147.5, and under S-5
//! from the first mark onward, and it passes under S-4 by
//! design, its closest margin standing unique. Pin 2 passes
//! under all six mutations by design and claims no mutation
//! failure, the mark line prefix it compares standing rendered
//! from the walk's mark records, which no mutation of this
//! round touches. Pin 3 fails under S-1 at both closes, the
//! closes becoming 142.5 and 155, under S-5 at both closes,
//! and under S-6 at both closes, and it passes under S-2 and
//! S-3 by design at both closes, the median and the mean
//! standing at the midhinge at each close's field, and under
//! S-4 by design, no counts line standing in it. Pin 4 fails
//! under S-1 and under S-6 by design, the holder's own figure
//! entering the field at every mark so a midhinge stands
//! where the pin states none, and it passes under S-2, S-3,
//! S-4, and S-5 by design. Pin 5 fails under S-1 at its
//! second, third, and fourth marks on value, the midhinges
//! becoming 100, 125, and 125, and under S-6 as under S-1,
//! under S-2 at its first four marks on value, the median
//! margins standing at 0, 0, 25, and 37.5, under S-3 at its
//! first mark and at its edge case on value, the mean
//! margins standing at 30 against 22.5 and at 18 against 0,
//! the edge field of 100, 100, 100, 100, and 190 carrying
//! its mean at 118 whole, and under S-5, the lagged margins
//! standing at the no other standing form and then 22.5,
//! 22.5, 35, and 41.25, and it passes under S-4 by design.
//! The edge case holds under S-1 and under S-6, the enlarged
//! field standing at a midhinge of 100 whole, and under
//! S-2, the median of the edge field standing at 100 whole,
//! and it passes under S-4 and fails under S-5, its closest
//! margin standing at its one mark alone so the loosened
//! scan cannot rename it and its one mark lagging to the no
//! other standing form under the lag, the two postures
//! computed from its single mark shape. Pin 6 fails
//! under S-1 at both marks on its pinned values, the
//! midhinges becoming 140 and 140, and under S-6 as under
//! S-1, under S-2 at both marks, the median margins standing
//! at 60 and 50 against the pinned 52.5 and 45, under S-3 at
//! its second mark alone, the mean margin standing at 90
//! against 45, and under S-5 from its first mark, and it
//! passes under S-4 by design. Pin 7 fails under S-2 at
//! every mark, the median margins standing at 60, 30, and 40
//! against the pinned 57.5, 95, and 155, under S-3 at the
//! fields of three and five, the mean margins standing at
//! 116.666667 and 134 against 95 and 155, under S-1 at the
//! fields of four and three, the midhinges becoming 145 and
//! 187.5, and under S-6 as under S-1, under S-5 from the
//! first mark of each meet, and it passes under S-4 by
//! design. Pin 8 fails under S-4, its
//! designed catcher, the loosened scan naming the fifth mark
//! at 75 where the pin states the second, under S-1 at its
//! second, third, fourth, and fifth marks on value, the
//! midhinges becoming 162.5, 202.5, 202.5, and 157.5, and
//! under S-6 as under S-1, under S-2, the median margins
//! standing at 50, 90, 90, and 70 over its figured marks,
//! under S-3, the mean margins standing at 83.333333,
//! 136.666667, 136.666667, and 76.666667, and under S-5 from
//! its second mark. Pin 9 fails under S-1 at every step, the
//! midhinges becoming 125, 152.5, 175, 195, and 240, and
//! under S-6 as under S-1, under S-2 at its third, fourth,
//! and fifth steps, the medians standing at 210, 225, and
//! 240, under S-3 at its third step alone, the mean standing
//! at 200 against 202.5, and under S-5 from its first step,
//! and it passes under S-4 by design. Pin 10 fails under S-1
//! at its first mark on the identity, the midhinge margin
//! standing at 25 against the mean of the quartile margins
//! at 50, under S-2 at its third and fourth marks on the
//! identity, under S-3 at its third mark on the identity,
//! the mean margin standing at 46.666667 against 47.5, and
//! under S-5 at its second mark onward on its pinned values,
//! and under S-6 its pinned values fail as under S-1, the
//! midhinge margins becoming 25, 42.5, 35, 45, and 55, while
//! its identity clause holds by design, the identity
//! standing for any field the shared helper returns, and it
//! passes under S-4 by design. Pin 11 passes under all six
//! mutations by design, no mark standing for any selection,
//! vector, or closing scan to move. Pins 12 to 15 pass under
//! all six mutations by design, the guards standing ahead of
//! any selection or scan, and they fail on a built tree whose
//! reading skipped the FIX-PG-10 figure guard, the FIX-PG-11
//! label fencing, or the day count preconditions of PG-47.
//! Pin 16, the exact mean pin, fails under the rounding
//! variant that rounds both quartile standings to six decimal
//! places before averaging, that variant printing a midhinge
//! standing of 1 and margins of 0 at its second and third
//! marks against the exact margins the pin states in full,
//! under S-1 at all three marks, a midhinge standing at its
//! first mark where the pin states none, its second mark's
//! margin becoming 2^-23, and its third mark's becoming
//! 2^-22, and under S-6 as under S-1, and under S-5, its
//! second mark printing the no other standing form and its
//! third the second mark's margin, and it passes under S-2
//! and S-3 by design, every figured field of its fixture
//! counting two or fewer, and under S-4 by design, its
//! closest margin standing unique at its second mark.

use paceghost::fieldday::{DayRun, FieldDay};
use paceghost::meet::{
    load_meet_text, meet_margins, meet_marks, meet_mean, meet_median, meet_middle, meet_midhinge,
    meet_quartiles, meet_spread, meet_standings, meet_tails, Meet,
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

struct MidhingeMark {
    prefix: String,
    standing: Option<(f64, f64)>,
}

fn midhinge_mark_parts(mark_line: &str) -> MidhingeMark {
    let prefix = record_prefix(mark_line);
    if mark_line.contains(", and no other standing") {
        return MidhingeMark {
            prefix,
            standing: None,
        };
    }
    let figure_text = mark_line
        .split(", midhinge standing ")
        .nth(1)
        .and_then(|rest| rest.split(',').next())
        .unwrap_or_else(|| panic!("a midhinge standing stands in {}", mark_line));
    let margin_text = mark_line
        .split(", midhinge margin ")
        .nth(1)
        .unwrap_or_else(|| panic!("a midhinge margin stands in {}", mark_line));
    MidhingeMark {
        prefix,
        standing: Some((
            figure_text
                .parse()
                .unwrap_or_else(|_| panic!("the midhinge standing parses in {}", mark_line)),
            margin_text
                .parse()
                .unwrap_or_else(|_| panic!("the midhinge margin parses in {}", mark_line)),
        )),
    }
}

fn midhinge_of(mark_line: &str) -> Option<(f64, f64)> {
    midhinge_mark_parts(mark_line).standing
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

fn median_of(figures: &[f64]) -> f64 {
    let count = figures.len();
    if count % 2 == 1 {
        figures[count / 2]
    } else {
        (figures[count / 2 - 1] + figures[count / 2]) / 2.0
    }
}

fn midhinge_meet() -> Meet {
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
fn test_the_midhinge_reading_over_a_hand_built_meet_line_by_line() {
    let meet = midhinge_meet();
    let expected: Vec<String> = [
        "meet midhinge, 3 days",
        "day, the first day, marks 2",
        "mark 1, at 100 seconds on the first day, a at a projection of 100, midhinge standing 150, midhinge margin 50",
        "mark 2, at 200 seconds on the first day, a at a projection of 100, midhinge standing 160, midhinge margin 60",
        "day, the second day, marks 0",
        "day, the third day, marks 3",
        "mark 3, at 100 seconds on the third day, a at a projection of 100, midhinge standing 147.5, midhinge margin 47.5",
        "mark 4, at 200 seconds on the third day, a at a projection of 100, midhinge standing 157.5, midhinge margin 57.5",
        "mark 5, at 300 seconds on the third day, a at a projection of 100, midhinge standing 170, midhinge margin 70",
        "midhinge counts, marks walked 5, marks with another standing 5, days 3, closest midhinge margin 47.5 at 100 seconds on the third day",
    ]
    .iter()
    .map(|line| line.to_string())
    .collect();
    assert_eq!(meet_midhinge(&meet), Ok(expected));
}

#[test]
fn test_midhinge_mark_lines_agree_with_the_marks_reading_line_by_line() {
    let meet = midhinge_meet();
    let marks = meet_marks(&meet).expect("the marks reading stands");
    let midhinge = meet_midhinge(&meet).expect("the midhinge reading stands");
    assert_eq!(
        midhinge.first().map(String::as_str),
        Some("meet midhinge, 3 days")
    );
    assert_eq!(
        marks.first().map(String::as_str),
        Some("meet marks, 3 days")
    );
    let marks_days: Vec<&String> = marks
        .iter()
        .filter(|line| line.starts_with("day,"))
        .collect();
    let midhinge_days: Vec<&String> = midhinge
        .iter()
        .filter(|line| line.starts_with("day,"))
        .collect();
    assert_eq!(marks_days, midhinge_days);
    let marks_marks = mark_lines(&marks);
    let midhinge_marks = mark_lines(&midhinge);
    assert_eq!(marks_marks.len(), midhinge_marks.len());
    for (place, (marks_line, midhinge_line)) in
        marks_marks.iter().zip(midhinge_marks.iter()).enumerate()
    {
        let parts = midhinge_mark_parts(midhinge_line);
        assert_eq!(
            parts.prefix,
            record_prefix(marks_line),
            "mark {}",
            place + 1
        );
        assert!(
            midhinge_line.starts_with(&format!("mark {}, ", place + 1)),
            "mark {} carries its place in order",
            place + 1
        );
    }
    let marks_counts = marks.last().expect("the marks counts line stands");
    let midhinge_counts = midhinge.last().expect("the midhinge counts line stands");
    assert_eq!(
        count_after(marks_counts, "marks walked "),
        count_after(midhinge_counts, "marks walked ")
    );
}

#[test]
fn test_the_last_mark_of_a_day_agrees_with_the_standings_reading() {
    let meet = midhinge_meet();
    let midhinge = meet_midhinge(&meet).expect("the midhinge reading stands");
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
    let mut midhinge_sections: Vec<Vec<&String>> = Vec::new();
    for line in &midhinge {
        if line.starts_with("day, the ") {
            midhinge_sections.push(Vec::new());
        }
        if let Some(section) = midhinge_sections.last_mut() {
            section.push(line);
        }
    }
    assert_eq!(sections.len(), midhinge_sections.len());
    let expected_counts = [2usize, 0, 5];
    let expected_midhinges = [160.0f64, 0.0, 170.0];
    for (day_index, (section, midhinge_section)) in
        sections.iter().zip(midhinge_sections.iter()).enumerate()
    {
        let day_marks: Vec<&&String> = midhinge_section
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
        let expected =
            (median_of(&others[..count]) + median_of(&others[others.len() - count..])) / 2.0;
        let named = expected_midhinges[day_index];
        assert!(
            (expected - named).abs() <= 0.000002,
            "day {} closes at the named midhinge",
            day_index + 1
        );
        let last = day_marks.last().expect("the day holds a mark");
        assert_eq!(
            midhinge_of(last).map(|(figure, _)| figure),
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
    let lines = meet_midhinge(&meet).expect("the midhinge read");
    assert_eq!(
        lines,
        vec![
            "meet midhinge, 3 days",
            "day, the first day, marks 1",
            "mark 1, at 100 seconds on the first day, solo at a projection of 100, and no other standing",
            "day, the second day, marks 1",
            "mark 2, at 200 seconds on the second day, solo at a projection of 150, and no other standing",
            "day, the third day, marks 0",
            "midhinge counts, marks walked 2, marks with another standing 0, days 3, and, no other standing at any mark",
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

fn midhinge_zero_condition_holds(field: &[f64], record: f64) -> bool {
    let count = field.len().div_ceil(2);
    let mut figures = middle_place_figures(&field[..count]);
    figures.extend(middle_place_figures(&field[field.len() - count..]));
    figures.iter().all(|figure| *figure == record)
}

#[test]
fn test_the_midhinge_margin_stands_at_zero_exactly_where_both_sub_fields_middle_places_stand_at_the_record(
) {
    // The zero fixture of the quartiles file standing
    // whole. At the first mark the field behind the holder
    // stands at 100, 100, and 190 and the midhinge margin
    // stands at 22.5. At the second mark the field stands
    // at 100, 100, 100, and 190 and the margin stands at
    // 22.5. At the third mark the field stands at 100, 100,
    // 150, and 190 and the margin stands at 35. At the
    // fourth mark the field stands at 100, 125, 150, and
    // 190 and the margin stands at 41.25. At the fifth mark
    // every standing figure of the field stands at 100 and
    // the margin stands at zero. The midhinge zero set is
    // exactly the fifth mark, while the lower quartile
    // margin stands at zero at the first, second, third,
    // and fifth marks, the upper quartile margin at the
    // fifth alone, and the median margin at the first,
    // second, and fifth, and the zero condition of PG-47, every middle
    // place figure of the lowest figures and every middle
    // place figure of the highest figures standing at the
    // record figure together, holds at the fifth mark
    // alone, checked in both directions at every mark from
    // the fields the fixture states. The edge case stands
    // in a meet of its own behind the same holder, its
    // field standing at 100, 100, 100, 100, and 190, the
    // lower sub-field at 100, 100, and 100 and the upper
    // sub-field at 100, 100, and 190, so the 190 stands in
    // the upper sub-field away from its middle place while
    // every middle place figure of both sub-fields stands
    // at the record, and its midhinge stands at 100 with a
    // margin of zero, the zero condition holding there in
    // both directions on a field one figure of which stands
    // away from the record.
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
    let lines = meet_midhinge(&meet).expect("the midhinge read");
    let marks = mark_lines(&lines);
    assert_eq!(marks.len(), 5);
    let margins: Vec<f64> = marks
        .iter()
        .map(|line| midhinge_of(line).expect("a midhinge stands").1)
        .collect();
    assert_eq!(margins, vec![22.5, 22.5, 35.0, 41.25, 0.0]);
    let fields: [Vec<f64>; 5] = [
        vec![100.0, 100.0, 190.0],
        vec![100.0, 100.0, 100.0, 190.0],
        vec![100.0, 100.0, 150.0, 190.0],
        vec![100.0, 125.0, 150.0, 190.0],
        vec![100.0, 100.0, 100.0, 100.0],
    ];
    for (place, (margin, field)) in margins.iter().zip(fields.iter()).enumerate() {
        assert_eq!(
            *margin == 0.0,
            midhinge_zero_condition_holds(field, 100.0),
            "mark {}",
            place + 1
        );
    }
    let midhinge_zero: Vec<usize> = margins
        .iter()
        .enumerate()
        .filter(|(_, margin)| **margin == 0.0)
        .map(|(index, _)| index + 1)
        .collect();
    assert_eq!(midhinge_zero, vec![5]);
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
    let median = meet_median(&meet).expect("the median read");
    let median_marks = mark_lines(&median);
    let median_zero: Vec<usize> = median_marks
        .iter()
        .enumerate()
        .filter(|(_, line)| median_margin_of(line) == Some(0.0))
        .map(|(index, _)| index + 1)
        .collect();
    assert_eq!(median_zero, vec![1, 2, 5]);
    let counts = lines.last().expect("the midhinge closes");
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
    let edge_lines = meet_midhinge(&edge).expect("the midhinge read");
    let edge_marks = mark_lines(&edge_lines);
    assert_eq!(edge_marks.len(), 1);
    let edge_standing = midhinge_of(edge_marks[0]).expect("a midhinge stands");
    assert_eq!(edge_standing, (100.0, 0.0));
    assert_eq!(
        edge_standing.1 == 0.0,
        midhinge_zero_condition_holds(&[100.0, 100.0, 100.0, 100.0, 190.0], 100.0)
    );
}

#[test]
fn test_the_midhinge_separates_from_below_against_the_margin_the_lower_quartile_the_middle_and_the_median(
) {
    // The below meet of the quartiles file standing whole,
    // at two marks behind a record of 100. At the first
    // mark the field stands at 110, 150, 170, and 180, its
    // midhinge at 152.5 with margin 52.5, above the margin
    // of PG-30 at 10, the lower quartile margin at 30, and
    // the middle margin at 50, and below the median margin
    // at 60, and standing at its mean margin whole by the
    // field of four. At the second mark the field stands
    // at 110, 120, 150, 170, and 400, its midhinge at 145
    // with margin 45, above the lower tail margin at
    // 26.666667 and below the middle and median margins at
    // 50 and 50, the below direction of the no fixed order
    // relations of PG-48 in pin form.
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
    let midhinge = meet_midhinge(&meet).expect("the midhinge read");
    let margins = meet_margins(&meet).expect("the margins read");
    let quartiles = meet_quartiles(&meet).expect("the quartiles read");
    let middle = meet_middle(&meet).expect("the middle read");
    let median = meet_median(&meet).expect("the median read");
    let mean = meet_mean(&meet).expect("the mean read");
    let tails = meet_tails(&meet).expect("the tails read");
    let marks = mark_lines(&midhinge);
    assert_eq!(marks.len(), 2);
    assert_eq!(midhinge_of(marks[0]), Some((152.5, 52.5)));
    let margin = margin_of(mark_lines(&margins)[0]).expect("a margin stands");
    assert_eq!(margin, 10.0);
    let lower_margin = quartiles_mark_parts(mark_lines(&quartiles)[0])
        .lower
        .expect("a lower quartile stands")
        .1;
    assert_eq!(lower_margin, 30.0);
    let middle_margin = middle_margin_of(mark_lines(&middle)[0]).expect("a middle margin stands");
    assert_eq!(middle_margin, 50.0);
    let median_margin = median_margin_of(mark_lines(&median)[0]).expect("a median margin stands");
    assert_eq!(median_margin, 60.0);
    let mean_margin = mean_margin_of(mark_lines(&mean)[0]).expect("a mean margin stands");
    assert_eq!(mean_margin, 52.5);
    assert!(
        52.5 > margin && 52.5 > lower_margin && 52.5 > middle_margin && 52.5 < median_margin,
        "midhinge 52.5, margin {}, lower quartile {}, middle {}, median {}",
        margin,
        lower_margin,
        middle_margin,
        median_margin
    );
    assert_eq!(midhinge_of(marks[1]), Some((145.0, 45.0)));
    let (lower_tail_margin, _) =
        tail_margins_of(mark_lines(&tails)[1]).expect("tail margins stand");
    assert!(
        (lower_tail_margin - 26.666667).abs() <= 0.000002,
        "lower tail margin {}",
        lower_tail_margin
    );
    let middle_margin = middle_margin_of(mark_lines(&middle)[1]).expect("a middle margin stands");
    assert_eq!(middle_margin, 50.0);
    let median_margin = median_margin_of(mark_lines(&median)[1]).expect("a median margin stands");
    assert_eq!(median_margin, 50.0);
    assert!(
        45.0 > lower_tail_margin && 45.0 < middle_margin && 45.0 < median_margin,
        "midhinge 45, lower tail {}, middle {}, median {}",
        lower_tail_margin,
        middle_margin,
        median_margin
    );
}

#[test]
fn test_the_midhinge_separates_from_above_against_the_median_the_mean_the_upper_tail_and_the_spread(
) {
    // The above meets of the quartiles file standing in
    // field form. The field of four, 120, 150, 170, and
    // 190, carries its midhinge at 157.5 with margin 57.5,
    // below the median margin at 60 and the spread at 90.
    // The field of three, 120, 130, and 400, carries its
    // midhinge at 195 with margin 95, above the median
    // margin at 30 and below the mean margin at
    // 116.666667. The field of five, 120, 130, 140, 380,
    // and 400, carries its midhinge at 255 with margin 155,
    // above the median margin at 40 and the mean margin at
    // 134 and below the upper tail margin at 206.666667
    // and the spread at 300, the above direction of the no
    // fixed order relations of PG-48 standing in this pin
    // where pin 6 states the below direction.
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
    let midhinge = meet_midhinge(&first).expect("the midhinge read");
    let median = meet_median(&first).expect("the median read");
    let spread = meet_spread(&first).expect("the spread read");
    let marks = mark_lines(&midhinge);
    assert_eq!(marks.len(), 1);
    assert_eq!(midhinge_of(marks[0]), Some((157.5, 57.5)));
    let median_margin = median_margin_of(mark_lines(&median)[0]).expect("a median margin stands");
    assert_eq!(median_margin, 60.0);
    let spread = spread_of(mark_lines(&spread)[0]).expect("a spread stands");
    assert_eq!(spread, 90.0);
    assert!(
        57.5 < median_margin && 57.5 < spread,
        "midhinge 57.5, median {}, spread {}",
        median_margin,
        spread
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
    let midhinge = meet_midhinge(&second).expect("the midhinge read");
    let median = meet_median(&second).expect("the median read");
    let mean = meet_mean(&second).expect("the mean read");
    let marks = mark_lines(&midhinge);
    assert_eq!(marks.len(), 1);
    assert_eq!(midhinge_of(marks[0]), Some((195.0, 95.0)));
    let median_margin = median_margin_of(mark_lines(&median)[0]).expect("a median margin stands");
    assert_eq!(median_margin, 30.0);
    let mean_margin = mean_margin_of(mark_lines(&mean)[0]).expect("a mean margin stands");
    assert!(
        (mean_margin - 116.666667).abs() <= 0.000002,
        "mean margin {}",
        mean_margin
    );
    assert!(
        95.0 > median_margin && 95.0 < mean_margin,
        "midhinge 95, median {}, mean {}",
        median_margin,
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
    let midhinge = meet_midhinge(&third).expect("the midhinge read");
    let median = meet_median(&third).expect("the median read");
    let mean = meet_mean(&third).expect("the mean read");
    let tails = meet_tails(&third).expect("the tails read");
    let spread = meet_spread(&third).expect("the spread read");
    let marks = mark_lines(&midhinge);
    assert_eq!(marks.len(), 1);
    assert_eq!(midhinge_of(marks[0]), Some((255.0, 155.0)));
    let median_margin = median_margin_of(mark_lines(&median)[0]).expect("a median margin stands");
    assert_eq!(median_margin, 40.0);
    let mean_margin = mean_margin_of(mark_lines(&mean)[0]).expect("a mean margin stands");
    assert_eq!(mean_margin, 134.0);
    let (_, upper_tail_margin) =
        tail_margins_of(mark_lines(&tails)[0]).expect("tail margins stand");
    assert!(
        (upper_tail_margin - 206.666667).abs() <= 0.000002,
        "upper tail margin {}",
        upper_tail_margin
    );
    let spread = spread_of(mark_lines(&spread)[0]).expect("a spread stands");
    assert_eq!(spread, 300.0);
    assert!(
        155.0 > median_margin && 155.0 > mean_margin && 155.0 < upper_tail_margin && 155.0 < spread,
        "midhinge 155, median {}, mean {}, upper tail {}, spread {}",
        median_margin,
        mean_margin,
        upper_tail_margin,
        spread
    );
}

#[test]
fn test_the_closest_midhinge_margin_tie_names_the_earliest_mark() {
    // Five marks stand on the first day, the first with no
    // other standing, the second carrying a field of 140,
    // 150, and 260 with a midhinge of 175 and a margin of
    // 75, the third and fourth carrying a field of 120,
    // 190, and 400 with a midhinge of 225 and a margin of
    // 125, the fourth standing as the holder's own re-read
    // at its standing figure, and the fifth carrying a
    // field of 130, 170, and 230 with a midhinge of 175 and
    // a margin of 75. The fifth mark stands at 600 seconds,
    // the earliest elapsed at which the field of the fifth
    // mark stands behind the third mark's field under the
    // loader's rule that a run's usages never decrease. The
    // counting names the second mark,
    // the earliest at the smallest margin, the tie real in
    // the printed lines at two marks whose fields and
    // quartiles differ whole.
    let meet = load_meet(&[
        day(
            1000,
            "a",
            &format!(
                "[{}, {}, {}, {}]",
                run("a", 500, &[(100, 10), (400, 40)]),
                run("b", 400, &[(200, 28), (300, 36), (600, 78)]),
                run("c", 300, &[(200, 30), (300, 57), (600, 102)]),
                run("d", 250, &[(200, 52), (300, 120), (600, 138)])
            ),
        ),
        quiet_day(1000, "a", "a"),
        quiet_day(1000, "a", "a"),
    ]);
    let lines = meet_midhinge(&meet).expect("the midhinge read");
    assert_eq!(
        lines,
        vec![
            "meet midhinge, 3 days",
            "day, the first day, marks 5",
            "mark 1, at 100 seconds on the first day, a at a projection of 100, and no other standing",
            "mark 2, at 200 seconds on the first day, a at a projection of 100, midhinge standing 175, midhinge margin 75",
            "mark 3, at 300 seconds on the first day, a at a projection of 100, midhinge standing 225, midhinge margin 125",
            "mark 4, at 400 seconds on the first day, a at a projection of 100, midhinge standing 225, midhinge margin 125",
            "mark 5, at 600 seconds on the first day, a at a projection of 100, midhinge standing 175, midhinge margin 75",
            "day, the second day, marks 0",
            "day, the third day, marks 0",
            "midhinge counts, marks walked 5, marks with another standing 4, days 3, closest midhinge margin 75 at 200 seconds on the first day",
        ]
    );
}

#[test]
fn test_the_midhinge_steps_as_the_field_grows() {
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
    let lines = meet_midhinge(&meet).expect("the midhinge read");
    let marks = mark_lines(&lines);
    let standings: Vec<f64> = marks
        .iter()
        .map(|line| midhinge_of(line).expect("a midhinge stands").0)
        .collect();
    assert_eq!(standings, vec![150.0, 180.0, 202.5, 232.5, 270.0]);
    assert_eq!(
        marks[0].as_str(),
        "mark 1, at 100 seconds on the first day, a at a projection of 100, midhinge standing 150, midhinge margin 50"
    );
    assert_eq!(
        marks[4].as_str(),
        "mark 5, at 500 seconds on the first day, a at a projection of 100, midhinge standing 270, midhinge margin 170"
    );
}

#[test]
fn test_the_quartile_margin_identity_and_the_margin_chain_hold_line_by_line() {
    // The identity and the chain of PG-48 over the hand
    // built meet of pin 1. At every mark the midhinge
    // margin is the exact mean of the lower quartile
    // margin and the upper quartile margin, and the chain
    // holds line by line, the margin of PG-30 never more
    // than the lower quartile margin, the lower quartile
    // margin never more than the midhinge margin, the
    // midhinge margin never more than the upper quartile
    // margin, and the upper quartile margin never more
    // than the spread of PG-32, the five midhinge margins
    // pinned at 50, 60, 47.5, 57.5, and 70.
    let meet = midhinge_meet();
    let midhinge = meet_midhinge(&meet).expect("the midhinge read");
    let quartiles = meet_quartiles(&meet).expect("the quartiles read");
    let margins = meet_margins(&meet).expect("the margins read");
    let spread = meet_spread(&meet).expect("the spread read");
    let midhinge_marks = mark_lines(&midhinge);
    let quartile_marks = mark_lines(&quartiles);
    let margin_marks = mark_lines(&margins);
    let spread_marks = mark_lines(&spread);
    assert_eq!(midhinge_marks.len(), quartile_marks.len());
    assert_eq!(midhinge_marks.len(), margin_marks.len());
    let expected_margins = [50.0, 60.0, 47.5, 57.5, 70.0];
    for (place, (((midhinge_line, quartile_line), margin_line), spread_line)) in midhinge_marks
        .iter()
        .zip(quartile_marks.iter())
        .zip(margin_marks.iter())
        .zip(spread_marks.iter())
        .enumerate()
    {
        let (_, midhinge_margin) = midhinge_of(midhinge_line).expect("a midhinge stands");
        let parts = quartiles_mark_parts(quartile_line);
        let (_, lower_margin) = parts.lower.expect("a lower quartile stands");
        let (_, upper_margin) = parts.upper.expect("an upper quartile stands");
        let margin = margin_of(margin_line).expect("a margin stands");
        let spread = spread_of(spread_line).expect("a spread stands");
        assert_eq!(
            midhinge_margin,
            expected_margins[place],
            "mark {}",
            place + 1
        );
        assert!(
            (midhinge_margin - (lower_margin + upper_margin) / 2.0).abs() <= 0.000002,
            "mark {}",
            place + 1
        );
        assert!(margin <= lower_margin + 0.000002, "mark {}", place + 1);
        assert!(
            lower_margin <= midhinge_margin + 0.000002,
            "mark {}",
            place + 1
        );
        assert!(
            midhinge_margin <= upper_margin + 0.000002,
            "mark {}",
            place + 1
        );
        assert!(upper_margin <= spread + 0.000002, "mark {}", place + 1);
        assert!(midhinge_margin >= 0.0, "mark {}", place + 1);
    }
}

#[test]
fn test_a_meet_in_which_no_read_projects_reads_with_marks_walked_zero() {
    let meet = load_meet(&[
        quiet_day(600, "a", "a"),
        quiet_day(600, "a", "a"),
        quiet_day(600, "a", "a"),
    ]);
    let lines = meet_midhinge(&meet).expect("the midhinge read");
    assert_eq!(
        lines,
        vec![
            "meet midhinge, 3 days",
            "day, the first day, marks 0",
            "day, the second day, marks 0",
            "day, the third day, marks 0",
            "midhinge counts, marks walked 0, marks with another standing 0, days 3, and, no other standing at any mark",
        ]
    );
}

// The guard pins. The midhinge reading stands inside the
// fencing closure of FIX-PG-11 from its birth, and under
// the figure guard of FIX-PG-10 and the day count errors
// of PG-47, in the one walk it shares with the sixteen
// readings before it.

#[test]
fn test_a_hand_built_meet_with_no_days_draws_the_named_error_through_midhinge() {
    let meet = Meet { days: vec![] };
    let error = meet_midhinge(&meet).expect_err("the reading names the precondition");
    assert_eq!(error, "the meet holds no days");
}

#[test]
fn test_a_hand_built_meet_with_too_many_days_draws_the_named_error_through_midhinge() {
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
    let error = meet_midhinge(&meet).expect_err("the reading names the precondition");
    assert_eq!(
        error,
        "the meet holds more days than the ordinal table names"
    );
}

#[test]
fn test_a_hand_built_non_finite_figure_draws_the_named_error_through_midhinge() {
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
    let error = meet_midhinge(&meet).expect_err("the reading names the precondition");
    assert_eq!(error, "the window is not a finite number");
}

#[test]
fn test_a_hand_built_break_bearing_label_draws_the_named_error_through_midhinge() {
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
    let error = meet_midhinge(&meet).expect_err("the reading names the precondition");
    assert_eq!(error, "a runner label holds a line break");
}

// The exact mean pin. The midhinge of PG-48 is the mean
// of the two quartile standings computed on exact values,
// and this pin states marks whose two quartile standings
// render alike at six decimal places while their exact
// mean does not stand at the record figure.

#[test]
fn test_the_midhinge_is_the_exact_mean_of_quartiles_that_render_alike() {
    // The field of the second mark counts one figure at
    // 1 + 2^-22, the standing of the one competitor behind
    // the holder, rendering as 1, its midhinge standing at
    // that figure with a margin of 2^-22 rendered in full.
    // The field of the third mark counts two figures at
    // 1 + 2^-22 and 1 + 2^-21, both rendering as 1, its
    // midhinge standing at 1 + 3 * 2^-23, rendering as 1,
    // with a margin of 3 * 2^-23 rendered in full. A
    // midhinge computed by averaging the rendered quartile
    // standings would stand at 1 with margins of 0 at both
    // marks. The holder stands at a projection of 1 whole,
    // read under the first day's window at the window
    // itself.
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
    let lines = meet_midhinge(&meet).expect("the midhinge read");
    assert_eq!(
        lines,
        vec![
            "meet midhinge, 3 days",
            "day, the first day, marks 2",
            "mark 1, at 4194304 seconds on the first day, b at a projection of 1, and no other standing",
            "mark 2, at 4194305 seconds on the first day, a at a projection of 1, midhinge standing 1, midhinge margin 0.0000002384185791015625",
            "day, the second day, marks 1",
            "mark 3, at 2097152 seconds on the second day, a at a projection of 1, midhinge standing 1, midhinge margin 0.00000035762786865234375",
            "day, the third day, marks 0",
            "midhinge counts, marks walked 3, marks with another standing 2, days 3, closest midhinge margin 0.0000002384185791015625 at 4194305 seconds on the first day",
        ]
    );
}
