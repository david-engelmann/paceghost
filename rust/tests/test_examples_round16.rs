//! Tests for the middle goldens of PG-34 over the standing
//! example meets.
//!
//! Written for Round 16 in the shape of the Round 15 example
//! tests. The two byte for byte pins fail before the round
//! by the absence of the goldens and of the reading, and the
//! hand count pins close the closing counts of both goldens
//! against the counts stated in the design note. The
//! limitation of this file stands stated in the design and
//! repeated here: both shipped fields stand at three or
//! fewer, so the middle figures on both goldens equal the
//! nearest figures. As executed, these six pins pass under
//! M-4 while the construction is wrong, pin 20's own defect
//! at the head before its repair aside, and under M-1 pins
//! 16, 18, and 21 fail, the shipped meets exercising the
//! even field of two at nearly every mark, where the M-1
//! place separates from the designed place. The remaining
//! discrimination against M-1, the whole of M-2, and the
//! rest of M-4 stand in the hand built pins of
//! rust/tests/test_middle.rs and in the generated pin,
//! whose meets are built or generated at fields of four
//! and more.

use std::path::PathBuf;

use paceghost::meet::{load_meet_text, meet_middle, meet_standings, Meet};

fn examples_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples")
}

fn golden_lines(name: &str) -> Vec<String> {
    let golden = std::fs::read_to_string(examples_dir().join(name)).expect("the golden file reads");
    assert!(golden.ends_with('\n'));
    golden.lines().map(str::to_string).collect()
}

fn load_example(name: &str) -> Meet {
    let text = std::fs::read_to_string(examples_dir().join(name)).expect("the example reads");
    let (meet, defects) = load_meet_text(&text);
    assert_eq!(defects, Vec::<String>::new());
    meet.expect("the example loads")
}

fn mark_lines(lines: &[String]) -> Vec<&String> {
    lines
        .iter()
        .filter(|line| line.starts_with("mark "))
        .collect()
}

/// The middle margin a middle mark line states, where the
/// line carries another standing.
fn middle_margin_of(line: &str) -> Option<f64> {
    line.rsplit(", middle margin ")
        .next()
        .filter(|_| line.contains(", middle standing "))
        .and_then(|word| word.parse().ok())
}

#[test]
fn test_the_shipped_middle_golden_stands_byte_for_byte() {
    let meet = load_example("meet.json");
    let lines = meet_middle(&meet).expect("the middle read");
    assert_eq!(lines, golden_lines("meet-middle-golden.txt"));
}

#[test]
fn test_the_constructed_middle_golden_stands_byte_for_byte() {
    let meet = load_example("constructed-meet.json");
    let lines = meet_middle(&meet).expect("the middle read");
    assert_eq!(lines, golden_lines("constructed-meet-middle-golden.txt"));
}

#[test]
fn test_the_shipped_middle_counts_close_as_counted_by_hand() {
    let meet = load_example("meet.json");
    let lines = meet_middle(&meet).expect("the middle read");
    let marks = mark_lines(&lines);
    assert_eq!(marks.len(), 16);
    let figured = marks
        .iter()
        .filter(|line| line.contains(", middle standing "))
        .count();
    assert_eq!(figured, 15);
    assert_eq!(
        lines.last().map(String::as_str),
        Some("middle counts, marks walked 16, marks with another standing 15, days 3, closest middle margin 58.181818 at 1100 seconds on the first day")
    );
}

#[test]
fn test_the_constructed_middle_counts_close_as_counted_by_hand() {
    let meet = load_example("constructed-meet.json");
    let lines = meet_middle(&meet).expect("the middle read");
    let marks = mark_lines(&lines);
    assert_eq!(marks.len(), 7);
    let figured = marks
        .iter()
        .filter(|line| line.contains(", middle standing "))
        .count();
    assert_eq!(figured, 7);
    assert_eq!(
        lines.last().map(String::as_str),
        Some("middle counts, marks walked 7, marks with another standing 7, days 3, closest middle margin 50 at 100 seconds on the second day")
    );
}

#[test]
fn test_the_shipped_goldens_closest_middle_margin_names_the_sixth_mark() {
    let lines = golden_lines("meet-middle-golden.txt");
    let marks = mark_lines(&lines);
    let sixth = marks[5];
    assert!(
        sixth.starts_with("mark 6, at 1100 seconds on the first day,"),
        "the sixth mark stands at 1100 seconds on the first day, {}",
        sixth
    );
    assert_eq!(middle_margin_of(sixth), Some(58.181818));
    for earlier in &marks[..5] {
        if let Some(margin) = middle_margin_of(earlier) {
            assert!(
                margin > 58.181818,
                "no earlier mark states a smaller middle margin, {}",
                earlier
            );
        }
    }
    let mut figured = 0usize;
    for mark in &marks {
        if let Some(margin) = middle_margin_of(mark) {
            figured += 1;
            assert!(margin >= 58.181818, "{}", mark);
        }
    }
    assert_eq!(figured, 15);
}

#[test]
fn test_the_day_close_relation_holds_over_the_shipped_example() {
    let meet = load_example("meet.json");
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
    assert_eq!(sections.len(), 3);
    assert_eq!(middle_sections.len(), 3);
    for (section, middle_section) in sections.iter().zip(middle_sections.iter()) {
        let closing = section
            .iter()
            .find(|line| line.starts_with("close of the "))
            .expect("a standings section closes");
        let holder = closing
            .split("record held by ")
            .nth(1)
            .and_then(|rest| rest.split(" at a projection of ").next())
            .expect("a closing line names its holder");
        let mut others: Vec<(&str, f64)> = Vec::new();
        for line in section
            .iter()
            .filter(|line| line.starts_with("competitor "))
        {
            let Some((label, figure)) = line
                .strip_prefix("competitor ")
                .and_then(|rest| rest.split_once(", standing projection "))
                .and_then(|(label, rest)| {
                    rest.strip_suffix(" at the close")
                        .map(|figure| (label, figure))
                })
            else {
                continue;
            };
            if label == holder {
                continue;
            }
            others.push((label, figure.parse().expect("a standing figure parses")));
        }
        let mut figures: Vec<f64> = others.iter().map(|(_, value)| *value).collect();
        figures.sort_by(|a, b| a.partial_cmp(b).expect("figures order"));
        let expected = figures[figures.len().div_ceil(2) - 1];
        let expected_label = others
            .iter()
            .find(|(_, value)| (*value - expected).abs() <= 0.000002)
            .map(|(label, _)| *label)
            .expect("a competitor stands at the middle figure");
        let last = middle_section
            .iter()
            .rfind(|line| line.starts_with("mark "))
            .expect("the day holds a mark");
        let stated = last
            .split(", middle standing ")
            .nth(1)
            .expect("the last mark states its middle standing");
        let (figure, rest) = stated
            .split_once(" held by ")
            .expect("the middle standing names its holder");
        let stated_value: f64 = figure.parse().expect("a middle figure parses");
        assert!(
            (stated_value - expected).abs() <= 0.000002,
            "the day close middle figure, {}",
            last
        );
        assert_eq!(
            rest.split(", middle margin ")
                .next()
                .expect("the middle standing names its holder in full"),
            expected_label,
            "the day close middle holder, {}",
            last
        );
    }
}
