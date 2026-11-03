//! The field day and the stolen record, for paceghost.
//!
//! The field day loader never raises on malformed content and names
//! every defect it finds, reusing the phrases of the recorded run
//! shape and of the Round 2 run extension under a run index prefix.
//! The field day report is a pure function of the loaded day, and
//! every projection is computed on exact values and rendered under
//! the rules the contract already states. The obligations this
//! module carries are PG-7 and PG-8 in CONTRACT.md, in the Round 3
//! section. The behavior mirrors the Python module of the same name,
//! which stands as the spec under PORT-1.

use std::collections::{BTreeSet, HashMap};
use std::path::Path;

use num_rational::BigRational;

use crate::json::{parse_json, JsonValue};
use crate::pynum::{format_seconds, format_seconds_fraction, Num};
use crate::pytext::PText;
use crate::runs::{check_text_field, load_reads, load_usage_value, read_text_file, Read};

/// One run of a field day.
#[derive(Clone, Debug, PartialEq)]
pub struct DayRun {
    pub runner: PText,
    pub usage: Option<Num>,
    pub reads: Option<Vec<Read>>,
}

/// A loaded field day.
#[derive(Clone, Debug, PartialEq)]
pub struct FieldDay {
    pub window_seconds: Num,
    pub operator: PText,
    pub runs: Vec<DayRun>,
}

/// Validate a label field and return its value, if sound.
///
/// The phrases are the ones check_text_field draws, plus the line
/// break phrase, because a label reaches the printer in the field
/// day report and a label holding a line break would forge a report
/// line.
fn check_label(
    data: &JsonValue,
    name: &str,
    defects: &mut Vec<String>,
    prefix: &str,
) -> Option<PText> {
    let value = check_text_field(data, name, defects, prefix)?;
    if value.holds_line_break() {
        defects.push(format!("{}field {} holds a line break", prefix, name));
        return None;
    }
    Some(value)
}

/// Validate one run object of a field day.
///
/// Returns a pair of the clean run, or nothing when this run drew
/// any defect, and the label of the run, or nothing when the label
/// itself drew a defect. The label is reported even for a run that
/// drew other defects, so the duplicate and operator checks can
/// read it.
fn load_day_run(
    entry: &JsonValue,
    index: usize,
    defects: &mut Vec<String>,
) -> (Option<DayRun>, Option<PText>) {
    let prefix = format!("run {} ", index);
    if !entry.is_object() {
        defects.push(format!("{}is not an object", prefix));
        return (None, None);
    }
    let before = defects.len();
    let label = check_label(entry, "runner", defects, &prefix);
    let usage = load_usage_value(entry, "usage", defects, &prefix);
    let reads = load_reads(entry, defects, &prefix);
    let has_reads = entry.get("reads").is_some();
    if has_reads && entry.get("usage").is_none() {
        defects.push(format!("{}missing field usage", prefix));
    }
    if defects.len() > before {
        return (None, label);
    }
    let run = DayRun {
        runner: label.clone().expect("a clean day run holds its label"),
        usage: if entry.get("usage").is_some() {
            usage
        } else {
            None
        },
        reads: if has_reads { reads } else { None },
    };
    (Some(run), label)
}

/// Parse field day text into a field day, naming every defect found.
///
/// Returns a pair of the field day and the defect list. A malformed
/// input returns no field day, and this function never raises on bad
/// content.
pub fn load_field_day_text(text: &str) -> (Option<FieldDay>, Vec<String>) {
    let data = match parse_json(text) {
        Ok(value) => value,
        Err(_) => return (None, vec!["input is not valid JSON".to_string()]),
    };
    if !data.is_object() {
        return (None, vec!["input is not a JSON object".to_string()]);
    }
    load_field_day_data(&data)
}

