//! Tests for the tails goldens of PG-36 over the standing
//! example meets.
//!
//! Written for Round 17 in the shape of the Round 16 example
//! tests. The two byte for byte pins fail before the round
//! by the absence of the goldens and of the reading, and the
//! hand count pins close the closing counts of both goldens
//! against the counts stated in the design note. Pin by pin
//! under the round's mutations: pin 16 fails under T-1 at
//! every figured mark, the record standing strictly below
//! the nearest figure at all fifteen, under T-2 at the
//! second mark only, the shipped meet's one mark at a field
//! of one behind the holder, whose line unfigures, its
//! fields of two behind standing at the same tail count
//! under both constructions so the pin passes there by
//! design, under T-3 from the third mark on, the first mark
//! whose tails separate, the second mark's sole figure
//! making its two tails one so the pin passes there by
//! design, and under T-5, the lower closing naming the
//! seventh mark in place of the sixth. Pin 17 fails under
//! T-1 at every mark and under T-2 at every mark, the
//! constructed meet standing at one behind throughout, so
//! every line unfigures, and under T-5, both closings
//! naming the sixth mark in place of the third; under T-3
//! it passes by design at every mark, the two tails
//! coinciding over a field of one, and the pin claims no
//! T-3 failure. Pin 18 fails under T-5 on its lower clause,
//! the closing naming 1200 seconds in place of 1100 while
//! the upper clause stands, its margin standing at one mark
//! only. Pin 19 fails under T-5 on both clauses. Pin 20
//! fails under T-1, the sixth mark's lower tail dropping as
//! the holder figures into its field, and under T-5 it
//! stands by design, the pin reading mark lines and not the
//! closing. Pin 21 fails under T-1 at every close and under
//! T-2 it passes by design at all three closes, each close
//! field standing at two behind the holder, where the two
//! counts agree, and the pin claims no T-2 failure. The
//! limitation of this file stands stated in the design and
//! repeated here: both shipped fields stand at three or
//! fewer, the holder counted, so the tails figures on both
//! goldens equal the nearest and farthest figures, and
//! under a nearest and farthest degeneration of the tail
//! figures these six pins pass while the construction is
//! wrong. The separation at fields of three and more behind
//! the holder stands in the hand built pins of
//! rust/tests/test_tails.rs and in the generated pin, whose
//! meets are built or generated at those fields.

use std::path::PathBuf;

use paceghost::meet::{load_meet_text, meet_tails, Meet};

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

/// The lower tail margin a tails mark line states, where
/// the line carries another standing.
fn lower_tail_margin_of(line: &str) -> Option<f64> {
    line.split(", lower tail margin ")
        .nth(1)
        .and_then(|rest| rest.split(", upper tail standing ").next())
        .and_then(|word| word.parse().ok())
}

/// The tail figures a tails mark line states, its lower
/// tail figure and its upper tail figure, where the line
/// carries another standing.
fn tail_figures_of(line: &str) -> Option<(f64, f64)> {
    let lower = line
        .split(", lower tail standing ")
        .nth(1)
        .and_then(|rest| rest.split(", lower tail margin ").next())
        .and_then(|word| word.parse().ok())?;
    let upper = line
        .split(", upper tail standing ")
        .nth(1)
        .and_then(|rest| rest.split(", upper tail margin ").next())
        .and_then(|word| word.parse().ok())?;
    Some((lower, upper))
}

#[test]
fn test_the_shipped_tails_golden_stands_byte_for_byte() {
    let meet = load_example("meet.json");
    let lines = meet_tails(&meet).expect("the tails read");
    assert_eq!(lines, golden_lines("meet-tails-golden.txt"));
}

#[test]
fn test_the_constructed_tails_golden_stands_byte_for_byte() {
    let meet = load_example("constructed-meet.json");
    let lines = meet_tails(&meet).expect("the tails read");
    assert_eq!(lines, golden_lines("constructed-meet-tails-golden.txt"));
}

