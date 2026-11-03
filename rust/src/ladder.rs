//! The window ladder over one field day, for paceghost.
//!
//! The ladder loader never raises on malformed content and names
//! every defect it finds, running the field day validation over the
//! day the ladder file embeds and prefixing its defects with the
//! word day, and naming the window list in phrases of the same
//! shape. The ladder report reads the day as it stood at each window
//! in the list through the same record walk that computes the field
//! day report, and closes with one summary line over the closing
//! holders. The obligations this module carries are PG-9 and PG-10
//! in CONTRACT.md, in the Round 4 section. The behavior mirrors the
//! Python module of the same name, which stands as the spec under
//! PORT-1.

use std::collections::HashMap;
use std::path::Path;

use num_bigint::BigInt;
use num_rational::BigRational;

use crate::fieldday::{load_field_day_data, record_walk, FieldDay};
use crate::json::{parse_json, JsonValue};
use crate::pynum::{format_seconds, Num};
use crate::runs::read_text_file;

/// A loaded ladder, one field day and its window lengths.
#[derive(Clone, Debug, PartialEq)]
pub struct Ladder {
    pub day: FieldDay,
    pub windows: Vec<BigInt>,
}

/// Validate the windows field of a ladder and return its list.
///
/// Returns the window lengths as whole numbers in list order, or
/// nothing when the field is absent or of the wrong kind or empty.
/// A length that draws a defect takes no place in the list, and the
/// duplicate check reads only the lengths that loaded clean.
fn load_windows(data: &JsonValue, defects: &mut Vec<String>) -> Option<Vec<BigInt>> {
    let raw = match data.get("windows") {
        None => {
            defects.push("missing field windows".to_string());
            return None;
        }
        Some(JsonValue::Arr(items)) => items,
        Some(_) => {
            defects.push("field windows has the wrong type".to_string());
            return None;
        }
    };
    if raw.is_empty() {
        defects.push("field windows is empty".to_string());
        return None;
    }
    let zero = Num::from_i64(0);
    let mut windows: Vec<BigInt> = Vec::new();
    let mut first_seen: HashMap<BigInt, usize> = HashMap::new();
    for (index, value) in raw.iter().enumerate() {
        let prefix = format!("window {} ", index);
        let number = match value {
            JsonValue::Num(number) => number,
            _ => {
                defects.push(format!("{}has the wrong type", prefix));
                continue;
            }
        };
        if !number.is_finite() {
            defects.push(format!("{}is not a finite number", prefix));
            continue;
        }
        if number.cmp_py(&zero) != std::cmp::Ordering::Greater {
            defects.push(format!("{}must be positive", prefix));
            continue;
        }
        let fraction = number.to_rational();
        if !fraction.is_integer() {
            defects.push(format!("{}is not a whole number", prefix));
            continue;
        }
        let whole = fraction.to_integer();
        if let Some(first) = first_seen.get(&whole) {
            defects.push(format!("{}duplicates window {}", prefix, first));
            continue;
        }
        first_seen.insert(whole.clone(), index);
        windows.push(whole);
    }
    Some(windows)
}

/// Parse ladder text into a ladder, naming every defect found.
///
/// Returns a pair of the ladder and the defect list. A malformed
/// input returns no ladder, and this function never raises on bad
/// content.
pub fn load_ladder_text(text: &str) -> (Option<Ladder>, Vec<String>) {
    let data = match parse_json(text) {
        Ok(value) => value,
        Err(_) => return (None, vec!["input is not valid JSON".to_string()]),
    };
    if !data.is_object() {
        return (None, vec!["input is not a JSON object".to_string()]);
    }
    let mut defects: Vec<String> = Vec::new();
    let mut day: Option<FieldDay> = None;
    match data.get("day") {
        None => defects.push("missing field day".to_string()),
        Some(value) if !value.is_object() => {
            defects.push("field day is not an object".to_string());
        }
        Some(value) => {
            let (loaded, day_defects) = load_field_day_data(value);
            day = loaded;
            defects.extend(day_defects.iter().map(|defect| format!("day {}", defect)));
        }
    }
    let windows = load_windows(&data, &mut defects);
    if !defects.is_empty() {
        return (None, defects);
    }
    (
        Some(Ladder {
            day: day.expect("a clean ladder holds its day"),
            windows: windows.expect("a clean ladder holds its windows"),
        }),
        defects,
    )
}

/// Read a ladder from disk, with the same contract as text.
pub fn load_ladder_file(path: &Path) -> (Option<Ladder>, Vec<String>) {
    match read_text_file(path) {
        None => (None, vec!["file cannot be read".to_string()]),
        Some(text) => load_ladder_text(&text),
    }
}

/// Return the ladder lines for one loaded ladder.
///
/// One section stands per window in list order, each computed by the
/// record walk with the window standing as the horizon, so the
/// section is the field day report of the day as it stood at that
/// window. The summary line names every window after the first whose
/// closing holder is not the closing holder of the first window, and
/// reads that the closing holder never changed when no window after
/// the first differs. Handed a value the loader would refuse, the
/// report names the broken precondition as its error, an empty
/// windows list here and, through the walk, an operator naming no
/// run among the runs, in the discipline of FIX-PG-8.
pub fn ladder_report(ladder: &Ladder) -> Result<Vec<String>, String> {
    if ladder.windows.is_empty() {
        return Err("the ladder holds no windows".to_string());
    }
    let day = &ladder.day;
    let mut lines: Vec<String> = Vec::new();
    let mut holders: Vec<Option<usize>> = Vec::new();
    for window in &ladder.windows {
        let (section, holder) = record_walk(
            &day.runs,
            &day.operator,
            &Num::Int(window.clone()),
            Some(&BigRational::from(window.clone())),
        )?;
        lines.extend(section);
        holders.push(holder);
    }
    let first = holders[0];
    let differing: Vec<&BigInt> = ladder
        .windows
        .iter()
        .enumerate()
        .skip(1)
        .filter(|(index, _)| holders[*index] != first)
        .map(|(_, window)| window)
        .collect();
    if differing.is_empty() {
        lines.push("ladder, the closing holder never changed across the ladder".to_string());
    } else {
        let named = differing
            .iter()
            .map(|window| format!("{} seconds", format_seconds(&Num::Int((*window).clone()))))
            .collect::<Vec<String>>()
            .join(", ");
        lines.push(format!(
            "ladder, the closing holder differs from the first window at {}",
            named
        ));
    }
    Ok(lines)
}
