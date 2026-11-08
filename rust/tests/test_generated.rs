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
    load_meet_text, meet_days, meet_marks, meet_reign, meet_report, meet_standings, meet_table,
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