#[test]
fn test_the_shipped_tails_counts_close_as_counted_by_hand() {
    let meet = load_example("meet.json");
    let lines = meet_tails(&meet).expect("the tails read");
    let marks = mark_lines(&lines);
    assert_eq!(marks.len(), 16);
    let figured = marks
        .iter()
        .filter(|line| line.contains(", lower tail standing "))
        .count();
    assert_eq!(figured, 15);
    assert_eq!(
        lines.last().map(String::as_str),
        Some("tails counts, marks walked 16, marks with another standing 15, days 3, closest lower tail margin 58.181818 at 1100 seconds on the first day, closest upper tail margin 90 at 400 seconds on the first day")
    );
}

#[test]
fn test_the_constructed_tails_counts_close_as_counted_by_hand() {
    let meet = load_example("constructed-meet.json");
    let lines = meet_tails(&meet).expect("the tails read");
    let marks = mark_lines(&lines);
    assert_eq!(marks.len(), 7);
    let figured = marks
        .iter()
        .filter(|line| line.contains(", lower tail standing "))
        .count();
    assert_eq!(figured, 7);
    assert_eq!(
        lines.last().map(String::as_str),
        Some("tails counts, marks walked 7, marks with another standing 7, days 3, closest lower tail margin 50 at 100 seconds on the second day, closest upper tail margin 50 at 100 seconds on the second day")
    );
}

#[test]
fn test_the_shipped_goldens_closest_lower_tail_margin_names_the_sixth_mark() {
    let lines = golden_lines("meet-tails-golden.txt");
    let marks = mark_lines(&lines);
    let sixth = marks[5];
    assert!(
        sixth.starts_with("mark 6, at 1100 seconds on the first day,"),
        "the sixth mark stands at 1100 seconds on the first day, {}",
        sixth
    );
    assert_eq!(lower_tail_margin_of(sixth), Some(58.181818));
    for earlier in &marks[..5] {
        if let Some(margin) = lower_tail_margin_of(earlier) {
            assert!(
                margin > 58.181818,
                "no earlier mark states a smaller lower tail margin, {}",
                earlier
            );
        }
    }
    let mut figured = 0usize;
    for mark in &marks {
        if let Some(margin) = lower_tail_margin_of(mark) {
            figured += 1;
            assert!(margin >= 58.181818, "{}", mark);
        }
    }
    assert_eq!(figured, 15);
}

#[test]
fn test_the_day_close_relation_holds_over_the_shipped_example() {
    let meet = load_example("meet.json");
    let tails = meet_tails(&meet).expect("the tails read");
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
    let mut tails_sections: Vec<Vec<&String>> = Vec::new();
    for line in &tails {
        if line.starts_with("day, the ") {
            tails_sections.push(Vec::new());
        }
        if let Some(section) = tails_sections.last_mut() {
            section.push(line);
        }
    }
    assert_eq!(sections.len(), 3);
    assert_eq!(tails_sections.len(), 3);
    let named = [(998.181818, 1140.0), (900.0, 1008.0), (720.0, 750.0)];
    for (index, (section, tails_section)) in sections.iter().zip(tails_sections.iter()).enumerate()
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
        figures.sort_by(|a, b| a.partial_cmp(b).expect("figures order"));
        let count = figures.len().div_ceil(2);
        let expected_lower = figures[..count].iter().sum::<f64>() / count as f64;
        let expected_upper = figures[figures.len() - count..].iter().sum::<f64>() / count as f64;
        assert!(
            (expected_lower - named[index].0).abs() <= 0.000002,
            "the day close lower tail figure counted from the golden, day {}",
            index
        );
        assert!(
            (expected_upper - named[index].1).abs() <= 0.000002,
            "the day close upper tail figure counted from the golden, day {}",
            index
        );
        let last = tails_section
            .iter()
            .rfind(|line| line.starts_with("mark "))
            .expect("the day holds a mark");
        let (stated_lower, stated_upper) =
            tail_figures_of(last).expect("the last mark states its tails");
        assert!(
            (stated_lower - expected_lower).abs() <= 0.000002,
            "the day close lower tail figure, {}",
            last
        );
        assert!(
            (stated_upper - expected_upper).abs() <= 0.000002,
            "the day close upper tail figure, {}",
            last
        );
    }
}
