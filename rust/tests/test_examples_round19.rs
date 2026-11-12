//! Tests for the median goldens of PG-40 over the standing
//! example meets.
//!
//! Written for Round 19 in the shape of the Round 18 example
//! tests. The two byte for byte pins fail before the round
//! by the absence of the goldens and of the reading, and the
//! hand count pins close the closing counts of both goldens
//! against the counts stated in the design note. Pin by pin
//! under the round's mutations: pin 16 fails under P-1 at
//! the first figured mark, the second mark's median
//! becoming 1305 against the golden's 1350, under P-2 at
//! its fields of two behind the holder, whose midpoint
//! the lower figure replaces, its fields of one passing
//! there by design, under P-5, and under P-6, and it
//! passes under P-3 by design, the shipped meet standing
//! as the round's widest coincidence cell, its median
//! figure equalling its mean figure at
//! every one of its fifteen figured marks, so a margin
//! printed off the mean standing moves nothing on it, and
//! under P-4 by design, the shipped closest standing
//! unique. Pin 17 fails under P-1, under P-5, and under
//! P-6, and under P-4, its closest standing tied at the
//! third and sixth marks so the counting names the sixth
//! in place of the third; under P-2 and P-3 it passes by
//! design at every mark, the constructed meet standing at
//! one behind throughout, so the middle of a field of one
//! and the mean of a field of one are the figure whole.
//! Pin 18 fails under P-1, P-2, P-5, and P-6, its counts
//! line moving with the figures, and passes under P-3 by
//! design on the shipped coincidence and under P-4 by
//! design, its closest standing unique. Pin 19 fails under
//! P-1, P-4, P-5, and P-6, and passes under P-2 and P-3 by
//! design on the same field of one ground as pin 17. Pin
//! 20 fails under P-1, P-2, P-5, and P-6, reading the
//! produced reading's counts line against the shipped
//! golden's own marks, and it passes under P-3 by design
//! on the shipped coincidence and under P-4 by design, the
//! shipped closest standing unique. Pin 21 fails under
//! P-1, P-2, P-5, and P-6, and it passes under P-3 by
//! design, comparing median figures at the closes and
//! never a margin, and under P-4 by design, reading no
//! closest naming. The closes of pin 21 stand at fields of
//! two behind the holder at all three days, counted from
//! the standings golden itself, where the median and the
//! mean of two figures are the one midpoint, so the named
//! closes are the design note's counts and not a
//! separation. The limitation of this file stands stated
//! in the design and repeated here: both example meets
//! stand at fields behind the holder whose median figure
//! equals its mean figure at every figured mark, the
//! shipped meet by the coincidence above and the
//! constructed meet by its fields of one, so under a
//! degeneration of the median to any statistic that agrees
//! with it on those fields these pins pass while the
//! construction is wrong. The separation at fields of
//! three and more behind the holder, and at even fields
//! whose two middle figures stand apart from their
//! neighbours, stands in the hand built pins of
//! rust/tests/test_median.rs and in the generated pin,
//! whose meets are built or generated at those fields.
//! Pin 22 asserts the README median section's example
//! block stands byte for byte as the shipped median
//! golden whole, under the first line in the form of the
//! command's invocation. It passes under all six
//! mutations by construction, comparing two standing
//! texts, the README block and the golden file, which no
//! mutation of the reading reaches, and its
//! discrimination stands at the pre round tree, where
//! neither the golden nor the README median section
//! stands, and wherever the block and the golden
//! diverge. Pin
//! 23 asserts the README Usage section for the median
//! reading stands in its standing form, its heading, its
//! prose paragraph, and its example block standing where
//! the section states, the section standing after the
//! mean section. It passes under all six mutations by
//! construction, reading README text alone, which no
//! mutation of the reading reaches, and its discrimination
//! stands at the pre round tree, where no median section
//! stands.

use std::path::PathBuf;

use paceghost::meet::{load_meet_text, meet_median, Meet};

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

/// The median margin a median mark line states, where the
/// line carries another standing.
fn median_margin_of(line: &str) -> Option<f64> {
    line.split(", median margin ")
        .nth(1)
        .and_then(|word| word.parse().ok())
}

