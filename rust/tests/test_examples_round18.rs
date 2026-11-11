//! Tests for the mean goldens of PG-38 over the standing
//! example meets.
//!
//! Written for Round 18 in the shape of the Round 17 example
//! tests. The two byte for byte pins fail before the round
//! by the absence of the goldens and of the reading, and the
//! hand count pins close the closing counts of both goldens
//! against the counts stated in the design note. Pin by pin
//! under the round's mutations: pin 16 fails under N-1 at
//! every figured mark, under N-2 and N-3 at its fields of
//! two behind the holder, whose midpoint the lower figure
//! and the middle figure each replace, its field of one
//! passing there by design, under N-5, and under N-6, and
//! it passes under N-4 by design, the shipped closest
//! standing unique. Pin 17 fails under N-1, under N-5, and
//! under N-6, and under N-4, its closest standing tied at
//! the third and sixth marks so the counting names the
//! sixth in place of the third; under N-2 and N-3 it passes
//! by design at every mark, the constructed meet standing
//! at one behind throughout, so the lowest half and the
//! middle of a field of one are the figure whole. Pin 18
//! fails under N-1, N-2, N-3, N-5, and N-6, its counts line
//! moving with the figures, and passes under N-4 by
//! design. Pin 19 fails under N-1, N-4, N-5, and N-6, and
//! passes under N-2 and N-3 by design on the same field of
//! one ground as pin 17. Pin 20 passes under all six
//! mutations by construction, reading the shipped golden's
//! text alone, which no mutation of the reading reaches,
//! and it fails at the pre round tree, where the golden
//! does not stand, and against a golden whose own closest
//! naming disagrees with its mark lines. Pin 21
//! fails under N-1, N-2, N-5, and N-6, and it passes under
//! N-3 by design, comparing mean figures at the closes and
//! never a margin, and under N-4 by design, reading no
//! closest naming. The closes of pin 21 stand at fields of
//! two behind the holder at all three days, counted from
//! the standings golden itself. The limitation of this
//! file stands stated in the design and repeated here:
//! both example fields stand at two or fewer behind the
//! holder, so the mean figure on both goldens equals the
//! midpoint of its two tail figures at every figured mark,
//! and under a midpoint degeneration of the mean these six
//! pins pass while the construction is wrong. The
//! separation at fields of three and more behind the
//! holder stands in the hand built pins of
//! rust/tests/test_mean.rs and in the generated pin, whose
//! meets are built or generated at those fields.
//!
//! Two further pins stand appended under the Round 18
//! repair, landing the design plan's pins 18 and 19, whose
//! places the two hand count pins numbered 18 and 19 above
//! took at build time, the substitution corrected and
//! disclosed by this repair. The two pins are numbered 22
//! and 23 here so the landed pins above keep the numbers
//! their mutation record cites. Pin 22 asserts the README
//! mean section's example block stands byte for byte as
//! the middle section of the produced shipped golden, the
//! lines after the golden's first line and before its
//! counts line, under the first line in the form meet
//! mean, 3 days. It fails under N-1, N-2, N-3, N-5, and
//! N-6 with the golden it mirrors, the README text
//! standing still while the produced middle section
//! moves, and it passes under N-4 by design, the counts
//! line standing outside the compared middle section, so
//! the loosened scan's naming never enters the pin. Pin
//! 23 asserts the README Usage section for the mean
//! reading stands in its standing form, its heading, its
//! prose paragraph, and its example block standing where
//! the section states, the section standing last among
//! the Usage sections, and the README's prose naming
//! PG-37 and PG-38 in standing order. It passes under all
//! six mutations by construction, reading README text
//! alone, which no mutation of the reading reaches, and
//! its discrimination stands at the pre round tree, where
//! no mean section stands.

use std::path::PathBuf;

use paceghost::meet::{load_meet_text, meet_mean, Meet};

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

