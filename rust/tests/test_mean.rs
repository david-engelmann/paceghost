//! Tests for PG-37 and PG-38, the mean reading over a meet.
//!
//! Written for Round 18. Each pin's failure mode stands
//! stated here pin by pin, not as one blanket claim. Pins 1
//! to 15 fail before the round by the absence of the reading,
//! the meet_mean function not standing on the pre round
//! tree. On the built tree, pin 1 fails under N-1, under
//! N-2 at its fields of three and more, its field of one
//! coinciding by construction, under N-3 at its leaning
//! marks, where the middle figure and the mean differ,
//! under N-5, and under N-6, and it passes under N-4 by
//! design, reading mark lines and never the closings. Pin 2
//! passes under all six mutations by design and claims no
//! mutation failure: under N-5, the pre mark mutation
//! standing on the mean vector, the mark line prefix it
//! compares stands rendered from the walk's mark records,
//! which no mutation of this round touches, the lag living
//! only in the mean figures behind the prefix. Pin 3 fails
//! under N-1, N-5, and N-6 at every close, under N-2 at its
//! closes of three and five, passing at its close of one
//! by design, and under N-3 at the closes where the middle
//! figure and the mean differ, which are its closes of
//! three and five as built, and it passes under N-4 by
//! design. Pin 4 fails under N-1 and under N-6 by design,
//! the holder's own figure entering the field at every
//! mark so a mean standing stands where the pin states
//! none, the asymmetry against pin 4 of the tails file
//! stated in the design, whose own N-1 analogue stands
//! inside its selection alone, and it passes under N-2,
//! N-3, N-4, and N-5 by design. Pin 5 fails under N-2, its
//! designed catcher, under N-3 the same way, and under
//! N-5, and under N-1 and N-6, where the holder figure
//! admission the design's coincidence cell weighed at the
//! figured marks also surfaces a figured first mark, the
//! holder standing alone there, so the zero set the pin
//! counts gains the first mark, the coincidence reasoning
//! itself standing at the marks already figured, where no
//! margin moves across zero, and it passes under N-4 by
//! design, reading margins at marks and never the
//! closest naming. Pin 6 fails under N-2 and N-3, its
//! designed catchers, and under N-5, and it passes under
//! N-1, the holder figure admission moving its mean
//! margin only to 225, still above both bounds it
//! asserts, under N-4 by design, and under N-6, its
//! bounds standing read from the middle and tails
//! readings, which share the one collection N-6 mutates
//! and move with the mean. Pin 7 fails under N-3, its
//! designed catcher, and under N-6, and it passes under
//! N-1 and N-5, the holder figure admission and the pre
//! mark lag each moving the printed figure toward the
//! record, away from the upper bounds it asserts, under
//! N-2 by design, the lower substitution moving the
//! printed figure downward, away from every bound the
//! pin asserts, and under N-4 by design. Pin 8
//! fails under N-1, under N-2, the substituted figures
//! standing at 125 and 120 at its tied marks so the
//! counted margin moves to the later mark, under N-3, the
//! middle margins at its tied marks standing at 40 and
//! 30 so the counting names the later mark, under N-4,
//! its designed catcher, and under N-5 and N-6. Pin 9
//! fails under N-1, under N-2 at its fields of three and
//! four, its fields of one and two standing as computed
//! under both constructions, under N-3, its two byte
//! exact lines carrying the mean margin text the middle
//! substitution rewrites, under N-5, and under N-6, and
//! it passes under N-4 by design, reading no closest
//! naming. Pin 10 fails under N-1 and
//! N-6, under N-2 at the marks separating the lower link,
//! and under N-5, and it passes under N-3, the chain
//! reading the printed mean margin itself, the middle
//! margin the substitution prints standing always inside
//! the bracket the chain asserts, and under N-4 by
//! design, reading mark lines alone. Pin 11 passes
//! under all six
//! mutations by design, no mark standing for any
//! selection, margin, vector, or closing scan to move.
//! Pins 12 to 15 pass under all six mutations by design,
//! the guards standing ahead of any selection or margin
//! computation, and they fail on a built tree whose
//! reading skipped the FIX-PG-10 figure guard, the
//! FIX-PG-11 label fencing, or the day count
//! preconditions of PG-37.

