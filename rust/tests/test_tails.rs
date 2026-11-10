//! Tests for PG-35 and PG-36, the tails reading over a meet.
//!
//! Written for Round 17. Each pin's failure mode stands
//! stated here pin by pin, not as one blanket claim. Pins 1
//! to 15 fail before the round by the absence of the reading,
//! the meet_tails function not standing on the pre round
//! tree. On the built tree, pin 1 fails under T-1 at every
//! figured mark, under T-2 at its odd field marks, the field
//! of three and the field of five losing the shared middle
//! figure from both tails, under T-3 at every figured mark,
//! its tails separating from the first figured mark on,
//! under T-4 at its separating marks, and under T-6 at every
//! mark after the first, that last stated as a claim until
//! the round executes it. Pin 2 fails under T-6 wherever a
//! mark moves the record, stated as a claim until executed.
//! Pin 3 fails under T-1 at every closing mark, under T-2 at
//! the closing marks whose fields stand odd, its closes
//! carrying fields of three and five behind, and under T-3
//! at the closing marks whose tails separate. Pin 4 fails
//! under T-1, a field taken whole figuring the holder
//! itself. Pin 5 fails under T-1 at the mark whose lower
//! half stands partly at the record, the holder's own record
//! figure completing that half at the record and reading the
//! lower tail margin as zero against the pin's positive
//! figure, and under T-4 at the same mark, where the lower
//! tail margin stands below the middle margin. Pin 6 fails
//! under T-2 at once, the lower tail over one figure reading
//! the nearest figure, under T-3, the lower tail margin
//! reading the upper tail's, and under T-4, the lower tail
//! margin reading the middle margin it must stand below.
//! Pin 7 fails under T-2 at once, the upper tail over one
//! figure reading the farthest figure, under T-3, the upper
//! tail margin reading the lower tail's, and under T-4, the
//! upper tail margin reading the middle margin it must stand
//! above. Pin 8 fails under T-5 while both closing figures
//! stand. Pin 9 fails under T-1 at every step and under T-2
//! at the one behind mark, whose line unfigures, and at the
//! three behind mark, whose tails lose the shared middle
//! figure. Pin 10 fails under T-3 at its separating marks,
//! the chain inverting around the middle, and under T-4 at
//! its separating marks, the chain's tail links collapsing
//! onto the middle margin. Pin 11 pins the no read form of
//! the counts line. Pins 12 to 15 fail on a built tree whose
//! reading skipped the FIX-PG-10 figure guard, the FIX-PG-11
//! label fencing, or the day count preconditions of PG-35.

