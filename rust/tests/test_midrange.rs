//! Tests for PG-49 and PG-50, the midrange reading over a meet.
//!
//! Written for Round 24. Each pin's failure mode stands
//! stated here pin by pin, not as one blanket claim. Pins 1
//! to 16 fail before the round by the absence of the reading,
//! the meet_midrange function not standing on the pre round
//! tree. On the built tree, pin 1 fails under S-1 at all five
//! marks, the standings becoming 125, 135, 135, 145, and 160
//! over the enlarged fields, and under S-6 as under S-1,
//! under S-2 at its third and fourth marks, the medians
//! standing at 150 and 160 against the midranges at 145 and
//! 155, under S-3 at its third and fourth marks, the means
//! standing at 146.666667 and 157.5 against the midranges,
//! and under S-5 from the first mark onward, and it passes
//! under S-4 by design, its closest margin standing unique.
//! Pin 2 passes under all six mutations by design and claims
//! no mutation failure, the mark line prefix it compares
//! standing rendered from the walk's mark records, which no
//! mutation of this round touches. Pin 3 fails under S-1 at
//! both closes, under S-5 at both closes, and under S-6 at
//! both closes, and it passes under S-2 and S-3 by design at
//! both closes, the median and the mean standing at the
//! midrange at each close's field, the third day's field of
//! five carrying its mean at 170 whole, and under S-4 by
//! design, no counts line standing in it. Pin 4 fails under
//! S-1 and under S-6 by design, the holder's own figure
//! entering the field at every mark so a midrange stands
//! where the pin states none, and it passes under S-2, S-3,
//! S-4, and S-5 by design. Pin 5 passes under S-1 whole by
//! the extremes' blindness, the nearest standing standing at
//! the record at all five marks so the enlarged field moves
//! neither extreme, a posture parting from its midhinge
//! analogue stated in the open, and it fails under S-2 at
//! its first four marks, the median margins standing at 0,
//! 0, 25, and 37.5 against the midrange margins at 45, under
//! S-3 at its first four marks, the mean margins standing at
//! 30, 22.5, 35, and 41.25, under S-5 at its first and fifth
//! marks, and under S-6 on its median zero set clause alone,
//! the enlarged fields carrying the median margin to zero at
//! the third mark so the median zero set becomes the first,
//! second, third, and fifth marks against the pinned first,
//! second, and fifth, its own midrange values holding under
//! S-6 entire, and it passes under S-4 by design, its closest
//! margin, zero at the fifth mark, standing unique. The edge
//! case holds under S-1 and under S-6, the enlarged field
//! standing at a midrange of 145 whole, and under S-2, the
//! median of the edge field standing at 100 whole against
//! the midrange at 145, and it fails under S-3 and under
//! S-5. Pin 6 fails under S-1 at both marks on its pinned
//! values, the midranges becoming 140 and 137.5 over the
//! enlarged fields, and under S-6 as under S-1, under S-2 at
//! both marks, the median margins standing at 60 and 60
//! against the pinned 45 and 42.5, under S-3 at both marks,
//! the mean margins standing at 52.5 and 51.25, and under
//! S-5 from its first mark, and it passes under S-4 by
//! design, its margins standing at 45 and 42.5 with the
//! closest unique. Pin 7 fails under S-1 at every mark of
//! each meet, under S-6 as under S-1, under S-2 at every
//! mark, the median margins standing at 60, 30, and 40
//! against the pinned 55, 160, and 160, under S-3 at every
//! mark, the mean margins standing at 57.5, 116.666667, and
//! 134 against the pinned values, and under S-5 at the one
//! mark of each meet, and it passes under S-4 by design,
//! each meet standing at one figured mark. Pin 8 fails under
//! S-4, its designed catcher, the loosened scan naming the
//! fifth mark at 100 where the pin states the second, under
//! S-1 at its first to fifth marks, a midrange standing at
//! its first mark where the pin states none and the
//! standings becoming 180, 250, 250, and 185 over its figured
//! marks, and under S-6 as under S-1, under S-2 and under
//! S-3 at its second to fifth marks, and under S-5 from its
//! second mark. Pin 9 fails under S-1 at every step, the
//! midranges becoming 125, 155, 170, 215, and 260, and
//! under S-6 as under S-1, under S-2 at its third, fourth,
//! and fifth steps, under S-3 at its third, fourth, and
//! fifth steps, and under S-5 from its first step, and it
//! passes under S-4 by design, its margins standing at 50,
//! 80, 95, 140, and 185 with the closest unique at the first
//! step. Pin 10 fails under S-1 on the identity at every
//! mark, the midrange margins becoming 25, 35, 35, 45, and
//! 60 against the mean of the margin and the spread, under
//! S-2 and S-3 at its third and fourth marks on the
//! identity, and under S-5 from its first mark on its pinned
//! values, and under S-6 on its pinned values, its identity
//! clause holding under S-6 by design, the identity standing
//! for any field the shared helper returns, and it passes
//! under S-4 by design. Pin 11 passes under all six mutations
//! by design, no mark standing for any selection, vector, or
//! closing scan to move. Pins 12 to 15 pass under all six
//! mutations by design, the guards standing ahead of any
//! selection or scan, and they fail on a built tree whose
//! reading skipped the FIX-PG-10 figure guard, the FIX-PG-11
//! label fencing, or the day count preconditions of PG-49.
//! Pin 16, the exact mean pin, fails under the rounding
//! variant that rounds both extreme standings to six decimal
//! places before averaging, that variant printing a midrange
//! standing of 1 and margins of 0 at its second and third
//! marks against the exact margins the pin states in full,
//! under S-1 at all three marks, a midrange standing at its
//! first mark where the pin states none, and under S-6 as
//! under S-1, and under S-5, its second mark printing the no
//! other standing form and its third the second mark's
//! margin, and it passes under S-2 and S-3 by design, every
//! figured field of its fixture counting two figures or
//! fewer, and under S-4 by design, its closest margin
//! standing unique at its second mark.

