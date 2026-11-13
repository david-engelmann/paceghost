//! Tests for the range goldens of PG-44 over the
//! standing example meets, and for the README Usage
//! section the round seats.
//!
//! Written for Round 21 in the shape of the Round 20
//! example tests. The two byte for byte pins fail before
//! the round by the absence of the goldens and of the
//! reading. Pin by pin under the round's mutations: pin
//! 16 fails under R-1 at the second mark, its range
//! moving from 0 to 90 over the enlarged field, under
//! R-5 at the third mark, the lagged vector printing 0
//! against 90 there, and under R-6 as under R-1, and it
//! passes under R-2 and R-3 by design, every figured
//! field of the shipped meet counting one or two behind
//! its holder, so each mark's range is its field width whole
//! and its difference of the tails whole, the widest
//! coincidence cell of the round, stated here rather
//! than left for a reviewer to find, and under R-4 by
//! design, its narrowest range standing unique at the
//! second mark. Pin 17 fails under R-1 at the first
//! mark, its range moving from 0 to 200, under R-4, the
//! seven marks tying at zero so the loosened scan names
//! the seventh, under R-5, the first mark printing no
//! standing, and under R-6 as under R-1, and it passes
//! under R-2 and R-3 by design, the constructed meet
//! standing at one behind throughout, so the range of a
//! field of one under any of the three statistics is
//! zero whole. Pins 18 and 19 pass under all six
//! mutations by design, counting the goldens' own lines
//! by hand, the counts they state standing in the golden
//! text the mutations do not reach, and their
//! discrimination stands with pins 16 and 17, which bind
//! the produced readings to those goldens byte for byte.
//! Pin 20 fails under R-1 and R-6, the second mark's
//! range becoming 90 and the narrowest naming moving to
//! a later mark, and under R-5, and it passes under R-4
//! by design, the narrowest range standing at the second
//! mark alone with no tie behind it, and under R-2 and
//! R-3 by design, both statistics standing at 0 at the
//! same second mark and nowhere smaller. Pin 21 passes
//! under R-2, R-3, and R-4 by design, its closes standing
//! at fields of two where the range is the field width
//! and the difference of the tails whole, and comparing
//! figures at the closes and never a narrowest naming,
//! and it fails under R-1 and R-6, the first day's
//! enlarged closing field carrying its range to 150
//! against the asserted 141.818182, and under R-5. Pins
//! 22 and 23 pass under all six mutations by
//! construction, comparing two standing texts, the
//! README block and the golden file, which no mutation
//! of the reading reaches, and their discrimination
//! stands at the pre round tree, where neither the
//! golden nor the README range section stands.

use std::path::PathBuf;

use paceghost::meet::{load_meet_text, meet_range, Meet};

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

/// The interquartile range a range mark line states,
/// where the line carries another standing.
fn range_of(line: &str) -> Option<f64> {
    line.split(", interquartile range ")
        .nth(1)
        .and_then(|word| word.parse().ok())
}

fn readme_text() -> String {
    std::fs::read_to_string(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("README.md"))
        .expect("the README reads")
}

/// The lines of one README Usage section, from its heading
/// to the next heading.
fn usage_section(readme: &str, heading: &str) -> Vec<String> {
    let lines: Vec<&str> = readme.lines().collect();
    let start = lines
        .iter()
        .position(|line| *line == heading)
        .expect("the usage heading stands");
    let mut section: Vec<String> = Vec::new();
    for line in &lines[start..] {
        if !section.is_empty() && line.starts_with('#') {
            break;
        }
        section.push((*line).to_string());
    }
    section
}

/// The lines of a section's example block, from the line
/// after its text fence to its closing fence.
fn example_block(section: &[String]) -> Vec<String> {
    let fence = section
        .iter()
        .position(|line| line == "```text")
        .expect("the section carries its example block");
    let mut block: Vec<String> = Vec::new();
    for line in &section[fence + 1..] {
        if line == "```" {
            break;
        }
        block.push(line.clone());
    }
    block
}

#[test]
fn test_the_shipped_range_golden_stands_byte_for_byte() {
    let meet = load_example("meet.json");
    let lines = meet_range(&meet).expect("the range read");
    assert_eq!(lines, golden_lines("meet-range-golden.txt"));
}

#[test]
fn test_the_constructed_range_golden_stands_byte_for_byte() {
    let meet = load_example("constructed-meet.json");
    let lines = meet_range(&meet).expect("the range read");
    assert_eq!(lines, golden_lines("constructed-meet-range-golden.txt"));
}

#[test]
fn test_the_shipped_range_counts_close_as_counted_by_hand() {
    let lines = golden_lines("meet-range-golden.txt");
    let marks = mark_lines(&lines);
    assert_eq!(marks.len(), 16);
    let figured = marks
        .iter()
        .filter(|line| line.contains(", interquartile range "))
        .count();
    assert_eq!(figured, 15);
    assert_eq!(
        lines.last().map(String::as_str),
        Some("range counts, marks walked 16, marks with another standing 15, days 3, narrowest interquartile range 0 at 400 seconds on the first day")
    );
}

