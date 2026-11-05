//! The rematch over two field days, for paceghost.
//!
//! The rematch loader never raises on malformed content and names
//! every defect it finds, running the field day validation over
//! each day the rematch file embeds and prefixing its defects with
//! the key of that day. The rematch report walks one record
//! progression across the pair, the first day's marks before the
//! second day's, under the tie discipline of PG-7 with each day's
//! own window standing over its own reads, and closes by naming
//! the holder across both days and where the record changed hands
//! between the days. The obligations this module carries are
//! PG-11 and PG-12 in CONTRACT.md, in the Round 5 section. No
//! Python module stands behind this one, so the contract text is
//! the spec and the pins in the test suite hold it.

use std::path::Path;

use num_rational::BigRational;

use crate::fieldday::{
    check_finite_figures, collect_events, load_field_day_data, select_holder, CollectedEvents,
    FieldDay,
};
use crate::json::parse_json;
use crate::pynum::{format_seconds, format_seconds_fraction};
use crate::runs::read_text_file;
use crate::season::format_usage;

/// A loaded rematch, two field days named first and second.
#[derive(Clone, Debug, PartialEq)]
pub struct Rematch {
    pub first: FieldDay,
    pub second: FieldDay,
}

/// Parse rematch text into a rematch, naming every defect found.
///
/// Returns a pair of the rematch and the defect list. A malformed
/// input returns no rematch, and this function never raises on bad
/// content.
pub fn load_rematch_text(text: &str) -> (Option<Rematch>, Vec<String>) {
    let data = match parse_json(text) {
        Ok(value) => value,
        Err(_) => return (None, vec!["input is not valid JSON".to_string()]),
    };
    if !data.is_object() {
        return (None, vec!["input is not a JSON object".to_string()]);
    }
    let mut defects: Vec<String> = Vec::new();
    let mut first: Option<FieldDay> = None;
    let mut second: Option<FieldDay> = None;
    for (key, slot) in [("first", &mut first), ("second", &mut second)] {
        match data.get(key) {
            None => defects.push(format!("missing field {}", key)),
            Some(value) if !value.is_object() => {
                defects.push(format!("field {} is not an object", key));
            }
            Some(value) => {
                let (loaded, day_defects) = load_field_day_data(value);
                *slot = loaded;
                defects.extend(
                    day_defects
                        .iter()
                        .map(|defect| format!("{} {}", key, defect)),
                );
            }
        }
    }
    if !defects.is_empty() {
        return (None, defects);
    }
    (
        Some(Rematch {
            first: first.expect("a clean rematch holds its first day"),
            second: second.expect("a clean rematch holds its second day"),
        }),
        defects,
    )
}

/// Read a rematch from disk, with the same contract as text.
pub fn load_rematch_file(path: &Path) -> (Option<Rematch>, Vec<String>) {
    match read_text_file(path) {
        None => (None, vec!["file cannot be read".to_string()]),
        Some(text) => load_rematch_text(&text),
    }
}

/// Return the rematch lines for one loaded rematch.
///
/// The walk reads the two days as one field of competitors, a
/// runner value standing in both days standing as one competitor,
/// and walks the first day's marks in increasing order and then
/// the second day's. A competitor's standing projection is the
/// projection of its latest read in that order, each read
/// projecting under the window of its own day, and the record
/// after a mark is the lowest standing projection, held by the
/// standing holder while it stands at the figure and otherwise by
/// the competitor whose figure was set at the earliest mark, the
/// earlier competitor place breaking a tie at one mark. The
/// closing lines name the holder at the close and the first mark
/// of the second day at which the record moved away from the
/// closing holder of the first day, or that it never moved, or
/// that no record stood on the first day to be taken. Handed a
/// pair carrying a figure its loader would refuse as not finite,
/// the report names the broken precondition as its error, the
/// first day checked before the second, in the discipline of
/// FIX-PG-10, and never fails bare.
pub fn rematch_report(rematch: &Rematch) -> Result<Vec<String>, String> {
    check_finite_figures(&rematch.first.window_seconds, &rematch.first.runs)?;
    check_finite_figures(&rematch.second.window_seconds, &rematch.second.runs)?;
    let days = [&rematch.first, &rematch.second];
    let CollectedEvents {
        labels,
        events,
        marks,
    } = collect_events(days.iter().copied());
    let mut lines = vec![format!(
        "rematch, first day window {} seconds, {} runs, second day window {} seconds, {} runs",
        format_seconds(&rematch.first.window_seconds),
        rematch.first.runs.len(),
        format_seconds(&rematch.second.window_seconds),
        rematch.second.runs.len()
    )];
    let mut standing: Vec<Option<BigRational>> = vec![None; labels.len()];
    let mut set_at: Vec<Option<(usize, BigRational)>> = vec![None; labels.len()];
    let mut holder: Option<usize> = None;
    let mut emitted: Option<(usize, BigRational)> = None;
    let mut first_day_holder: Option<usize> = None;
    let mut crossed = false;
    let mut changed: Option<(BigRational, usize, usize)> = None;
    for (day_index, mark) in &marks {
        if *day_index == 1 && !crossed {
            first_day_holder = holder;
            crossed = true;
        }
        for competitor in 0..labels.len() {
            if let Some(projection) = events[competitor].get(&(*day_index, mark.clone())) {
                standing[competitor] = Some(projection.clone());
                set_at[competitor] = Some((*day_index, mark.clone()));
            }
        }
        let (record, new_holder) = match select_holder(&standing, &set_at, holder) {
            Some(pair) => pair,
            None => continue,
        };
        if *day_index == 1 && changed.is_none() {
            if let Some(first_holder) = first_day_holder {
                if new_holder != first_holder {
                    changed = Some((mark.clone(), new_holder, first_holder));
                }
            }
        }
        holder = Some(new_holder);
        if emitted != Some((new_holder, record.clone())) {
            lines.push(format!(
                "record at {} seconds on the {} day, {} at a projection of {}",
                format_seconds_fraction(mark),
                if *day_index == 0 { "first" } else { "second" },
                labels[new_holder],
                format_usage(&record)
            ));
            emitted = Some((new_holder, record));
        }
    }
    if !crossed {
        first_day_holder = holder;
    }
    let holder = match holder {
        None => {
            lines.push("finish, no record was set".to_string());
            return Ok(lines);
        }
        Some(holder) => holder,
    };
    let record = standing[holder]
        .clone()
        .expect("the holder stands at the record");
    lines.push(format!(
        "finish, record held by {} at a projection of {}",
        labels[holder],
        format_usage(&record)
    ));
    match (first_day_holder, changed) {
        (None, _) => lines.push("no record stood on the first day".to_string()),
        (Some(_), Some((mark, taker, first_holder))) => lines.push(format!(
            "record changed hands on the second day at {} seconds, {} took the record from {}",
            format_seconds_fraction(&mark),
            labels[taker],
            labels[first_holder]
        )),
        (Some(_), None) => lines.push("record never changed hands on the second day".to_string()),
    }
    Ok(lines)
}
