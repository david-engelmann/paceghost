//! Tests for the Round 4 example ladder and its golden report.
//!
//! Translated from tests/test_examples_round4.py.

use std::path::PathBuf;

use num_bigint::BigInt;

use paceghost::cli::{dispatch, CliOutcome};
use paceghost::fieldday::{load_field_day_file, FieldDay};
use paceghost::ladder::{ladder_report, load_ladder_file, Ladder};
use paceghost::pynum::Num;
use paceghost::pytext::PText;

fn examples() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples")
}

fn reader(bytes: &[u8]) -> Option<String> {
    let path = String::from_utf8_lossy(bytes).into_owned();
    let data = std::fs::read(path).ok()?;
    String::from_utf8(data).ok()
}

fn cli(args: &[&str]) -> CliOutcome {
    let argv: Vec<PText> = args.iter().map(|arg| PText::from_text(arg)).collect();
    dispatch(&argv, &reader)
}

fn load_example() -> Ladder {
    let (ladder, defects) = load_ladder_file(&examples().join("ladder.json"));
    assert_eq!(defects, Vec::<String>::new());
    ladder.expect("the ladder loads")
}

fn sections_of(lines: &[String]) -> Vec<Vec<String>> {
    let mut sections: Vec<Vec<String>> = Vec::new();
    for line in lines {
        if line.starts_with("ladder, ") {
            continue;
        }
        if line.starts_with("field day, window ") {
            sections.push(Vec::new());
        }
        sections
            .last_mut()
            .expect("a section stands open")
            .push(line.clone());
    }
    sections
}

fn day_as_it_stood(day: &FieldDay, window: &BigInt) -> String {
    let mut runs_json: Vec<String> = Vec::new();
    for run in &day.runs {
        let usage = match &run.usage {
            Some(Num::Int(value)) => value.to_string(),
            Some(Num::Float(value)) => format!("{}", value),
            None => "0".to_string(),
        };
        let mut text = format!(
            "{{\"runner\": {}, \"usage\": {}}}",
            serde_free_json_string(&run.runner.to_display_string()),
            usage
        );
        if let Some(reads) = &run.reads {
            let kept: Vec<String> = reads
                .iter()
                .filter(|read| matches!(&read.elapsed_seconds, Num::Int(value) if value <= window))
                .map(|read| {
                    format!(
                        "{{\"elapsed_seconds\": {}, \"usage\": {}}}",
                        num_json(&read.elapsed_seconds),
                        num_json(&read.usage)
                    )
                })
                .collect();
            if !kept.is_empty() {
                text = format!(
                    "{{\"runner\": {}, \"usage\": {}, \"reads\": [{}]}}",
                    serde_free_json_string(&run.runner.to_display_string()),
                    usage,
                    kept.join(", ")
                );
            }
        }
        runs_json.push(text);
    }
    format!(
        "{{\"window_seconds\": {}, \"operator\": {}, \"runs\": [{}]}}",
        window,
        serde_free_json_string(&day.operator.to_display_string()),
        runs_json.join(", ")
    )
}

fn num_json(value: &Num) -> String {
    match value {
        Num::Int(value) => value.to_string(),
        Num::Float(value) => format!("{}", value),
    }
}

fn serde_free_json_string(text: &str) -> String {
    let mut out = String::from("\"");
    for c in text.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

#[test]
fn test_the_example_ladder_loads_clean() {
    let ladder = load_example();
    assert_eq!(
        ladder.windows,
        vec![BigInt::from(600), BigInt::from(900), BigInt::from(1800)]
    );
    assert!(ladder.day.operator.eq_str("operator"));
    assert_eq!(ladder.day.window_seconds, Num::from_i64(1800));
    assert_eq!(ladder.day.runs.len(), 4);
}

#[test]
fn test_the_example_embeds_the_round_three_field_day_whole() {
    let ladder = load_example();
    let (day, defects) = load_field_day_file(&examples().join("field-day.json"));
    assert_eq!(defects, Vec::<String>::new());
    assert_eq!(ladder.day, day.expect("the field day loads"));
}

#[test]
fn test_the_example_report_matches_the_golden_bytes() {
    let ladder = load_example();
    let golden = std::fs::read(examples().join("ladder-golden.txt")).expect("the golden reads");
    let rendered = ladder_report(&ladder).expect("the report reads").join("\n") + "\n";
    assert_eq!(rendered.as_bytes(), golden.as_slice());
}

#[test]
fn test_the_ladder_command_prints_the_golden_report() {
    let path = examples()
        .join("ladder.json")
        .to_string_lossy()
        .into_owned();
    let outcome = cli(&["ladder", &path]);
    assert_eq!(outcome.code, 0);
    let golden = std::fs::read(examples().join("ladder-golden.txt")).expect("the golden reads");
    let printed = outcome.stdout.join("\n") + "\n";
    assert_eq!(printed.as_bytes(), golden.as_slice());
}

#[test]
fn test_the_widest_section_matches_the_round_three_golden() {
    let ladder = load_example();
    let sections = sections_of(&ladder_report(&ladder).expect("the report reads"));
    let golden =
        std::fs::read_to_string(examples().join("field-day-golden.txt")).expect("the golden reads");
    let golden_lines: Vec<String> = golden.lines().map(|line| line.to_string()).collect();
    assert_eq!(sections[2], golden_lines);
}

#[test]
fn test_the_closing_holder_changes_across_the_ladder() {
    let ladder = load_example();
    let lines = ladder_report(&ladder).expect("the report reads");
    let finishes: Vec<&str> = lines
        .iter()
        .filter(|line| line.starts_with("finish, record held by"))
        .map(|line| line.as_str())
        .collect();
    assert_eq!(
        finishes,
        vec![
            "finish, record held by dawn fork at a projection of 400",
            "finish, record held by night fork at a projection of 470",
            "finish, record held by night fork at a projection of 840",
        ]
    );
    assert_eq!(
        lines[lines.len() - 1],
        "ladder, the closing holder differs from the first window at 900 seconds, 1800 seconds"
    );
}

#[test]
fn test_each_section_matches_the_fieldday_command_at_that_window() {
    let ladder = load_example();
    let sections = sections_of(&ladder_report(&ladder).expect("the report reads"));
    let dir = std::env::temp_dir().join(format!("paceghost-examples4-{}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("the scratch dir is created");
    for (window, section) in ladder.windows.iter().zip(sections.iter()) {
        let path = dir.join(format!("day-{}.json", window));
        std::fs::write(&path, day_as_it_stood(&ladder.day, window))
            .expect("the scratch file writes");
        let path_text = path.to_string_lossy().into_owned();
        let outcome = cli(&["fieldday", &path_text]);
        assert_eq!(outcome.code, 0);
        assert_eq!(&outcome.stdout, section, "at window {}", window);
    }
}
