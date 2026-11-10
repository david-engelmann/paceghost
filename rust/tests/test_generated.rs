//! The generated fixture family of Round 12, a seeded generator
//! and its relation pins standing together in one file.
//!
//! The parameters are the standalone audit's, and they are
//! justified from public agent run reporting as numbers only.
//! Public run reports state counts and ranges, splits per run
//! and timed reads per run in the low tens at most, field sizes
//! in the tens to just past a hundred runs over a handful of
//! models, and multi day meets in the single digits of days
//! with a handful to a dozen runs a day. The generator takes
//! those ranges as its parameters, splits per run from 3 to
//! 12, reads per run from 3 to 10, season fields of 40 to 120
//! runs over 3 to 6 models, and meets of 5 to 10 days with 6
//! to 12 runs a day. No record and no text is carried from any
//! source. Every word of the generated content is synthetic
//! and written for this repo, so SETUP-4 holds untouched.
//!
//! The generator is a plain integer recurrence written here,
//! because the crate's dependency set holds no randomness
//! source and none is added for a test fixture. Its seeds are
//! stated constants, so the same seeds produce the same
//! fixtures on every run, and every assertion names the seed
//! it failed under, so a failure reproduces from the seed
//! alone. Every generated text loads clean through its own
//! loader, and that round trip is itself a pin, because a
//! generator that produces a file its loader refuses has
//! produced nothing. The fixtures are pinned by the closing
//! relations the contract already states, never by hand
//! counted goldens, and the relations, not the fixture, are
//! what the suite believes.

use paceghost::meet::{
    load_meet_text, meet_days, meet_margins, meet_marks, meet_middle, meet_reign, meet_report,
    meet_spread, meet_standings, meet_table, meet_tails, meet_traces,
};
use paceghost::pynum::Num;
use paceghost::runs::load_run_text;
use paceghost::season::{board_report, load_field_text, season_report};

const RUN_SEED: u64 = 0x12_00_01;
const FIELD_SEED: u64 = 0x12_00_02;
const MEET_SEED: u64 = 0x12_00_03;

/// A plain integer recurrence, a linear congruential step over
/// the wrapping u64 state, read from its high half.
struct Gen {
    state: u64,
}

impl Gen {
    fn new(seed: u64) -> Gen {
        Gen {
            state: seed ^ 0x9E37_79B9_7F4A_7C15,
        }
    }

    fn next(&mut self) -> u64 {
        self.state = self
            .state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        self.state >> 33
    }

    /// A value in the range low..=high, both ends standing.
    fn range(&mut self, low: u64, high: u64) -> u64 {
        low + self.next() % (high - low + 1)
    }
}

fn run_text(gen: &mut Gen) -> (String, usize) {
    let split_count = gen.range(3, 12) as usize;
    let mut elapsed = 0i64;
    let mut splits: Vec<String> = Vec::new();
    for index in 0..split_count {
        elapsed += gen.range(20, 400) as i64;
        splits.push(format!(
            "{{\"phase\": \"phase {}\", \"elapsed_seconds\": {}}}",
            index, elapsed
        ));
    }
    let total = elapsed;
    let window = total + gen.range(100, 900) as i64;
    let read_count = gen.range(3, 10) as usize;
    let mut marks: Vec<i64> = Vec::new();
    while marks.len() < read_count {
        let mark = gen.range(1, total as u64 - 1) as i64;
        if !marks.contains(&mark) {
            marks.push(mark);
        }
    }
    marks.sort_unstable();
    let mut usage = 0i64;
    let mut reads: Vec<String> = Vec::new();
    for mark in &marks {
        usage += gen.range(5, 120) as i64;
        reads.push(format!(
            "{{\"elapsed_seconds\": {}, \"usage\": {}}}",
            mark, usage
        ));
    }
    let total_usage = usage + gen.range(0, 200) as i64;
    let budget = total_usage + gen.range(0, 300) as i64;
    let text = format!(
        "{{\"task\": \"generated task\", \"window_seconds\": {}, \"runner\": \"gen runner\", \"recorded_at\": \"2026-02-01\", \"total_seconds\": {}, \"splits\": [{}], \"reads\": [{}], \"usage\": {}, \"budget\": {}}}",
        window,
        total,
        splits.join(", "),
        reads.join(", "),
        total_usage,
        budget
    );
    (text, read_count)
}

#[test]
fn test_generated_runs_load_clean_and_the_board_counts_the_reads() {
    for index in 0..8u64 {
        let seed = RUN_SEED + index;
        let mut gen = Gen::new(seed);
        let (text, read_count) = run_text(&mut gen);
        let (run, defects) = load_run_text(&text);
        assert_eq!(defects, Vec::<String>::new(), "seed {}", seed);
        let run = run.expect("the generated run loads");
        let lines = board_report(&run, &Num::from_i64(1_000_000));
        assert_eq!(lines.len(), read_count + 1, "seed {}", seed);
    }
}

fn field_text(gen: &mut Gen) -> (String, usize) {
    let model_count = gen.range(3, 6) as usize;
    let run_count = gen.range(40, 120) as usize;
    let mut runs: Vec<String> = Vec::new();
    for _ in 0..run_count {
        let model = gen.range(0, model_count as u64 - 1);
        let window = gen.range(600, 3600) as i64;
        let total = gen.range(60, window as u64) as i64;
        let usage = gen.range(20, 900) as i64;
        let budget = usage + gen.range(0, 400) as i64;
        runs.push(format!(
            "{{\"task\": \"generated field task\", \"window_seconds\": {}, \"runner\": \"gen runner\", \"recorded_at\": \"2026-02-01\", \"total_seconds\": {}, \"splits\": [{{\"phase\": \"only\", \"elapsed_seconds\": {}}}], \"usage\": {}, \"budget\": {}, \"model\": \"model {}\"}}",
            window, total, total, usage, budget, model
        ));
    }
    (format!("{{\"runs\": [{}]}}", runs.join(", ")), run_count)
}

#[test]
fn test_generated_fields_load_clean_and_the_season_blocks_sum_to_the_field() {
    for index in 0..6u64 {
        let seed = FIELD_SEED + index;
        let mut gen = Gen::new(seed);
        let (text, run_count) = field_text(&mut gen);
        let (runs, defects) = load_field_text(&text);
        assert_eq!(defects, Vec::<String>::new(), "seed {}", seed);
        let runs = runs.expect("the generated field loads");
        assert_eq!(runs.len(), run_count, "seed {}", seed);
        let lines = season_report(&runs, &Num::from_i64(450));
        let mut raced_sum = 0usize;
        let mut current_raced: Option<usize> = None;
        for line in &lines {
            if let Some(rest) = line.strip_prefix("runs raced ") {
                let raced: usize = rest.parse().expect("a raced count parses");
                raced_sum += raced;
                current_raced = Some(raced);
            }
            if let Some(rest) = line.strip_prefix("head to head against the ghost, wins ") {
                let wins: usize = rest
                    .split(' ')
                    .next()
                    .and_then(|word| word.parse().ok())
                    .expect("a win count parses");
                let raced = current_raced.expect("a block stands open");
                assert!(wins <= raced, "seed {}", seed);
            }
        }
        assert_eq!(raced_sum, run_count, "seed {}", seed);
    }
}