use paceghost::fieldday::{DayRun, FieldDay};
use paceghost::meet::{
    load_meet_text, meet_margins, meet_marks, meet_middle, meet_spread, meet_standings, meet_tails,
    Meet,
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

/// The parts of one tails mark line the pins read, its place
/// in the walk, its seconds, its day, its holder, and its
/// record figure, and where another standing stands, the
/// lower tail figure and margin and the upper tail figure
/// and margin. The parse walks the whole line, so a line
/// malformed at any part of it fails the parse. No tail
/// names a holder, so no label stands in the figured parts.
struct TailsMark {
    place: usize,
    seconds: String,
    day: String,
    holder: String,
    record: String,
    tails: Option<(String, String, String, String)>,
}

fn tails_mark_parts(line: &str) -> TailsMark {
    let rest = line
        .strip_prefix("mark ")
        .expect("a tails mark line opens with its place");
    let (place, rest) = rest
        .split_once(", at ")
        .expect("a tails mark line names its mark");
    let (seconds, rest) = rest
        .split_once(" seconds on the ")
        .expect("a tails mark line names its seconds");
    let (day, rest) = rest
        .split_once(" day, ")
        .expect("a tails mark line names its day");
    let (holder, rest) = rest
        .split_once(" at a projection of ")
        .expect("a tails mark line names its holder");
    if let Some(record) = rest.strip_suffix(", and no other standing") {
        return TailsMark {
            place: place.parse().expect("a place parses"),
            seconds: seconds.to_string(),
            day: day.to_string(),
            holder: holder.to_string(),
            record: record.to_string(),
            tails: None,
        };
    }
    let (record, rest) = rest
        .split_once(", lower tail standing ")
        .expect("a figured tails mark line names its lower tail standing");
    let (lower_figure, rest) = rest
        .split_once(", lower tail margin ")
        .expect("a lower tail standing names its margin");
    let (lower_margin, rest) = rest
        .split_once(", upper tail standing ")
        .expect("a figured tails mark line names its upper tail standing");
    let (upper_figure, upper_margin) = rest
        .split_once(", upper tail margin ")
        .expect("an upper tail standing names its margin");
    TailsMark {
        place: place.parse().expect("a place parses"),
        seconds: seconds.to_string(),
        day: day.to_string(),
        holder: holder.to_string(),
        record: record.to_string(),
        tails: Some((
            lower_figure.to_string(),
            lower_margin.to_string(),
            upper_figure.to_string(),
            upper_margin.to_string(),
        )),
    }
}

/// A meet over six competitors where the holder stands alone
/// at the first mark, the field behind the holder grows to
/// two at distinct figures at the second mark and to four on
/// the first day and to five on the third, the second day
/// holds no marks at all, and the holder's own later read
/// re-prices the record on the third day while the field
/// behind it stands unmoved.
fn tails_meet() -> Meet {
    load_meet(&[
        day(
            1000,
            "a",
            &format!(
                "[{}, {}, {}, {}, {}]",
                run("a", 500, &[(100, 10)]),
                run("b", 400, &[(200, 66)]),
                run("c", 300, &[(200, 80)]),
                run("d", 250, &[(300, 150)]),
                run("e", 200, &[(400, 264)])
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
                run("f", 600, &[(100, 72)])
            ),
        ),
    ])
}

#[test]
fn test_the_tails_reading_over_a_hand_built_meet_line_by_line() {
    let lines = meet_tails(&tails_meet()).expect("the tails read");
    assert_eq!(
        lines,
        vec![
            "meet tails, 3 days",
            "day, the first day, marks 4",
            "mark 1, at 100 seconds on the first day, a at a projection of 100, and no other standing",
            "mark 2, at 200 seconds on the first day, a at a projection of 100, lower tail standing 330, lower tail margin 230, upper tail standing 400, upper tail margin 300",
            "mark 3, at 300 seconds on the first day, a at a projection of 100, lower tail standing 365, lower tail margin 265, upper tail standing 450, upper tail margin 350",
            "mark 4, at 400 seconds on the first day, a at a projection of 100, lower tail standing 365, lower tail margin 265, upper tail standing 580, upper tail margin 480",
            "day, the second day, marks 0",
            "day, the third day, marks 2",
            "mark 5, at 100 seconds on the third day, a at a projection of 100, lower tail standing 410, lower tail margin 310, upper tail standing 626.666667, upper tail margin 526.666667",
            "mark 6, at 200 seconds on the third day, a at a projection of 120, lower tail standing 410, lower tail margin 290, upper tail standing 626.666667, upper tail margin 506.666667",
            "tails counts, marks walked 6, marks with another standing 5, days 3, closest lower tail margin 230 at 200 seconds on the first day, closest upper tail margin 300 at 200 seconds on the first day",
        ]
    );
}

// The relation pins of PG-36. The tails reading agrees
// with the marks reading on the record side of every
// mark, line by line, and at the last mark of each day
// holding marks its tail figures are the tail figures
// the standings reading's own figures compute at that
// day's close.

#[test]
fn test_tails_mark_lines_agree_with_the_marks_reading_line_by_line() {
    let meet = tails_meet();
    let tails = meet_tails(&meet).expect("the tails read");
    let marks = meet_marks(&meet).expect("the marks read");
    assert_eq!(tails[0], "meet tails, 3 days");
    assert_eq!(marks[0], "meet marks, 3 days");
    let tails_sections: Vec<&String> = tails
        .iter()
        .filter(|line| line.starts_with("day, the "))
        .collect();
    let mark_sections: Vec<&String> = marks
        .iter()
        .filter(|line| line.starts_with("day, the "))
        .collect();
    assert_eq!(tails_sections, mark_sections);
    let tails_marks = mark_lines(&tails);
    let reading_marks = mark_lines(&marks);
    assert_eq!(tails_marks.len(), reading_marks.len());
    for (tails_line, mark_line) in tails_marks.iter().zip(reading_marks.iter()) {
        let parts = tails_mark_parts(tails_line);
        let rebuilt = format!(
            "mark {}, at {} seconds on the {} day, {} at a projection of {}",
            parts.place, parts.seconds, parts.day, parts.holder, parts.record
        );
        assert_eq!(rebuilt, record_prefix(mark_line));
    }
    let places: Vec<usize> = tails_marks
        .iter()
        .map(|line| tails_mark_parts(line).place)
        .collect();
    assert_eq!(places, (1..=tails_marks.len()).collect::<Vec<usize>>());
    let walked = count_after(tails.last().expect("the tails close"), "marks walked ");
    let reading_walked = count_after(marks.last().expect("the marks close"), "marks walked ");
    assert_eq!(walked, reading_walked);
    assert_eq!(walked, tails_marks.len());
}

#[test]
fn test_the_last_mark_of_a_day_agrees_with_the_standings_reading() {
    let meet = load_meet(&[
        day(
            1000,
            "a",
            &format!(
                "[{}, {}, {}, {}]",
                run("a", 500, &[(100, 10)]),
                run("b", 400, &[(200, 66)]),
                run("c", 300, &[(300, 120)]),
                run("d", 250, &[(400, 200)])
            ),
        ),
        day(
            1000,
            "a",
            &format!(
                "[{}, {}, {}, {}, {}, {}]",
                run("a", 500, &[]),
                run("b", 400, &[]),
                run("c", 300, &[]),
                run("d", 250, &[]),
                run("e", 200, &[(100, 60)]),
                run("f", 600, &[(200, 132)])
            ),
        ),
        quiet_day(1000, "a", "a"),
    ]);
    let tails = meet_tails(&meet).expect("the tails read");
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
    let mut tails_sections: Vec<Vec<&String>> = Vec::new();
    for line in &tails {
        if line.starts_with("day, the ") {
            tails_sections.push(Vec::new());
        }
        if let Some(section) = tails_sections.last_mut() {
            section.push(line);
        }
    }
    assert_eq!(sections.len(), tails_sections.len());
    for (day_index, (section, tails_section)) in
        sections.iter().zip(tails_sections.iter()).enumerate()
    {
        let day_marks: Vec<&&String> = tails_section
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
        others.sort_by(|a, b| a.partial_cmp(b).expect("figures order"));
        let last = day_marks.last().expect("the day holds a mark");
        let parts = tails_mark_parts(last);
        match (others.is_empty(), parts.tails) {
            (true, None) => {}
            (false, Some((lower_figure, lower_margin, upper_figure, upper_margin))) => {
                let count = others.len().div_ceil(2);
                let lower_mean = others[..count].iter().sum::<f64>() / count as f64;
                let upper_mean = others[others.len() - count..].iter().sum::<f64>() / count as f64;
                let stated_lower: f64 = lower_figure.parse().expect("a tail figure parses");
                let stated_upper: f64 = upper_figure.parse().expect("a tail figure parses");
                assert!(
                    (stated_lower - lower_mean).abs() <= 0.000002,
                    "day {} last mark {}",
                    day_index,
                    last
                );
                assert!(
                    (stated_upper - upper_mean).abs() <= 0.000002,
                    "day {} last mark {}",
                    day_index,
                    last
                );
                let stated_lower_margin: f64 = lower_margin.parse().expect("a tail margin parses");
                let stated_upper_margin: f64 = upper_margin.parse().expect("a tail margin parses");
                assert!(
                    (stated_lower - record - stated_lower_margin).abs() <= 0.000002,
                    "day {} last mark {}",
                    day_index,
                    last
                );
                assert!(
                    (stated_upper - record - stated_upper_margin).abs() <= 0.000002,
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
    let lines = meet_tails(&meet).expect("the tails read");
    assert_eq!(
        lines,
        vec![
            "meet tails, 3 days",
            "day, the first day, marks 1",
            "mark 1, at 100 seconds on the first day, solo at a projection of 100, and no other standing",
            "day, the second day, marks 1",
            "mark 2, at 200 seconds on the second day, solo at a projection of 150, and no other standing",
            "day, the third day, marks 0",
            "tails counts, marks walked 2, marks with another standing 0, days 3, and, no other standing at any mark",
        ]
    );
}

#[test]
fn test_the_tail_margins_stand_at_zero_only_where_the_whole_half_stands_at_the_record() {
    let meet = load_meet(&[
        day(
            1000,
            "a",
            &format!(
                "[{}, {}, {}, {}]",
                run("a", 500, &[(100, 10)]),
                run("b", 400, &[(200, 20), (300, 30)]),
                run("c", 300, &[(200, 60), (600, 60)]),
                run("d", 250, &[(200, 80), (800, 80)])
            ),
        ),
        quiet_day(1000, "a", "a"),
        quiet_day(1000, "a", "a"),
    ]);
    let lines = meet_tails(&meet).expect("the tails read");
    assert_eq!(
        lines,
        vec![
            "meet tails, 3 days",
            "day, the first day, marks 5",
            "mark 1, at 100 seconds on the first day, a at a projection of 100, and no other standing",
            "mark 2, at 200 seconds on the first day, a at a projection of 100, lower tail standing 200, lower tail margin 100, upper tail standing 350, upper tail margin 250",
            "mark 3, at 300 seconds on the first day, a at a projection of 100, lower tail standing 200, lower tail margin 100, upper tail standing 350, upper tail margin 250",
            "mark 4, at 600 seconds on the first day, a at a projection of 100, lower tail standing 100, lower tail margin 0, upper tail standing 250, upper tail margin 150",
            "mark 5, at 800 seconds on the first day, a at a projection of 100, lower tail standing 100, lower tail margin 0, upper tail standing 100, upper tail margin 0",
            "day, the second day, marks 0",
            "day, the third day, marks 0",
            "tails counts, marks walked 5, marks with another standing 4, days 3, closest lower tail margin 0 at 600 seconds on the first day, closest upper tail margin 0 at 800 seconds on the first day",
        ]
    );
    // At the second mark the lowest figure of the field
    // stands at the record figure while the whole lower
    // half does not, and the lower tail margin stands
    // positive above zero there; at the fourth mark the
    // lower half stands wholly at the record while the
    // upper half does not; at the fifth mark every figure
    // of the field stands at the record figure and both
    // tail margins stand at zero. The lower tail margin
    // stands at zero at exactly the fourth and fifth
    // marks and the upper tail margin at exactly the
    // fifth, and no other mark states a zero tail margin.
    let second = tails_mark_parts(
        lines
            .iter()
            .find(|line| line.starts_with("mark 2,"))
            .expect("the second mark stands"),
    );
    let (lower_figure, lower_margin, _, _) = second.tails.expect("the second mark is figured");
    assert_eq!(lower_figure, "200");
    assert_eq!(lower_margin, "100");
    let fifth = tails_mark_parts(
        lines
            .iter()
            .find(|line| line.starts_with("mark 5,"))
            .expect("the fifth mark stands"),
    );
    let (lower_figure, lower_margin, upper_figure, upper_margin) =
        fifth.tails.expect("the fifth mark is figured");
    assert_eq!(lower_figure, fifth.record);
    assert_eq!(lower_margin, "0");
    assert_eq!(upper_figure, fifth.record);
    assert_eq!(upper_margin, "0");
    let zero_lower: Vec<&String> = lines
        .iter()
        .filter(|line| line.starts_with("mark "))
        .filter(|line| {
            tails_mark_parts(line)
                .tails
                .map(|(_, margin, _, _)| margin == "0")
                .unwrap_or(false)
        })
        .collect();
    assert_eq!(zero_lower.len(), 2);
    let zero_upper: Vec<&String> = lines
        .iter()
        .filter(|line| line.starts_with("mark "))
        .filter(|line| {
            tails_mark_parts(line)
                .tails
                .map(|(_, _, _, margin)| margin == "0")
                .unwrap_or(false)
        })
        .collect();
    assert_eq!(zero_upper.len(), 1);
}

/// A meet whose fourth mark carries three competitors
/// behind the holder at distinct figures and whose fifth
/// carries four, so both tails separate from every standing
/// statistic at those marks.
fn separation_meet() -> Meet {
    load_meet(&[
        day(
            1000,
            "a",
            &format!(
                "[{}, {}, {}, {}, {}]",
                run("a", 500, &[(100, 10)]),
                run("b", 400, &[(200, 60)]),
                run("c", 300, &[(300, 135)]),
                run("d", 250, &[(400, 240)]),
                run("e", 200, &[(500, 350)])
            ),
        ),
        quiet_day(1000, "a", "a"),
        quiet_day(1000, "a", "a"),
    ])
}

fn margin_value(line: &str, phrase: &str) -> f64 {
    line.rsplit(phrase)
        .next()
        .and_then(|word| word.parse().ok())
        .unwrap_or_else(|| panic!("the line names its figure after {}, {}", phrase, line))
}

#[test]
fn test_the_lower_tail_separates_from_the_nearest_and_the_middle() {
    let meet = separation_meet();
    let tails = meet_tails(&meet).expect("the tails read");
    let margins = meet_margins(&meet).expect("the margins read");
    let middle = meet_middle(&meet).expect("the middle read");
    let tails_line = tails
        .iter()
        .find(|line| line.starts_with("mark 4,"))
        .expect("the fourth mark stands");
    assert_eq!(
        tails_line,
        "mark 4, at 400 seconds on the first day, a at a projection of 100, lower tail standing 375, lower tail margin 275, upper tail standing 525, upper tail margin 425"
    );
    let margins_line = margins
        .iter()
        .find(|line| line.starts_with("mark 4,"))
        .expect("the fourth mark stands");
    let middle_line = middle
        .iter()
        .find(|line| line.starts_with("mark 4,"))
        .expect("the fourth mark stands");
    let margin = margin_value(margins_line, ", margin ");
    let middle_margin = margin_value(middle_line, ", middle margin ");
    let parts = tails_mark_parts(tails_line);
    let (_, lower_margin, _, _) = parts.tails.expect("the fourth mark is figured");
    let lower_margin: f64 = lower_margin.parse().expect("a tail margin parses");
    assert!(
        margin < lower_margin && lower_margin < middle_margin,
        "margin {} below lower tail margin {} below middle margin {}",
        margin,
        lower_margin,
        middle_margin
    );
}

#[test]
fn test_the_upper_tail_separates_from_the_middle_and_the_farthest() {
    let meet = separation_meet();
    let tails = meet_tails(&meet).expect("the tails read");
    let middle = meet_middle(&meet).expect("the middle read");
    let spread = meet_spread(&meet).expect("the spread read");
    let tails_line = tails
        .iter()
        .find(|line| line.starts_with("mark 4,"))
        .expect("the fourth mark stands");
    let middle_line = middle
        .iter()
        .find(|line| line.starts_with("mark 4,"))
        .expect("the fourth mark stands");
    let spread_line = spread
        .iter()
        .find(|line| line.starts_with("mark 4,"))
        .expect("the fourth mark stands");
    let middle_margin = margin_value(middle_line, ", middle margin ");
    let spread_value = margin_value(spread_line, ", spread ");
    let parts = tails_mark_parts(tails_line);
    let (_, _, _, upper_margin) = parts.tails.expect("the fourth mark is figured");
    let upper_margin: f64 = upper_margin.parse().expect("a tail margin parses");
    assert!(
        middle_margin < upper_margin && upper_margin < spread_value,
        "middle margin {} below upper tail margin {} below spread {}",
        middle_margin,
        upper_margin,
        spread_value
    );
}

#[test]
fn test_the_closest_tail_margins_tie_names_the_earliest_marks() {
    let meet = load_meet(&[
        day(
            1000,
            "a",
            &format!(
                "[{}, {}, {}, {}]",
                run("a", 500, &[(100, 10)]),
                run("b", 400, &[(200, 40), (300, 60)]),
                run("c", 300, &[(200, 180), (400, 200)]),
                run("d", 250, &[(400, 140)])
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
                run("e", 200, &[(200, 70)])
            ),
        ),
        quiet_day(1000, "a", "a"),
    ]);
    let lines = meet_tails(&meet).expect("the tails read");
    let third = lines
        .iter()
        .find(|line| line.starts_with("mark 3,"))
        .expect("the third mark stands");
    let third_parts = tails_mark_parts(third);
    let (_, third_lower, _, _) = third_parts.tails.expect("the third mark is figured");
    assert_eq!(
        third_lower, "100",
        "the lower tie is real at the third mark"
    );
    let fifth = lines
        .iter()
        .find(|line| line.starts_with("mark 5,"))
        .expect("the fifth mark stands");
    let fifth_parts = tails_mark_parts(fifth);
    let (_, _, _, fifth_upper) = fifth_parts.tails.expect("the fifth mark is figured");
    assert_eq!(
        fifth_upper, "325",
        "the upper tie is real at the fifth mark"
    );
    assert_eq!(
        lines.last().map(String::as_str),
        Some("tails counts, marks walked 5, marks with another standing 4, days 3, closest lower tail margin 100 at 200 seconds on the first day, closest upper tail margin 325 at 400 seconds on the first day")
    );
}

#[test]
fn test_the_tails_step_as_the_field_grows() {
    let meet = load_meet(&[
        day(
            1000,
            "a",
            &format!(
                "[{}, {}, {}, {}]",
                run("a", 500, &[(100, 10)]),
                run("b", 400, &[(200, 60)]),
                run("c", 300, &[(300, 150)]),
                run("d", 250, &[(400, 280)])
            ),
        ),
        quiet_day(1000, "a", "a"),
        quiet_day(1000, "a", "a"),
    ]);
    let lines = meet_tails(&meet).expect("the tails read");
    let figured: Vec<(String, String)> = mark_lines(&lines)
        .iter()
        .filter_map(|line| tails_mark_parts(line).tails)
        .map(|(lower, _, upper, _)| (lower, upper))
        .collect();
    assert_eq!(
        figured,
        vec![
            ("300".to_string(), "300".to_string()),
            ("300".to_string(), "500".to_string()),
            ("400".to_string(), "600".to_string()),
        ]
    );
    let second = lines
        .iter()
        .find(|line| line.starts_with("mark 2,"))
        .expect("the second mark stands");
    assert_eq!(
        second,
        "mark 2, at 200 seconds on the first day, a at a projection of 100, lower tail standing 300, lower tail margin 200, upper tail standing 300, upper tail margin 200"
    );
    let fourth = lines
        .iter()
        .find(|line| line.starts_with("mark 4,"))
        .expect("the fourth mark stands");
    assert_eq!(
        fourth,
        "mark 4, at 400 seconds on the first day, a at a projection of 100, lower tail standing 400, lower tail margin 300, upper tail standing 600, upper tail margin 500"
    );
}

#[test]
fn test_the_bracketing_chain_holds_line_by_line() {
    let meet = separation_meet();
    let tails = meet_tails(&meet).expect("the tails read");
    let margins = meet_margins(&meet).expect("the margins read");
    let middle = meet_middle(&meet).expect("the middle read");
    let spread = meet_spread(&meet).expect("the spread read");
    let tails_marks = mark_lines(&tails);
    let margin_marks = mark_lines(&margins);
    let middle_marks = mark_lines(&middle);
    let spread_marks = mark_lines(&spread);
    assert_eq!(tails_marks.len(), margin_marks.len());
    assert_eq!(tails_marks.len(), middle_marks.len());
    assert_eq!(tails_marks.len(), spread_marks.len());
    let mut strict_marks = 0usize;
    for (((tails_line, margin_line), middle_line), spread_line) in tails_marks
        .iter()
        .zip(margin_marks.iter())
        .zip(middle_marks.iter())
        .zip(spread_marks.iter())
    {
        let parts = tails_mark_parts(tails_line);
        let Some((_, lower_margin, _, upper_margin)) = parts.tails else {
            continue;
        };
        let lower_margin: f64 = lower_margin.parse().expect("a tail margin parses");
        let upper_margin: f64 = upper_margin.parse().expect("a tail margin parses");
        let margin = margin_value(margin_line, ", margin ");
        let middle_margin = margin_value(middle_line, ", middle margin ");
        let spread_value = margin_value(spread_line, ", spread ");
        assert!(
            margin <= lower_margin
                && lower_margin <= middle_margin
                && middle_margin <= upper_margin
                && upper_margin <= spread_value,
            "the chain at {}",
            tails_line
        );
        if margin < lower_margin
            && lower_margin < middle_margin
            && middle_margin < upper_margin
            && upper_margin < spread_value
        {
            strict_marks += 1;
        }
    }
    assert_eq!(
        strict_marks, 2,
        "the chain stands strict at the two separating marks"
    );
}

#[test]
fn test_a_meet_in_which_no_read_projects_reads_with_marks_walked_zero() {
    let meet = load_meet(&[
        quiet_day(600, "solo", "solo"),
        quiet_day(600, "solo", "solo"),
        quiet_day(600, "solo", "solo"),
    ]);
    let lines = meet_tails(&meet).expect("the tails read");
    assert_eq!(
        lines,
        vec![
            "meet tails, 3 days",
            "day, the first day, marks 0",
            "day, the second day, marks 0",
            "day, the third day, marks 0",
            "tails counts, marks walked 0, marks with another standing 0, days 3, and, no other standing at any mark",
        ]
    );
}

// The guard pins. The tails reading stands inside the
// fencing closure of FIX-PG-11 from its birth, and under
// the figure guard of FIX-PG-10 and the day count errors
// of PG-35, in the one walk it shares with the eleven
// readings before it.

#[test]
fn test_a_hand_built_non_finite_figure_draws_the_named_error_through_tails() {
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
    let error = meet_tails(&meet).expect_err("the reading names the precondition");
    assert_eq!(error, "the window is not a finite number");
}

#[test]
fn test_a_hand_built_break_bearing_label_draws_the_named_error_through_tails() {
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
    let error = meet_tails(&meet).expect_err("the reading names the precondition");
    assert_eq!(error, "a runner label holds a line break");
}

#[test]
fn test_a_hand_built_meet_with_no_days_draws_the_named_error_through_tails() {
    let meet = Meet { days: vec![] };
    let error = meet_tails(&meet).expect_err("the reading names the precondition");
    assert_eq!(error, "the meet holds no days");
}

#[test]
fn test_a_hand_built_meet_with_too_many_days_draws_the_named_error_through_tails() {
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
    let error = meet_tails(&meet).expect_err("the reading names the precondition");
    assert_eq!(
        error,
        "the meet holds more days than the ordinal table names"
    );
}
