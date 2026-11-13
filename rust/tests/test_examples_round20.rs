//! Tests for the quartiles goldens of PG-42 over the
//! standing example meets, and for the README Usage
//! sections the round seats.
//!
//! Written for Round 20 in the shape of the Round 19
//! example tests. The two byte for byte pins fail before
//! the round by the absence of the goldens and of the
//! reading. Pin by pin under the round's mutations: pin
//! 16 fails under Q-1 at the first figured mark onward,
//! under Q-3, its quartile margins moving to the mean
//! standing's margins at its fields of two and three,
//! under Q-4 on its lower tie, its closest lower
//! quartile margin standing tied at the sixth and
//! seventh marks so the loosened scan names the seventh,
//! under Q-5, and under Q-6, and it passes under Q-2 by
//! the shipped coincidence, the shipped meet's fields
//! behind the holder counting three or fewer at every
//! figured mark, where the quartiles are the tails whole.
//! Pin 17 fails under Q-1, under Q-4 on the constructed
//! ties, both its closest margins standing tied at the
//! third and sixth marks so the loosened scans name the
//! sixth, under Q-5, and under Q-6, and it passes under
//! Q-2 and Q-3 by design, the constructed meet standing
//! at one behind throughout, so the quartiles of a field
//! of one and the mean of a field of one are the figure
//! whole. Pins 18 and 19 pass under all six mutations by
//! design, counting the goldens' own lines by hand, the
//! counts they state standing in the golden text the
//! mutations do not reach, and their discrimination
//! stands with pins 16 and 17, which bind the produced
//! readings to those goldens byte for byte. Pin 20 fails
//! under Q-1, Q-3, Q-4, its designed catcher on the lower
//! scan, Q-5, and Q-6, and it passes under Q-2 on the
//! tails' own closest values, the shipped fields counting
//! three or fewer where the tails' closest margins are
//! the quartiles' own. Pin 21 passes under Q-2, Q-3, and
//! Q-4 by design, its closes standing at fields of two
//! where the quartiles are the tails, and comparing
//! figures at the closes and never a margin or a closest
//! naming, and it fails under Q-1, Q-5, and Q-6. The
//! closes of pin 21 stand at fields of two behind the
//! holder at all three days, counted from the standings
//! golden itself, where each quartile of two figures is
//! the figure at its own end of the pair. Pins 22 to 27
//! pass under all six mutations by construction,
//! comparing two standing texts, the README blocks and
//! the golden files, which no mutation of the reading
//! reaches, and their discrimination stands at the pre
//! round tree, pins 22 and 23 where neither the golden
//! nor the README quartiles section stands, and pins 24
//! to 27 where the README marks, standings, traces, and
//! margins sections do not stand, their goldens standing
//! on the pre round tree since their own rounds.

use std::path::PathBuf;

use paceghost::meet::{load_meet_text, meet_quartiles, Meet};

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

/// The lower quartile margin a quartiles mark line
/// states, where the line carries another standing.
fn lower_margin_of(line: &str) -> Option<f64> {
    line.split(", lower quartile margin ")
        .nth(1)
        .and_then(|rest| rest.split(',').next())
        .and_then(|word| word.parse().ok())
}

/// The upper quartile margin a quartiles mark line
/// states, where the line carries another standing.
fn upper_margin_of(line: &str) -> Option<f64> {
    line.split(", upper quartile margin ")
        .nth(1)
        .and_then(|rest| rest.split(',').next())
        .and_then(|word| word.parse().ok())
}

/// The lower quartile figure a quartiles mark line
/// states, where the line carries another standing.
fn lower_figure_of(line: &str) -> Option<f64> {
    line.split(", lower quartile standing ")
        .nth(1)
        .and_then(|rest| rest.split(", lower quartile margin ").next())
        .and_then(|word| word.parse().ok())
}

