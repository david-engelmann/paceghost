//! Tests for the width goldens of PG-52 over the
//! standing example meets, and for the README Usage
//! section the round seats.
//!
//! Written for Round 25 in the shape of the Round 22
//! example tests. The two byte for byte pins fail before
//! the round by the absence of the goldens and of the
//! reading. Pin by pin under the round's mutations: pin
//! 33, the shipped golden pin, pin 37, the shipped
//! narrowest names the second mark pin, and pin 38, the
//! day close relation pin, fail under S-1, S-5, and S-6
//! and pass under S-2, S-3, and S-4 by design, on the
//! same written-meet ground as the commands pins, every
//! figured field of both written meets counting two or
//! fewer, where the interquartile range and the
//! difference of the tails stand at the field width and
//! the narrowest width stands unique in the shipped
//! meet, pin 37 failing under S-1 by a field width of 0
//! standing at the shipped meet's first mark under the
//! enlarged field carrying the narrowest there at 300
//! seconds on the first day, and under S-5, the lagged
//! narrowest standing at the third mark. Pin 34, the
//! constructed golden pin, fails under S-1, S-5, and
//! S-6 and passes under S-2 and S-3 by design on the
//! same ground, and it fails under S-4, the constructed
//! meet's narrowest field width, 0, standing tied at
//! all seven of its marks, so the loosened scan renames
//! its narrowest seat to the seventh mark at 300
//! seconds on the third day and the pin stands as an
//! S-4 catcher beside pin 8 of the reading file. Pins
//! 35 and 36, the two counts pins, pass under all six
//! mutations by design, counting from the golden text
//! alone and never calling the reading, the Round 21
//! form, so no mutation of the reading moves them.
//! Pins 39 and 40, the two README pins, pass under all
//! six mutations by design, standing as static text, and
//! their discrimination stands at the pre round tree,
//! where neither the golden nor the README width
//! section stands.

use std::path::PathBuf;

use paceghost::meet::{load_meet_text, meet_width, Meet};

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

/// The field width a width mark line states, where the
/// line carries another standing.
fn width_of(line: &str) -> Option<f64> {
    line.split(", field width ")
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
fn test_the_shipped_width_golden_stands_byte_for_byte() {
    let meet = load_example("meet.json");
    let lines = meet_width(&meet).expect("the width read");
    assert_eq!(lines, golden_lines("meet-width-golden.txt"));
}

#[test]
fn test_the_constructed_width_golden_stands_byte_for_byte() {
    let meet = load_example("constructed-meet.json");
    let lines = meet_width(&meet).expect("the width read");
    assert_eq!(lines, golden_lines("constructed-meet-width-golden.txt"));
}

#[test]
fn test_the_shipped_width_counts_close_as_counted_by_hand() {
    let lines = golden_lines("meet-width-golden.txt");
    let marks = mark_lines(&lines);
    assert_eq!(marks.len(), 16);
    let figured = marks
        .iter()
        .filter(|line| line.contains(", field width "))
        .count();
    assert_eq!(figured, 15);
    assert_eq!(
        lines.last().map(String::as_str),
        Some("width counts, marks walked 16, marks with another standing 15, days 3, narrowest field width 0 at 400 seconds on the first day")
    );
}

#[test]
fn test_the_constructed_width_counts_close_as_counted_by_hand() {
    let lines = golden_lines("constructed-meet-width-golden.txt");
    let marks = mark_lines(&lines);
    assert_eq!(marks.len(), 7);
    let figured = marks
        .iter()
        .filter(|line| line.contains(", field width "))
        .count();
    assert_eq!(figured, 7);
    assert_eq!(
        lines.last().map(String::as_str),
        Some("width counts, marks walked 7, marks with another standing 7, days 3, narrowest field width 0 at 100 seconds on the first day")
    );
}

#[test]
fn test_the_shipped_goldens_narrowest_field_width_names_the_second_mark() {
    let meet = load_example("meet.json");
    let produced = meet_width(&meet).expect("the width read");
    assert_eq!(
        produced.last().map(String::as_str),
        Some("width counts, marks walked 16, marks with another standing 15, days 3, narrowest field width 0 at 400 seconds on the first day")
    );
    let lines = golden_lines("meet-width-golden.txt");
    let marks = mark_lines(&lines);
    let second = marks[1];
    assert!(
        second.starts_with("mark 2, at 400 seconds on the first day,"),
        "the second mark stands at 400 seconds on the first day, {}",
        second
    );
    assert_eq!(width_of(second), Some(0.0));
    let mut figured = 0usize;
    for (place, mark) in marks.iter().enumerate() {
        if let Some(width) = width_of(mark) {
            figured += 1;
            assert!(width >= 0.0, "{}", mark);
            if place != 1 {
                assert!(
                    width > 0.0,
                    "no other mark states the narrowest width, {}",
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
    let width = meet_width(&meet).expect("the width read");
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
    let mut width_sections: Vec<Vec<&String>> = Vec::new();
    for line in &width {
        if line.starts_with("day, the ") {
            width_sections.push(Vec::new());
        }
        if let Some(section) = width_sections.last_mut() {
            section.push(line);
        }
    }
    assert_eq!(sections.len(), 3);
    assert_eq!(width_sections.len(), 3);
    let named = [141.818182, 108.0, 30.0];
    for (index, (section, width_section)) in sections.iter().zip(width_sections.iter()).enumerate()
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
        let expected = figures[figures.len() - 1] - figures[0];
        assert!(
            (expected - named[index]).abs() <= 0.000002,
            "the day close width counted from the golden, day {}",
            index
        );
        let last = width_section
            .iter()
            .rfind(|line| line.starts_with("mark "))
            .expect("the day holds a mark");
        let stated = width_of(last).expect("the last mark states its field width");
        assert!(
            (stated - expected).abs() <= 0.000002,
            "the day close width figure, {}",
            last
        );
    }
}

#[test]
fn test_the_readme_width_block_stands_byte_for_byte_as_the_shipped_golden() {
    let readme = readme_text();
    let section = usage_section(&readme, "### width");
    let block = example_block(&section);
    assert_eq!(
        block.first().map(String::as_str),
        Some("$ paceghost width examples/meet.json")
    );
    assert_eq!(block[1..], golden_lines("meet-width-golden.txt")[..]);
}

#[test]
fn test_the_readme_width_usage_section_stands_in_its_standing_form() {
    let readme = readme_text();
    let section = usage_section(&readme, "### width");
    assert_eq!(section.first().map(String::as_str), Some("### width"));
    let lines: Vec<&str> = readme.lines().collect();
    let midrange = lines
        .iter()
        .position(|line| *line == "### midrange")
        .expect("the midrange heading stands");
    let width = lines
        .iter()
        .position(|line| *line == "### width")
        .expect("the width heading stands");
    assert!(
        midrange < width,
        "the width section stands after the midrange section"
    );
    assert!(section.contains(
        &"The width command prints the meet read mark by mark, naming the holder after each mark, the record figure after it, and the field width held after it by the field behind its holder, the farthest standing less the nearest standing of that field."
            .to_string()
    ));
    let block = example_block(&section);
    assert_eq!(
        block.first().map(String::as_str),
        Some("$ paceghost width examples/meet.json")
    );
}
