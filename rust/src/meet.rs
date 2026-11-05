//! The meet over three or more field days, for paceghost.
//!
//! The meet loader never raises on malformed content and names
//! every defect it finds, running the field day validation over
//! each day the meet file embeds and prefixing its defects with
//! the word day and the index of that day. The meet report walks
//! one record progression across all the days, the marks of an
//! earlier day before the marks of a later day, under the tie
//! discipline of PG-7 with each day's own window standing over its
//! own reads, and it names the closing holder of each day as that
//! day closes and, at the close of the meet, the holder of the
//! standing record and the day and mark at which its figure was
//! set. The obligations this module carries are PG-13 and PG-14
//! in CONTRACT.md, in the Round 6 section. No Python module stands
//! behind this one, so the contract text is the spec and the pins
//! in the test suite hold it.

use std::path::Path;

use num_rational::BigRational;

use crate::fieldday::{
    check_finite_figures, collect_events, load_field_day_data, select_holder, CollectedEvents,
    FieldDay,
};
use crate::json::{parse_json, JsonValue};
use crate::pynum::format_seconds_fraction;
use crate::pytext::PText;
use crate::runs::read_text_file;
use crate::season::format_usage;

/// The ordinal words that name the days of a meet, in day order.
///
/// The table is the bound of a meet as PG-13 states it, so a meet
/// holds at most as many days as the table holds words.
pub const ORDINALS: [&str; 10] = [
    "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth",
];

/// A loaded meet, three or more field days in list order.
#[derive(Clone, Debug, PartialEq)]
pub struct Meet {
    pub days: Vec<FieldDay>,
}

/// Parse meet text into a meet, naming every defect found.
///
/// Returns a pair of the meet and the defect list. A malformed
/// input returns no meet, and this function never raises on bad
/// content.
pub fn load_meet_text(text: &str) -> (Option<Meet>, Vec<String>) {
    let data = match parse_json(text) {
        Ok(value) => value,
        Err(_) => return (None, vec!["input is not valid JSON".to_string()]),
    };
    if !data.is_object() {
        return (None, vec!["input is not a JSON object".to_string()]);
    }
    let mut defects: Vec<String> = Vec::new();
    let mut days: Vec<FieldDay> = Vec::new();
    match data.get("days") {
        None => defects.push("missing field days".to_string()),
        Some(JsonValue::Arr(items)) => {
            match items.len() {
                0 => defects.push("field days is empty".to_string()),
                1 => defects.push("field days holds one day".to_string()),
                2 => {
                    defects.push("field days holds two days, the count of the rematch".to_string())
                }
                _ => {}
            }
            if items.len() > ORDINALS.len() {
                defects.push("field days holds more days than the ordinal table names".to_string());
            }
            for (index, item) in items.iter().enumerate() {
                if !item.is_object() {
                    defects.push(format!("day {} is not an object", index));
                    continue;
                }
                let (loaded, day_defects) = load_field_day_data(item);
                if let Some(day) = loaded {
                    days.push(day);
                }
                defects.extend(
                    day_defects
                        .iter()
                        .map(|defect| format!("day {} {}", index, defect)),
                );
            }
        }
        Some(_) => defects.push("field days has the wrong type".to_string()),
    }
    if !defects.is_empty() {
        return (None, defects);
    }
    (Some(Meet { days }), defects)
}

/// Read a meet from disk, with the same contract as text.
pub fn load_meet_file(path: &Path) -> (Option<Meet>, Vec<String>) {
    match read_text_file(path) {
        None => (None, vec!["file cannot be read".to_string()]),
        Some(text) => load_meet_text(&text),
    }
}

/// The closing line of one day of the meet reading.
///
/// The line names the holder and the record figure as they stood
/// at the close of the day, or that no record stood at that close.
fn close_line(
    day_index: usize,
    holder: Option<usize>,
    standing: &[Option<BigRational>],
    labels: &[PText],
) -> String {
    match holder {
        None => format!("close of the {} day, no record stood", ORDINALS[day_index]),
        Some(holder) => format!(
            "close of the {} day, record held by {} at a projection of {}",
            ORDINALS[day_index],
            labels[holder],
            format_usage(
                standing[holder]
                    .as_ref()
                    .expect("the holder stands at the record")
            )
        ),
    }
}

/// Return the meet lines for one loaded meet.
///
/// The walk reads the days as one field of competitors, a runner
/// value standing in more than one day standing as one competitor,
/// and walks the marks of each day in increasing order, the days
/// in list order. A competitor's standing projection is the
/// projection of its latest read in that order, each read
/// projecting under the window of its own day, and the record
/// after a mark is the lowest standing projection, held by the
/// standing holder while it stands at the figure and otherwise by
/// the competitor whose figure was set at the earliest mark, the
/// earlier competitor place breaking a tie at one mark. After the
/// marks of each day the closing line of that day stands, and the
/// report closes with the finish line, the line naming the day
/// and the mark at which the figure the holder stands at was set,
/// and the count of days read. Handed a meet value built by hand
/// holding no days, or more days than the ordinal table names,
/// the report names that precondition as its error, and handed a
/// day carrying a figure its loader would refuse as not finite it
/// names the broken precondition as its error, the days checked
/// in list order, in the discipline of FIX-PG-8 and FIX-PG-10,
/// and never fails bare.
pub fn meet_report(meet: &Meet) -> Result<Vec<String>, String> {
    if meet.days.is_empty() {
        return Err("the meet holds no days".to_string());
    }
    if meet.days.len() > ORDINALS.len() {
        return Err("the meet holds more days than the ordinal table names".to_string());
    }
    for day in &meet.days {
        check_finite_figures(&day.window_seconds, &day.runs)?;
    }
    let CollectedEvents {
        labels,
        events,
        marks,
    } = collect_events(meet.days.iter());
    let mut lines = vec![format!("meet, {} days", meet.days.len())];
    let mut standing: Vec<Option<BigRational>> = vec![None; labels.len()];
    let mut set_at: Vec<Option<(usize, BigRational)>> = vec![None; labels.len()];
    let mut holder: Option<usize> = None;
    let mut emitted: Option<(usize, BigRational)> = None;
    let mut closed_day = 0usize;
    for (day_index, mark) in &marks {
        while closed_day < *day_index {
            lines.push(close_line(closed_day, holder, &standing, &labels));
            closed_day += 1;
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
        holder = Some(new_holder);
        if emitted != Some((new_holder, record.clone())) {
            lines.push(format!(
                "record at {} seconds on the {} day, {} at a projection of {}",
                format_seconds_fraction(mark),
                ORDINALS[*day_index],
                labels[new_holder],
                format_usage(&record)
            ));
            emitted = Some((new_holder, record));
        }
    }
    while closed_day < meet.days.len() {
        lines.push(close_line(closed_day, holder, &standing, &labels));
        closed_day += 1;
    }
    match holder {
        None => lines.push("finish, no record was set".to_string()),
        Some(holder) => {
            let record = standing[holder]
                .clone()
                .expect("the holder stands at the record");
            lines.push(format!(
                "finish, record held by {} at a projection of {}",
                labels[holder],
                format_usage(&record)
            ));
            let (set_day, set_mark) = set_at[holder]
                .clone()
                .expect("the figure the holder stands at was set at some mark");
            lines.push(format!(
                "record set on the {} day at {} seconds",
                ORDINALS[set_day],
                format_seconds_fraction(&set_mark)
            ));
        }
    }
    lines.push(format!("days read, {}", meet.days.len()));
    Ok(lines)
}