/// Validate one parsed field day object, naming every defect.
///
/// Returns a pair of the field day and the defect list, with the
/// same contract as the text entry point. The ladder loader of
/// PG-10 runs this validation over the day a ladder file embeds, so
/// one validation serves both shapes.
pub(crate) fn load_field_day_data(data: &JsonValue) -> (Option<FieldDay>, Vec<String>) {
    let mut defects: Vec<String> = Vec::new();
    let mut window: Option<Num> = None;
    match data.get("window_seconds") {
        None => defects.push("missing field window_seconds".to_string()),
        Some(JsonValue::Num(value)) => {
            if !value.is_finite() {
                defects.push("field window_seconds is not a finite number".to_string());
            } else if value.cmp_py(&Num::from_i64(0)) != std::cmp::Ordering::Greater {
                defects.push("field window_seconds must be positive".to_string());
            } else {
                window = Some(value.clone());
            }
        }
        Some(_) => defects.push("field window_seconds has the wrong type".to_string()),
    }
    let operator = check_label(data, "operator", &mut defects, "");
    let mut runs: Vec<DayRun> = Vec::new();
    match data.get("runs") {
        None => defects.push("missing field runs".to_string()),
        Some(JsonValue::Arr(items)) => {
            if items.is_empty() {
                defects.push("field runs is empty".to_string());
            } else {
                let mut labels: Vec<Option<PText>> = Vec::new();
                let mut loaded: Vec<Option<DayRun>> = Vec::new();
                for (index, entry) in items.iter().enumerate() {
                    let (run, label) = load_day_run(entry, index, &mut defects);
                    loaded.push(run);
                    labels.push(label);
                }
                let mut first_seen: HashMap<PText, usize> = HashMap::new();
                for (index, label) in labels.iter().enumerate() {
                    let label = match label {
                        Some(label) => label,
                        None => continue,
                    };
                    if let Some(first) = first_seen.get(label) {
                        defects.push(format!(
                            "run {} field runner duplicates the label of run {}",
                            index, first
                        ));
                    } else {
                        first_seen.insert(label.clone(), index);
                    }
                }
                if let Some(operator) = &operator {
                    if !first_seen.contains_key(operator) {
                        defects.push("field operator names no run in the field day".to_string());
                    }
                }
                runs.extend(loaded.into_iter().flatten());
            }
        }
        Some(_) => defects.push("field runs has the wrong type".to_string()),
    }
    if !defects.is_empty() {
        return (None, defects);
    }
    (
        Some(FieldDay {
            window_seconds: window.expect("a clean field day holds a window"),
            operator: operator.expect("a clean field day holds an operator"),
            runs,
        }),
        defects,
    )
}

/// Read a field day from disk, with the same contract as text.
pub fn load_field_day_file(path: &Path) -> (Option<FieldDay>, Vec<String>) {
    match read_text_file(path) {
        None => (None, vec!["file cannot be read".to_string()]),
        Some(text) => load_field_day_text(&text),
    }
}

/// Return the stolen record lines for one loaded field day.
///
/// The walk reads every run through its timed reads in elapsed
/// order. A run projects the final usage of PG-6 from each read,
/// the record after each mark is the lowest standing projection,
/// and the holder keeps the record while it stands at the record
/// figure, so an equal figure alone never moves the record.
///
/// Handed a value the loader would refuse, the report names the
/// broken precondition as its error, in the discipline of FIX-PG-8.
pub fn field_day_report(day: &FieldDay) -> Result<Vec<String>, String> {
    let (lines, _holder) = record_walk(&day.runs, &day.operator, &day.window_seconds, None)?;
    Ok(lines)
}

