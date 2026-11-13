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
    load_meet_text, meet_days, meet_margins, meet_marks, meet_mean, meet_median, meet_middle,
    meet_midhinge, meet_quartiles, meet_range, meet_reign, meet_report, meet_spread,
    meet_standings, meet_table, meet_tails, meet_traces,
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
            assert_eq!(record, marks_record_figure(mark_line), "seed {}", seed);
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
            assert_eq!(record, marks_record_figure(mark_line), "seed {}", seed);
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
            assert_eq!(record, marks_record_figure(mark_line), "seed {}", seed);
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
            assert_eq!(record, marks_record_figure(mark_line), "seed {}", seed);
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

// The mean pin of the generated family, added with Round
// 18 beside the tails pin above. It fails before the round
// by the absence of meet_mean from this suite's imports,
// and on the built tree it fails against a mean
// construction that drifts from the marks, standings,
// margins, spread, or tails readings over generated
// shapes, which is the drift charge the family exists to
// catch. Its relations are the closings of PG-38 computed
// from the printed lines of the readings alone, the line
// by line agreement with the marks reading, the mean
// margin arithmetic of every mark line, the bracketing
// chain of PG-38 at every mark with its equalities
// standing where the bracket figures equal the figures
// they bracket against, the zero relation in both
// directions where the shapes produce it, the day close
// relation against the standings reading with the mean
// taken over all the figures of the competitors other
// than the closing holder, the counts arithmetic, and
// the closing naming the earliest mark at the smallest
// mean margin. The pin carries no closest tie posture:
// the family's closest mean margin is not hand counted,
// and the closing is checked against the printed mark
// lines alone.

/// The figures of one mean mark line: the record figure,
/// and where another standing stands, the mean figure and
/// the mean margin.
fn mean_mark_parts(line: &str) -> (String, Option<(String, String)>) {
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
    let mean_start = line
        .find(", mean standing ")
        .expect("a figured mark line names its mean standing")
        + ", mean standing ".len();
    let mean_end = line[mean_start..]
        .find(", mean margin ")
        .map(|offset| mean_start + offset)
        .expect("a mean standing names its margin");
    let figure = line[mean_start..mean_end].to_string();
    let margin = line[mean_end + ", mean margin ".len()..].to_string();
    (record, Some((figure, margin)))
}

/// The elapsed figure and ordinal phrase a mean mark line
/// names, read the way the margins helper reads them.
fn mean_mark_position(line: &str) -> (String, String) {
    margins_mark_position(line)
}