/// The mean margin a mean mark line states, where the line
/// carries another standing.
fn mean_margin_of(line: &str) -> Option<f64> {
    line.split(", mean margin ")
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

/// The mean figure a mean mark line states, where the line
/// carries another standing.
fn mean_figure_of(line: &str) -> Option<f64> {
    line.split(", mean standing ")
        .nth(1)
        .and_then(|rest| rest.split(", mean margin ").next())
        .and_then(|word| word.parse().ok())
}

#[test]
fn test_the_shipped_mean_golden_stands_byte_for_byte() {
    let meet = load_example("meet.json");
    let lines = meet_mean(&meet).expect("the mean read");
    assert_eq!(lines, golden_lines("meet-mean-golden.txt"));
}

#[test]
fn test_the_constructed_mean_golden_stands_byte_for_byte() {
    let meet = load_example("constructed-meet.json");
    let lines = meet_mean(&meet).expect("the mean read");
    assert_eq!(lines, golden_lines("constructed-meet-mean-golden.txt"));
}

#[test]
fn test_the_shipped_mean_counts_close_as_counted_by_hand() {
    let meet = load_example("meet.json");
    let lines = meet_mean(&meet).expect("the mean read");
    let marks = mark_lines(&lines);
    assert_eq!(marks.len(), 16);
    let figured = marks
        .iter()
        .filter(|line| line.contains(", mean standing "))
        .count();
    assert_eq!(figured, 15);
    assert_eq!(
        lines.last().map(String::as_str),
        Some("mean counts, marks walked 16, marks with another standing 15, days 3, closest mean margin 90 at 400 seconds on the first day")
    );
}

#[test]
fn test_the_constructed_mean_counts_close_as_counted_by_hand() {
    let meet = load_example("constructed-meet.json");
    let lines = meet_mean(&meet).expect("the mean read");
    let marks = mark_lines(&lines);
    assert_eq!(marks.len(), 7);
    let figured = marks
        .iter()
        .filter(|line| line.contains(", mean standing "))
        .count();
    assert_eq!(figured, 7);
    assert_eq!(
        lines.last().map(String::as_str),
        Some("mean counts, marks walked 7, marks with another standing 7, days 3, closest mean margin 50 at 100 seconds on the second day")
    );
}

#[test]
fn test_the_shipped_goldens_closest_mean_margin_names_the_second_mark() {
    let lines = golden_lines("meet-mean-golden.txt");
    let marks = mark_lines(&lines);
    let second = marks[1];
    assert!(
        second.starts_with("mark 2, at 400 seconds on the first day,"),
        "the second mark stands at 400 seconds on the first day, {}",
        second
    );
    assert_eq!(mean_margin_of(second), Some(90.0));
    for earlier in &marks[..1] {
        if let Some(margin) = mean_margin_of(earlier) {
            assert!(
                margin > 90.0,
                "no earlier mark states a smaller mean margin, {}",
                earlier
            );
        }
    }
    let mut figured = 0usize;
    for mark in &marks {
        if let Some(margin) = mean_margin_of(mark) {
            figured += 1;
            assert!(margin >= 90.0, "{}", mark);
        }
    }
    assert_eq!(figured, 15);
}

#[test]
fn test_the_day_close_relation_holds_over_the_shipped_example() {
    let meet = load_example("meet.json");
    let mean = meet_mean(&meet).expect("the mean read");
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
    let mut mean_sections: Vec<Vec<&String>> = Vec::new();
    for line in &mean {
        if line.starts_with("day, the ") {
            mean_sections.push(Vec::new());
        }
        if let Some(section) = mean_sections.last_mut() {
            section.push(line);
        }
    }
    assert_eq!(sections.len(), 3);
    assert_eq!(mean_sections.len(), 3);
    let named = [1069.090909, 954.0, 735.0];
    for (index, (section, mean_section)) in sections.iter().zip(mean_sections.iter()).enumerate() {
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
        let expected = figures.iter().sum::<f64>() / figures.len() as f64;
        assert!(
            (expected - named[index]).abs() <= 0.000002,
            "the day close mean figure counted from the golden, day {}",
            index
        );
        let last = mean_section
            .iter()
            .rfind(|line| line.starts_with("mark "))
            .expect("the day holds a mark");
        let stated = mean_figure_of(last).expect("the last mark states its mean");
        assert!(
            (stated - expected).abs() <= 0.000002,
            "the day close mean figure, {}",
            last
        );
    }
}

#[test]
fn test_the_readme_mean_block_stands_byte_for_byte_as_the_produced_golden() {
    let readme = readme_text();
    let section = usage_section(&readme, "### mean");
    let block = example_block(&section);
    assert_eq!(
        block.first().map(String::as_str),
        Some("$ paceghost mean examples/meet.json")
    );
    let meet = load_example("meet.json");
    let produced = meet_mean(&meet).expect("the mean read");
    assert_eq!(
        produced.first().map(String::as_str),
        Some("meet mean, 3 days")
    );
    assert_eq!(block[1].as_str(), "meet mean, 3 days");
    // The middle section of the produced golden is the
    // lines after its first line and before its counts
    // line, and the README block carries the same lines
    // after its own first line and before its counts line.
    assert_eq!(block[2..block.len() - 1], produced[1..produced.len() - 1]);
}

#[test]
fn test_the_readme_mean_usage_section_stands_in_its_standing_form() {
    let readme = readme_text();
    let section = usage_section(&readme, "### mean");
    assert_eq!(section.first().map(String::as_str), Some("### mean"));
    let lines: Vec<&str> = readme.lines().collect();
    let tails = lines
        .iter()
        .position(|line| *line == "### tails")
        .expect("the tails heading stands");
    let mean = lines
        .iter()
        .position(|line| *line == "### mean")
        .expect("the mean heading stands");
    assert!(
        tails < mean,
        "the mean section stands after the tails section"
    );
    assert!(section.contains(
        &"The mean command prints the meet read mark by mark, naming the holder after each mark, the record figure after it, the mean standing held after it by the field behind its holder, and the mean margin by which the record stands ahead of the mean standing."
            .to_string()
    ));
    let block = example_block(&section);
    assert_eq!(
        block.first().map(String::as_str),
        Some("$ paceghost mean examples/meet.json")
    );
}