fn meet_text(gen: &mut Gen) -> String {
    let day_count = gen.range(5, 10) as usize;
    let pool = gen.range(6, 10) as usize;
    let mut days: Vec<String> = Vec::new();
    for _ in 0..day_count {
        let window = gen.range(800, 2000) as i64;
        let run_count = gen.range(6, 12) as usize;
        let mut labels: Vec<usize> = Vec::new();
        while labels.len() < run_count.min(pool) {
            let label = gen.range(0, pool as u64 - 1) as usize;
            if !labels.contains(&label) {
                labels.push(label);
            }
        }
        let mut runs: Vec<String> = Vec::new();
        for label in &labels {
            let read_count = gen.range(0, 4) as usize;
            let mut marks: Vec<i64> = Vec::new();
            while marks.len() < read_count {
                let mark = gen.range(10, window as u64 - 100) as i64;
                if !marks.contains(&mark) {
                    marks.push(mark);
                }
            }
            marks.sort_unstable();
            let mut usage = 0i64;
            let mut reads: Vec<String> = Vec::new();
            for mark in &marks {
                usage += gen.range(5, 150) as i64;
                reads.push(format!(
                    "{{\"elapsed_seconds\": {}, \"usage\": {}}}",
                    mark, usage
                ));
            }
            let total_usage = usage + gen.range(50, 500) as i64;
            if reads.is_empty() {
                runs.push(format!(
                    "{{\"runner\": \"gen {}\", \"usage\": {}}}",
                    label, total_usage
                ));
            } else {
                runs.push(format!(
                    "{{\"runner\": \"gen {}\", \"usage\": {}, \"reads\": [{}]}}",
                    label,
                    total_usage,
                    reads.join(", ")
                ));
            }
        }
        days.push(format!(
            "{{\"window_seconds\": {}, \"operator\": \"gen {}\", \"runs\": [{}]}}",
            window,
            labels[0],
            runs.join(", ")
        ));
    }
    format!("{{\"days\": [{}]}}", days.join(", "))
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

#[test]
fn test_generated_meets_load_clean_and_the_readings_close_over_them() {
    for index in 0..8u64 {
        let seed = MEET_SEED + index;
        let mut gen = Gen::new(seed);
        let text = meet_text(&mut gen);
        let (meet, defects) = load_meet_text(&text);
        assert_eq!(defects, Vec::<String>::new(), "seed {}", seed);
        let meet = meet.expect("the generated meet loads");

        let marks = meet_marks(&meet).expect("the marks read");
        let days = meet_days(&meet).expect("the days read");
        let reign = meet_reign(&meet).expect("the reign reads");
        let report = meet_report(&meet).expect("the report reads");
        let table = meet_table(&meet).expect("the table reads");
        let standings = meet_standings(&meet).expect("the standings read");

        // The marks walked of PG-23 equal the sum of the marks
        // of the day lines of the days reading.
        let walked = count_after(marks.last().expect("the marks close"), "marks walked ");
        let per_day: usize = days
            .iter()
            .filter(|line| line.starts_with("day, the "))
            .map(|line| count_after(line, "marks "))
            .sum();
        assert_eq!(walked, per_day, "seed {}", seed);

        // The marks that set or took the record equal the spell
        // count of the reign reading, as PG-23 states.
        let set_or_took = marks
            .iter()
            .filter(|line| {
                line.contains("set the standing record") || line.contains("took the record from")
            })
            .count();
        let spells = count_after(
            reign
                .iter()
                .find(|line| line.starts_with("reign counts,"))
                .expect("the reign closes"),
            "spells ",
        );
        assert_eq!(set_or_took, spells, "seed {}", seed);

        // The held counts of the meet table sum to the count of
        // marks after which a record stood, the sum of the
        // spell lengths of the reign reading, as PG-16 states.
        let held_sum: usize = table
            .iter()
            .filter(|line| line.starts_with("competitor "))
            .map(|line| count_after(line, "record held after "))
            .sum();
        let spell_lengths: usize = reign
            .iter()
            .filter(|line| line.starts_with("spell, "))
            .map(|line| {
                let tail = line.rsplit(", ").next().expect("a spell line ends");
                tail.split(' ')
                    .next()
                    .and_then(|word| word.parse::<usize>().ok())
                    .expect("a spell names its length")
            })
            .sum();
        assert_eq!(held_sum, spell_lengths, "seed {}", seed);

        // The closings of PG-26. The section closing lines are
        // the closing lines of the meet report in order, the
        // standings held is the sum of the section openings,
        // and no competitor falls back into the none form.
        let report_closes: Vec<&String> = report
            .iter()
            .filter(|line| line.starts_with("close of the "))
            .collect();
        let section_closes: Vec<&String> = standings
            .iter()
            .filter(|line| line.starts_with("close of the "))
            .collect();
        assert_eq!(section_closes, report_closes, "seed {}", seed);
        let opened: usize = standings
            .iter()
            .filter(|line| line.starts_with("day, the "))
            .map(|line| count_after(line, "standings "))
            .sum();
        let held = count_after(
            standings.last().expect("the standings close"),
            "standings held ",
        );
        assert_eq!(held, opened, "seed {}", seed);
        let mut stood: Vec<bool> = Vec::new();
        let mut in_section = false;
        let mut competitor = 0usize;
        for line in &standings {
            if line.starts_with("day, the ") {
                in_section = true;
                competitor = 0;
                continue;
            }
            if in_section && line.starts_with("competitor ") {
                if stood.len() <= competitor {
                    stood.push(false);
                }
                if line.contains("no standing projection at the close") {
                    assert!(!stood[competitor], "seed {}", seed);
                } else {
                    stood[competitor] = true;
                }
                competitor += 1;
            }
        }

        // The competitor lines of the last section state the
        // standings the meet table states at the close.
        let last_section: Vec<&String> = standings
            .iter()
            .rev()
            .take_while(|line| !line.starts_with("day, the "))
            .filter(|line| line.starts_with("competitor "))
            .collect();
        let table_lines: Vec<&String> = table
            .iter()
            .filter(|line| line.starts_with("competitor "))
            .collect();
        assert_eq!(last_section.len(), table_lines.len(), "seed {}", seed);
        for (line, table_line) in last_section.iter().rev().zip(table_lines.iter()) {
            if line.contains("no standing projection at the close") {
                assert!(
                    table_line.ends_with("no standing projection at the close"),
                    "seed {}",
                    seed
                );
            } else {
                let figure = line
                    .strip_prefix("competitor ")
                    .and_then(|rest| rest.split(", standing projection ").nth(1))
                    .and_then(|rest| rest.strip_suffix(" at the close"))
                    .expect("a competitor line names its figure");
                assert!(
                    table_line.ends_with(&format!("standing projection {} at the close", figure)),
                    "seed {}",
                    seed
                );
            }
        }
    }
}

// The traces pin of the generated family, added with Round
// 13 beside the standings closings above. It fails before the
// round by the absence of meet_traces from this suite's
// imports, and on the built tree it fails against a traces
// construction that drifts from the standings reading over
// generated shapes, which is the drift charge the family
// exists to catch.
#[test]
fn test_generated_meets_close_under_the_traces_reading() {
    for index in 0..8u64 {
        let seed = MEET_SEED + index;
        let mut gen = Gen::new(seed);
        let text = meet_text(&mut gen);
        let (meet, defects) = load_meet_text(&text);
        assert_eq!(defects, Vec::<String>::new(), "seed {}", seed);
        let meet = meet.expect("the generated meet loads");

        let standings = meet_standings(&meet).expect("the standings read");
        let traces = meet_traces(&meet).expect("the traces read");

        // The closings of PG-28, computed from the printed
        // lines of the two readings alone. Every traces day
        // line states the close form the standings reading
        // states for that competitor in that day's section,
        // the transposition relation.
        fn close_form(line: &str) -> String {
            if line.contains("no standing projection at the close") {
                return "no standing projection at the close".to_string();
            }
            let start = line
                .find("standing projection ")
                .expect("a figured line names its figure");
            line[start..].to_string()
        }
        let mut by_day: Vec<Vec<String>> = Vec::new();
        for line in &standings {
            if line.starts_with("day, the ") {
                by_day.push(Vec::new());
            }
            if let Some(section) = by_day.last_mut() {
                section.push(line.clone());
            }
        }
        let mut by_competitor: Vec<Vec<String>> = Vec::new();
        for line in traces.iter().skip(1) {
            if line.starts_with("competitor ") {
                by_competitor.push(Vec::new());
            }
            if let Some(section) = by_competitor.last_mut() {
                section.push(line.clone());
            }
        }
        for (competitor, section) in by_competitor.iter().enumerate() {
            let day_lines: Vec<&String> = section
                .iter()
                .filter(|line| line.starts_with("the "))
                .collect();
            assert_eq!(day_lines.len(), by_day.len(), "seed {}", seed);
            let mut stood = false;
            for (day_index, day_line) in day_lines.iter().enumerate() {
                let standings_line = by_day[day_index]
                    .iter()
                    .filter(|other| other.starts_with("competitor "))
                    .nth(competitor)
                    .expect("the standings section names the competitor");
                assert_eq!(
                    close_form(day_line),
                    close_form(standings_line),
                    "seed {}",
                    seed
                );
                // Within a section the no standing form never
                // stands after a figured day, because a
                // standing once set is never cleared.
                if day_line.contains("no standing projection at the close") {
                    assert!(!stood, "seed {}", seed);
                } else {
                    stood = true;
                }
            }
        }

        // The standings held of the traces counts line equals
        // the standings held of the standings counts line.
        let traces_held = count_after(traces.last().expect("the traces close"), "standings held ");
        let standings_held = count_after(
            standings.last().expect("the standings close"),
            "standings held ",
        );
        assert_eq!(traces_held, standings_held, "seed {}", seed);
    }
}

// The margins pin of the generated family, added with Round
// 14 beside the traces pin above. It fails before the round
// by the absence of meet_margins from this suite's imports,
// and on the built tree it fails against a margins
// construction that drifts from the marks or standings
// readings over generated shapes, which is the drift charge
// the family exists to catch. Its relations are the closings
// of PG-30 computed from the printed lines of the readings
// alone, the line by line agreement with the marks reading,
// the margin arithmetic of every mark line, the day close
// relation against the standings reading, the counts
// arithmetic, and the zero relation, a margin of zero
// standing only where the line's nearest standing figure
// equals its record figure, checked where the generated
// shapes produce one and stated as the relation PG-30
// fixes, not as a shape the family owes.

fn margins_record_prefix(mark_line: &str) -> String {
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

/// The figures of one margins mark line: the record figure,
/// and where another standing stands, the nearest standing
/// figure, the label of its holder, and the margin figure.
fn margins_mark_parts(line: &str) -> (String, Option<(String, String, String)>) {
    let start = line
        .find("at a projection of ")
        .expect("a mark line names its record figure");
    let figure_start = start + "at a projection of ".len();
    let figure_end = line[figure_start..]
        .find(',')
        .map(|offset| figure_start + offset)
        .expect("a record figure ends at a comma");
    let record = line[figure_start..figure_end].to_string();
    if line.ends_with(", and no other standing") {
        return (record, None);
    }
    let nearest_start = line
        .find(", nearest standing ")
        .expect("a figured mark line names its nearest standing")
        + ", nearest standing ".len();
    let held = line[nearest_start..]
        .find(" held by ")
        .map(|offset| nearest_start + offset)
        .expect("a nearest standing names its holder");
    let figure = line[nearest_start..held].to_string();
    let label_start = held + " held by ".len();
    let margin_mark = line[label_start..]
        .find(", margin ")
        .map(|offset| label_start + offset)
        .expect("a figured mark line names its margin");
    let label = line[label_start..margin_mark].to_string();
    let margin = line[margin_mark + ", margin ".len()..].to_string();
    (record, Some((figure, label, margin)))
}

/// The elapsed figure and ordinal phrase a mark line names.
fn margins_mark_position(line: &str) -> (String, String) {
    let at = line.find(", at ").expect("a mark line names its mark") + ", at ".len();
    let seconds = line[at..]
        .find(" seconds on the ")
        .map(|offset| at + offset)
        .expect("a mark line names its seconds");
    let elapsed = line[at..seconds].to_string();
    let day_start = seconds + " seconds on the ".len();
    let day_end = line[day_start..]
        .find(" day,")
        .map(|offset| day_start + offset)
        .expect("a mark line names its day");
    (elapsed, line[day_start..day_end].to_string())
}

#[test]
fn test_generated_meets_close_under_the_margins_reading() {
    for index in 0..8u64 {
        let seed = MEET_SEED + index;
        let mut gen = Gen::new(seed);
        let text = meet_text(&mut gen);
        let (meet, defects) = load_meet_text(&text);
        assert_eq!(defects, Vec::<String>::new(), "seed {}", seed);
        let meet = meet.expect("the generated meet loads");

        let marks = meet_marks(&meet).expect("the marks read");
        let margins = meet_margins(&meet).expect("the margins read");
        let standings = meet_standings(&meet).expect("the standings read");

        let reading_marks: Vec<&String> = marks
            .iter()
            .filter(|line| line.starts_with("mark "))
            .collect();
        let margin_marks: Vec<&String> = margins
            .iter()
            .filter(|line| line.starts_with("mark "))
            .collect();
        assert_eq!(margin_marks.len(), reading_marks.len(), "seed {}", seed);

        // The line by line agreement of PG-30, the margin
        // arithmetic of every mark line, and the zero
        // relation where the shapes produce one.
        let mut with_another = 0usize;
        for (margin_line, mark_line) in margin_marks.iter().zip(reading_marks.iter()) {
            assert!(
                margin_line.starts_with(&margins_record_prefix(mark_line)),
                "seed {}",
                seed
            );
            let (record, nearest) = margins_mark_parts(margin_line);
            if let Some((figure, _label, margin)) = nearest {
                with_another += 1;
                let record_value: f64 = record.parse().expect("a record figure parses");
                let nearest_value: f64 = figure.parse().expect("a nearest figure parses");
                let margin_value: f64 = margin.parse().expect("a margin figure parses");
                assert!(margin_value >= 0.0, "seed {}", seed);
                assert!(
                    (nearest_value - record_value - margin_value).abs() <= 0.000002,
                    "seed {}",
                    seed
                );
                if margin == "0" {
                    assert_eq!(figure, record, "seed {}", seed);
                }
            }
        }

        // The counts arithmetic of PG-30, and the closest
        // margin standing at the earliest mark stating it.
        let counts = margins.last().expect("the margins close");
        let walked = count_after(counts, "marks walked ");
        assert_eq!(walked, margin_marks.len(), "seed {}", seed);
        let stated_with = count_after(counts, "marks with another standing ");
        assert_eq!(stated_with, with_another, "seed {}", seed);
        let no_other = margin_marks
            .iter()
            .filter(|line| line.ends_with(", and no other standing"))
            .count();
        assert_eq!(with_another + no_other, walked, "seed {}", seed);
        if with_another > 0 {
            let closest = counts
                .split("closest margin ")
                .nth(1)
                .expect("the counts line names its closest margin");
            let (figure, rest) = closest
                .split_once(" at ")
                .expect("the closest margin names its mark");
            let (elapsed, ordinal) = rest
                .split_once(" seconds on the ")
                .expect("the closest margin names its seconds");
            let ordinal = ordinal
                .strip_suffix(" day")
                .expect("the closest margin names its day");
            let first = margin_marks
                .iter()
                .find(|line| {
                    matches!(margins_mark_parts(line).1, Some((_, _, ref margin)) if margin == figure)
                })
                .expect("a mark line states the closest margin");
            assert_eq!(
                margins_mark_position(first),
                (elapsed.to_string(), ordinal.to_string()),
                "seed {}",
                seed
            );
        } else {
            assert!(
                counts.ends_with(", and, no other standing at any mark"),
                "seed {}",
                seed
            );
        }

        // The day close relation of PG-30, computed from
        // the standings sections and the margins sections.
        let mut standing_sections: Vec<Vec<String>> = Vec::new();
        for line in &standings {
            if line.starts_with("day, the ") {
                standing_sections.push(Vec::new());
            }
            if let Some(section) = standing_sections.last_mut() {
                section.push(line.clone());
            }
        }
        let mut margin_sections: Vec<Vec<&String>> = Vec::new();
        for line in &margins {
            if line.starts_with("day, the ") {
                margin_sections.push(Vec::new());
            }
            if let Some(section) = margin_sections.last_mut() {
                section.push(line);
            }
        }
        assert_eq!(
            standing_sections.len(),
            margin_sections.len(),
            "seed {}",
            seed
        );
        for (section, margin_section) in standing_sections.iter().zip(margin_sections.iter()) {
            let day_marks: Vec<&&String> = margin_section
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
            let record = closing
                .rsplit("at a projection of ")
                .next()
                .expect("a closing line names its record figure");
            let record_value: f64 = record.parse().expect("a record figure parses");
            let others: Vec<(&str, &str)> = section
                .iter()
                .filter(|line| line.starts_with("competitor "))
                .filter(|line| !line.starts_with(&format!("competitor {},", holder)))
                .filter_map(|line| {
                    line.strip_prefix("competitor ")
                        .and_then(|rest| rest.split_once(", standing projection "))
                        .and_then(|(label, rest)| {
                            rest.strip_suffix(" at the close")
                                .map(|figure| (label, figure))
                        })
                })
                .collect();
            let last = day_marks.last().expect("the day holds a mark");
            let (_last_record, last_nearest) = margins_mark_parts(last);
            match (others.is_empty(), last_nearest) {
                (true, None) => {}
                (false, Some((figure, label, margin))) => {
                    let lowest = others
                        .iter()
                        .map(|(_, other)| other.parse::<f64>().expect("a figure parses"))
                        .fold(f64::INFINITY, f64::min);
                    let figure_value: f64 = figure.parse().expect("a figure parses");
                    assert_eq!(figure_value, lowest, "seed {}", seed);
                    assert!(
                        others.iter().any(|(other_label, other_figure)| {
                            *other_label == label && other_figure.parse::<f64>() == Ok(lowest)
                        }),
                        "seed {}",
                        seed
                    );
                    let margin_value: f64 = margin.parse().expect("a margin parses");
                    assert!(
                        (figure_value - record_value - margin_value).abs() <= 0.000002,
                        "seed {}",
                        seed
                    );
                }
                _ => panic!("the day close relation fails, seed {}", seed),
            }
        }
    }
}

// The spread pin of the generated family, added with Round
// 15 beside the margins pin above. It fails before the
// round by the absence of meet_spread from this suite's
// imports, and on the built tree it fails against a
// spread construction that drifts from the marks,
// standings, or margins readings over generated shapes,
// which is the drift charge the family exists to catch.
// Its relations are the closings of PG-32 computed from
// the printed lines of the readings alone, the line by
// line agreement with the marks reading, the spread
// arithmetic of every mark line, the day close relation
// against the standings reading, the counts arithmetic,
// the fullest field standing at the earliest mark
// stating it, the cross relation against the margins
// reading, a spread never below the margin at the same
// mark with the two equal exactly where the farthest
// and nearest standing figures of the two lines agree,
// and the zero relation, a spread of zero standing only
// where the line's farthest standing figure equals its
// record figure, checked where the generated shapes
// produce one and stated as the relation PG-32 fixes,
// not as a shape the family owes.

/// The figures of one spread mark line: the record
/// figure, the field, and where another standing
/// stands, the farthest standing figure, the label of
/// its holder, and the spread figure.
fn spread_mark_parts(line: &str) -> (String, usize, Option<(String, String, String)>) {
    let start = line
        .find("at a projection of ")
        .expect("a mark line names its record figure");
    let figure_start = start + "at a projection of ".len();
    let figure_end = line[figure_start..]
        .find(',')
        .map(|offset| figure_start + offset)
        .expect("a record figure ends at a comma");
    let record = line[figure_start..figure_end].to_string();
    let field = count_after(line, "field of ");
    if line.ends_with(", and no other standing") {
        return (record, field, None);
    }
    let farthest_start = line
        .find(", farthest standing ")
        .expect("a figured mark line names its farthest standing")
        + ", farthest standing ".len();
    let held = line[farthest_start..]
        .find(" held by ")
        .map(|offset| farthest_start + offset)
        .expect("a farthest standing names its holder");
    let figure = line[farthest_start..held].to_string();
    let label_start = held + " held by ".len();
    let spread_mark = line[label_start..]
        .find(", spread ")
        .map(|offset| label_start + offset)
        .expect("a figured mark line names its spread");
    let label = line[label_start..spread_mark].to_string();
    let spread = line[spread_mark + ", spread ".len()..].to_string();
    (record, field, Some((figure, label, spread)))
}

/// The elapsed figure and ordinal phrase a spread mark
/// line names, read the way the margins helper reads
/// them.
fn spread_mark_position(line: &str) -> (String, String) {
    margins_mark_position(line)
}

#[test]
fn test_generated_meets_close_under_the_spread_reading() {
    for index in 0..8u64 {
        let seed = MEET_SEED + index;
        let mut gen = Gen::new(seed);
        let text = meet_text(&mut gen);
        let (meet, defects) = load_meet_text(&text);
        assert_eq!(defects, Vec::<String>::new(), "seed {}", seed);
        let meet = meet.expect("the generated meet loads");

        let marks = meet_marks(&meet).expect("the marks read");
        let margins = meet_margins(&meet).expect("the margins read");
        let spread = meet_spread(&meet).expect("the spread read");
        let standings = meet_standings(&meet).expect("the standings read");

        let reading_marks: Vec<&String> = marks
            .iter()
            .filter(|line| line.starts_with("mark "))
            .collect();
        let margin_marks: Vec<&String> = margins
            .iter()
            .filter(|line| line.starts_with("mark "))
            .collect();
        let spread_marks: Vec<&String> = spread
            .iter()
            .filter(|line| line.starts_with("mark "))
            .collect();
        assert_eq!(spread_marks.len(), reading_marks.len(), "seed {}", seed);
        assert_eq!(spread_marks.len(), margin_marks.len(), "seed {}", seed);

        // The line by line agreement of PG-32, the spread
        // arithmetic of every mark line, the zero
        // relation where the shapes produce one, and the
        // cross relation against the margins reading at
        // the same mark.
        let mut with_another = 0usize;
        for ((spread_line, mark_line), margin_line) in spread_marks
            .iter()
            .zip(reading_marks.iter())
            .zip(margin_marks.iter())
        {
            assert!(
                spread_line.starts_with(&margins_record_prefix(mark_line)),
                "seed {}",
                seed
            );
            let (record, _field, farthest) = spread_mark_parts(spread_line);
            let (_margin_record, nearest) = margins_mark_parts(margin_line);
            if let Some((figure, _label, spread_figure)) = farthest {
                with_another += 1;
                let record_value: f64 = record.parse().expect("a record figure parses");
                let farthest_value: f64 = figure.parse().expect("a farthest figure parses");
                let spread_value: f64 = spread_figure.parse().expect("a spread figure parses");
                assert!(spread_value >= 0.0, "seed {}", seed);
                assert!(
                    (farthest_value - record_value - spread_value).abs() <= 0.000002,
                    "seed {}",
                    seed
                );
                if spread_figure == "0" {
                    assert_eq!(figure, record, "seed {}", seed);
                }
                let (nearest_figure, _nearest_label, margin_figure) =
                    nearest.expect("the margins line states a nearest standing");
                let nearest_value: f64 = nearest_figure.parse().expect("a nearest figure parses");
                let margin_value: f64 = margin_figure.parse().expect("a margin figure parses");
                assert!(spread_value + 0.000002 >= margin_value, "seed {}", seed);
                if figure == nearest_figure {
                    assert_eq!(spread_figure, margin_figure, "seed {}", seed);
                }
                if spread_figure == margin_figure {
                    assert!(
                        (farthest_value - nearest_value).abs() <= 0.000002,
                        "seed {}",
                        seed
                    );
                }
            } else {
                assert!(nearest.is_none(), "seed {}", seed);
            }
        }

        // The counts arithmetic of PG-32, and the fullest
        // field standing at the earliest mark stating it.
        let counts = spread.last().expect("the spread close");
        let walked = count_after(counts, "marks walked ");
        assert_eq!(walked, spread_marks.len(), "seed {}", seed);
        let stated_with = count_after(counts, "marks with another standing ");
        assert_eq!(stated_with, with_another, "seed {}", seed);
        let no_other = spread_marks
            .iter()
            .filter(|line| line.ends_with(", and no other standing"))
            .count();
        assert_eq!(with_another + no_other, walked, "seed {}", seed);
        if with_another > 0 {
            let fullest = counts
                .split("fullest field ")
                .nth(1)
                .expect("the counts line names its fullest field");
            let (figure, rest) = fullest
                .split_once(" at ")
                .expect("the fullest field names its mark");
            let (elapsed, ordinal) = rest
                .split_once(" seconds on the ")
                .expect("the fullest field names its seconds");
            let ordinal = ordinal
                .strip_suffix(" day")
                .expect("the fullest field names its day");
            let greatest = spread_marks
                .iter()
                .map(|line| spread_mark_parts(line).1)
                .max()
                .expect("a mark line states a field");
            assert_eq!(figure, greatest.to_string(), "seed {}", seed);
            let first = spread_marks
                .iter()
                .find(|line| spread_mark_parts(line).1 == greatest)
                .expect("a mark line states the fullest field");
            assert_eq!(
                spread_mark_position(first),
                (elapsed.to_string(), ordinal.to_string()),
                "seed {}",
                seed
            );
        } else {
            assert!(
                counts.ends_with(", and, no other standing at any mark"),
                "seed {}",
                seed
            );
        }

        // The day close relation of PG-32, computed from
        // the standings sections and the spread sections.
        let mut standing_sections: Vec<Vec<String>> = Vec::new();
        for line in &standings {
            if line.starts_with("day, the ") {
                standing_sections.push(Vec::new());
            }
            if let Some(section) = standing_sections.last_mut() {
                section.push(line.clone());
            }
        }
        let mut spread_sections: Vec<Vec<&String>> = Vec::new();
        for line in &spread {
            if line.starts_with("day, the ") {
                spread_sections.push(Vec::new());
            }
            if let Some(section) = spread_sections.last_mut() {
                section.push(line);
            }
        }
        assert_eq!(
            standing_sections.len(),
            spread_sections.len(),
            "seed {}",
            seed
        );
        for (section, spread_section) in standing_sections.iter().zip(spread_sections.iter()) {
            let day_marks: Vec<&&String> = spread_section
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
            let record = closing
                .rsplit("at a projection of ")
                .next()
                .expect("a closing line names its record figure");
            let record_value: f64 = record.parse().expect("a record figure parses");
            let others: Vec<(&str, &str)> = section
                .iter()
                .filter(|line| line.starts_with("competitor "))
                .filter(|line| !line.starts_with(&format!("competitor {},", holder)))
                .filter_map(|line| {
                    line.strip_prefix("competitor ")
                        .and_then(|rest| rest.split_once(", standing projection "))
                        .and_then(|(label, rest)| {
                            rest.strip_suffix(" at the close")
                                .map(|figure| (label, figure))
                        })
                })
                .collect();
            let standing_count = section
                .iter()
                .filter(|line| line.starts_with("competitor "))
                .filter_map(|line| {
                    line.strip_prefix("competitor ")
                        .and_then(|rest| rest.split_once(", standing projection "))
                        .and_then(|(_, rest)| rest.strip_suffix(" at the close"))
                })
                .count();
            let last = day_marks.last().expect("the day holds a mark");
            let (_last_record, last_field, last_farthest) = spread_mark_parts(last);
            assert_eq!(last_field, standing_count, "seed {}", seed);
            match (others.is_empty(), last_farthest) {
                (true, None) => {}
                (false, Some((figure, label, spread_figure))) => {
                    let highest = others
                        .iter()
                        .map(|(_, other)| other.parse::<f64>().expect("a figure parses"))
                        .fold(f64::NEG_INFINITY, f64::max);
                    let figure_value: f64 = figure.parse().expect("a figure parses");
                    assert_eq!(figure_value, highest, "seed {}", seed);
                    assert!(
                        others.iter().any(|(other_label, other_figure)| {
                            *other_label == label && other_figure.parse::<f64>() == Ok(highest)
                        }),
                        "seed {}",
                        seed
                    );
                    let spread_value: f64 = spread_figure.parse().expect("a spread parses");
                    assert!(
                        (figure_value - record_value - spread_value).abs() <= 0.000002,
                        "seed {}",
                        seed
                    );
                }
                _ => panic!("the day close relation fails, seed {}", seed),
            }
        }
    }
}

// The middle pin of the generated family, added with Round
// 16 beside the spread pin above. It fails before the round
// by the absence of meet_middle from this suite's imports,
// and on the built tree it fails against a middle
// construction that drifts from the marks, standings,
// margins, or spread readings over generated shapes, which
// is the drift charge the family exists to catch. Its
// relations are the closings of PG-34 computed from the
// printed lines of the readings alone, the line by line
// agreement with the marks reading, the middle margin
// arithmetic of every mark line, the bracketing of the
// middle margin between the margin and the spread at the
// same mark with equality exactly where the middle figure
// equals the nearest or the farthest figure of that mark,
// the zero relation in both directions, the day close
// relation against the standings reading with the middle
// figure taken at the place PG-33 states, the counts
// arithmetic, and the closing naming the earliest mark at
// the smallest middle margin.

/// The figures of one middle mark line: the record figure,
/// and where another standing stands, the middle standing
/// figure, the label of its holder, and the middle margin.
fn middle_mark_parts(line: &str) -> (String, Option<(String, String, String)>) {
    let start = line
        .find("at a projection of ")
        .expect("a mark line names its record figure");
    let figure_start = start + "at a projection of ".len();
    let figure_end = line[figure_start..]
        .find(',')
        .map(|offset| figure_start + offset)
        .expect("a record figure ends at a comma");
    let record = line[figure_start..figure_end].to_string();
    if line.ends_with(", and no other standing") {
        return (record, None);
    }
    let middle_start = line
        .find(", middle standing ")
        .expect("a figured mark line names its middle standing")
        + ", middle standing ".len();
    let held = line[middle_start..]
        .find(" held by ")
        .map(|offset| middle_start + offset)
        .expect("a middle standing names its holder");
    let figure = line[middle_start..held].to_string();
    let label_start = held + " held by ".len();
    let margin_mark = line[label_start..]
        .find(", middle margin ")
        .map(|offset| label_start + offset)
        .expect("a figured mark line names its middle margin");
    let label = line[label_start..margin_mark].to_string();
    let margin = line[margin_mark + ", middle margin ".len()..].to_string();
    (record, Some((figure, label, margin)))
}

/// The elapsed figure and ordinal phrase a middle mark
/// line names, read the way the margins helper reads them.
fn middle_mark_position(line: &str) -> (String, String) {
    margins_mark_position(line)
}

#[test]
fn test_generated_meets_close_under_the_middle_reading() {
    for index in 0..8u64 {
        let seed = MEET_SEED + index;
        let mut gen = Gen::new(seed);
        let text = meet_text(&mut gen);
        let (meet, defects) = load_meet_text(&text);
        assert_eq!(defects, Vec::<String>::new(), "seed {}", seed);
        let meet = meet.expect("the generated meet loads");

        let marks = meet_marks(&meet).expect("the marks read");
        let margins = meet_margins(&meet).expect("the margins read");
        let spread = meet_spread(&meet).expect("the spread read");
        let middle = meet_middle(&meet).expect("the middle read");
        let standings = meet_standings(&meet).expect("the standings read");

        let reading_marks: Vec<&String> = marks
            .iter()
            .filter(|line| line.starts_with("mark "))
            .collect();
        let margin_marks: Vec<&String> = margins
            .iter()
            .filter(|line| line.starts_with("mark "))
            .collect();
        let spread_marks: Vec<&String> = spread
            .iter()
            .filter(|line| line.starts_with("mark "))
            .collect();
        let middle_marks: Vec<&String> = middle
            .iter()
            .filter(|line| line.starts_with("mark "))
            .collect();
        assert_eq!(middle_marks.len(), reading_marks.len(), "seed {}", seed);
        assert_eq!(middle_marks.len(), margin_marks.len(), "seed {}", seed);
        assert_eq!(middle_marks.len(), spread_marks.len(), "seed {}", seed);

        // The line by line agreement of PG-34, the middle
        // margin arithmetic of every mark line, the zero
        // relation in both directions where the shapes
        // produce one, and the bracketing against the
        // margins and spread readings at the same mark.
        let mut with_another = 0usize;
        for (((middle_line, mark_line), margin_line), spread_line) in middle_marks
            .iter()
            .zip(reading_marks.iter())
            .zip(margin_marks.iter())
            .zip(spread_marks.iter())
        {
            assert!(
                middle_line.starts_with(&margins_record_prefix(mark_line)),
                "seed {}",
                seed
            );
            let (record, middle_part) = middle_mark_parts(middle_line);
            let (_margin_record, nearest) = margins_mark_parts(margin_line);
            let (_spread_record, _field, farthest) = spread_mark_parts(spread_line);
            if let Some((figure, _label, middle_margin)) = middle_part {
                with_another += 1;
                let record_value: f64 = record.parse().expect("a record figure parses");
                let middle_value: f64 = figure.parse().expect("a middle figure parses");
                let middle_margin_value: f64 =
                    middle_margin.parse().expect("a middle margin parses");
                assert!(middle_margin_value >= 0.0, "seed {}", seed);
                assert!(
                    (middle_value - record_value - middle_margin_value).abs() <= 0.000002,
                    "seed {}",
                    seed
                );
                if middle_margin == "0" {
                    assert_eq!(figure, record, "seed {}", seed);
                }
                if figure == record {
                    assert_eq!(middle_margin, "0", "seed {}", seed);
                }
                let (nearest_figure, _nearest_label, margin_figure) =
                    nearest.expect("the margins line states a nearest standing");
                let nearest_value: f64 = nearest_figure.parse().expect("a nearest figure parses");
                let margin_value: f64 = margin_figure.parse().expect("a margin figure parses");
                let (farthest_figure, _farthest_label, spread_figure) =
                    farthest.expect("the spread line states a farthest standing");
                let farthest_value: f64 =
                    farthest_figure.parse().expect("a farthest figure parses");
                let spread_value: f64 = spread_figure.parse().expect("a spread figure parses");
                assert!(
                    margin_value <= middle_margin_value + 0.000002,
                    "seed {}",
                    seed
                );
                assert!(
                    middle_margin_value <= spread_value + 0.000002,
                    "seed {}",
                    seed
                );
                if figure == nearest_figure {
                    assert_eq!(middle_margin, margin_figure, "seed {}", seed);
                }
                if middle_margin == margin_figure {
                    assert!(
                        (middle_value - nearest_value).abs() <= 0.000002,
                        "seed {}",
                        seed
                    );
                }
                if figure == farthest_figure {
                    assert_eq!(middle_margin, spread_figure, "seed {}", seed);
                }
                if middle_margin == spread_figure {
                    assert!(
                        (middle_value - farthest_value).abs() <= 0.000002,
                        "seed {}",
                        seed
                    );
                }
            } else {
                assert!(nearest.is_none(), "seed {}", seed);
                assert!(farthest.is_none(), "seed {}", seed);
            }
        }

        // The counts arithmetic of PG-34, and the closing
        // naming the earliest mark at the smallest middle
        // margin.
        let counts = middle.last().expect("the middle close");
        let walked = count_after(counts, "marks walked ");
        assert_eq!(walked, middle_marks.len(), "seed {}", seed);
        let stated_with = count_after(counts, "marks with another standing ");
        assert_eq!(stated_with, with_another, "seed {}", seed);
        let no_other = middle_marks
            .iter()
            .filter(|line| line.ends_with(", and no other standing"))
            .count();
        assert_eq!(with_another + no_other, walked, "seed {}", seed);
        if with_another > 0 {
            let closest = counts
                .split("closest middle margin ")
                .nth(1)
                .expect("the counts line names its closest middle margin");
            let (figure, rest) = closest
                .split_once(" at ")
                .expect("the closest middle margin names its mark");
            let (elapsed, ordinal) = rest
                .split_once(" seconds on the ")
                .expect("the closest middle margin names its seconds");
            let ordinal = ordinal
                .strip_suffix(" day")
                .expect("the closest middle margin names its day");
            let stated_value: f64 = figure.parse().expect("a closest figure parses");
            let smallest = middle_marks
                .iter()
                .filter_map(|line| middle_mark_parts(line).1)
                .map(|(_, _, margin)| margin.parse::<f64>().expect("a middle margin parses"))
                .fold(f64::INFINITY, f64::min);
            assert!((stated_value - smallest).abs() <= 0.000002, "seed {}", seed);
            let first = middle_marks
                .iter()
                .find(|line| {
                    middle_mark_parts(line)
                        .1
                        .map(|(_, _, margin)| {
                            (margin.parse::<f64>().expect("a middle margin parses") - smallest)
                                .abs()
                                <= 0.000002
                        })
                        .unwrap_or(false)
                })
                .expect("a mark line states the smallest middle margin");
            assert_eq!(
                middle_mark_position(first),
                (elapsed.to_string(), ordinal.to_string()),
                "seed {}",
                seed
            );
            let named = middle_mark_parts(first)
                .1
                .expect("the named mark is figured");
            assert_eq!(named.2, figure, "seed {}", seed);
        } else {
            assert!(
                counts.ends_with(", and, no other standing at any mark"),
                "seed {}",
                seed
            );
        }

        // The day close relation of PG-34, computed from
        // the standings sections and the middle sections,
        // the middle figure taken at the place PG-33
        // states over the figures of the competitors
        // other than the closing holder.
        let mut standing_sections: Vec<Vec<String>> = Vec::new();
        for line in &standings {
            if line.starts_with("day, the ") {
                standing_sections.push(Vec::new());
            }
            if let Some(section) = standing_sections.last_mut() {
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
        assert_eq!(
            standing_sections.len(),
            middle_sections.len(),
            "seed {}",
            seed
        );
        for (section, middle_section) in standing_sections.iter().zip(middle_sections.iter()) {
            let day_marks: Vec<&&String> = middle_section
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
            let record = closing
                .rsplit("at a projection of ")
                .next()
                .expect("a closing line names its record figure");
            let record_value: f64 = record.parse().expect("a record figure parses");
            let others: Vec<(&str, &str)> = section
                .iter()
                .filter(|line| line.starts_with("competitor "))
                .filter(|line| !line.starts_with(&format!("competitor {},", holder)))
                .filter_map(|line| {
                    line.strip_prefix("competitor ")
                        .and_then(|rest| rest.split_once(", standing projection "))
                        .and_then(|(label, rest)| {
                            rest.strip_suffix(" at the close")
                                .map(|figure| (label, figure))
                        })
                })
                .collect();
            let last = day_marks.last().expect("the day holds a mark");
            let (_last_record, last_middle) = middle_mark_parts(last);
            match (others.is_empty(), last_middle) {
                (true, None) => {}
                (false, Some((figure, label, middle_margin))) => {
                    let mut figures: Vec<f64> = others
                        .iter()
                        .map(|(_, other)| other.parse::<f64>().expect("a figure parses"))
                        .collect();
                    figures.sort_by(|a, b| a.partial_cmp(b).expect("figures order"));
                    let expected = figures[figures.len().div_ceil(2) - 1];
                    let figure_value: f64 = figure.parse().expect("a figure parses");
                    assert_eq!(figure_value, expected, "seed {}", seed);
                    assert!(
                        others.iter().any(|(other_label, other_figure)| {
                            *other_label == label && other_figure.parse::<f64>() == Ok(expected)
                        }),
                        "seed {}",
                        seed
                    );
                    let margin_value: f64 = middle_margin.parse().expect("a middle margin parses");
                    assert!(
                        (figure_value - record_value - margin_value).abs() <= 0.000002,
                        "seed {}",
                        seed
                    );
                }
                _ => panic!("the day close relation fails, seed {}", seed),
            }
        }
    }
}

// The tails pin of the generated family, added with Round
// 17 beside the middle pin above. It fails before the round
// by the absence of meet_tails from this suite's imports,
// and on the built tree it fails against a tails
// construction that drifts from the marks, standings,
// margins, spread, or middle readings over generated
// shapes, which is the drift charge the family exists to
// catch. Its relations are the closings of PG-36 computed
// from the printed lines of the readings alone, the line
// by line agreement with the marks reading, the tail
// margin arithmetic of every mark line, the bracketing
// chain of PG-36 at every mark with its equalities
// standing where the tail figures equal the figures they
// bracket against, the zero relations in both directions
// where the shapes produce them, the day close relation
// against the standings reading with the tails taken at
// the count PG-35 states, the counts arithmetic, and the
// two closings naming the earliest marks at the smallest
// tail margins.

/// The figures of one tails mark line: the record figure,
/// and where another standing stands, the lower tail
/// figure and margin and the upper tail figure and margin.
fn tails_mark_parts(line: &str) -> (String, Option<(String, String, String, String)>) {
    let start = line
        .find("at a projection of ")
        .expect("a mark line names its record figure");
    let figure_start = start + "at a projection of ".len();
    let figure_end = line[figure_start..]
        .find(',')
        .map(|offset| figure_start + offset)
        .expect("a record figure ends at a comma");
    let record = line[figure_start..figure_end].to_string();
    if line.ends_with(", and no other standing") {
        return (record, None);
    }
    let lower_start = line
        .find(", lower tail standing ")
        .expect("a figured mark line names its lower tail standing")
        + ", lower tail standing ".len();
    let lower_end = line[lower_start..]
        .find(", lower tail margin ")
        .map(|offset| lower_start + offset)
        .expect("a lower tail standing names its margin");
    let lower_figure = line[lower_start..lower_end].to_string();
    let lower_margin_start = lower_end + ", lower tail margin ".len();
    let lower_margin_end = line[lower_margin_start..]
        .find(", upper tail standing ")
        .map(|offset| lower_margin_start + offset)
        .expect("a figured mark line names its upper tail standing");
    let lower_margin = line[lower_margin_start..lower_margin_end].to_string();
    let upper_start = lower_margin_end + ", upper tail standing ".len();
    let upper_end = line[upper_start..]
        .find(", upper tail margin ")
        .map(|offset| upper_start + offset)
        .expect("an upper tail standing names its margin");
    let upper_figure = line[upper_start..upper_end].to_string();
    let upper_margin = line[upper_end + ", upper tail margin ".len()..].to_string();
    (
        record,
        Some((lower_figure, lower_margin, upper_figure, upper_margin)),
    )
}

/// The elapsed figure and ordinal phrase a tails mark
/// line names, read the way the margins helper reads them.
fn tails_mark_position(line: &str) -> (String, String) {
    margins_mark_position(line)
}

#[test]
fn test_generated_meets_close_under_the_tails_reading() {
    for index in 0..8u64 {
        let seed = MEET_SEED + index;
        let mut gen = Gen::new(seed);
        let text = meet_text(&mut gen);
        let (meet, defects) = load_meet_text(&text);
        assert_eq!(defects, Vec::<String>::new(), "seed {}", seed);
        let meet = meet.expect("the generated meet loads");

        let marks = meet_marks(&meet).expect("the marks read");
        let margins = meet_margins(&meet).expect("the margins read");
        let spread = meet_spread(&meet).expect("the spread read");
        let middle = meet_middle(&meet).expect("the middle read");
        let tails = meet_tails(&meet).expect("the tails read");
        let standings = meet_standings(&meet).expect("the standings read");

        let reading_marks: Vec<&String> = marks
            .iter()
            .filter(|line| line.starts_with("mark "))
            .collect();
        let margin_marks: Vec<&String> = margins
            .iter()
            .filter(|line| line.starts_with("mark "))
            .collect();
        let spread_marks: Vec<&String> = spread
            .iter()
            .filter(|line| line.starts_with("mark "))
            .collect();
        let middle_marks: Vec<&String> = middle
            .iter()
            .filter(|line| line.starts_with("mark "))
            .collect();
        let tails_marks: Vec<&String> = tails
            .iter()
            .filter(|line| line.starts_with("mark "))
            .collect();
        assert_eq!(tails_marks.len(), reading_marks.len(), "seed {}", seed);
        assert_eq!(tails_marks.len(), margin_marks.len(), "seed {}", seed);
        assert_eq!(tails_marks.len(), spread_marks.len(), "seed {}", seed);
        assert_eq!(tails_marks.len(), middle_marks.len(), "seed {}", seed);

        // The line by line agreement of PG-36, the tail
        // margin arithmetic of every mark line, the zero
        // relations in both directions where the shapes
        // produce them, and the bracketing chain of PG-36
        // against the margins, middle, and spread readings
        // at the same mark.
        let mut with_another = 0usize;
        for ((((tails_line, mark_line), margin_line), spread_line), middle_line) in tails_marks
            .iter()
            .zip(reading_marks.iter())
            .zip(margin_marks.iter())
            .zip(spread_marks.iter())
            .zip(middle_marks.iter())
        {
            assert!(
                tails_line.starts_with(&margins_record_prefix(mark_line)),
                "seed {}",
                seed
            );
            let (record, tails_part) = tails_mark_parts(tails_line);
            let (_margin_record, nearest) = margins_mark_parts(margin_line);
            let (_spread_record, _field, farthest) = spread_mark_parts(spread_line);
            let (_middle_record, middle_part) = middle_mark_parts(middle_line);
            if let Some((lower_figure, lower_margin, upper_figure, upper_margin)) = tails_part {
                with_another += 1;
                let record_value: f64 = record.parse().expect("a record figure parses");
                let lower_value: f64 = lower_figure.parse().expect("a lower tail figure parses");
                let lower_margin_value: f64 =
                    lower_margin.parse().expect("a lower tail margin parses");
                let upper_value: f64 = upper_figure.parse().expect("an upper tail figure parses");
                let upper_margin_value: f64 =
                    upper_margin.parse().expect("an upper tail margin parses");
                assert!(lower_margin_value >= 0.0, "seed {}", seed);
                assert!(upper_margin_value >= 0.0, "seed {}", seed);
                assert!(
                    (lower_value - record_value - lower_margin_value).abs() <= 0.000002,
                    "seed {}",
                    seed
                );
                assert!(
                    (upper_value - record_value - upper_margin_value).abs() <= 0.000002,
                    "seed {}",
                    seed
                );
                if lower_margin == "0" {
                    assert_eq!(lower_figure, record, "seed {}", seed);
                }
                if lower_figure == record {
                    assert_eq!(lower_margin, "0", "seed {}", seed);
                }
                if upper_margin == "0" {
                    assert_eq!(upper_figure, record, "seed {}", seed);
                }
                if upper_figure == record {
                    assert_eq!(upper_margin, "0", "seed {}", seed);
                }
                let (nearest_figure, _nearest_label, margin_figure) =
                    nearest.expect("the margins line states a nearest standing");
                let nearest_value: f64 = nearest_figure.parse().expect("a nearest figure parses");
                let margin_value: f64 = margin_figure.parse().expect("a margin figure parses");
                let (farthest_figure, _farthest_label, spread_figure) =
                    farthest.expect("the spread line states a farthest standing");
                let farthest_value: f64 =
                    farthest_figure.parse().expect("a farthest figure parses");
                let spread_value: f64 = spread_figure.parse().expect("a spread figure parses");
                let (middle_figure, _middle_label, middle_margin) =
                    middle_part.expect("the middle line states a middle standing");
                let middle_value: f64 = middle_figure.parse().expect("a middle figure parses");
                let middle_margin_value: f64 =
                    middle_margin.parse().expect("a middle margin parses");
                if lower_margin == "0" {
                    assert_eq!(nearest_figure, record, "seed {}", seed);
                }
                if upper_margin == "0" {
                    assert_eq!(farthest_figure, record, "seed {}", seed);
                }
                assert!(
                    margin_value <= lower_margin_value + 0.000002,
                    "seed {}",
                    seed
                );
                assert!(
                    lower_margin_value <= middle_margin_value + 0.000002,
                    "seed {}",
                    seed
                );
                assert!(
                    middle_margin_value <= upper_margin_value + 0.000002,
                    "seed {}",
                    seed
                );
                assert!(
                    upper_margin_value <= spread_value + 0.000002,
                    "seed {}",
                    seed
                );
                if lower_figure == nearest_figure {
                    assert_eq!(lower_margin, margin_figure, "seed {}", seed);
                }
                if lower_margin == margin_figure {
                    assert!(
                        (lower_value - nearest_value).abs() <= 0.000002,
                        "seed {}",
                        seed
                    );
                }
                if lower_figure == middle_figure {
                    assert_eq!(lower_margin, middle_margin, "seed {}", seed);
                }
                if lower_margin == middle_margin {
                    assert!(
                        (lower_value - middle_value).abs() <= 0.000002,
                        "seed {}",
                        seed
                    );
                }
                if upper_figure == middle_figure {
                    assert_eq!(upper_margin, middle_margin, "seed {}", seed);
                }
                if upper_margin == middle_margin {
                    assert!(
                        (upper_value - middle_value).abs() <= 0.000002,
                        "seed {}",
                        seed
                    );
                }
                if upper_figure == farthest_figure {
                    assert_eq!(upper_margin, spread_figure, "seed {}", seed);
                }
                if upper_margin == spread_figure {
                    assert!(
                        (upper_value - farthest_value).abs() <= 0.000002,
                        "seed {}",
                        seed
                    );
                }
            } else {
                assert!(nearest.is_none(), "seed {}", seed);
                assert!(farthest.is_none(), "seed {}", seed);
                assert!(middle_part.is_none(), "seed {}", seed);
            }
        }

        // The counts arithmetic of PG-36, and the two
        // closings naming the earliest marks at the
        // smallest tail margins.
        let counts = tails.last().expect("the tails close");
        let walked = count_after(counts, "marks walked ");
        assert_eq!(walked, tails_marks.len(), "seed {}", seed);
        let stated_with = count_after(counts, "marks with another standing ");
        assert_eq!(stated_with, with_another, "seed {}", seed);
        let stated_days = count_after(counts, "days ");
        let no_other = tails_marks
            .iter()
            .filter(|line| line.ends_with(", and no other standing"))
            .count();
        assert_eq!(with_another + no_other, walked, "seed {}", seed);
        if with_another > 0 {
            let lower_part = counts
                .split("closest lower tail margin ")
                .nth(1)
                .expect("the counts line names its closest lower tail margin");
            let (lower_figure, rest) = lower_part
                .split_once(" at ")
                .expect("the closest lower tail margin names its mark");
            let (lower_elapsed, rest) = rest
                .split_once(" seconds on the ")
                .expect("the closest lower tail margin names its seconds");
            let (lower_ordinal, upper_part) = rest
                .split_once(" day, closest upper tail margin ")
                .expect("the closest lower tail margin names its day");
            let (upper_figure, rest) = upper_part
                .split_once(" at ")
                .expect("the closest upper tail margin names its mark");
            let (upper_elapsed, upper_ordinal) = rest
                .split_once(" seconds on the ")
                .expect("the closest upper tail margin names its seconds");
            let upper_ordinal = upper_ordinal
                .strip_suffix(" day")
                .expect("the closest upper tail margin names its day");
            let lower_stated: f64 = lower_figure.parse().expect("a closest figure parses");
            let smallest_lower = tails_marks
                .iter()
                .filter_map(|line| tails_mark_parts(line).1)
                .map(|(_, margin, _, _)| margin.parse::<f64>().expect("a lower tail margin parses"))
                .fold(f64::INFINITY, f64::min);
            assert!(
                (lower_stated - smallest_lower).abs() <= 0.000002,
                "seed {}",
                seed
            );
            let first_lower = tails_marks
                .iter()
                .find(|line| {
                    tails_mark_parts(line)
                        .1
                        .map(|(_, margin, _, _)| {
                            (margin.parse::<f64>().expect("a lower tail margin parses")
                                - smallest_lower)
                                .abs()
                                <= 0.000002
                        })
                        .unwrap_or(false)
                })
                .expect("a mark line states the smallest lower tail margin");
            assert_eq!(
                tails_mark_position(first_lower),
                (lower_elapsed.to_string(), lower_ordinal.to_string()),
                "seed {}",
                seed
            );
            let named_lower = tails_mark_parts(first_lower)
                .1
                .expect("the named mark is figured");
            assert_eq!(named_lower.1, lower_figure, "seed {}", seed);
            let upper_stated: f64 = upper_figure.parse().expect("a closest figure parses");
            let smallest_upper = tails_marks
                .iter()
                .filter_map(|line| tails_mark_parts(line).1)
                .map(|(_, _, _, margin)| {
                    margin.parse::<f64>().expect("an upper tail margin parses")
                })
                .fold(f64::INFINITY, f64::min);
            assert!(
                (upper_stated - smallest_upper).abs() <= 0.000002,
                "seed {}",
                seed
            );
            let first_upper = tails_marks
                .iter()
                .find(|line| {
                    tails_mark_parts(line)
                        .1
                        .map(|(_, _, _, margin)| {
                            (margin.parse::<f64>().expect("an upper tail margin parses")
                                - smallest_upper)
                                .abs()
                                <= 0.000002
                        })
                        .unwrap_or(false)
                })
                .expect("a mark line states the smallest upper tail margin");
            assert_eq!(
                tails_mark_position(first_upper),
                (upper_elapsed.to_string(), upper_ordinal.to_string()),
                "seed {}",
                seed
            );
            let named_upper = tails_mark_parts(first_upper)
                .1
                .expect("the named mark is figured");
            assert_eq!(named_upper.3, upper_figure, "seed {}", seed);
        } else {
            assert!(
                counts.ends_with(", and, no other standing at any mark"),
                "seed {}",
                seed
            );
        }

        // The day close relation of PG-36, computed from
        // the standings sections and the tails sections,
        // the tails taken in the order and at the count
        // PG-35 states over the figures of the competitors
        // other than the closing holder.
        let mut standing_sections: Vec<Vec<String>> = Vec::new();
        for line in &standings {
            if line.starts_with("day, the ") {
                standing_sections.push(Vec::new());
            }
            if let Some(section) = standing_sections.last_mut() {
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
        assert_eq!(
            standing_sections.len(),
            tails_sections.len(),
            "seed {}",
            seed
        );
        assert_eq!(stated_days, standing_sections.len(), "seed {}", seed);
        for (section, tails_section) in standing_sections.iter().zip(tails_sections.iter()) {
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
            let record = closing
                .rsplit("at a projection of ")
                .next()
                .expect("a closing line names its record figure");
            let record_value: f64 = record.parse().expect("a record figure parses");
            let others: Vec<(&str, &str)> = section
                .iter()
                .filter(|line| line.starts_with("competitor "))
                .filter(|line| !line.starts_with(&format!("competitor {},", holder)))
                .filter_map(|line| {
                    line.strip_prefix("competitor ")
                        .and_then(|rest| rest.split_once(", standing projection "))
                        .and_then(|(label, rest)| {
                            rest.strip_suffix(" at the close")
                                .map(|figure| (label, figure))
                        })
                })
                .collect();
            let last = day_marks.last().expect("the day holds a mark");
            let (_last_record, last_tails) = tails_mark_parts(last);
            match (others.is_empty(), last_tails) {
                (true, None) => {}
                (false, Some((lower_figure, lower_margin, upper_figure, upper_margin))) => {
                    let mut figures: Vec<f64> = others
                        .iter()
                        .map(|(_, other)| other.parse::<f64>().expect("a figure parses"))
                        .collect();
                    figures.sort_by(|a, b| a.partial_cmp(b).expect("figures order"));
                    let count = figures.len().div_ceil(2);
                    let expected_lower = figures[..count].iter().sum::<f64>() / count as f64;
                    let expected_upper =
                        figures[figures.len() - count..].iter().sum::<f64>() / count as f64;
                    let lower_value: f64 = lower_figure.parse().expect("a tail figure parses");
                    let upper_value: f64 = upper_figure.parse().expect("a tail figure parses");
                    assert!(
                        (lower_value - expected_lower).abs() <= 0.000002,
                        "seed {}",
                        seed
                    );
                    assert!(
                        (upper_value - expected_upper).abs() <= 0.000002,
                        "seed {}",
                        seed
                    );
                    let lower_margin_value: f64 =
                        lower_margin.parse().expect("a tail margin parses");
                    let upper_margin_value: f64 =
                        upper_margin.parse().expect("a tail margin parses");
                    assert!(
                        (lower_value - record_value - lower_margin_value).abs() <= 0.000002,
                        "seed {}",
                        seed
                    );
                    assert!(
                        (upper_value - record_value - upper_margin_value).abs() <= 0.000002,
                        "seed {}",
                        seed
                    );
                }
                _ => panic!("the day close relation fails, seed {}", seed),
            }
        }
    }
}