/// Walk the record progression and return its lines and holder.
///
/// Returns a pair of the report lines and the index of the holder at
/// the close, or none for the holder when no record stands. The
/// window value scales every projection and stands in the first
/// line. The horizon bounds the reads the walk reads, so a read
/// whose elapsed value is greater than the horizon takes no part,
/// and no horizon reads every read in the runs. The field day report
/// reads with no horizon, and the ladder reading of PG-9 reads each
/// window section through this same walk with the window standing
/// as its horizon. Handed a value the loader would refuse, in which
/// the operator names no run among the runs, the walk names that
/// precondition as its error, in the discipline of FIX-PG-8, and
/// never fails bare.
pub(crate) fn record_walk(
    runs: &[DayRun],
    operator: &PText,
    window_value: &Num,
    horizon: Option<&BigRational>,
) -> Result<(Vec<String>, Option<usize>), String> {
    let window = window_value.to_rational();
    let operator_index = match runs.iter().position(|run| &run.runner == operator) {
        Some(index) => index,
        None => return Err("the operator names no run among the runs".to_string()),
    };
    let mut reads_by_run: Vec<Vec<(BigRational, BigRational)>> = Vec::new();
    let mut marks: BTreeSet<BigRational> = BTreeSet::new();
    for run in runs {
        let mut by_elapsed: Vec<(BigRational, BigRational)> = Vec::new();
        if let Some(reads) = &run.reads {
            for read in reads {
                let elapsed = read.elapsed_seconds.to_rational();
                if elapsed > BigRational::from(num_bigint::BigInt::from(0))
                    && horizon.is_none_or(|bound| elapsed <= *bound)
                {
                    match by_elapsed.iter_mut().find(|pair| pair.0 == elapsed) {
                        Some(pair) => pair.1 = read.usage.to_rational(),
                        None => {
                            by_elapsed.push((elapsed.clone(), read.usage.to_rational()));
                        }
                    }
                    marks.insert(elapsed);
                }
            }
        }
        reads_by_run.push(by_elapsed);
    }
    let mut lines = vec![format!(
        "field day, window {} seconds, {} runs, operator {}",
        format_seconds(window_value),
        runs.len(),
        operator
    )];
    let mut standing: Vec<Option<BigRational>> = vec![None; runs.len()];
    let mut set_at: Vec<Option<BigRational>> = vec![None; runs.len()];
    let mut holder: Option<usize> = None;
    let mut emitted: Option<(usize, BigRational)> = None;
    let mut operator_held = false;
    let mut last_stolen: Option<(BigRational, usize)> = None;
    for mark in &marks {
        for index in 0..runs.len() {
            let usage = reads_by_run[index]
                .iter()
                .find(|pair| pair.0 == *mark)
                .map(|pair| &pair.1);
            if let Some(usage) = usage {
                standing[index] = Some(usage.clone() * &window / mark);
                set_at[index] = Some(mark.clone());
            }
        }
        let record = match standing.iter().flatten().min() {
            Some(record) => record.clone(),
            None => continue,
        };
        let new_holder = match holder {
            Some(current) if standing[current].as_ref() == Some(&record) => current,
            _ => {
                let mut best: Option<usize> = None;
                for (index, figure) in standing.iter().enumerate() {
                    if figure.as_ref() == Some(&record) {
                        let take = match best {
                            None => true,
                            Some(champion) => {
                                (set_at[index].as_ref(), index)
                                    < (set_at[champion].as_ref(), champion)
                            }
                        };
                        if take {
                            best = Some(index);
                        }
                    }
                }
                best.expect("a record always has a holder")
            }
        };
        if Some(new_holder) != holder {
            if holder == Some(operator_index) {
                last_stolen = Some((mark.clone(), new_holder));
            }
            holder = Some(new_holder);
        }
        if holder == Some(operator_index) {
            operator_held = true;
        }
        let holder_index = holder.expect("a record always has a holder");
        if emitted != Some((holder_index, record.clone())) {
            lines.push(format!(
                "record at {} seconds, {} at a projection of {}",
                format_seconds_fraction(mark),
                runs[holder_index].runner,
                crate::season::format_usage(&record)
            ));
            emitted = Some((holder_index, record));
        }
    }
    let holder = match holder {
        None => {
            lines.push("finish, no record was set".to_string());
            return Ok((lines, None));
        }
        Some(holder) => holder,
    };
    let record = standing[holder]
        .clone()
        .expect("the holder stands at the record");
    lines.push(format!(
        "finish, record held by {} at a projection of {}",
        runs[holder].runner,
        crate::season::format_usage(&record)
    ));
    if holder == operator_index {
        let others: Vec<BigRational> = standing
            .iter()
            .enumerate()
            .filter(|(index, figure)| *index != operator_index && figure.is_some())
            .map(|(_, figure)| figure.clone().expect("filtered to standing figures"))
            .collect();
        let mut line = format!("record kept by {} to the close", operator);
        if let Some(lowest) = others.iter().min() {
            let margin = lowest - &record;
            line += &format!(
                ", margin {} at the close",
                crate::season::format_usage(&margin)
            );
        }
        lines.push(line);
    } else if operator_held {
        let (mark, thief) = last_stolen.expect("a lost record was stolen at some mark");
        let margin = standing[operator_index]
            .clone()
            .expect("the operator stands at a figure")
            - &record;
        lines.push(format!(
            "record stolen from {} at {} seconds by {}, margin {} at the close",
            operator,
            format_seconds_fraction(&mark),
            runs[thief].runner,
            crate::season::format_usage(&margin)
        ));
    } else if let Some(operator_figure) = &standing[operator_index] {
        let margin = operator_figure - &record;
        lines.push(format!(
            "record never held by {}, margin {} at the close",
            operator,
            crate::season::format_usage(&margin)
        ));
    } else {
        lines.push(format!(
            "record never held by {}, whose run projected no figure",
            operator
        ));
    }
    Ok((lines, Some(holder)))
}