use paceghost::fieldday::{DayRun, FieldDay};
use paceghost::meet::{
    load_meet_text, meet_margins, meet_marks, meet_mean, meet_median, meet_middle, meet_midhinge,
    meet_midrange, meet_quartiles, meet_spread, meet_standings, meet_tails, Meet,
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

struct MidrangeMark {
    prefix: String,
    standing: Option<(f64, f64)>,
}

fn midrange_mark_parts(mark_line: &str) -> MidrangeMark {
    let prefix = record_prefix(mark_line);
    if mark_line.contains(", and no other standing") {
        return MidrangeMark {
            prefix,
            standing: None,
        };
    }
    let figure_text = mark_line
        .split(", midrange standing ")
        .nth(1)
        .and_then(|rest| rest.split(',').next())
        .unwrap_or_else(|| panic!("a midrange standing stands in {}", mark_line));
    let margin_text = mark_line
        .split(", midrange margin ")
        .nth(1)
        .unwrap_or_else(|| panic!("a midrange margin stands in {}", mark_line));
    MidrangeMark {
        prefix,
        standing: Some((
            figure_text
                .parse()
                .unwrap_or_else(|_| panic!("the midrange standing parses in {}", mark_line)),
            margin_text
                .parse()
                .unwrap_or_else(|_| panic!("the midrange margin parses in {}", mark_line)),
        )),
    }
}

fn midrange_of(mark_line: &str) -> Option<(f64, f64)> {
    midrange_mark_parts(mark_line).standing
}

fn midhinge_of(mark_line: &str) -> Option<(f64, f64)> {
    if mark_line.contains(", and no other standing") {
        return None;
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
    Some((
        figure_text
            .parse()
            .unwrap_or_else(|_| panic!("the midhinge standing parses in {}", mark_line)),
        margin_text
            .parse()
            .unwrap_or_else(|_| panic!("the midhinge margin parses in {}", mark_line)),
    ))
}

fn lower_quartile_of(mark_line: &str) -> Option<(f64, f64)> {
    if mark_line.contains(", and no other standing") {
        return None;
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
    Some((
        lower_figure_text
            .parse()
            .unwrap_or_else(|_| panic!("the lower quartile standing parses in {}", mark_line)),
        lower_margin_text
            .parse()
            .unwrap_or_else(|_| panic!("the lower quartile margin parses in {}", mark_line)),
    ))
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

fn midrange_meet() -> Meet {
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
fn test_the_midrange_reading_over_a_hand_built_meet_line_by_line() {
    let meet = midrange_meet();
    let expected: Vec<String> = [
        "meet midrange, 3 days",
        "day, the first day, marks 2",
        "mark 1, at 100 seconds on the first day, a at a projection of 100, midrange standing 150, midrange margin 50",
        "mark 2, at 200 seconds on the first day, a at a projection of 100, midrange standing 160, midrange margin 60",
        "day, the second day, marks 0",
        "day, the third day, marks 3",
        "mark 3, at 100 seconds on the third day, a at a projection of 100, midrange standing 145, midrange margin 45",
        "mark 4, at 200 seconds on the third day, a at a projection of 100, midrange standing 155, midrange margin 55",
        "mark 5, at 300 seconds on the third day, a at a projection of 100, midrange standing 170, midrange margin 70",
        "midrange counts, marks walked 5, marks with another standing 5, days 3, closest midrange margin 45 at 100 seconds on the third day",
    ]
    .iter()
    .map(|line| line.to_string())
    .collect();
    assert_eq!(meet_midrange(&meet), Ok(expected));
}

#[test]
fn test_midrange_mark_lines_agree_with_the_marks_reading_line_by_line() {
    let meet = midrange_meet();
    let marks = meet_marks(&meet).expect("the marks reading stands");
    let midrange = meet_midrange(&meet).expect("the midrange reading stands");
    assert_eq!(
        midrange.first().map(String::as_str),
        Some("meet midrange, 3 days")
    );
    assert_eq!(
        marks.first().map(String::as_str),
        Some("meet marks, 3 days")
    );
    let marks_days: Vec<&String> = marks
        .iter()
        .filter(|line| line.starts_with("day,"))
        .collect();
    let midrange_days: Vec<&String> = midrange
        .iter()
        .filter(|line| line.starts_with("day,"))
        .collect();
    assert_eq!(marks_days, midrange_days);
    let marks_marks = mark_lines(&marks);
    let midrange_marks = mark_lines(&midrange);
    assert_eq!(marks_marks.len(), midrange_marks.len());
    for (place, (marks_line, midrange_line)) in
        marks_marks.iter().zip(midrange_marks.iter()).enumerate()
    {
        let parts = midrange_mark_parts(midrange_line);
        assert_eq!(
            parts.prefix,
            record_prefix(marks_line),
            "mark {}",
            place + 1
        );
        assert!(
            midrange_line.starts_with(&format!("mark {}, ", place + 1)),
            "mark {} carries its place in order",
            place + 1
        );
    }
    let marks_counts = marks.last().expect("the marks counts line stands");
    let midrange_counts = midrange.last().expect("the midrange counts line stands");
    assert_eq!(
        count_after(marks_counts, "marks walked "),
        count_after(midrange_counts, "marks walked ")
    );
}

#[test]
fn test_the_last_mark_of_a_day_agrees_with_the_standings_reading() {
    let meet = midrange_meet();
    let midrange = meet_midrange(&meet).expect("the midrange reading stands");
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
    let mut midrange_sections: Vec<Vec<&String>> = Vec::new();
    for line in &midrange {
        if line.starts_with("day, the ") {
            midrange_sections.push(Vec::new());
        }
        if let Some(section) = midrange_sections.last_mut() {
            section.push(line);
        }
    }
    assert_eq!(sections.len(), midrange_sections.len());
    let expected_counts = [2usize, 0, 5];
    let expected_midranges = [160.0f64, 0.0, 170.0];
    for (day_index, (section, midrange_section)) in
        sections.iter().zip(midrange_sections.iter()).enumerate()
    {
        let day_marks: Vec<&&String> = midrange_section
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
        let expected = (others[0] + others[others.len() - 1]) / 2.0;
        let named = expected_midranges[day_index];
        assert!(
            (expected - named).abs() <= 0.000002,
            "day {} closes at the named midrange",
            day_index + 1
        );
        let last = day_marks.last().expect("the day holds a mark");
        assert_eq!(
            midrange_of(last).map(|(figure, _)| figure),
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
    let lines = meet_midrange(&meet).expect("the midrange read");
    assert_eq!(
        lines,
        vec![
            "meet midrange, 3 days",
            "day, the first day, marks 1",
            "mark 1, at 100 seconds on the first day, solo at a projection of 100, and no other standing",
            "day, the second day, marks 1",
            "mark 2, at 200 seconds on the second day, solo at a projection of 150, and no other standing",
            "day, the third day, marks 0",
            "midrange counts, marks walked 2, marks with another standing 0, days 3, and, no other standing at any mark",
        ]
    );
}

fn midrange_zero_condition_holds(field: &[f64], record: f64) -> bool {
    field.iter().all(|figure| *figure == record)
}

#[test]
fn test_the_midrange_margin_stands_at_zero_exactly_where_every_figure_of_the_field_stands_at_the_record(
) {
    // The zero fixture of the quartiles file standing
    // whole. At the first mark the field behind the holder
    // stands at 100, 100, and 190 and the midrange margin
    // stands at 45. At the second mark the field stands at
    // 100, 100, 100, and 190 and the margin stands at 45.
    // At the third mark the field stands at 100, 100, 150,
    // and 190 and the margin stands at 45. At the fourth
    // mark the field stands at 100, 125, 150, and 190 and
    // the margin stands at 45. At the fifth mark every
    // standing figure of the field stands at 100 and the
    // margin stands at zero. The midrange zero set is
    // exactly the fifth mark, the set of the spread of
    // PG-32, while the margin of PG-30 stands at zero at
    // all five marks and the median margin at the first,
    // second, and fifth, and the zero condition of PG-49,
    // every figure of the field standing at the record
    // figure, holds at the fifth mark alone, checked in
    // both directions at every mark from the fields the
    // fixture states. The edge case stands in a meet of
    // its own behind the same holder, its field standing
    // at 100, 100, 100, 100, and 190, its midrange standing
    // at 145 with a margin of 45 exactly where the midhinge
    // of the same field stands at 100 with a margin of
    // zero, the zero condition failing there in both
    // directions on a field four figures of which stand at
    // the record.
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
    let lines = meet_midrange(&meet).expect("the midrange read");
    let marks = mark_lines(&lines);
    assert_eq!(marks.len(), 5);
    let margins: Vec<f64> = marks
        .iter()
        .map(|line| midrange_of(line).expect("a midrange stands").1)
        .collect();
    assert_eq!(margins, vec![45.0, 45.0, 45.0, 45.0, 0.0]);
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
            midrange_zero_condition_holds(field, 100.0),
            "mark {}",
            place + 1
        );
    }
    let midrange_zero: Vec<usize> = margins
        .iter()
        .enumerate()
        .filter(|(_, margin)| **margin == 0.0)
        .map(|(index, _)| index + 1)
        .collect();
    assert_eq!(midrange_zero, vec![5]);
    let spread = meet_spread(&meet).expect("the spread read");
    let spread_marks = mark_lines(&spread);
    let spread_zero: Vec<usize> = spread_marks
        .iter()
        .enumerate()
        .filter(|(_, line)| spread_of(line) == Some(0.0))
        .map(|(index, _)| index + 1)
        .collect();
    assert_eq!(spread_zero, vec![5]);
    assert_eq!(midrange_zero, spread_zero);
    let margins_reading = meet_margins(&meet).expect("the margins read");
    let margin_marks = mark_lines(&margins_reading);
    let margin_zero: Vec<usize> = margin_marks
        .iter()
        .enumerate()
        .filter(|(_, line)| margin_of(line) == Some(0.0))
        .map(|(index, _)| index + 1)
        .collect();
    assert_eq!(margin_zero, vec![1, 2, 3, 4, 5]);
    assert_ne!(midrange_zero, margin_zero);
    let median = meet_median(&meet).expect("the median read");
    let median_marks = mark_lines(&median);
    let median_zero: Vec<usize> = median_marks
        .iter()
        .enumerate()
        .filter(|(_, line)| median_margin_of(line) == Some(0.0))
        .map(|(index, _)| index + 1)
        .collect();
    assert_eq!(median_zero, vec![1, 2, 5]);
    let counts = lines.last().expect("the midrange closes");
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
    let edge_lines = meet_midrange(&edge).expect("the midrange read");
    let edge_marks = mark_lines(&edge_lines);
    assert_eq!(edge_marks.len(), 1);
    let edge_standing = midrange_of(edge_marks[0]).expect("a midrange stands");
    assert_eq!(edge_standing, (145.0, 45.0));
    assert_eq!(
        edge_standing.1 == 0.0,
        midrange_zero_condition_holds(&[100.0, 100.0, 100.0, 100.0, 190.0], 100.0)
    );
    let edge_midhinge = meet_midhinge(&edge).expect("the midhinge read");
    let edge_midhinge_marks = mark_lines(&edge_midhinge);
    assert_eq!(edge_midhinge_marks.len(), 1);
    assert_eq!(
        midhinge_of(edge_midhinge_marks[0]).expect("a midhinge stands"),
        (100.0, 0.0)
    );
}

#[test]
fn test_the_midrange_separates_from_below_against_the_margin_the_lower_tail_the_lower_quartile_and_the_middle(
) {
    // The below meet of the quartiles file recomposed for
    // the midrange. As the fixture stands shipped, its
    // second mark carries a field of 110, 120, 150, 170,
    // and 400, the 400 throwing the midrange above every
    // interior statistic at that mark, so the last read of
    // the fifth run stands at 35 in place of 80, a
    // projection of 175 in place of 400, and the sixth run
    // stands dropped, its arrival at the second mark being
    // the other figure the shipped field carries that the
    // recomposition removes. As recomposed, at the first
    // mark the field stands at 110, 150, 170, and 180, its
    // midrange at 145 with margin 45, above the margin of
    // PG-30 at 10 and the lower tail and lower quartile
    // margins at 30, and below the middle margin at 50, the
    // mean and midhinge margins at 52.5, and the median
    // margin at 60. At the second mark the field stands at
    // 110, 150, 170, and 175, its midrange at 142.5 with
    // margin 42.5 on the same relations, the middle margin
    // at 50, the mean and midhinge margins at 51.25, and
    // the median margin at 60, the below direction of the
    // no fixed order relations of PG-50 in pin form.
    let meet = load_meet(&[
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
    ]);
    let midrange = meet_midrange(&meet).expect("the midrange read");
    let margins = meet_margins(&meet).expect("the margins read");
    let quartiles = meet_quartiles(&meet).expect("the quartiles read");
    let middle = meet_middle(&meet).expect("the middle read");
    let median = meet_median(&meet).expect("the median read");
    let mean = meet_mean(&meet).expect("the mean read");
    let midhinge = meet_midhinge(&meet).expect("the midhinge read");
    let tails = meet_tails(&meet).expect("the tails read");
    let marks = mark_lines(&midrange);
    assert_eq!(marks.len(), 2);
    assert_eq!(midrange_of(marks[0]), Some((145.0, 45.0)));
    let margin = margin_of(mark_lines(&margins)[0]).expect("a margin stands");
    assert_eq!(margin, 10.0);
    let (lower_tail_margin, _) =
        tail_margins_of(mark_lines(&tails)[0]).expect("tail margins stand");
    assert_eq!(lower_tail_margin, 30.0);
    let lower_margin = lower_quartile_of(mark_lines(&quartiles)[0])
        .expect("a lower quartile stands")
        .1;
    assert_eq!(lower_margin, 30.0);
    let middle_margin = middle_margin_of(mark_lines(&middle)[0]).expect("a middle margin stands");
    assert_eq!(middle_margin, 50.0);
    let mean_margin = mean_margin_of(mark_lines(&mean)[0]).expect("a mean margin stands");
    assert_eq!(mean_margin, 52.5);
    let midhinge_margin = midhinge_of(mark_lines(&midhinge)[0])
        .expect("a midhinge stands")
        .1;
    assert_eq!(midhinge_margin, 52.5);
    let median_margin = median_margin_of(mark_lines(&median)[0]).expect("a median margin stands");
    assert_eq!(median_margin, 60.0);
    assert!(
        45.0 > margin
            && 45.0 > lower_tail_margin
            && 45.0 > lower_margin
            && 45.0 < middle_margin
            && 45.0 < mean_margin
            && 45.0 < midhinge_margin
            && 45.0 < median_margin,
        "midrange 45, margin {}, lower tail {}, lower quartile {}, middle {}, mean {}, midhinge {}, median {}",
        margin,
        lower_tail_margin,
        lower_margin,
        middle_margin,
        mean_margin,
        midhinge_margin,
        median_margin
    );
    assert_eq!(midrange_of(marks[1]), Some((142.5, 42.5)));
    let margin = margin_of(mark_lines(&margins)[1]).expect("a margin stands");
    assert_eq!(margin, 10.0);
    let (lower_tail_margin, _) =
        tail_margins_of(mark_lines(&tails)[1]).expect("tail margins stand");
    assert_eq!(lower_tail_margin, 30.0);
    let lower_margin = lower_quartile_of(mark_lines(&quartiles)[1])
        .expect("a lower quartile stands")
        .1;
    assert_eq!(lower_margin, 30.0);
    let middle_margin = middle_margin_of(mark_lines(&middle)[1]).expect("a middle margin stands");
    assert_eq!(middle_margin, 50.0);
    let mean_margin = mean_margin_of(mark_lines(&mean)[1]).expect("a mean margin stands");
    assert_eq!(mean_margin, 51.25);
    let midhinge_margin = midhinge_of(mark_lines(&midhinge)[1])
        .expect("a midhinge stands")
        .1;
    assert_eq!(midhinge_margin, 51.25);
    let median_margin = median_margin_of(mark_lines(&median)[1]).expect("a median margin stands");
    assert_eq!(median_margin, 60.0);
    assert!(
        42.5 > margin
            && 42.5 > lower_tail_margin
            && 42.5 > lower_margin
            && 42.5 < middle_margin
            && 42.5 < mean_margin
            && 42.5 < midhinge_margin
            && 42.5 < median_margin,
        "midrange 42.5, margin {}, lower tail {}, lower quartile {}, middle {}, mean {}, midhinge {}, median {}",
        margin,
        lower_tail_margin,
        lower_margin,
        middle_margin,
        mean_margin,
        midhinge_margin,
        median_margin
    );
}

#[test]
fn test_the_midrange_separates_from_above_against_the_median_the_mean_the_midhinge_and_the_spread()
{
    // The above meets of the quartiles file standing in
    // field form. The field of four, 120, 150, 170, and
    // 190, carries its midrange at 155 with margin 55,
    // below the median margin at 60, the mean and midhinge
    // margins at 57.5, and the spread at 90, the count at
    // which the midrange parts from the midhinge while the
    // midhinge stands at the mean whole. The field of
    // three, 120, 130, and 400, carries its midrange at 260
    // with margin 160, above the median margin at 30 and
    // the mean margin at 116.666667 and the midhinge margin
    // at 95, and below the upper tail margin at 165 and the
    // spread at 300. The field of five, 120, 130, 140, 380,
    // and 400, carries its midrange at 260 with margin 160,
    // above the median margin at 40 and the mean margin at
    // 134 and the midhinge margin at 155, and below the
    // upper tail margin at 206.666667 and the spread at
    // 300, the above direction of the no fixed order
    // relations of PG-50 standing in this pin where pin 6
    // states the below direction.
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
    let midrange = meet_midrange(&first).expect("the midrange read");
    let median = meet_median(&first).expect("the median read");
    let mean = meet_mean(&first).expect("the mean read");
    let midhinge = meet_midhinge(&first).expect("the midhinge read");
    let spread = meet_spread(&first).expect("the spread read");
    let marks = mark_lines(&midrange);
    assert_eq!(marks.len(), 1);
    assert_eq!(midrange_of(marks[0]), Some((155.0, 55.0)));
    let median_margin = median_margin_of(mark_lines(&median)[0]).expect("a median margin stands");
    assert_eq!(median_margin, 60.0);
    let mean_margin = mean_margin_of(mark_lines(&mean)[0]).expect("a mean margin stands");
    assert_eq!(mean_margin, 57.5);
    let midhinge_margin = midhinge_of(mark_lines(&midhinge)[0])
        .expect("a midhinge stands")
        .1;
    assert_eq!(midhinge_margin, 57.5);
    let spread = spread_of(mark_lines(&spread)[0]).expect("a spread stands");
    assert_eq!(spread, 90.0);
    assert!(
        55.0 < median_margin && 55.0 < mean_margin && 55.0 < midhinge_margin && 55.0 < spread,
        "midrange 55, median {}, mean {}, midhinge {}, spread {}",
        median_margin,
        mean_margin,
        midhinge_margin,
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
    let midrange = meet_midrange(&second).expect("the midrange read");
    let median = meet_median(&second).expect("the median read");
    let mean = meet_mean(&second).expect("the mean read");
    let midhinge = meet_midhinge(&second).expect("the midhinge read");
    let tails = meet_tails(&second).expect("the tails read");
    let spread = meet_spread(&second).expect("the spread read");
    let marks = mark_lines(&midrange);
    assert_eq!(marks.len(), 1);
    assert_eq!(midrange_of(marks[0]), Some((260.0, 160.0)));
    let median_margin = median_margin_of(mark_lines(&median)[0]).expect("a median margin stands");
    assert_eq!(median_margin, 30.0);
    let mean_margin = mean_margin_of(mark_lines(&mean)[0]).expect("a mean margin stands");
    assert!(
        (mean_margin - 116.666667).abs() <= 0.000002,
        "mean margin {}",
        mean_margin
    );
    let midhinge_margin = midhinge_of(mark_lines(&midhinge)[0])
        .expect("a midhinge stands")
        .1;
    assert_eq!(midhinge_margin, 95.0);
    let (_, upper_tail_margin) =
        tail_margins_of(mark_lines(&tails)[0]).expect("tail margins stand");
    assert_eq!(upper_tail_margin, 165.0);
    let spread = spread_of(mark_lines(&spread)[0]).expect("a spread stands");
    assert_eq!(spread, 300.0);
    assert!(
        160.0 > median_margin
            && 160.0 > mean_margin
            && 160.0 > midhinge_margin
            && 160.0 < upper_tail_margin
            && 160.0 < spread,
        "midrange 160, median {}, mean {}, midhinge {}, upper tail {}, spread {}",
        median_margin,
        mean_margin,
        midhinge_margin,
        upper_tail_margin,
        spread
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
    let midrange = meet_midrange(&third).expect("the midrange read");
    let median = meet_median(&third).expect("the median read");
    let mean = meet_mean(&third).expect("the mean read");
    let midhinge = meet_midhinge(&third).expect("the midhinge read");
    let tails = meet_tails(&third).expect("the tails read");
    let spread = meet_spread(&third).expect("the spread read");
    let marks = mark_lines(&midrange);
    assert_eq!(marks.len(), 1);
    assert_eq!(midrange_of(marks[0]), Some((260.0, 160.0)));
    let median_margin = median_margin_of(mark_lines(&median)[0]).expect("a median margin stands");
    assert_eq!(median_margin, 40.0);
    let mean_margin = mean_margin_of(mark_lines(&mean)[0]).expect("a mean margin stands");
    assert_eq!(mean_margin, 134.0);
    let midhinge_margin = midhinge_of(mark_lines(&midhinge)[0])
        .expect("a midhinge stands")
        .1;
    assert_eq!(midhinge_margin, 155.0);
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
        160.0 > median_margin
            && 160.0 > mean_margin
            && 160.0 > midhinge_margin
            && 160.0 < upper_tail_margin
            && 160.0 < spread,
        "midrange 160, median {}, mean {}, midhinge {}, upper tail {}, spread {}",
        median_margin,
        mean_margin,
        midhinge_margin,
        upper_tail_margin,
        spread
    );
}

#[test]
fn test_the_closest_midrange_margin_tie_names_the_earliest_mark() {
    // Five marks stand on the first day, the first with no
    // other standing, the second carrying a field of 140,
    // 150, and 260 with a midrange of 200 and a margin of
    // 100, the third and fourth carrying a field of 120,
    // 190, and 400 with a midrange of 260 and a margin of
    // 160, the fourth standing as the holder's own re-read
    // at its standing figure, and the fifth carrying a
    // field of 130, 170, and 270 with a midrange of 200 and
    // a margin of 100. The tie meet of the midhinge file
    // stands recomposed for the midrange, the last read of
    // the fourth run standing at 162 in place of 138, a
    // projection of 270 at 600 seconds in place of 230, the
    // run's usages still rising 52, 120, and 162 under the
    // loader's rule, the midhinge's tie at 75 not surviving
    // the change of statistic. The counting names the
    // second mark, the earliest at the smallest margin, the
    // tie real in the printed lines at two marks whose
    // fields differ whole.
    let meet = load_meet(&[
        day(
            1000,
            "a",
            &format!(
                "[{}, {}, {}, {}]",
                run("a", 500, &[(100, 10), (400, 40)]),
                run("b", 400, &[(200, 28), (300, 36), (600, 78)]),
                run("c", 300, &[(200, 30), (300, 57), (600, 102)]),
                run("d", 250, &[(200, 52), (300, 120), (600, 162)])
            ),
        ),
        quiet_day(1000, "a", "a"),
        quiet_day(1000, "a", "a"),
    ]);
    let lines = meet_midrange(&meet).expect("the midrange read");
    assert_eq!(
        lines,
        vec![
            "meet midrange, 3 days",
            "day, the first day, marks 5",
            "mark 1, at 100 seconds on the first day, a at a projection of 100, and no other standing",
            "mark 2, at 200 seconds on the first day, a at a projection of 100, midrange standing 200, midrange margin 100",
            "mark 3, at 300 seconds on the first day, a at a projection of 100, midrange standing 260, midrange margin 160",
            "mark 4, at 400 seconds on the first day, a at a projection of 100, midrange standing 260, midrange margin 160",
            "mark 5, at 600 seconds on the first day, a at a projection of 100, midrange standing 200, midrange margin 100",
            "day, the second day, marks 0",
            "day, the third day, marks 0",
            "midrange counts, marks walked 5, marks with another standing 4, days 3, closest midrange margin 100 at 200 seconds on the first day",
        ]
    );
}

#[test]
fn test_the_midrange_steps_as_the_field_grows() {
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
    let lines = meet_midrange(&meet).expect("the midrange read");
    let marks = mark_lines(&lines);
    let standings: Vec<f64> = marks
        .iter()
        .map(|line| midrange_of(line).expect("a midrange stands").0)
        .collect();
    assert_eq!(standings, vec![150.0, 180.0, 195.0, 240.0, 285.0]);
    assert_eq!(
        marks[0].as_str(),
        "mark 1, at 100 seconds on the first day, a at a projection of 100, midrange standing 150, midrange margin 50"
    );
    assert_eq!(
        marks[4].as_str(),
        "mark 5, at 500 seconds on the first day, a at a projection of 100, midrange standing 285, midrange margin 185"
    );
}

#[test]
fn test_the_extreme_margin_identity_and_the_margin_chain_hold_line_by_line() {
    // The identity and the chain of PG-50 over the hand
    // built meet of pin 1. At every mark the midrange
    // margin is the exact mean of the margin of PG-30 and
    // the spread of PG-32, and the chain holds line by
    // line, the margin never more than the midrange margin
    // and the midrange margin never more than the spread,
    // the five midrange margins pinned at 50, 60, 45, 55,
    // and 70. This pin asserts both equalities of the
    // chain under the one condition, the spread standing
    // equal to the margin, the midrange margin equal to
    // the margin exactly where that condition holds, at
    // the first mark of this meet, the field there
    // counting one figure, and equal to the spread exactly
    // where the same condition holds, the two equalities
    // standing or falling together at every mark, checked
    // as biconditionals at every mark. PG-50 as first
    // stated carried the second equality under another
    // condition, the margin standing at zero there, and
    // that clause stood refuted in both directions by
    // fixtures inside this slice, at the first mark of
    // this meet the margin and the spread both standing
    // at 50 with the midrange margin equal to the spread
    // and the margin standing apart from zero, and at the
    // first four marks of pin 5 the margin standing at
    // zero with the midrange margin standing at 45
    // against a spread of 90. FIX-PG-20 restates the
    // clause to the condition this pin asserts. The
    // margin standing at zero at no mark of this meet,
    // where the three would stand equal whole, stands as
    // this meet's own posture and not as the condition of
    // either equality.
    let meet = midrange_meet();
    let midrange = meet_midrange(&meet).expect("the midrange read");
    let margins = meet_margins(&meet).expect("the margins read");
    let spread = meet_spread(&meet).expect("the spread read");
    let midrange_marks = mark_lines(&midrange);
    let margin_marks = mark_lines(&margins);
    let spread_marks = mark_lines(&spread);
    assert_eq!(midrange_marks.len(), margin_marks.len());
    assert_eq!(midrange_marks.len(), spread_marks.len());
    let expected_margins = [50.0, 60.0, 45.0, 55.0, 70.0];
    for (place, ((midrange_line, margin_line), spread_line)) in midrange_marks
        .iter()
        .zip(margin_marks.iter())
        .zip(spread_marks.iter())
        .enumerate()
    {
        let (_, midrange_margin) = midrange_of(midrange_line).expect("a midrange stands");
        let margin = margin_of(margin_line).expect("a margin stands");
        let spread = spread_of(spread_line).expect("a spread stands");
        assert_eq!(
            midrange_margin,
            expected_margins[place],
            "mark {}",
            place + 1
        );
        assert!(
            (midrange_margin - (margin + spread) / 2.0).abs() <= 0.000002,
            "mark {}",
            place + 1
        );
        assert!(margin <= midrange_margin + 0.000002, "mark {}", place + 1);
        assert!(midrange_margin <= spread + 0.000002, "mark {}", place + 1);
        assert_eq!(
            midrange_margin == margin,
            spread == margin,
            "mark {}",
            place + 1
        );
        assert_eq!(
            midrange_margin == spread,
            spread == margin,
            "mark {}",
            place + 1
        );
        assert!(midrange_margin >= 0.0, "mark {}", place + 1);
    }
}

#[test]
fn test_a_meet_in_which_no_read_projects_reads_with_marks_walked_zero() {
    let meet = load_meet(&[
        quiet_day(600, "a", "a"),
        quiet_day(600, "a", "a"),
        quiet_day(600, "a", "a"),
    ]);
    let lines = meet_midrange(&meet).expect("the midrange read");
    assert_eq!(
        lines,
        vec![
            "meet midrange, 3 days",
            "day, the first day, marks 0",
            "day, the second day, marks 0",
            "day, the third day, marks 0",
            "midrange counts, marks walked 0, marks with another standing 0, days 3, and, no other standing at any mark",
        ]
    );
}

// The guard pins. The midrange reading stands inside the
// fencing closure of FIX-PG-11 from its birth, and under
// the figure guard of FIX-PG-10 and the day count errors
// of PG-49, in the one walk it shares with the seventeen
// readings before it.

#[test]
fn test_a_hand_built_meet_with_no_days_draws_the_named_error_through_midrange() {
    let meet = Meet { days: vec![] };
    let error = meet_midrange(&meet).expect_err("the reading names the precondition");
    assert_eq!(error, "the meet holds no days");
}

#[test]
fn test_a_hand_built_meet_with_too_many_days_draws_the_named_error_through_midrange() {
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
    let error = meet_midrange(&meet).expect_err("the reading names the precondition");
    assert_eq!(
        error,
        "the meet holds more days than the ordinal table names"
    );
}

#[test]
fn test_a_hand_built_non_finite_figure_draws_the_named_error_through_midrange() {
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
    let error = meet_midrange(&meet).expect_err("the reading names the precondition");
    assert_eq!(error, "the window is not a finite number");
}

#[test]
fn test_a_hand_built_break_bearing_label_draws_the_named_error_through_midrange() {
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
    let error = meet_midrange(&meet).expect_err("the reading names the precondition");
    assert_eq!(error, "a runner label holds a line break");
}

// The exact mean pin. The midrange of PG-50 is the mean
// of the nearest standing and the farthest standing
// computed on exact values, and this pin states marks
// whose two extreme standings render alike at six decimal
// places while their exact mean does not stand at the
// record figure.

#[test]
fn test_the_midrange_is_the_exact_mean_of_extremes_that_render_alike() {
    // The field of the second mark counts one figure at
    // 1 + 2^-22, the standing of the one competitor behind
    // the holder, rendering as 1, its midrange standing at
    // that figure with a margin of 2^-22 rendered in full.
    // The field of the third mark counts two figures at
    // 1 + 2^-22 and 1 + 2^-21, both rendering as 1, its
    // midrange standing at 1 + 3 * 2^-23, rendering as 1,
    // with a margin of 3 * 2^-23 rendered in full. A
    // midrange computed by averaging the rendered extreme
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
    let lines = meet_midrange(&meet).expect("the midrange read");
    assert_eq!(
        lines,
        vec![
            "meet midrange, 3 days",
            "day, the first day, marks 2",
            "mark 1, at 4194304 seconds on the first day, b at a projection of 1, and no other standing",
            "mark 2, at 4194305 seconds on the first day, a at a projection of 1, midrange standing 1, midrange margin 0.0000002384185791015625",
            "day, the second day, marks 1",
            "mark 3, at 2097152 seconds on the second day, a at a projection of 1, midrange standing 1, midrange margin 0.00000035762786865234375",
            "day, the third day, marks 0",
            "midrange counts, marks walked 3, marks with another standing 2, days 3, closest midrange margin 0.0000002384185791015625 at 4194305 seconds on the first day",
        ]
    );
}