#[test]
fn test_the_constructed_range_counts_close_as_counted_by_hand() {
    let lines = golden_lines("constructed-meet-range-golden.txt");
    let marks = mark_lines(&lines);
    assert_eq!(marks.len(), 7);
    let figured = marks
        .iter()
        .filter(|line| line.contains(", interquartile range "))
        .count();
    assert_eq!(figured, 7);
    assert_eq!(
        lines.last().map(String::as_str),
        Some("range counts, marks walked 7, marks with another standing 7, days 3, narrowest interquartile range 0 at 100 seconds on the first day")
    );
}

#[test]
fn test_the_shipped_goldens_narrowest_range_names_the_second_mark() {
    let meet = load_example("meet.json");
    let produced = meet_range(&meet).expect("the range read");
    assert_eq!(
        produced.last().map(String::as_str),
        Some("range counts, marks walked 16, marks with another standing 15, days 3, narrowest interquartile range 0 at 400 seconds on the first day")
    );
    let lines = golden_lines("meet-range-golden.txt");
    let marks = mark_lines(&lines);
    let second = marks[1];
    assert!(
        second.starts_with("mark 2, at 400 seconds on the first day,"),
        "the second mark stands at 400 seconds on the first day, {}",
        second
    );
    assert_eq!(range_of(second), Some(0.0));
    let mut figured = 0usize;
    for (place, mark) in marks.iter().enumerate() {
        if let Some(range) = range_of(mark) {
            figured += 1;
            assert!(range >= 0.0, "{}", mark);
            if place != 1 {
                assert!(
                    range > 0.0,
                    "no other mark states the narrowest range, {}",
                    mark
                );
            }
        }
    }
    assert_eq!(figured, 15);
}

#[test]
fn test_the_day_close_relation_holds_over_the_shipped_example() {
    let meet = load_example("meet.json");
    let range = meet_range(&meet).expect("the range read");
    let standings = golden_lines("meet-standings-golden.txt");
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
    assert_eq!(sections.len(), 3);
    assert_eq!(range_sections.len(), 3);
    let named = [141.818182, 108.0, 30.0];
    for (index, (section, range_section)) in sections.iter().zip(range_sections.iter()).enumerate()
    {
        let closing = section
            .iter()
            .find(|line| line.starts_with("close of the "))
            .expect("a standings section closes");
        let holder = closing
            .split("record held by ")
            .nth(1)
            .and_then(|rest| rest.split(" at a projection of ").next())
            .expect("a closing line names its holder");
        let mut figures: Vec<f64> = Vec::new();
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
            figures.push(figure.parse().expect("a standing figure parses"));
        }
        assert_eq!(figures.len(), 2, "the close field counted, day {}", index);
        figures.sort_by(|a, b| a.partial_cmp(b).expect("figures order"));
        let count = figures.len().div_ceil(2);
        let median_of = |part: &[f64]| {
            if part.len() % 2 == 1 {
                part[part.len() / 2]
            } else {
                (part[part.len() / 2 - 1] + part[part.len() / 2]) / 2.0
            }
        };
        let expected = median_of(&figures[figures.len() - count..]) - median_of(&figures[..count]);
        assert!(
            (expected - named[index]).abs() <= 0.000002,
            "the day close range counted from the golden, day {}",
            index
        );
        let last = range_section
            .iter()
            .rfind(|line| line.starts_with("mark "))
            .expect("the day holds a mark");
        let stated = range_of(last).expect("the last mark states its range");
        assert!(
            (stated - expected).abs() <= 0.000002,
            "the day close range figure, {}",
            last
        );
    }
}

#[test]
fn test_the_readme_range_block_stands_byte_for_byte_as_the_shipped_golden() {
    let readme = readme_text();
    let section = usage_section(&readme, "### range");
    let block = example_block(&section);
    assert_eq!(
        block.first().map(String::as_str),
        Some("$ paceghost range examples/meet.json")
    );
    assert_eq!(block[1..], golden_lines("meet-range-golden.txt")[..]);
}

#[test]
fn test_the_readme_range_usage_section_stands_in_its_standing_form() {
    let readme = readme_text();
    let section = usage_section(&readme, "### range");
    assert_eq!(section.first().map(String::as_str), Some("### range"));
    let lines: Vec<&str> = readme.lines().collect();
    let quartiles = lines
        .iter()
        .position(|line| *line == "### quartiles")
        .expect("the quartiles heading stands");
    let range = lines
        .iter()
        .position(|line| *line == "### range")
        .expect("the range heading stands");
    assert!(
        quartiles < range,
        "the range section stands after the quartiles section"
    );
    assert!(section.contains(
        &"The range command prints the meet read mark by mark, naming the holder after each mark, the record figure after it, and the interquartile range held after it by the field behind its holder, the upper quartile standing less the lower quartile standing of that field."
            .to_string()
    ));
    let block = example_block(&section);
    assert_eq!(
        block.first().map(String::as_str),
        Some("$ paceghost range examples/meet.json")
    );
}