/// The upper quartile figure a quartiles mark line
/// states, where the line carries another standing.
fn upper_figure_of(line: &str) -> Option<f64> {
    line.split(", upper quartile standing ")
        .nth(1)
        .and_then(|rest| rest.split(", upper quartile margin ").next())
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
fn test_the_shipped_quartiles_golden_stands_byte_for_byte() {
    let meet = load_example("meet.json");
    let lines = meet_quartiles(&meet).expect("the quartiles read");
    assert_eq!(lines, golden_lines("meet-quartiles-golden.txt"));
}

#[test]
fn test_the_constructed_quartiles_golden_stands_byte_for_byte() {
    let meet = load_example("constructed-meet.json");
    let lines = meet_quartiles(&meet).expect("the quartiles read");
    assert_eq!(lines, golden_lines("constructed-meet-quartiles-golden.txt"));
}

#[test]
fn test_the_shipped_quartiles_counts_close_as_counted_by_hand() {
    let lines = golden_lines("meet-quartiles-golden.txt");
    let marks = mark_lines(&lines);
    assert_eq!(marks.len(), 16);
    let figured = marks
        .iter()
        .filter(|line| line.contains(", lower quartile standing "))
        .count();
    assert_eq!(figured, 15);
    assert_eq!(
        lines.last().map(String::as_str),
        Some("quartiles counts, marks walked 16, marks with another standing 15, days 3, closest lower quartile margin 58.181818 at 1100 seconds on the first day, closest upper quartile margin 90 at 400 seconds on the first day")
    );
}

#[test]
fn test_the_constructed_quartiles_counts_close_as_counted_by_hand() {
    let lines = golden_lines("constructed-meet-quartiles-golden.txt");
    let marks = mark_lines(&lines);
    assert_eq!(marks.len(), 7);
    let figured = marks
        .iter()
        .filter(|line| line.contains(", lower quartile standing "))
        .count();
    assert_eq!(figured, 7);
    assert_eq!(
        lines.last().map(String::as_str),
        Some("quartiles counts, marks walked 7, marks with another standing 7, days 3, closest lower quartile margin 50 at 100 seconds on the second day, closest upper quartile margin 50 at 100 seconds on the second day")
    );
}

#[test]
fn test_the_shipped_goldens_closest_quartile_margins_name_the_earliest_marks() {
    let meet = load_example("meet.json");
    let produced = meet_quartiles(&meet).expect("the quartiles read");
    assert_eq!(
        produced.last().map(String::as_str),
        Some("quartiles counts, marks walked 16, marks with another standing 15, days 3, closest lower quartile margin 58.181818 at 1100 seconds on the first day, closest upper quartile margin 90 at 400 seconds on the first day")
    );
    let lines = golden_lines("meet-quartiles-golden.txt");
    let marks = mark_lines(&lines);
    let sixth = marks[5];
    assert!(
        sixth.starts_with("mark 6, at 1100 seconds on the first day,"),
        "the sixth mark stands at 1100 seconds on the first day, {}",
        sixth
    );
    assert_eq!(lower_margin_of(sixth), Some(58.181818));
    for earlier in &marks[..5] {
        if let Some(margin) = lower_margin_of(earlier) {
            assert!(
                margin > 58.181818,
                "no earlier mark states a smaller lower quartile margin, {}",
                earlier
            );
        }
    }
    let seventh = marks[6];
    assert_eq!(lower_margin_of(seventh), Some(58.181818));
    let mut figured = 0usize;
    for mark in &marks {
        if let Some(margin) = lower_margin_of(mark) {
            figured += 1;
            assert!(margin >= 58.181818, "{}", mark);
        }
    }
    assert_eq!(figured, 15);
    let second = marks[1];
    assert!(
        second.starts_with("mark 2, at 400 seconds on the first day,"),
        "the second mark stands at 400 seconds on the first day, {}",
        second
    );
    assert_eq!(upper_margin_of(second), Some(90.0));
    for (place, mark) in marks.iter().enumerate() {
        if place == 1 {
            continue;
        }
        if let Some(margin) = upper_margin_of(mark) {
            assert!(
                margin > 90.0,
                "no other mark states the closest upper quartile margin, {}",
                mark
            );
        }
    }
}

#[test]
fn test_the_day_close_relation_holds_over_the_shipped_example() {
    let meet = load_example("meet.json");
    let quartiles = meet_quartiles(&meet).expect("the quartiles read");
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
    let mut quartiles_sections: Vec<Vec<&String>> = Vec::new();
    for line in &quartiles {
        if line.starts_with("day, the ") {
            quartiles_sections.push(Vec::new());
        }
        if let Some(section) = quartiles_sections.last_mut() {
            section.push(line);
        }
    }
    assert_eq!(sections.len(), 3);
    assert_eq!(quartiles_sections.len(), 3);
    let named = [(998.181818, 1140.0), (900.0, 1008.0), (720.0, 750.0)];
    for (index, (section, quartiles_section)) in
        sections.iter().zip(quartiles_sections.iter()).enumerate()
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
        let count = (figures.len() + 1) / 2;
        let median_of = |part: &[f64]| {
            if part.len() % 2 == 1 {
                part[part.len() / 2]
            } else {
                (part[part.len() / 2 - 1] + part[part.len() / 2]) / 2.0
            }
        };
        let expected_lower = median_of(&figures[..count]);
        let expected_upper = median_of(&figures[figures.len() - count..]);
        assert!(
            (expected_lower - named[index].0).abs() <= 0.000002,
            "the day close lower quartile counted from the golden, day {}",
            index
        );
        assert!(
            (expected_upper - named[index].1).abs() <= 0.000002,
            "the day close upper quartile counted from the golden, day {}",
            index
        );
        let last = quartiles_section
            .iter()
            .rfind(|line| line.starts_with("mark "))
            .expect("the day holds a mark");
        let stated_lower = lower_figure_of(last).expect("the last mark states its lower quartile");
        let stated_upper = upper_figure_of(last).expect("the last mark states its upper quartile");
        assert!(
            (stated_lower - expected_lower).abs() <= 0.000002,
            "the day close lower quartile figure, {}",
            last
        );
        assert!(
            (stated_upper - expected_upper).abs() <= 0.000002,
            "the day close upper quartile figure, {}",
            last
        );
    }
}