use paceghost::fieldday::{DayRun, FieldDay};
use paceghost::meet::{
    load_meet_text, meet_margins, meet_marks, meet_mean, meet_middle, meet_spread, meet_standings,
    meet_tails, Meet,
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

/// The parts of one mean mark line the pins read, its place
/// in the walk, its seconds, its day, its holder, and its
/// record figure, and where another standing stands, the
/// mean figure and the mean margin. The parse walks the
/// whole line, so a line malformed at any part of it fails
/// the parse. The mean names no holder, so no label stands
/// in the figured parts.
struct MeanMark {
    place: usize,
    seconds: String,
    day: String,
    holder: String,
    record: String,
    mean: Option<(String, String)>,
}

fn mean_mark_parts(line: &str) -> MeanMark {
    let rest = line
        .strip_prefix("mark ")
        .expect("a mean mark line opens with its place");
    let (place, rest) = rest
        .split_once(", at ")
        .expect("a mean mark line names its mark");
    let (seconds, rest) = rest
        .split_once(" seconds on the ")
        .expect("a mean mark line names its seconds");
    let (day, rest) = rest
        .split_once(" day, ")
        .expect("a mean mark line names its day");
    let (holder, rest) = rest
        .split_once(" at a projection of ")
        .expect("a mean mark line names its holder");
    if let Some(record) = rest.strip_suffix(", and no other standing") {
        return MeanMark {
            place: place.parse().expect("a place parses"),
            seconds: seconds.to_string(),
            day: day.to_string(),
            holder: holder.to_string(),
            record: record.to_string(),
            mean: None,
        };
    }
    let (record, rest) = rest
        .split_once(", mean standing ")
        .expect("a figured mean mark line names its mean standing");
    let (figure, margin) = rest
        .split_once(", mean margin ")
        .expect("a mean standing names its margin");
    MeanMark {
        place: place.parse().expect("a place parses"),
        seconds: seconds.to_string(),
        day: day.to_string(),
        holder: holder.to_string(),
        record: record.to_string(),
        mean: Some((figure.to_string(), margin.to_string())),
    }
}

/// The mean figure a mean mark line states, where the line
/// carries another standing.
fn mean_figure_of(line: &str) -> Option<String> {
    mean_mark_parts(line).mean.map(|(figure, _)| figure)
}

/// The mean margin a mean mark line states, where the line
/// carries another standing.
fn mean_margin_of(line: &str) -> Option<f64> {
    mean_mark_parts(line)
        .mean
        .and_then(|(_, margin)| margin.parse().ok())
}

/// The middle margin a middle mark line states, where the
/// line carries another standing.
fn middle_margin_of(line: &str) -> Option<f64> {
    line.split(", middle margin ")
        .nth(1)
        .and_then(|word| word.parse().ok())
}

/// The tail margins a tails mark line states, its lower
/// tail margin and its upper tail margin, where the line
/// carries another standing.
fn tail_margins_of(line: &str) -> Option<(f64, f64)> {
    let lower = line
        .split(", lower tail margin ")
        .nth(1)
        .and_then(|rest| rest.split(", upper tail standing ").next())
        .and_then(|word| word.parse().ok())?;
    let upper = line
        .split(", upper tail margin ")
        .nth(1)
        .and_then(|word| word.parse().ok())?;
    Some((lower, upper))
}

/// The margin a margins mark line states, where the line
/// carries another standing.
fn margin_of(line: &str) -> Option<f64> {
    line.split(", margin ")
        .nth(1)
        .and_then(|word| word.parse().ok())
}

/// The spread a spread mark line states, where the line
/// carries another standing.
fn spread_of(line: &str) -> Option<f64> {
    line.rsplit(", spread ")
        .next()
        .and_then(|word| word.parse().ok())
}

/// A meet over six competitors where the holder stands alone
/// at the first mark, one competitor stands behind the
/// holder at the second mark, three stand behind at the
/// third, the second day holds no marks at all, and the
/// field behind the holder grows to four and then to five
/// on the third day, every figure distinct.
fn mean_meet() -> Meet {
    load_meet(&[
        day(
            1000,
            "a",
            &format!(
                "[{}, {}, {}, {}]",
                run("a", 500, &[(100, 10)]),
                run("b", 400, &[(200, 66)]),
                run("c", 300, &[(300, 120)]),
                run("d", 250, &[(300, 165)])
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
                run("d", 250, &[]),
                run("e", 200, &[(100, 60)]),
                run("f", 600, &[(200, 132)])
            ),
        ),
    ])
}

#[test]
fn test_the_mean_reading_over_a_hand_built_meet_line_by_line() {
    // The means by hand. The second mark's field stands at
    // 330 alone, its mean the figure whole, 330, and its
    // margin 230. The third mark's field stands at 330,
    // 400, and 550, its mean 1280 over 3, and its margin
    // 980 over 3. The fourth mark's field stands at 330,
    // 400, 550, and 600, its mean 470, and the fifth's at
    // 330, 400, 550, 600, and 660, its mean 508. The day
    // closes stand at fields of three and five behind.
    let lines = meet_mean(&mean_meet()).expect("the mean read");
    assert_eq!(
        lines,
        vec![
            "meet mean, 3 days",
            "day, the first day, marks 3",
            "mark 1, at 100 seconds on the first day, a at a projection of 100, and no other standing",
            "mark 2, at 200 seconds on the first day, a at a projection of 100, mean standing 330, mean margin 230",
            "mark 3, at 300 seconds on the first day, a at a projection of 100, mean standing 426.666667, mean margin 326.666667",
            "day, the second day, marks 0",
            "day, the third day, marks 2",
            "mark 4, at 100 seconds on the third day, a at a projection of 100, mean standing 470, mean margin 370",
            "mark 5, at 200 seconds on the third day, a at a projection of 100, mean standing 508, mean margin 408",
            "mean counts, marks walked 5, marks with another standing 4, days 3, closest mean margin 230 at 200 seconds on the first day",
        ]
    );
}

// The relation pins of PG-38. The mean reading agrees
// with the marks reading on the record side of every
// mark, line by line, and at the last mark of each day
// holding marks its mean figure is the mean the
// standings reading's own figures compute at that
// day's close.

#[test]
fn test_mean_mark_lines_agree_with_the_marks_reading_line_by_line() {
    let meet = mean_meet();
    let mean = meet_mean(&meet).expect("the mean read");
    let marks = meet_marks(&meet).expect("the marks read");
    assert_eq!(mean[0], "meet mean, 3 days");
    assert_eq!(marks[0], "meet marks, 3 days");
    let mean_sections: Vec<&String> = mean
        .iter()
        .filter(|line| line.starts_with("day, the "))
        .collect();
    let mark_sections: Vec<&String> = marks
        .iter()
        .filter(|line| line.starts_with("day, the "))
        .collect();
    assert_eq!(mean_sections, mark_sections);
    let mean_marks = mark_lines(&mean);
    let reading_marks = mark_lines(&marks);
    assert_eq!(mean_marks.len(), reading_marks.len());
    for (mean_line, mark_line) in mean_marks.iter().zip(reading_marks.iter()) {
        let parts = mean_mark_parts(mean_line);
        let rebuilt = format!(
            "mark {}, at {} seconds on the {} day, {} at a projection of {}",
            parts.place, parts.seconds, parts.day, parts.holder, parts.record
        );
        assert_eq!(rebuilt, record_prefix(mark_line));
    }
    let places: Vec<usize> = mean_marks
        .iter()
        .map(|line| mean_mark_parts(line).place)
        .collect();
    assert_eq!(places, (1..=mean_marks.len()).collect::<Vec<usize>>());
    let walked = count_after(mean.last().expect("the mean close"), "marks walked ");
    let reading_walked = count_after(marks.last().expect("the marks close"), "marks walked ");
    assert_eq!(walked, reading_walked);
    assert_eq!(walked, mean_marks.len());
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
                run("b", 400, &[(200, 66)])
            ),
        ),
        day(
            1000,
            "a",
            &format!(
                "[{}, {}, {}, {}]",
                run("a", 500, &[]),
                run("b", 400, &[]),
                run("c", 300, &[(100, 40)]),
                run("d", 250, &[(200, 110)])
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
    ]);
    let mean = meet_mean(&meet).expect("the mean read");
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
    let mut mean_sections: Vec<Vec<&String>> = Vec::new();
    for line in &mean {
        if line.starts_with("day, the ") {
            mean_sections.push(Vec::new());
        }
        if let Some(section) = mean_sections.last_mut() {
            section.push(line);
        }
    }
    assert_eq!(sections.len(), mean_sections.len());
    for (day_index, (section, mean_section)) in
        sections.iter().zip(mean_sections.iter()).enumerate()
    {
        let day_marks: Vec<&&String> = mean_section
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
        let others: Vec<f64> = section
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
        let last = day_marks.last().expect("the day holds a mark");
        let parts = mean_mark_parts(last);
        match (others.is_empty(), parts.mean) {
            (true, None) => {}
            (false, Some((figure, margin))) => {
                let expected = others.iter().sum::<f64>() / others.len() as f64;
                let stated: f64 = figure.parse().expect("a mean figure parses");
                assert!(
                    (stated - expected).abs() <= 0.000002,
                    "day {} last mark {}",
                    day_index,
                    last
                );
                let stated_margin: f64 = margin.parse().expect("a mean margin parses");
                assert!(
                    (stated - record - stated_margin).abs() <= 0.000002,
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
    let lines = meet_mean(&meet).expect("the mean read");
    assert_eq!(
        lines,
        vec![
            "meet mean, 3 days",
            "day, the first day, marks 1",
            "mark 1, at 100 seconds on the first day, solo at a projection of 100, and no other standing",
            "day, the second day, marks 1",
            "mark 2, at 200 seconds on the second day, solo at a projection of 150, and no other standing",
            "day, the third day, marks 0",
            "mean counts, marks walked 2, marks with another standing 0, days 3, and, no other standing at any mark",
        ]
    );
}

#[test]
fn test_the_mean_margin_stands_at_zero_only_where_the_whole_field_stands_at_the_record() {
    // Every competitor behind the holder projects at the
    // record figure itself at the first two figured marks,
    // the whole field standing at 100, and the field leans
    // away at the last two marks. The pin reads the margins
    // at the marks and the counts, and never the closest
    // naming and never the figures at the leaning marks.
    // The holder figure admission of N-1 and N-6 figures
    // the first mark, the holder standing alone there, and
    // gives it a mean margin of zero, so the pin fails
    // under both, the counted zero set gaining a mark the
    // design's coincidence cell did not weigh.
    let meet = load_meet(&[
        day(
            1000,
            "a",
            &format!(
                "[{}, {}, {}, {}, {}]",
                run("a", 500, &[(100, 10)]),
                run("b", 400, &[(200, 20)]),
                run("c", 300, &[(300, 30)]),
                run("d", 250, &[(400, 160)]),
                run("e", 200, &[(500, 350)])
            ),
        ),
        quiet_day(1000, "a", "a"),
        quiet_day(1000, "a", "a"),
    ]);
    let lines = meet_mean(&meet).expect("the mean read");
    let marks = mark_lines(&lines);
    assert_eq!(marks.len(), 5);
    assert!(mean_margin_of(marks[0]).is_none());
    let zero_marks: Vec<usize> = marks
        .iter()
        .enumerate()
        .filter(|(_, line)| mean_margin_of(line) == Some(0.0))
        .map(|(index, _)| index + 1)
        .collect();
    assert_eq!(zero_marks, vec![2, 3]);
    for line in &marks[3..] {
        let margin = mean_margin_of(line).expect("a leaning mark states its mean margin");
        assert!(margin > 0.0, "{}", line);
    }
    let counts = lines.last().expect("the mean close");
    assert_eq!(count_after(counts, "marks walked "), 5);
    assert_eq!(count_after(counts, "marks with another standing "), 4);
    assert_eq!(count_after(counts, "days "), 3);
}

#[test]
fn test_the_mean_separates_from_below_against_the_lower_tail_and_the_middle() {
    // At the fourth mark the field behind the holder stands
    // at 100, 200, and 900: its mean margin stands at 300,
    // its middle margin at 100, and its lower tail margin
    // at 50. At the sixth mark the field stands at 100,
    // 200, 300, 900, and 950: its mean margin stands at
    // 390, its middle margin at 200, and its lower tail
    // margin at 100. At both marks the mean margin stands
    // strictly above the middle margin and the lower tail
    // margin, the separation from below this pin exists to
    // hold.
    let meet = load_meet(&[
        day(
            1000,
            "a",
            &format!(
                "[{}, {}, {}, {}, {}, {}]",
                run("a", 500, &[(100, 10)]),
                run("b", 400, &[(200, 20)]),
                run("c", 300, &[(300, 60)]),
                run("d", 250, &[(400, 360)]),
                run("g", 200, &[(450, 135)]),
                run("e", 600, &[(500, 475)])
            ),
        ),
        quiet_day(1000, "a", "a"),
        quiet_day(1000, "a", "a"),
    ]);
    let mean = meet_mean(&meet).expect("the mean read");
    let middle = meet_middle(&meet).expect("the middle read");
    let tails = meet_tails(&meet).expect("the tails read");
    let mean_marks = mark_lines(&mean);
    let middle_marks = mark_lines(&middle);
    let tails_marks = mark_lines(&tails);
    for place in [4usize, 6] {
        let mean_margin = mean_margin_of(mean_marks[place - 1]).expect("a mean margin stands");
        let middle_margin =
            middle_margin_of(middle_marks[place - 1]).expect("a middle margin stands");
        let (lower_margin, _) =
            tail_margins_of(tails_marks[place - 1]).expect("tail margins stand");
        assert!(
            mean_margin > middle_margin,
            "mark {} mean {} middle {}",
            place,
            mean_margin,
            middle_margin
        );
        assert!(
            mean_margin > lower_margin,
            "mark {} mean {} lower {}",
            place,
            mean_margin,
            lower_margin
        );
    }
}

#[test]
fn test_the_mean_separates_from_above_against_the_middle_the_upper_tail_and_the_spread() {
    // At the fourth mark the field behind the holder stands
    // at 100, 800, and 900: its mean margin stands at 500,
    // its middle margin at 700, its upper tail margin at
    // 750, and its spread at 800. At the fifth mark the
    // field stands at 100, 800, 850, and 900: its mean
    // margin stands at 562.5, its middle margin at 700, its
    // upper tail margin at 775, and its spread at 800. At
    // both marks the mean margin stands strictly below the
    // middle margin, the upper tail margin, and the
    // spread, the separation from above this pin exists to
    // hold.
    let meet = load_meet(&[
        day(
            1000,
            "a",
            &format!(
                "[{}, {}, {}, {}, {}]",
                run("a", 500, &[(100, 10)]),
                run("b", 400, &[(200, 20)]),
                run("c", 300, &[(300, 240)]),
                run("d", 250, &[(400, 360)]),
                run("e", 600, &[(500, 425)])
            ),
        ),
        quiet_day(1000, "a", "a"),
        quiet_day(1000, "a", "a"),
    ]);
    let mean = meet_mean(&meet).expect("the mean read");
    let middle = meet_middle(&meet).expect("the middle read");
    let tails = meet_tails(&meet).expect("the tails read");
    let spread = meet_spread(&meet).expect("the spread read");
    let mean_marks = mark_lines(&mean);
    let middle_marks = mark_lines(&middle);
    let tails_marks = mark_lines(&tails);
    let spread_marks = mark_lines(&spread);
    for place in [4usize, 5] {
        let mean_margin = mean_margin_of(mean_marks[place - 1]).expect("a mean margin stands");
        let middle_margin =
            middle_margin_of(middle_marks[place - 1]).expect("a middle margin stands");
        let (_, upper_margin) =
            tail_margins_of(tails_marks[place - 1]).expect("tail margins stand");
        let spread = spread_of(spread_marks[place - 1]).expect("a spread stands");
        assert!(
            mean_margin < middle_margin,
            "mark {} mean {} middle {}",
            place,
            mean_margin,
            middle_margin
        );
        assert!(
            mean_margin < upper_margin,
            "mark {} mean {} upper {}",
            place,
            mean_margin,
            upper_margin
        );
        assert!(
            mean_margin < spread,
            "mark {} mean {} spread {}",
            place,
            mean_margin,
            spread
        );
    }
}

#[test]
fn test_the_closest_mean_margin_tie_names_the_earliest_mark() {
    // The second mark's field stands at 110, 140, and 140
    // behind the record of 100, its mean 130 and its mean
    // margin 30. The third mark, on the second day, stands
    // at a field of 110, 130, 140, and 140 behind the same
    // record, its mean again 130 and its mean margin again
    // 30, the tie real in the printed lines. The counting
    // names the second mark, the earliest at the smallest
    // mean margin.
    let meet = load_meet(&[
        day(
            1000,
            "a",
            &format!(
                "[{}, {}, {}, {}]",
                run("a", 500, &[(100, 10)]),
                run("b", 400, &[(200, 22)]),
                run("c", 300, &[(200, 28)]),
                run("d", 250, &[(200, 28)])
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
                run("e", 200, &[(100, 13)])
            ),
        ),
        quiet_day(1000, "a", "a"),
    ]);
    let lines = meet_mean(&meet).expect("the mean read");
    assert_eq!(
        lines,
        vec![
            "meet mean, 3 days",
            "day, the first day, marks 2",
            "mark 1, at 100 seconds on the first day, a at a projection of 100, and no other standing",
            "mark 2, at 200 seconds on the first day, a at a projection of 100, mean standing 130, mean margin 30",
            "day, the second day, marks 1",
            "mark 3, at 100 seconds on the second day, a at a projection of 100, mean standing 130, mean margin 30",
            "day, the third day, marks 0",
            "mean counts, marks walked 3, marks with another standing 2, days 3, closest mean margin 30 at 200 seconds on the first day",
        ]
    );
}

#[test]
fn test_the_mean_steps_as_the_field_grows() {
    let meet = load_meet(&[
        day(
            1000,
            "a",
            &format!(
                "[{}, {}, {}, {}, {}]",
                run("a", 500, &[(100, 10)]),
                run("b", 400, &[(200, 60)]),
                run("c", 300, &[(300, 90)]),
                run("d", 250, &[(400, 280)]),
                run("e", 600, &[(500, 425)])
            ),
        ),
        quiet_day(1000, "a", "a"),
        quiet_day(1000, "a", "a"),
    ]);
    let lines = meet_mean(&meet).expect("the mean read");
    let marks = mark_lines(&lines);
    let figures: Vec<String> = marks
        .iter()
        .filter_map(|line| mean_figure_of(line))
        .collect();
    assert_eq!(figures, vec!["300", "300", "433.333333", "537.5"]);
    assert_eq!(
        marks[1].as_str(),
        "mark 2, at 200 seconds on the first day, a at a projection of 100, mean standing 300, mean margin 200"
    );
    assert_eq!(
        marks[4].as_str(),
        "mark 5, at 500 seconds on the first day, a at a projection of 100, mean standing 537.5, mean margin 437.5"
    );
}

#[test]
fn test_the_bracketing_chain_holds_line_by_line() {
    // The chain of PG-38 at every figured mark: the margin
    // to the nearest standing stands at or below the lower
    // tail margin, which stands at or below the mean
    // margin, which stands at or below the upper tail
    // margin, which stands at or below the spread. The
    // chain stands strict at the fourth and fifth marks,
    // whose fields stand at 100, 200, and 900, and at 100,
    // 200, 900, and 950, and nowhere else on this meet.
    let meet = load_meet(&[
        day(
            1000,
            "a",
            &format!(
                "[{}, {}, {}, {}, {}]",
                run("a", 500, &[(100, 10)]),
                run("b", 400, &[(200, 20)]),
                run("c", 300, &[(300, 60)]),
                run("d", 250, &[(400, 360)]),
                run("e", 600, &[(500, 475)])
            ),
        ),
        quiet_day(1000, "a", "a"),
        quiet_day(1000, "a", "a"),
    ]);
    let mean = meet_mean(&meet).expect("the mean read");
    let margins = meet_margins(&meet).expect("the margins read");
    let tails = meet_tails(&meet).expect("the tails read");
    let spread = meet_spread(&meet).expect("the spread read");
    let mean_marks = mark_lines(&mean);
    let margin_marks = mark_lines(&margins);
    let tails_marks = mark_lines(&tails);
    let spread_marks = mark_lines(&spread);
    assert_eq!(mean_marks.len(), margin_marks.len());
    assert_eq!(mean_marks.len(), tails_marks.len());
    assert_eq!(mean_marks.len(), spread_marks.len());
    let mut strict_marks = 0usize;
    for ((((mean_line, margin_line), tails_line), spread_line), place) in mean_marks
        .iter()
        .zip(margin_marks.iter())
        .zip(tails_marks.iter())
        .zip(spread_marks.iter())
        .zip(1..)
    {
        let Some(mean_margin) = mean_margin_of(mean_line) else {
            assert!(margin_of(margin_line).is_none(), "mark {}", place);
            assert!(tail_margins_of(tails_line).is_none(), "mark {}", place);
            assert!(spread_of(spread_line).is_none(), "mark {}", place);
            continue;
        };
        let margin = margin_of(margin_line).expect("a margin stands");
        let (lower_margin, upper_margin) = tail_margins_of(tails_line).expect("tail margins stand");
        let spread = spread_of(spread_line).expect("a spread stands");
        assert!(margin <= lower_margin + 0.000002, "mark {}", place);
        assert!(lower_margin <= mean_margin + 0.000002, "mark {}", place);
        assert!(mean_margin <= upper_margin + 0.000002, "mark {}", place);
        assert!(upper_margin <= spread + 0.000002, "mark {}", place);
        if margin < lower_margin
            && lower_margin < mean_margin
            && mean_margin < upper_margin
            && upper_margin < spread
        {
            strict_marks += 1;
        }
    }
    assert_eq!(strict_marks, 2);
}

#[test]
fn test_a_meet_in_which_no_read_projects_reads_with_marks_walked_zero() {
    let meet = load_meet(&[
        quiet_day(600, "a", "a"),
        quiet_day(600, "a", "a"),
        quiet_day(600, "a", "a"),
    ]);
    let lines = meet_mean(&meet).expect("the mean read");
    assert_eq!(
        lines,
        vec![
            "meet mean, 3 days",
            "day, the first day, marks 0",
            "day, the second day, marks 0",
            "day, the third day, marks 0",
            "mean counts, marks walked 0, marks with another standing 0, days 3, and, no other standing at any mark",
        ]
    );
}

// The guard pins. The mean reading stands inside the
// fencing closure of FIX-PG-11 from its birth, and under
// the figure guard of FIX-PG-10 and the day count errors
// of PG-37, in the one walk it shares with the twelve
// readings before it.

#[test]
fn test_a_hand_built_non_finite_figure_draws_the_named_error_through_mean() {
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
    let error = meet_mean(&meet).expect_err("the reading names the precondition");
    assert_eq!(error, "the window is not a finite number");
}

#[test]
fn test_a_hand_built_break_bearing_label_draws_the_named_error_through_mean() {
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
    let error = meet_mean(&meet).expect_err("the reading names the precondition");
    assert_eq!(error, "a runner label holds a line break");
}

#[test]
fn test_a_hand_built_meet_with_no_days_draws_the_named_error_through_mean() {
    let meet = Meet { days: vec![] };
    let error = meet_mean(&meet).expect_err("the reading names the precondition");
    assert_eq!(error, "the meet holds no days");
}

#[test]
fn test_a_hand_built_meet_with_too_many_days_draws_the_named_error_through_mean() {
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
    let error = meet_mean(&meet).expect_err("the reading names the precondition");
    assert_eq!(
        error,
        "the meet holds more days than the ordinal table names"
    );
}