/// The median figure a median mark line states, where the
/// line carries another standing.
fn median_figure_of(line: &str) -> Option<f64> {
    line.split(", median standing ")
        .nth(1)
        .and_then(|rest| rest.split(", median margin ").next())
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
fn test_the_shipped_median_golden_stands_byte_for_byte() {
    let meet = load_example("meet.json");
    let lines = meet_median(&meet).expect("the median read");
    assert_eq!(lines, golden_lines("meet-median-golden.txt"));
}

#[test]
fn test_the_constructed_median_golden_stands_byte_for_byte() {
    let meet = load_example("constructed-meet.json");
    let lines = meet_median(&meet).expect("the median read");
    assert_eq!(lines, golden_lines("constructed-meet-median-golden.txt"));
}

#[test]
fn test_the_shipped_median_counts_close_as_counted_by_hand() {
    let meet = load_example("meet.json");
    let lines = meet_median(&meet).expect("the median read");
    let marks = mark_lines(&lines);
    assert_eq!(marks.len(), 16);
    let figured = marks
        .iter()
        .filter(|line| line.contains(", median standing "))
        .count();
    assert_eq!(figured, 15);
    assert_eq!(
        lines.last().map(String::as_str),
        Some("median counts, marks walked 16, marks with another standing 15, days 3, closest median margin 90 at 400 seconds on the first day")
    );
}

#[test]
fn test_the_constructed_median_counts_close_as_counted_by_hand() {
    let meet = load_example("constructed-meet.json");
    let lines = meet_median(&meet).expect("the median read");
    let marks = mark_lines(&lines);
    assert_eq!(marks.len(), 7);
    let figured = marks
        .iter()
        .filter(|line| line.contains(", median standing "))
        .count();
    assert_eq!(figured, 7);
    assert_eq!(
        lines.last().map(String::as_str),
        Some("median counts, marks walked 7, marks with another standing 7, days 3, closest median margin 50 at 100 seconds on the second day")
    );
}

#[test]
fn test_the_shipped_goldens_closest_median_margin_names_the_second_mark() {
    let meet = load_example("meet.json");
    let produced = meet_median(&meet).expect("the median read");
    assert_eq!(
        produced.last().map(String::as_str),
        Some("median counts, marks walked 16, marks with another standing 15, days 3, closest median margin 90 at 400 seconds on the first day")
    );
    let lines = golden_lines("meet-median-golden.txt");
    let marks = mark_lines(&lines);
    let second = marks[1];
    assert!(
        second.starts_with("mark 2, at 400 seconds on the first day,"),
        "the second mark stands at 400 seconds on the first day, {}",
        second
    );
    assert_eq!(median_margin_of(second), Some(90.0));
    for earlier in &marks[..1] {
        if let Some(margin) = median_margin_of(earlier) {
            assert!(
                margin > 90.0,
                "no earlier mark states a smaller median margin, {}",
                earlier
            );
        }
    }
    let mut figured = 0usize;
    for mark in &marks {
        if let Some(margin) = median_margin_of(mark) {
            figured += 1;
            assert!(margin >= 90.0, "{}", mark);
        }
    }
    assert_eq!(figured, 15);
}

#[test]
fn test_the_day_close_relation_holds_over_the_shipped_example() {
    let meet = load_example("meet.json");
    let median = meet_median(&meet).expect("the median read");
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
    let mut median_sections: Vec<Vec<&String>> = Vec::new();
    for line in &median {
        if line.starts_with("day, the ") {
            median_sections.push(Vec::new());
        }
        if let Some(section) = median_sections.last_mut() {
            section.push(line);
        }
    }
    assert_eq!(sections.len(), 3);
    assert_eq!(median_sections.len(), 3);
    let named = [1069.090909, 954.0, 735.0];
    for (index, (section, median_section)) in
        sections.iter().zip(median_sections.iter()).enumerate()
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
        let expected = (figures[0] + figures[1]) / 2.0;
        assert!(
            (expected - named[index]).abs() <= 0.000002,
            "the day close median figure counted from the golden, day {}",
            index
        );
        let last = median_section
            .iter()
            .rfind(|line| line.starts_with("mark "))
            .expect("the day holds a mark");
        let stated = median_figure_of(last).expect("the last mark states its median");
        assert!(
            (stated - expected).abs() <= 0.000002,
            "the day close median figure, {}",
            last
        );
    }
}

#[test]
fn test_the_readme_median_block_stands_byte_for_byte_as_the_shipped_golden() {
    let readme = readme_text();
    let section = usage_section(&readme, "### median");
    let block = example_block(&section);
    assert_eq!(
        block.first().map(String::as_str),
        Some("$ paceghost median examples/meet.json")
    );
    assert_eq!(block[1..], golden_lines("meet-median-golden.txt")[..]);
}

#[test]
fn test_the_readme_median_usage_section_stands_in_its_standing_form() {
    let readme = readme_text();
    let section = usage_section(&readme, "### median");
    assert_eq!(section.first().map(String::as_str), Some("### median"));
    let lines: Vec<&str> = readme.lines().collect();
    let mean = lines
        .iter()
        .position(|line| *line == "### mean")
        .expect("the mean heading stands");
    let median = lines
        .iter()
        .position(|line| *line == "### median")
        .expect("the median heading stands");
    assert!(
        mean < median,
        "the median section stands after the mean section"
    );
    assert!(section.contains(
        &"The median command prints the meet read mark by mark, naming the holder after each mark, the record figure after it, the median standing held after it by the field behind its holder, and the median margin by which the record stands ahead of the median standing."
            .to_string()
    ));
    let block = example_block(&section);
    assert_eq!(
        block.first().map(String::as_str),
        Some("$ paceghost median examples/meet.json")
    );
}