#[test]
fn test_the_readme_quartiles_block_stands_byte_for_byte_as_the_shipped_golden() {
    let readme = readme_text();
    let section = usage_section(&readme, "### quartiles");
    let block = example_block(&section);
    assert_eq!(
        block.first().map(String::as_str),
        Some("$ paceghost quartiles examples/meet.json")
    );
    assert_eq!(block[1..], golden_lines("meet-quartiles-golden.txt")[..]);
}

#[test]
fn test_the_readme_quartiles_usage_section_stands_in_its_standing_form() {
    let readme = readme_text();
    let section = usage_section(&readme, "### quartiles");
    assert_eq!(section.first().map(String::as_str), Some("### quartiles"));
    let lines: Vec<&str> = readme.lines().collect();
    let median = lines
        .iter()
        .position(|line| *line == "### median")
        .expect("the median heading stands");
    let quartiles = lines
        .iter()
        .position(|line| *line == "### quartiles")
        .expect("the quartiles heading stands");
    assert!(
        median < quartiles,
        "the quartiles section stands after the median section"
    );
    assert!(section.contains(
        &"The quartiles command prints the meet read mark by mark, naming the holder after each mark, the record figure after it, the lower quartile standing and the upper quartile standing held after it by the field behind its holder, and the lower quartile margin and the upper quartile margin by which the record stands ahead of each."
            .to_string()
    ));
    let block = example_block(&section);
    assert_eq!(
        block.first().map(String::as_str),
        Some("$ paceghost quartiles examples/meet.json")
    );
}

#[test]
fn test_the_readme_marks_block_stands_byte_for_byte_as_the_shipped_golden() {
    let readme = readme_text();
    let section = usage_section(&readme, "### marks");
    let block = example_block(&section);
    assert_eq!(
        block.first().map(String::as_str),
        Some("$ paceghost marks examples/meet.json")
    );
    assert_eq!(block[1..], golden_lines("meet-marks-golden.txt")[..]);
}

#[test]
fn test_the_readme_standings_block_stands_byte_for_byte_as_the_shipped_golden() {
    let readme = readme_text();
    let section = usage_section(&readme, "### standings");
    let block = example_block(&section);
    assert_eq!(
        block.first().map(String::as_str),
        Some("$ paceghost standings examples/meet.json")
    );
    assert_eq!(block[1..], golden_lines("meet-standings-golden.txt")[..]);
}

#[test]
fn test_the_readme_traces_block_stands_byte_for_byte_as_the_shipped_golden() {
    let readme = readme_text();
    let section = usage_section(&readme, "### traces");
    let block = example_block(&section);
    assert_eq!(
        block.first().map(String::as_str),
        Some("$ paceghost traces examples/meet.json")
    );
    assert_eq!(block[1..], golden_lines("meet-traces-golden.txt")[..]);
}

#[test]
fn test_the_readme_margins_block_stands_byte_for_byte_as_the_shipped_golden() {
    let readme = readme_text();
    let section = usage_section(&readme, "### margins");
    let block = example_block(&section);
    assert_eq!(
        block.first().map(String::as_str),
        Some("$ paceghost margins examples/meet.json")
    );
    assert_eq!(block[1..], golden_lines("meet-margins-golden.txt")[..]);
}