#[test]
fn test_generated_meets_close_under_the_mean_reading() {
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
        let tails = meet_tails(&meet).expect("the tails read");
        let mean = meet_mean(&meet).expect("the mean read");
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
        let tails_marks: Vec<&String> = tails
            .iter()
            .filter(|line| line.starts_with("mark "))
            .collect();
        let mean_marks: Vec<&String> = mean
            .iter()
            .filter(|line| line.starts_with("mark "))
            .collect();
        assert_eq!(mean_marks.len(), reading_marks.len(), "seed {}", seed);
        assert_eq!(mean_marks.len(), margin_marks.len(), "seed {}", seed);
        assert_eq!(mean_marks.len(), spread_marks.len(), "seed {}", seed);
        assert_eq!(mean_marks.len(), tails_marks.len(), "seed {}", seed);

        // The line by line agreement of PG-38, the mean
        // margin arithmetic of every mark line, the zero
        // relation in both directions where the shapes
        // produce it, and the bracketing chain of PG-38
        // against the margins, tails, and spread readings
        // at the same mark.
        let mut with_another = 0usize;
        for ((((mean_line, mark_line), margin_line), spread_line), tails_line) in mean_marks
            .iter()
            .zip(reading_marks.iter())
            .zip(margin_marks.iter())
            .zip(spread_marks.iter())
            .zip(tails_marks.iter())
        {
            assert!(
                mean_line.starts_with(&margins_record_prefix(mark_line)),
                "seed {}",
                seed
            );
            let (record, mean_part) = mean_mark_parts(mean_line);
            assert_eq!(record, marks_record_figure(mark_line), "seed {}", seed);
            let (_margin_record, nearest) = margins_mark_parts(margin_line);
            let (_spread_record, _field, farthest) = spread_mark_parts(spread_line);
            let (_tails_record, tails_part) = tails_mark_parts(tails_line);
            if let Some((mean_figure, mean_margin)) = mean_part {
                with_another += 1;
                let record_value: f64 = record.parse().expect("a record figure parses");
                let mean_value: f64 = mean_figure.parse().expect("a mean figure parses");
                let mean_margin_value: f64 = mean_margin.parse().expect("a mean margin parses");
                assert!(mean_margin_value >= 0.0, "seed {}", seed);
                assert!(
                    (mean_value - record_value - mean_margin_value).abs() <= 0.000002,
                    "seed {}",
                    seed
                );
                if mean_margin == "0" {
                    assert_eq!(mean_figure, record, "seed {}", seed);
                }
                if mean_figure == record {
                    assert_eq!(mean_margin, "0", "seed {}", seed);
                }
                let (nearest_figure, _nearest_label, margin_figure) =
                    nearest.expect("the margins line states a nearest standing");
                let margin_value: f64 = margin_figure.parse().expect("a margin figure parses");
                let (farthest_figure, _farthest_label, spread_figure) =
                    farthest.expect("the spread line states a farthest standing");
                let spread_value: f64 = spread_figure.parse().expect("a spread figure parses");
                let (lower_figure, lower_margin, upper_figure, upper_margin) =
                    tails_part.expect("the tails line states its tails");
                let lower_value: f64 = lower_figure.parse().expect("a lower tail figure parses");
                let lower_margin_value: f64 =
                    lower_margin.parse().expect("a lower tail margin parses");
                let upper_value: f64 = upper_figure.parse().expect("an upper tail figure parses");
                let upper_margin_value: f64 =
                    upper_margin.parse().expect("an upper tail margin parses");
                if mean_margin == "0" {
                    assert_eq!(nearest_figure, record, "seed {}", seed);
                    assert_eq!(lower_figure, record, "seed {}", seed);
                }
                assert!(
                    margin_value <= lower_margin_value + 0.000002,
                    "seed {}",
                    seed
                );
                assert!(
                    lower_margin_value <= mean_margin_value + 0.000002,
                    "seed {}",
                    seed
                );
                assert!(
                    mean_margin_value <= upper_margin_value + 0.000002,
                    "seed {}",
                    seed
                );
                assert!(
                    upper_margin_value <= spread_value + 0.000002,
                    "seed {}",
                    seed
                );
                if lower_figure == mean_figure {
                    assert_eq!(lower_margin, mean_margin, "seed {}", seed);
                }
                if lower_margin == mean_margin {
                    assert!(
                        (lower_value - mean_value).abs() <= 0.000002,
                        "seed {}",
                        seed
                    );
                }
                if upper_figure == mean_figure {
                    assert_eq!(upper_margin, mean_margin, "seed {}", seed);
                }
                if upper_margin == mean_margin {
                    assert!(
                        (upper_value - mean_value).abs() <= 0.000002,
                        "seed {}",
                        seed
                    );
                }
                let _ = farthest_figure;
            } else {
                assert!(nearest.is_none(), "seed {}", seed);
                assert!(farthest.is_none(), "seed {}", seed);
                assert!(tails_part.is_none(), "seed {}", seed);
            }
        }

        // The counts arithmetic of PG-38, and the closing
        // naming the earliest mark at the smallest mean
        // margin.
        let counts = mean.last().expect("the mean close");
        let walked = count_after(counts, "marks walked ");
        assert_eq!(walked, mean_marks.len(), "seed {}", seed);
        let stated_with = count_after(counts, "marks with another standing ");
        assert_eq!(stated_with, with_another, "seed {}", seed);
        let stated_days = count_after(counts, "days ");
        let no_other = mean_marks
            .iter()
            .filter(|line| line.ends_with(", and no other standing"))
            .count();
        assert_eq!(with_another + no_other, walked, "seed {}", seed);
        if with_another > 0 {
            let closest_part = counts
                .split("closest mean margin ")
                .nth(1)
                .expect("the counts line names its closest mean margin");
            let (closest_figure, rest) = closest_part
                .split_once(" at ")
                .expect("the closest mean margin names its mark");
            let (closest_elapsed, closest_ordinal) = rest
                .split_once(" seconds on the ")
                .expect("the closest mean margin names its seconds");
            let closest_ordinal = closest_ordinal
                .strip_suffix(" day")
                .expect("the closest mean margin names its day");
            let stated: f64 = closest_figure.parse().expect("a closest figure parses");
            let smallest = mean_marks
                .iter()
                .filter_map(|line| mean_mark_parts(line).1)
                .map(|(_, margin)| margin.parse::<f64>().expect("a mean margin parses"))
                .fold(f64::INFINITY, f64::min);
            assert!((stated - smallest).abs() <= 0.000002, "seed {}", seed);
            let first = mean_marks
                .iter()
                .find(|line| {
                    mean_mark_parts(line)
                        .1
                        .map(|(_, margin)| {
                            (margin.parse::<f64>().expect("a mean margin parses") - smallest).abs()
                                <= 0.000002
                        })
                        .unwrap_or(false)
                })
                .expect("a mark line states the smallest mean margin");
            assert_eq!(
                mean_mark_position(first),
                (closest_elapsed.to_string(), closest_ordinal.to_string()),
                "seed {}",
                seed
            );
            let named = mean_mark_parts(first).1.expect("the named mark is figured");
            assert_eq!(named.1, closest_figure, "seed {}", seed);
        } else {
            assert!(
                counts.ends_with(", and, no other standing at any mark"),
                "seed {}",
                seed
            );
        }

        // The day close relation of PG-38, computed from
        // the standings sections and the mean sections,
        // the mean taken over all the figures of the
        // competitors other than the closing holder.
        let mut standing_sections: Vec<Vec<String>> = Vec::new();
        for line in &standings {
            if line.starts_with("day, the ") {
                standing_sections.push(Vec::new());
            }
            if let Some(section) = standing_sections.last_mut() {
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
        assert_eq!(
            standing_sections.len(),
            mean_sections.len(),
            "seed {}",
            seed
        );
        assert_eq!(stated_days, standing_sections.len(), "seed {}", seed);
        for (section, mean_section) in standing_sections.iter().zip(mean_sections.iter()) {
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
            let record = closing
                .rsplit("at a projection of ")
                .next()
                .expect("a closing line names its record figure");
            let record_value: f64 = record.parse().expect("a record figure parses");
            let others: Vec<f64> = section
                .iter()
                .filter(|line| line.starts_with("competitor "))
                .filter(|line| !line.starts_with(&format!("competitor {},", holder)))
                .filter_map(|line| {
                    line.strip_prefix("competitor ")
                        .and_then(|rest| rest.split_once(", standing projection "))
                        .and_then(|(_, rest)| {
                            rest.strip_suffix(" at the close")
                                .and_then(|figure| figure.parse::<f64>().ok())
                        })
                })
                .collect();
            let last = day_marks.last().expect("the day holds a mark");
            let (_last_record, last_mean) = mean_mark_parts(last);
            match (others.is_empty(), last_mean) {
                (true, None) => {}
                (false, Some((mean_figure, mean_margin))) => {
                    let expected = others.iter().sum::<f64>() / others.len() as f64;
                    let mean_value: f64 = mean_figure.parse().expect("a mean figure parses");
                    assert!((mean_value - expected).abs() <= 0.000002, "seed {}", seed);
                    let mean_margin_value: f64 = mean_margin.parse().expect("a mean margin parses");
                    assert!(
                        (mean_value - record_value - mean_margin_value).abs() <= 0.000002,
                        "seed {}",
                        seed
                    );
                }
                _ => panic!("the day close relation fails, seed {}", seed),
            }
        }
    }
}

// The median pin of the generated family, added with
// Round 19 beside the mean pin above. It fails before
// the round by the absence of meet_median from this
// suite's imports, and on the built tree it fails
// against a median construction that drifts from the
// marks, standings, margins, spread, middle, tails, or
// mean readings over generated shapes, which is the
// drift charge the family exists to catch. Its
// relations are the closings of PG-40 computed from the
// printed lines of the readings alone, the line by line
// agreement with the marks reading, the median margin
// arithmetic of every mark line, the bracketing chain
// of PG-40 at every mark, margin never above lower
// tail never above median never above upper tail never
// above spread, with the middle margin never above the
// median margin beside it and equal to it where the
// count of the field is odd, the zero relation's
// clauses in their conditional form, the day close
// relation against the standings reading with the
// median taken over the figures of the competitors
// other than the closing holder in the order and at
// the places PG-39 states, the counts arithmetic, and
// the closing naming the earliest mark at the smallest
// median margin. The family's executed shape stands
// behind the clauses: fields reach five to nine behind
// the holder, figured marks number 876 over the eight
// meets, the median separates from the mean at 842 of
// them and from the middle figure at 349, the day
// closes stand at even counts in seven of the eight
// seeds, and seed MEET_SEED plus 6 ties its closest
// median margin, 68.348907, at two marks, so the
// earliest naming the closing asserts is a real tie
// and not a vacuous one. No generated meet carries a
// median margin of zero, so the zero relation's
// clauses stand in their conditional form and are
// checked in neither direction by this family. The
// mean margin stands in no fixed order against the
// median margin, and the pin counts both directions
// over the family and asserts each stands somewhere,
/// The figures of one median mark line: the record
/// figure, and where another standing stands, the
/// median figure and the median margin.
fn median_mark_parts(line: &str) -> (String, Option<(String, String)>) {
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
    let median_start = line
        .find(", median standing ")
        .expect("a figured mark line names its median standing")
        + ", median standing ".len();
    let median_end = line[median_start..]
        .find(", median margin ")
        .map(|offset| median_start + offset)
        .expect("a median standing names its margin");
    let figure = line[median_start..median_end].to_string();
    let margin = line[median_end + ", median margin ".len()..].to_string();
    (record, Some((figure, margin)))
}

/// The elapsed figure and ordinal phrase a median mark
/// line names, read the way the margins helper reads
/// them.
fn median_mark_position(line: &str) -> (String, String) {
    margins_mark_position(line)
}

/// The record figure a marks reading line names, read
/// the way the family's helpers read it. The median pin
/// compares it against the median line's own record
/// figure, because the prefix the line by line agreement
/// opens with ends at the figure's last digit, and a
/// median line naming a longer figure under the same
/// opening digits would pass the prefix while its record
/// stood apart from the marks record.
fn marks_record_figure(line: &str) -> String {
    let start = line
        .find("at a projection of ")
        .expect("a mark line names its record figure");
    let figure_start = start + "at a projection of ".len();
    let figure_end = line[figure_start..]
        .find(',')
        .map(|offset| figure_start + offset)
        .expect("a record figure ends at a comma");
    line[figure_start..figure_end].to_string()
}

#[test]
fn test_generated_meets_close_under_the_median_reading() {
    let mut mean_above = 0usize;
    let mut mean_below = 0usize;
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
        let mean = meet_mean(&meet).expect("the mean read");
        let median = meet_median(&meet).expect("the median read");
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
        let mean_marks: Vec<&String> = mean
            .iter()
            .filter(|line| line.starts_with("mark "))
            .collect();
        let median_marks: Vec<&String> = median
            .iter()
            .filter(|line| line.starts_with("mark "))
            .collect();
        assert_eq!(median_marks.len(), reading_marks.len(), "seed {}", seed);
        assert_eq!(median_marks.len(), margin_marks.len(), "seed {}", seed);
        assert_eq!(median_marks.len(), spread_marks.len(), "seed {}", seed);
        assert_eq!(median_marks.len(), middle_marks.len(), "seed {}", seed);
        assert_eq!(median_marks.len(), tails_marks.len(), "seed {}", seed);
        assert_eq!(median_marks.len(), mean_marks.len(), "seed {}", seed);

        // The line by line agreement of PG-40, the
        // median margin arithmetic of every mark line,
        // the zero relation's clauses in their
        // conditional form, and the bracketing chain of
        // PG-40 against the margins, tails, middle, and
        // spread readings at the same mark.
        let mut with_another = 0usize;
        for (
            (((((median_line, mark_line), margin_line), spread_line), middle_line), tails_line),
            mean_line,
        ) in median_marks
            .iter()
            .zip(reading_marks.iter())
            .zip(margin_marks.iter())
            .zip(spread_marks.iter())
            .zip(middle_marks.iter())
            .zip(tails_marks.iter())
            .zip(mean_marks.iter())
        {
            assert!(
                median_line.starts_with(&margins_record_prefix(mark_line)),
                "seed {}",
                seed
            );
            let (record, median_part) = median_mark_parts(median_line);
            assert_eq!(record, marks_record_figure(mark_line), "seed {}", seed);
            let (_margin_record, nearest) = margins_mark_parts(margin_line);
            let (_spread_record, field, farthest) = spread_mark_parts(spread_line);
            let (_middle_record, middle_part) = middle_mark_parts(middle_line);
            let (_tails_record, tails_part) = tails_mark_parts(tails_line);
            let (_mean_record, mean_part) = mean_mark_parts(mean_line);
            if let Some((median_figure, median_margin)) = median_part {
                with_another += 1;
                let record_value: f64 = record.parse().expect("a record figure parses");
                let median_value: f64 = median_figure.parse().expect("a median figure parses");
                let median_margin_value: f64 =
                    median_margin.parse().expect("a median margin parses");
                assert!(median_margin_value >= 0.0, "seed {}", seed);
                assert!(
                    (median_value - record_value - median_margin_value).abs() <= 0.000002,
                    "seed {}",
                    seed
                );
                if median_margin == "0" {
                    assert_eq!(median_figure, record, "seed {}", seed);
                }
                if median_figure == record {
                    assert_eq!(median_margin, "0", "seed {}", seed);
                }
                let (nearest_figure, _nearest_label, margin_figure) =
                    nearest.expect("the margins line states a nearest standing");
                let margin_value: f64 = margin_figure.parse().expect("a margin figure parses");
                let (farthest_figure, _farthest_label, spread_figure) =
                    farthest.expect("the spread line states a farthest standing");
                let spread_value: f64 = spread_figure.parse().expect("a spread figure parses");
                let (lower_figure, lower_margin, upper_figure, upper_margin) =
                    tails_part.expect("the tails line states its tails");
                let lower_value: f64 = lower_figure.parse().expect("a lower tail figure parses");
                let lower_margin_value: f64 =
                    lower_margin.parse().expect("a lower tail margin parses");
                let upper_value: f64 = upper_figure.parse().expect("an upper tail figure parses");
                let upper_margin_value: f64 =
                    upper_margin.parse().expect("an upper tail margin parses");
                if median_margin == "0" {
                    assert_eq!(nearest_figure, record, "seed {}", seed);
                    assert_eq!(lower_figure, record, "seed {}", seed);
                }
                assert!(
                    margin_value <= lower_margin_value + 0.000002,
                    "seed {}",
                    seed
                );
                assert!(
                    lower_margin_value <= median_margin_value + 0.000002,
                    "seed {}",
                    seed
                );
                assert!(
                    median_margin_value <= upper_margin_value + 0.000002,
                    "seed {}",
                    seed
                );
                assert!(
                    upper_margin_value <= spread_value + 0.000002,
                    "seed {}",
                    seed
                );
                if lower_figure == median_figure {
                    assert_eq!(lower_margin, median_margin, "seed {}", seed);
                }
                if lower_margin == median_margin {
                    assert!(
                        (lower_value - median_value).abs() <= 0.000002,
                        "seed {}",
                        seed
                    );
                }
                if upper_figure == median_figure {
                    assert_eq!(upper_margin, median_margin, "seed {}", seed);
                }
                if upper_margin == median_margin {
                    assert!(
                        (upper_value - median_value).abs() <= 0.000002,
                        "seed {}",
                        seed
                    );
                }
                let (middle_figure, _middle_label, middle_margin) =
                    middle_part.expect("the middle line states a middle standing");
                let middle_value: f64 = middle_figure.parse().expect("a middle figure parses");
                let middle_margin_value: f64 =
                    middle_margin.parse().expect("a middle margin parses");
                assert!(
                    middle_margin_value <= median_margin_value + 0.000002,
                    "seed {}",
                    seed
                );
                let behind = field - 1;
                if behind % 2 == 1 {
                    assert_eq!(middle_margin, median_margin, "seed {}", seed);
                    assert!(
                        (middle_value - median_value).abs() <= 0.000002,
                        "seed {}",
                        seed
                    );
                }
                if middle_figure == median_figure {
                    assert_eq!(middle_margin, median_margin, "seed {}", seed);
                }
                if middle_margin == median_margin {
                    assert!(
                        (middle_value - median_value).abs() <= 0.000002,
                        "seed {}",
                        seed
                    );
                }
                let (_mean_figure, mean_margin) =
                    mean_part.expect("the mean line states a mean standing");
                let mean_margin_value: f64 = mean_margin.parse().expect("a mean margin parses");
                if mean_margin_value > median_margin_value + 0.000002 {
                    mean_above += 1;
                }
                if mean_margin_value + 0.000002 < median_margin_value {
                    mean_below += 1;
                }
                let _ = farthest_figure;
            } else {
                assert!(nearest.is_none(), "seed {}", seed);
                assert!(farthest.is_none(), "seed {}", seed);
                assert!(middle_part.is_none(), "seed {}", seed);
                assert!(tails_part.is_none(), "seed {}", seed);
                assert!(mean_part.is_none(), "seed {}", seed);
            }
        }

        // The counts arithmetic of PG-40, and the
        // closing naming the earliest mark at the
        // smallest median margin.
        let counts = median.last().expect("the median close");
        let walked = count_after(counts, "marks walked ");
        assert_eq!(walked, median_marks.len(), "seed {}", seed);
        let stated_with = count_after(counts, "marks with another standing ");
        assert_eq!(stated_with, with_another, "seed {}", seed);
        let stated_days = count_after(counts, "days ");
        let no_other = median_marks
            .iter()
            .filter(|line| line.ends_with(", and no other standing"))
            .count();
        assert_eq!(with_another + no_other, walked, "seed {}", seed);
        if with_another > 0 {
            let closest_part = counts
                .split("closest median margin ")
                .nth(1)
                .expect("the counts line names its closest median margin");
            let (closest_figure, rest) = closest_part
                .split_once(" at ")
                .expect("the closest median margin names its mark");
            let (closest_elapsed, closest_ordinal) = rest
                .split_once(" seconds on the ")
                .expect("the closest median margin names its seconds");
            let closest_ordinal = closest_ordinal
                .strip_suffix(" day")
                .expect("the closest median margin names its day");
            let stated: f64 = closest_figure.parse().expect("a closest figure parses");
            let smallest = median_marks
                .iter()
                .filter_map(|line| median_mark_parts(line).1)
                .map(|(_, margin)| margin.parse::<f64>().expect("a median margin parses"))
                .fold(f64::INFINITY, f64::min);
            assert!((stated - smallest).abs() <= 0.000002, "seed {}", seed);
            let first = median_marks
                .iter()
                .find(|line| {
                    median_mark_parts(line)
                        .1
                        .map(|(_, margin)| {
                            (margin.parse::<f64>().expect("a median margin parses") - smallest)
                                .abs()
                                <= 0.000002
                        })
                        .unwrap_or(false)
                })
                .expect("a mark line states the smallest median margin");
            assert_eq!(
                median_mark_position(first),
                (closest_elapsed.to_string(), closest_ordinal.to_string()),
                "seed {}",
                seed
            );
            let named = median_mark_parts(first)
                .1
                .expect("the named mark is figured");
            assert_eq!(named.1, closest_figure, "seed {}", seed);
        } else {
            assert!(
                counts.ends_with(", and, no other standing at any mark"),
                "seed {}",
                seed
            );
        }

        // The day close relation of PG-40, computed
        // from the standings sections and the median
        // sections, the median taken over the figures
        // of the competitors other than the closing
        // holder in the order and at the places PG-39
        // states.
        let mut standing_sections: Vec<Vec<String>> = Vec::new();
        for line in &standings {
            if line.starts_with("day, the ") {
                standing_sections.push(Vec::new());
            }
            if let Some(section) = standing_sections.last_mut() {
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
        assert_eq!(
            standing_sections.len(),
            median_sections.len(),
            "seed {}",
            seed
        );
        assert_eq!(stated_days, standing_sections.len(), "seed {}", seed);
        for (section, median_section) in standing_sections.iter().zip(median_sections.iter()) {
            let day_marks: Vec<&&String> = median_section
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
            let mut others: Vec<f64> = section
                .iter()
                .filter(|line| line.starts_with("competitor "))
                .filter(|line| !line.starts_with(&format!("competitor {},", holder)))
                .filter_map(|line| {
                    line.strip_prefix("competitor ")
                        .and_then(|rest| rest.split_once(", standing projection "))
                        .and_then(|(_, rest)| {
                            rest.strip_suffix(" at the close")
                                .and_then(|figure| figure.parse::<f64>().ok())
                        })
                })
                .collect();
            let last = day_marks.last().expect("the day holds a mark");
            let (_last_record, last_median) = median_mark_parts(last);
            match (others.is_empty(), last_median) {
                (true, None) => {}
                (false, Some((median_figure, median_margin))) => {
                    others.sort_by(|a, b| a.partial_cmp(b).expect("figures order"));
                    let expected = if others.len() % 2 == 1 {
                        others[others.len() / 2]
                    } else {
                        (others[others.len() / 2 - 1] + others[others.len() / 2]) / 2.0
                    };
                    let median_value: f64 = median_figure.parse().expect("a median figure parses");
                    assert!((median_value - expected).abs() <= 0.000002, "seed {}", seed);
                    let median_margin_value: f64 =
                        median_margin.parse().expect("a median margin parses");
                    assert!(
                        (median_value - record_value - median_margin_value).abs() <= 0.000002,
                        "seed {}",
                        seed
                    );
                }
                _ => panic!("the day close relation fails, seed {}", seed),
            }
        }
    }
    assert!(
        mean_above > 0,
        "the mean margin stands above the median margin at some mark of the family"
    );
    assert!(
        mean_below > 0,
        "the mean margin stands below the median margin at some mark of the family"
    );
}

/// The lower and upper quartile parts of one mark line,
/// each the standing figure and the margin as printed.
type QuartileParts = ((String, String), (String, String));

/// The figures of one quartiles mark line: the record
/// figure, and where another standing stands, the lower
/// quartile figure and margin and the upper quartile
/// figure and margin.
fn quartiles_mark_parts(line: &str) -> (String, Option<QuartileParts>) {
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
        .find(", lower quartile standing ")
        .expect("a figured mark line names its lower quartile standing")
        + ", lower quartile standing ".len();
    let lower_end = line[lower_start..]
        .find(", lower quartile margin ")
        .map(|offset| lower_start + offset)
        .expect("a lower quartile standing names its margin");
    let lower_figure = line[lower_start..lower_end].to_string();
    let lower_margin_start = lower_end + ", lower quartile margin ".len();
    let lower_margin_end = line[lower_margin_start..]
        .find(", upper quartile standing ")
        .map(|offset| lower_margin_start + offset)
        .expect("a lower quartile margin names the upper quartile standing");
    let lower_margin = line[lower_margin_start..lower_margin_end].to_string();
    let upper_start = lower_margin_end + ", upper quartile standing ".len();
    let upper_end = line[upper_start..]
        .find(", upper quartile margin ")
        .map(|offset| upper_start + offset)
        .expect("an upper quartile standing names its margin");
    let upper_figure = line[upper_start..upper_end].to_string();
    let upper_margin = line[upper_end + ", upper quartile margin ".len()..].to_string();
    (
        record,
        Some(((lower_figure, lower_margin), (upper_figure, upper_margin))),
    )
}

/// The elapsed figure and ordinal phrase a quartiles mark
/// line names, read the way the margins helper reads
/// them.
fn quartiles_mark_position(line: &str) -> (String, String) {
    margins_mark_position(line)
}

#[test]
fn test_generated_meets_close_under_the_quartiles_reading() {
    let mut lower_above = 0usize;
    let mut lower_below = 0usize;
    let mut upper_above = 0usize;
    let mut upper_below = 0usize;
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
        let median = meet_median(&meet).expect("the median read");
        let tails = meet_tails(&meet).expect("the tails read");
        let quartiles = meet_quartiles(&meet).expect("the quartiles read");
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
        let median_marks: Vec<&String> = median
            .iter()
            .filter(|line| line.starts_with("mark "))
            .collect();
        let tails_marks: Vec<&String> = tails
            .iter()
            .filter(|line| line.starts_with("mark "))
            .collect();
        let quartiles_marks: Vec<&String> = quartiles
            .iter()
            .filter(|line| line.starts_with("mark "))
            .collect();
        assert_eq!(quartiles_marks.len(), reading_marks.len(), "seed {}", seed);
        assert_eq!(quartiles_marks.len(), margin_marks.len(), "seed {}", seed);
        assert_eq!(quartiles_marks.len(), spread_marks.len(), "seed {}", seed);
        assert_eq!(quartiles_marks.len(), middle_marks.len(), "seed {}", seed);
        assert_eq!(quartiles_marks.len(), median_marks.len(), "seed {}", seed);
        assert_eq!(quartiles_marks.len(), tails_marks.len(), "seed {}", seed);

        // The line by line agreement of PG-42, the
        // quartile margin arithmetic of every mark line,
        // the zero relation's clauses in their
        // conditional form, the bracketing chain of
        // PG-42 against the margins, middle, median, and
        // spread readings at the same mark, and the no
        // fixed order relations against the tails reading
        // counted in both directions.
        let mut with_another = 0usize;
        for (
            (((((quartile_line, mark_line), margin_line), spread_line), middle_line), median_line),
            tails_line,
        ) in quartiles_marks
            .iter()
            .zip(reading_marks.iter())
            .zip(margin_marks.iter())
            .zip(spread_marks.iter())
            .zip(middle_marks.iter())
            .zip(median_marks.iter())
            .zip(tails_marks.iter())
        {
            assert!(
                quartile_line.starts_with(&margins_record_prefix(mark_line)),
                "seed {}",
                seed
            );
            let (record, quartile_part) = quartiles_mark_parts(quartile_line);
            assert_eq!(record, marks_record_figure(mark_line), "seed {}", seed);
            let (_margin_record, nearest) = margins_mark_parts(margin_line);
            let (_spread_record, _field, farthest) = spread_mark_parts(spread_line);
            let (_middle_record, middle_part) = middle_mark_parts(middle_line);
            let (_median_record, median_part) = median_mark_parts(median_line);
            let (_tails_record, tails_part) = tails_mark_parts(tails_line);
            if let Some(((lower_figure, lower_margin), (upper_figure, upper_margin))) =
                quartile_part
            {
                with_another += 1;
                let record_value: f64 = record.parse().expect("a record figure parses");
                let lower_value: f64 = lower_figure
                    .parse()
                    .expect("a lower quartile figure parses");
                let lower_margin_value: f64 = lower_margin
                    .parse()
                    .expect("a lower quartile margin parses");
                let upper_value: f64 = upper_figure
                    .parse()
                    .expect("an upper quartile figure parses");
                let upper_margin_value: f64 = upper_margin
                    .parse()
                    .expect("an upper quartile margin parses");
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
                let (_nearest_figure, _nearest_label, margin_figure) =
                    nearest.expect("the margins line states a nearest standing");
                let margin_value: f64 = margin_figure.parse().expect("a margin figure parses");
                let (_farthest_figure, _farthest_label, spread_figure) =
                    farthest.expect("the spread line states a farthest standing");
                let spread_value: f64 = spread_figure.parse().expect("a spread figure parses");
                let (_middle_figure, _middle_label, middle_margin) =
                    middle_part.expect("the middle line states a middle standing");
                let middle_margin_value: f64 =
                    middle_margin.parse().expect("a middle margin parses");
                let (_median_figure, median_margin) =
                    median_part.expect("the median line states a median standing");
                let median_margin_value: f64 =
                    median_margin.parse().expect("a median margin parses");
                let (_lower_tail_figure, lower_tail_margin, _upper_tail_figure, upper_tail_margin) =
                    tails_part.expect("the tails line states its tails");
                let lower_tail_margin_value: f64 = lower_tail_margin
                    .parse()
                    .expect("a lower tail margin parses");
                let upper_tail_margin_value: f64 = upper_tail_margin
                    .parse()
                    .expect("an upper tail margin parses");
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
                    middle_margin_value <= median_margin_value + 0.000002,
                    "seed {}",
                    seed
                );
                assert!(
                    median_margin_value <= upper_margin_value + 0.000002,
                    "seed {}",
                    seed
                );
                assert!(
                    upper_margin_value <= spread_value + 0.000002,
                    "seed {}",
                    seed
                );
                if lower_margin_value > lower_tail_margin_value + 0.000002 {
                    lower_above += 1;
                }
                if lower_margin_value + 0.000002 < lower_tail_margin_value {
                    lower_below += 1;
                }
                if upper_margin_value > upper_tail_margin_value + 0.000002 {
                    upper_above += 1;
                }
                if upper_margin_value + 0.000002 < upper_tail_margin_value {
                    upper_below += 1;
                }
            } else {
                assert!(nearest.is_none(), "seed {}", seed);
                assert!(farthest.is_none(), "seed {}", seed);
                assert!(middle_part.is_none(), "seed {}", seed);
                assert!(median_part.is_none(), "seed {}", seed);
                assert!(tails_part.is_none(), "seed {}", seed);
            }
        }

        // The counts arithmetic of PG-42, and the two
        // closings naming the earliest mark at each
        // smallest quartile margin.
        let counts = quartiles.last().expect("the quartiles close");
        let walked = count_after(counts, "marks walked ");
        assert_eq!(walked, quartiles_marks.len(), "seed {}", seed);
        let stated_with = count_after(counts, "marks with another standing ");
        assert_eq!(stated_with, with_another, "seed {}", seed);
        let stated_days = count_after(counts, "days ");
        let no_other = quartiles_marks
            .iter()
            .filter(|line| line.ends_with(", and no other standing"))
            .count();
        assert_eq!(with_another + no_other, walked, "seed {}", seed);
        if with_another > 0 {
            let lower_part = counts
                .split("closest lower quartile margin ")
                .nth(1)
                .expect("the counts line names its closest lower quartile margin");
            let (lower_closest_figure, lower_rest) = lower_part
                .split_once(" at ")
                .expect("the closest lower quartile margin names its mark");
            let (lower_closest_elapsed, lower_rest) = lower_rest
                .split_once(" seconds on the ")
                .expect("the closest lower quartile margin names its seconds");
            let (lower_closest_ordinal, upper_part) = lower_rest
                .split_once(" day, closest upper quartile margin ")
                .expect("the counts line names its closest upper quartile margin");
            let (upper_closest_figure, upper_rest) = upper_part
                .split_once(" at ")
                .expect("the closest upper quartile margin names its mark");
            let (upper_closest_elapsed, upper_closest_ordinal) = upper_rest
                .split_once(" seconds on the ")
                .expect("the closest upper quartile margin names its seconds");
            let upper_closest_ordinal = upper_closest_ordinal
                .strip_suffix(" day")
                .expect("the closest upper quartile margin names its day");
            let lower_stated: f64 = lower_closest_figure
                .parse()
                .expect("a closest lower figure parses");
            let lower_smallest = quartiles_marks
                .iter()
                .filter_map(|line| quartiles_mark_parts(line).1)
                .map(|((_, margin), _)| {
                    margin
                        .parse::<f64>()
                        .expect("a lower quartile margin parses")
                })
                .fold(f64::INFINITY, f64::min);
            assert!(
                (lower_stated - lower_smallest).abs() <= 0.000002,
                "seed {}",
                seed
            );
            let lower_first = quartiles_marks
                .iter()
                .find(|line| {
                    quartiles_mark_parts(line)
                        .1
                        .map(|((_, margin), _)| {
                            (margin
                                .parse::<f64>()
                                .expect("a lower quartile margin parses")
                                - lower_smallest)
                                .abs()
                                <= 0.000002
                        })
                        .unwrap_or(false)
                })
                .expect("a mark line states the smallest lower quartile margin");
            assert_eq!(
                quartiles_mark_position(lower_first),
                (
                    lower_closest_elapsed.to_string(),
                    lower_closest_ordinal.to_string()
                ),
                "seed {}",
                seed
            );
            let lower_named = quartiles_mark_parts(lower_first)
                .1
                .expect("the named mark is figured");
            assert_eq!(lower_named.0 .1, lower_closest_figure, "seed {}", seed);
            let upper_stated: f64 = upper_closest_figure
                .parse()
                .expect("a closest upper figure parses");
            let upper_smallest = quartiles_marks
                .iter()
                .filter_map(|line| quartiles_mark_parts(line).1)
                .map(|(_, (_, margin))| {
                    margin
                        .parse::<f64>()
                        .expect("an upper quartile margin parses")
                })
                .fold(f64::INFINITY, f64::min);
            assert!(
                (upper_stated - upper_smallest).abs() <= 0.000002,
                "seed {}",
                seed
            );
            let upper_first = quartiles_marks
                .iter()
                .find(|line| {
                    quartiles_mark_parts(line)
                        .1
                        .map(|(_, (_, margin))| {
                            (margin
                                .parse::<f64>()
                                .expect("an upper quartile margin parses")
                                - upper_smallest)
                                .abs()
                                <= 0.000002
                        })
                        .unwrap_or(false)
                })
                .expect("a mark line states the smallest upper quartile margin");
            assert_eq!(
                quartiles_mark_position(upper_first),
                (
                    upper_closest_elapsed.to_string(),
                    upper_closest_ordinal.to_string()
                ),
                "seed {}",
                seed
            );
            let upper_named = quartiles_mark_parts(upper_first)
                .1
                .expect("the named mark is figured");
            assert_eq!(upper_named.1 .1, upper_closest_figure, "seed {}", seed);
        } else {
            assert!(
                counts.ends_with(", and, no other standing at any mark"),
                "seed {}",
                seed
            );
        }

        // The day close relation of PG-42, computed
        // from the standings sections and the quartiles
        // sections, the quartiles taken over the figures
        // of the competitors other than the closing
        // holder in the order and at the count PG-41
        // states.
        let mut standing_sections: Vec<Vec<String>> = Vec::new();
        for line in &standings {
            if line.starts_with("day, the ") {
                standing_sections.push(Vec::new());
            }
            if let Some(section) = standing_sections.last_mut() {
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
        assert_eq!(
            standing_sections.len(),
            quartiles_sections.len(),
            "seed {}",
            seed
        );
        assert_eq!(stated_days, standing_sections.len(), "seed {}", seed);
        for (section, quartiles_section) in standing_sections.iter().zip(quartiles_sections.iter())
        {
            let day_marks: Vec<&&String> = quartiles_section
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
            let mut others: Vec<f64> = section
                .iter()
                .filter(|line| line.starts_with("competitor "))
                .filter(|line| !line.starts_with(&format!("competitor {},", holder)))
                .filter_map(|line| {
                    line.strip_prefix("competitor ")
                        .and_then(|rest| rest.split_once(", standing projection "))
                        .and_then(|(_, rest)| {
                            rest.strip_suffix(" at the close")
                                .and_then(|figure| figure.parse::<f64>().ok())
                        })
                })
                .collect();
            let last = day_marks.last().expect("the day holds a mark");
            let (_last_record, last_quartiles) = quartiles_mark_parts(last);
            match (others.is_empty(), last_quartiles) {
                (true, None) => {}
                (false, Some(((lower_figure, lower_margin), (upper_figure, upper_margin)))) => {
                    others.sort_by(|a, b| a.partial_cmp(b).expect("figures order"));
                    let count = others.len().div_ceil(2);
                    let median_of = |part: &[f64]| {
                        if part.len() % 2 == 1 {
                            part[part.len() / 2]
                        } else {
                            (part[part.len() / 2 - 1] + part[part.len() / 2]) / 2.0
                        }
                    };
                    let expected_lower = median_of(&others[..count]);
                    let expected_upper = median_of(&others[others.len() - count..]);
                    let lower_value: f64 = lower_figure
                        .parse()
                        .expect("a lower quartile figure parses");
                    let upper_value: f64 = upper_figure
                        .parse()
                        .expect("an upper quartile figure parses");
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
                    let lower_margin_value: f64 = lower_margin
                        .parse()
                        .expect("a lower quartile margin parses");
                    let upper_margin_value: f64 = upper_margin
                        .parse()
                        .expect("an upper quartile margin parses");
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
    assert!(
        lower_above > 0,
        "the lower quartile margin stands above the lower tail margin at some mark of the family"
    );
    assert!(
        lower_below > 0,
        "the lower quartile margin stands below the lower tail margin at some mark of the family"
    );
    assert!(
        upper_above > 0,
        "the upper quartile margin stands above the upper tail margin at some mark of the family"
    );
    assert!(
        upper_below > 0,
        "the upper quartile margin stands below the upper tail margin at some mark of the family"
    );
}

/// The figures of one range mark line: the record
/// figure, and where another standing stands, the
/// interquartile range figure as printed.
fn range_mark_parts(line: &str) -> (String, Option<String>) {
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
    let range_start = line
        .find(", interquartile range ")
        .expect("a figured mark line names its interquartile range")
        + ", interquartile range ".len();
    (record, Some(line[range_start..].to_string()))
}

/// The elapsed figure and ordinal phrase a range mark
/// line names, read the way the margins helper reads
/// them.
fn range_mark_position(line: &str) -> (String, String) {
    margins_mark_position(line)
}

/// The middle place figures of one part of a field, the
/// one middle figure where the part counts odd and the
/// two where it counts even.
fn middle_place_figures(part: &[f64]) -> Vec<f64> {
    if part.len() % 2 == 1 {
        vec![part[part.len() / 2]]
    } else {
        vec![part[part.len() / 2 - 1], part[part.len() / 2]]
    }
}

/// Whether every middle place figure of the lowest and
/// the highest sub-fields of one ordered field stands at
/// one and the same figure, the zero condition of PG-43
/// in its field form.
fn zero_condition_holds(field: &[f64]) -> bool {
    let count = field.len().div_ceil(2);
    let mut figures = middle_place_figures(&field[..count]);
    figures.extend(middle_place_figures(&field[field.len() - count..]));
    figures.iter().all(|figure| *figure == figures[0])
}

#[test]
fn test_generated_meets_close_under_the_range_reading() {
    let mut family_figured = 0usize;
    let mut family_above = 0usize;
    let mut family_below = 0usize;
    let mut family_equal = 0usize;
    let mut family_zero = 0usize;
    for index in 0..8u64 {
        let seed = MEET_SEED + index;
        let mut gen = Gen::new(seed);
        let text = meet_text(&mut gen);
        let (meet, defects) = load_meet_text(&text);
        assert_eq!(defects, Vec::<String>::new(), "seed {}", seed);
        let meet = meet.expect("the generated meet loads");

        let marks = meet_marks(&meet).expect("the marks read");
        let quartiles = meet_quartiles(&meet).expect("the quartiles read");
        let spread = meet_spread(&meet).expect("the spread read");
        let tails = meet_tails(&meet).expect("the tails read");
        let range = meet_range(&meet).expect("the range read");
        let standings = meet_standings(&meet).expect("the standings read");

        let reading_marks: Vec<&String> = marks
            .iter()
            .filter(|line| line.starts_with("mark "))
            .collect();
        let quartiles_marks: Vec<&String> = quartiles
            .iter()
            .filter(|line| line.starts_with("mark "))
            .collect();
        let spread_marks: Vec<&String> = spread
            .iter()
            .filter(|line| line.starts_with("mark "))
            .collect();
        let tails_marks: Vec<&String> = tails
            .iter()
            .filter(|line| line.starts_with("mark "))
            .collect();
        let range_marks: Vec<&String> = range
            .iter()
            .filter(|line| line.starts_with("mark "))
            .collect();
        assert_eq!(range_marks.len(), reading_marks.len(), "seed {}", seed);
        assert_eq!(range_marks.len(), quartiles_marks.len(), "seed {}", seed);
        assert_eq!(range_marks.len(), spread_marks.len(), "seed {}", seed);
        assert_eq!(range_marks.len(), tails_marks.len(), "seed {}", seed);

        // The closing relations of PG-44 line by line,
        // the range standing as the difference of the
        // quartile standings of the same mark, the
        // identity against the quartile margins, the
        // bound against the spread, the zero relation in
        // its line form, and the no fixed order relation
        // against the difference of the tails counted in
        // both directions.
        let mut with_another = 0usize;
        let mut seed_above = 0usize;
        let mut seed_below = 0usize;
        for ((((range_line, mark_line), quartile_line), spread_line), tails_line) in range_marks
            .iter()
            .zip(reading_marks.iter())
            .zip(quartiles_marks.iter())
            .zip(spread_marks.iter())
            .zip(tails_marks.iter())
        {
            assert!(
                range_line.starts_with(&margins_record_prefix(mark_line)),
                "seed {}",
                seed
            );
            let (record, range_part) = range_mark_parts(range_line);
            assert_eq!(record, marks_record_figure(mark_line), "seed {}", seed);
            let (_quartile_record, quartile_part) = quartiles_mark_parts(quartile_line);
            let (_spread_record, _field, farthest) = spread_mark_parts(spread_line);
            let (_tails_record, tails_part) = tails_mark_parts(tails_line);
            if let Some(range_figure) = range_part {
                with_another += 1;
                family_figured += 1;
                let ((lower_figure, lower_margin), (upper_figure, upper_margin)) =
                    quartile_part.expect("the quartiles line states its quartiles");
                let range_value: f64 = range_figure.parse().expect("a range figure parses");
                let lower_value: f64 = lower_figure
                    .parse()
                    .expect("a lower quartile figure parses");
                let upper_value: f64 = upper_figure
                    .parse()
                    .expect("an upper quartile figure parses");
                let lower_margin_value: f64 = lower_margin
                    .parse()
                    .expect("a lower quartile margin parses");
                let upper_margin_value: f64 = upper_margin
                    .parse()
                    .expect("an upper quartile margin parses");
                assert!(
                    (range_value - (upper_value - lower_value)).abs() <= 0.000002,
                    "seed {}",
                    seed
                );
                assert!(range_value >= 0.0, "seed {}", seed);
                assert!(
                    (lower_margin_value + range_value - upper_margin_value).abs() <= 0.000002,
                    "seed {}",
                    seed
                );
                let (_farthest_figure, _farthest_label, spread_figure) =
                    farthest.expect("the spread line states a farthest standing");
                let spread_value: f64 = spread_figure.parse().expect("a spread figure parses");
                assert!(range_value <= spread_value + 0.000002, "seed {}", seed);
                if range_figure == "0" {
                    assert_eq!(lower_figure, upper_figure, "seed {}", seed);
                    family_zero += 1;
                }
                if lower_figure == upper_figure {
                    assert_eq!(range_figure, "0", "seed {}", seed);
                }
                let (_lower_tail_figure, lower_tail_margin, _upper_tail_figure, upper_tail_margin) =
                    tails_part.expect("the tails line states its tails");
                let lower_tail_margin_value: f64 = lower_tail_margin
                    .parse()
                    .expect("a lower tail margin parses");
                let upper_tail_margin_value: f64 = upper_tail_margin
                    .parse()
                    .expect("an upper tail margin parses");
                let difference = upper_tail_margin_value - lower_tail_margin_value;
                if range_value > difference + 0.000002 {
                    seed_above += 1;
                    family_above += 1;
                } else if range_value + 0.000002 < difference {
                    seed_below += 1;
                    family_below += 1;
                } else {
                    family_equal += 1;
                }
            } else {
                assert!(quartile_part.is_none(), "seed {}", seed);
                assert!(farthest.is_none(), "seed {}", seed);
                assert!(tails_part.is_none(), "seed {}", seed);
            }
        }
        assert!(seed_above > 0, "seed {}", seed);
        assert!(seed_below > 0, "seed {}", seed);

        // The counts arithmetic of PG-44, and the
        // closing naming the earliest mark at the
        // smallest range, the smallest range of every
        // seed standing at zero.
        let counts = range.last().expect("the range closes");
        let walked = count_after(counts, "marks walked ");
        assert_eq!(walked, range_marks.len(), "seed {}", seed);
        let stated_with = count_after(counts, "marks with another standing ");
        assert_eq!(stated_with, with_another, "seed {}", seed);
        let stated_days = count_after(counts, "days ");
        let no_other = range_marks
            .iter()
            .filter(|line| line.ends_with(", and no other standing"))
            .count();
        assert_eq!(with_another + no_other, walked, "seed {}", seed);
        if with_another > 0 {
            let narrowest_part = counts
                .split("narrowest interquartile range ")
                .nth(1)
                .expect("the counts line names its narrowest range");
            let (narrowest_figure, narrowest_rest) = narrowest_part
                .split_once(" at ")
                .expect("the narrowest range names its mark");
            let (narrowest_elapsed, narrowest_ordinal) = narrowest_rest
                .split_once(" seconds on the ")
                .expect("the narrowest range names its seconds");
            let narrowest_ordinal = narrowest_ordinal
                .strip_suffix(" day")
                .expect("the narrowest range names its day");
            assert_eq!(narrowest_figure, "0", "seed {}", seed);
            let narrowest_stated: f64 = narrowest_figure.parse().expect("a range parses");
            let smallest = range_marks
                .iter()
                .filter_map(|line| range_mark_parts(line).1)
                .map(|figure| figure.parse::<f64>().expect("a range figure parses"))
                .fold(f64::INFINITY, f64::min);
            assert!(
                (narrowest_stated - smallest).abs() <= 0.000002,
                "seed {}",
                seed
            );
            let first = range_marks
                .iter()
                .find(|line| {
                    range_mark_parts(line)
                        .1
                        .map(|figure| {
                            (figure.parse::<f64>().expect("a range figure parses") - smallest).abs()
                                <= 0.000002
                        })
                        .unwrap_or(false)
                })
                .expect("a mark line states the smallest range");
            assert_eq!(
                range_mark_position(first),
                (narrowest_elapsed.to_string(), narrowest_ordinal.to_string()),
                "seed {}",
                seed
            );
            let named = range_mark_parts(first)
                .1
                .expect("the named mark is figured");
            assert_eq!(named, narrowest_figure, "seed {}", seed);
        } else {
            assert!(
                counts.ends_with(", and, no other standing at any mark"),
                "seed {}",
                seed
            );
        }

        // The day close relation of PG-44, computed from
        // the standings sections, the range taken over
        // the figures of the competitors other than the
        // closing holder in the order and at the count
        // PG-43 states, with the zero condition checked
        // in both directions at every close from the
        // closing field itself.
        let mut standing_sections: Vec<Vec<String>> = Vec::new();
        for line in &standings {
            if line.starts_with("day, the ") {
                standing_sections.push(Vec::new());
            }
            if let Some(section) = standing_sections.last_mut() {
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
        assert_eq!(
            standing_sections.len(),
            range_sections.len(),
            "seed {}",
            seed
        );
        assert_eq!(stated_days, standing_sections.len(), "seed {}", seed);
        for (section, range_section) in standing_sections.iter().zip(range_sections.iter()) {
            let day_marks: Vec<&&String> = range_section
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
            let mut others: Vec<f64> = section
                .iter()
                .filter(|line| line.starts_with("competitor "))
                .filter(|line| !line.starts_with(&format!("competitor {},", holder)))
                .filter_map(|line| {
                    line.strip_prefix("competitor ")
                        .and_then(|rest| rest.split_once(", standing projection "))
                        .and_then(|(_, rest)| {
                            rest.strip_suffix(" at the close")
                                .and_then(|figure| figure.parse::<f64>().ok())
                        })
                })
                .collect();
            let last = day_marks.last().expect("the day holds a mark");
            let (_last_record, last_range) = range_mark_parts(last);
            match (others.is_empty(), last_range) {
                (true, None) => {}
                (false, Some(range_figure)) => {
                    others.sort_by(|a, b| a.partial_cmp(b).expect("figures order"));
                    let count = others.len().div_ceil(2);
                    let median_of = |part: &[f64]| {
                        if part.len() % 2 == 1 {
                            part[part.len() / 2]
                        } else {
                            (part[part.len() / 2 - 1] + part[part.len() / 2]) / 2.0
                        }
                    };
                    let expected =
                        median_of(&others[others.len() - count..]) - median_of(&others[..count]);
                    let range_value: f64 = range_figure.parse().expect("a range figure parses");
                    assert!((range_value - expected).abs() <= 0.000002, "seed {}", seed);
                    assert_eq!(
                        range_value == 0.0,
                        zero_condition_holds(&others),
                        "seed {}",
                        seed
                    );
                }
                _ => panic!("the day close relation fails, seed {}", seed),
            }
        }
    }
    assert_eq!(family_figured, 876);
    assert_eq!(family_above, 174);
    assert_eq!(family_below, 582);
    assert_eq!(family_equal, 120);
    assert_eq!(family_zero, 13);
}

/// The figures of one midhinge mark line, the record
/// figure and, where another standing stands, the
/// midhinge standing figure and the midhinge margin
/// figure, read the way the median helper reads its line.
fn midhinge_mark_parts(line: &str) -> (String, Option<(String, String)>) {
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
    let standing_start = line
        .find(", midhinge standing ")
        .expect("a figured mark line names its midhinge standing")
        + ", midhinge standing ".len();
    let standing_end = line[standing_start..]
        .find(", midhinge margin ")
        .map(|offset| standing_start + offset)
        .expect("a midhinge standing names its margin");
    let figure = line[standing_start..standing_end].to_string();
    let margin = line[standing_end + ", midhinge margin ".len()..].to_string();
    (record, Some((figure, margin)))
}

/// The elapsed figure and ordinal phrase a midhinge mark
/// line names, read the way the margins helper reads
/// them.
fn midhinge_mark_position(line: &str) -> (String, String) {
    margins_mark_position(line)
}

/// Whether every middle place figure of the lowest and
/// the highest sub-fields of one ordered field stands at
/// the record figure, the zero condition of PG-47 in its
/// field form, the record figure standing as the figure
/// the sub-fields' middle places are carried against.
fn midhinge_zero_condition_holds(field: &[f64], record: f64) -> bool {
    let count = field.len().div_ceil(2);
    let mut figures = middle_place_figures(&field[..count]);
    figures.extend(middle_place_figures(&field[field.len() - count..]));
    figures.iter().all(|figure| *figure == record)
}

// The midhinge pin of the generated family, added with
// Round 23 beside the reading closings above. On the
// built tree it fails under S-1, S-2, S-3, S-5, and S-6,
// and it passes under S-4 by design, the closest
// midhinge margin standing unique in every seed of the
// family, so the loosened scan names the same seat in
// every seed, its S-4 catcher standing as pin 8 of
// test_midhinge.rs and not here.
#[test]
fn test_generated_meets_close_under_the_midhinge_reading() {
    let mut family_figured = 0usize;
    let mut family_median_below = 0usize;
    let mut family_median_equal = 0usize;
    let mut family_median_above = 0usize;
    let mut family_mean_below = 0usize;
    let mut family_mean_equal = 0usize;
    let mut family_mean_above = 0usize;
    let mut family_zero = 0usize;
    for index in 0..8u64 {
        let seed = MEET_SEED + index;
        let mut gen = Gen::new(seed);
        let text = meet_text(&mut gen);
        let (meet, defects) = load_meet_text(&text);
        assert_eq!(defects, Vec::<String>::new(), "seed {}", seed);
        let meet = meet.expect("the generated meet loads");

        let marks = meet_marks(&meet).expect("the marks read");
        let quartiles = meet_quartiles(&meet).expect("the quartiles read");
        let margins = meet_margins(&meet).expect("the margins read");
        let spread = meet_spread(&meet).expect("the spread read");
        let median = meet_median(&meet).expect("the median read");
        let mean = meet_mean(&meet).expect("the mean read");
        let midhinge = meet_midhinge(&meet).expect("the midhinge read");
        let standings = meet_standings(&meet).expect("the standings read");

        let reading_marks: Vec<&String> = marks
            .iter()
            .filter(|line| line.starts_with("mark "))
            .collect();
        let quartiles_marks: Vec<&String> = quartiles
            .iter()
            .filter(|line| line.starts_with("mark "))
            .collect();
        let margins_marks: Vec<&String> = margins
            .iter()
            .filter(|line| line.starts_with("mark "))
            .collect();
        let spread_marks: Vec<&String> = spread
            .iter()
            .filter(|line| line.starts_with("mark "))
            .collect();
        let median_marks: Vec<&String> = median
            .iter()
            .filter(|line| line.starts_with("mark "))
            .collect();
        let mean_marks: Vec<&String> = mean
            .iter()
            .filter(|line| line.starts_with("mark "))
            .collect();
        let midhinge_marks: Vec<&String> = midhinge
            .iter()
            .filter(|line| line.starts_with("mark "))
            .collect();
        assert_eq!(midhinge_marks.len(), reading_marks.len(), "seed {}", seed);
        assert_eq!(midhinge_marks.len(), quartiles_marks.len(), "seed {}", seed);
        assert_eq!(midhinge_marks.len(), margins_marks.len(), "seed {}", seed);
        assert_eq!(midhinge_marks.len(), spread_marks.len(), "seed {}", seed);
        assert_eq!(midhinge_marks.len(), median_marks.len(), "seed {}", seed);
        assert_eq!(midhinge_marks.len(), mean_marks.len(), "seed {}", seed);

        // The closing relations of PG-48 line by line,
        // the midhinge standing as the exact mean of the
        // quartile standings of the same mark, the
        // midhinge margin as the exact mean of the
        // quartile margins, the margin chain of PG-48,
        // the zero relation in its line form, and the no
        // fixed order relations against the median margin
        // and against the mean margin counted in both
        // directions.
        let mut with_another = 0usize;
        let mut seed_median_below = 0usize;
        let mut seed_median_above = 0usize;
        let mut seed_mean_below = 0usize;
        let mut seed_mean_above = 0usize;
        for (
            (
                ((((midhinge_line, mark_line), quartile_line), margin_line), spread_line),
                median_line,
            ),
            mean_line,
        ) in midhinge_marks
            .iter()
            .zip(reading_marks.iter())
            .zip(quartiles_marks.iter())
            .zip(margins_marks.iter())
            .zip(spread_marks.iter())
            .zip(median_marks.iter())
            .zip(mean_marks.iter())
        {
            assert!(
                midhinge_line.starts_with(&margins_record_prefix(mark_line)),
                "seed {}",
                seed
            );
            let (record, midhinge_part) = midhinge_mark_parts(midhinge_line);
            assert_eq!(record, marks_record_figure(mark_line), "seed {}", seed);
            let (_quartile_record, quartile_part) = quartiles_mark_parts(quartile_line);
            let (_margin_record, margin_part) = margins_mark_parts(margin_line);
            let (_spread_record, _field, farthest) = spread_mark_parts(spread_line);
            let (_median_record, median_part) = median_mark_parts(median_line);
            let (_mean_record, mean_part) = mean_mark_parts(mean_line);
            if let Some((midhinge_figure, midhinge_margin)) = midhinge_part {
                with_another += 1;
                family_figured += 1;
                let ((lower_figure, lower_margin), (upper_figure, upper_margin)) =
                    quartile_part.expect("the quartiles line states its quartiles");
                let midhinge_value: f64 =
                    midhinge_figure.parse().expect("a midhinge figure parses");
                let margin_value: f64 = midhinge_margin.parse().expect("a midhinge margin parses");
                let lower_value: f64 = lower_figure
                    .parse()
                    .expect("a lower quartile figure parses");
                let upper_value: f64 = upper_figure
                    .parse()
                    .expect("an upper quartile figure parses");
                let lower_margin_value: f64 = lower_margin
                    .parse()
                    .expect("a lower quartile margin parses");
                let upper_margin_value: f64 = upper_margin
                    .parse()
                    .expect("an upper quartile margin parses");
                assert!(
                    (midhinge_value - (lower_value + upper_value) / 2.0).abs() <= 0.000002,
                    "seed {}",
                    seed
                );
                assert!(
                    (margin_value - (lower_margin_value + upper_margin_value) / 2.0).abs()
                        <= 0.000002,
                    "seed {}",
                    seed
                );
                assert!(margin_value >= 0.0, "seed {}", seed);
                let (_nearest_figure, _nearest_label, nearest_margin) =
                    margin_part.expect("the margins line states a nearest standing");
                let nearest_margin_value: f64 = nearest_margin.parse().expect("a margin parses");
                let (_farthest_figure, _farthest_label, spread_figure) =
                    farthest.expect("the spread line states a farthest standing");
                let spread_value: f64 = spread_figure.parse().expect("a spread figure parses");
                assert!(
                    nearest_margin_value <= lower_margin_value + 0.000002,
                    "seed {}",
                    seed
                );
                assert!(
                    lower_margin_value <= margin_value + 0.000002,
                    "seed {}",
                    seed
                );
                assert!(
                    margin_value <= upper_margin_value + 0.000002,
                    "seed {}",
                    seed
                );
                assert!(
                    upper_margin_value <= spread_value + 0.000002,
                    "seed {}",
                    seed
                );
                if midhinge_margin == "0" {
                    assert_eq!(lower_margin, "0", "seed {}", seed);
                    assert_eq!(upper_margin, "0", "seed {}", seed);
                    family_zero += 1;
                }
                if lower_margin == "0" && upper_margin == "0" {
                    assert_eq!(midhinge_margin, "0", "seed {}", seed);
                }
                let (_median_figure, median_margin) =
                    median_part.expect("the median line states its median");
                let median_margin_value: f64 =
                    median_margin.parse().expect("a median margin parses");
                if margin_value > median_margin_value + 0.000002 {
                    seed_median_above += 1;
                    family_median_above += 1;
                } else if margin_value + 0.000002 < median_margin_value {
                    seed_median_below += 1;
                    family_median_below += 1;
                } else {
                    family_median_equal += 1;
                }
                let (_mean_figure, mean_margin) = mean_part.expect("the mean line states its mean");
                let mean_margin_value: f64 = mean_margin.parse().expect("a mean margin parses");
                if margin_value > mean_margin_value + 0.000002 {
                    seed_mean_above += 1;
                    family_mean_above += 1;
                } else if margin_value + 0.000002 < mean_margin_value {
                    seed_mean_below += 1;
                    family_mean_below += 1;
                } else {
                    family_mean_equal += 1;
                }
            } else {
                assert!(quartile_part.is_none(), "seed {}", seed);
                assert!(margin_part.is_none(), "seed {}", seed);
                assert!(farthest.is_none(), "seed {}", seed);
                assert!(median_part.is_none(), "seed {}", seed);
                assert!(mean_part.is_none(), "seed {}", seed);
            }
        }
        assert!(seed_median_above > 0, "seed {}", seed);
        assert!(seed_median_below > 0, "seed {}", seed);
        assert!(seed_mean_above > 0, "seed {}", seed);
        assert!(seed_mean_below > 0, "seed {}", seed);

        // The counts arithmetic of PG-48, and the
        // closing naming the earliest mark at the
        // smallest midhinge margin, the smallest
        // midhinge margin of every seed standing unique
        // in the family as the design record counts it.
        let counts = midhinge.last().expect("the midhinge closes");
        let walked = count_after(counts, "marks walked ");
        assert_eq!(walked, midhinge_marks.len(), "seed {}", seed);
        let stated_with = count_after(counts, "marks with another standing ");
        assert_eq!(stated_with, with_another, "seed {}", seed);
        let stated_days = count_after(counts, "days ");
        let no_other = midhinge_marks
            .iter()
            .filter(|line| line.ends_with(", and no other standing"))
            .count();
        assert_eq!(with_another + no_other, walked, "seed {}", seed);
        if with_another > 0 {
            let closest_part = counts
                .split("closest midhinge margin ")
                .nth(1)
                .expect("the counts line names its closest margin");
            let (closest_figure, closest_rest) = closest_part
                .split_once(" at ")
                .expect("the closest margin names its mark");
            let (closest_elapsed, closest_ordinal) = closest_rest
                .split_once(" seconds on the ")
                .expect("the closest margin names its seconds");
            let closest_ordinal = closest_ordinal
                .strip_suffix(" day")
                .expect("the closest margin names its day");
            let closest_stated: f64 = closest_figure.parse().expect("a margin parses");
            let smallest = midhinge_marks
                .iter()
                .filter_map(|line| midhinge_mark_parts(line).1)
                .map(|(_, margin)| margin.parse::<f64>().expect("a midhinge margin parses"))
                .fold(f64::INFINITY, f64::min);
            assert!(
                (closest_stated - smallest).abs() <= 0.000002,
                "seed {}",
                seed
            );
            let at_smallest = midhinge_marks
                .iter()
                .filter(|line| {
                    midhinge_mark_parts(line)
                        .1
                        .map(|(_, margin)| {
                            (margin.parse::<f64>().expect("a midhinge margin parses") - smallest)
                                .abs()
                                <= 0.000002
                        })
                        .unwrap_or(false)
                })
                .count();
            assert_eq!(at_smallest, 1, "seed {}", seed);
            let first = midhinge_marks
                .iter()
                .find(|line| {
                    midhinge_mark_parts(line)
                        .1
                        .map(|(_, margin)| {
                            (margin.parse::<f64>().expect("a midhinge margin parses") - smallest)
                                .abs()
                                <= 0.000002
                        })
                        .unwrap_or(false)
                })
                .expect("a mark line states the smallest margin");
            assert_eq!(
                midhinge_mark_position(first),
                (closest_elapsed.to_string(), closest_ordinal.to_string()),
                "seed {}",
                seed
            );
            let named = midhinge_mark_parts(first)
                .1
                .expect("the named mark is figured")
                .1;
            assert_eq!(named, closest_figure, "seed {}", seed);
        } else {
            assert!(
                counts.ends_with(", and, no other standing at any mark"),
                "seed {}",
                seed
            );
        }

        // The day close relation of PG-48, computed from
        // the standings sections, the midhinge taken over
        // the figures of the competitors other than the
        // closing holder in the order and at the count
        // PG-47 states, with the zero condition checked
        // in both directions at every close from the
        // closing field itself.
        let mut standing_sections: Vec<Vec<String>> = Vec::new();
        for line in &standings {
            if line.starts_with("day, the ") {
                standing_sections.push(Vec::new());
            }
            if let Some(section) = standing_sections.last_mut() {
                section.push(line.clone());
            }
        }
        let mut midhinge_sections: Vec<Vec<&String>> = Vec::new();
        for line in &midhinge {
            if line.starts_with("day, the ") {
                midhinge_sections.push(Vec::new());
            }
            if let Some(section) = midhinge_sections.last_mut() {
                section.push(line);
            }
        }
        assert_eq!(
            standing_sections.len(),
            midhinge_sections.len(),
            "seed {}",
            seed
        );
        assert_eq!(stated_days, standing_sections.len(), "seed {}", seed);
        for (section, midhinge_section) in standing_sections.iter().zip(midhinge_sections.iter()) {
            let day_marks: Vec<&&String> = midhinge_section
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
            let record_value: f64 = closing
                .split(" at a projection of ")
                .nth(1)
                .and_then(|rest| rest.split(' ').next())
                .and_then(|word| word.parse().ok())
                .expect("a closing line names its record figure");
            let mut others: Vec<f64> = section
                .iter()
                .filter(|line| line.starts_with("competitor "))
                .filter(|line| !line.starts_with(&format!("competitor {},", holder)))
                .filter_map(|line| {
                    line.strip_prefix("competitor ")
                        .and_then(|rest| rest.split_once(", standing projection "))
                        .and_then(|(_, rest)| {
                            rest.strip_suffix(" at the close")
                                .and_then(|figure| figure.parse::<f64>().ok())
                        })
                })
                .collect();
            let last = day_marks.last().expect("the day holds a mark");
            let (_last_record, last_midhinge) = midhinge_mark_parts(last);
            match (others.is_empty(), last_midhinge) {
                (true, None) => {}
                (false, Some((midhinge_figure, midhinge_margin))) => {
                    others.sort_by(|a, b| a.partial_cmp(b).expect("figures order"));
                    let count = others.len().div_ceil(2);
                    let median_of = |part: &[f64]| {
                        if part.len() % 2 == 1 {
                            part[part.len() / 2]
                        } else {
                            (part[part.len() / 2 - 1] + part[part.len() / 2]) / 2.0
                        }
                    };
                    let expected = (median_of(&others[..count])
                        + median_of(&others[others.len() - count..]))
                        / 2.0;
                    let midhinge_value: f64 =
                        midhinge_figure.parse().expect("a midhinge figure parses");
                    let margin_value: f64 =
                        midhinge_margin.parse().expect("a midhinge margin parses");
                    assert!(
                        (midhinge_value - expected).abs() <= 0.000002,
                        "seed {}",
                        seed
                    );
                    assert!(
                        (margin_value - (expected - record_value)).abs() <= 0.000002,
                        "seed {}",
                        seed
                    );
                    assert_eq!(
                        margin_value == 0.0,
                        midhinge_zero_condition_holds(&others, record_value),
                        "seed {}",
                        seed
                    );
                }
                _ => panic!("the day close relation fails, seed {}", seed),
            }
        }
    }
    assert_eq!(family_figured, 876);
    assert_eq!(family_median_below, 309);
    assert_eq!(family_median_equal, 34);
    assert_eq!(family_median_above, 533);
    assert_eq!(family_mean_below, 589);
    assert_eq!(family_mean_equal, 96);
    assert_eq!(family_mean_above, 191);
    assert_eq!(family_zero, 0);
}
