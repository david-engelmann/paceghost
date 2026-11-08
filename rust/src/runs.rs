//! Recorded runs, the ghost rule, and the race report for paceghost.
//!
//! Every function in this module is a pure function of its inputs,
//! and the loader never raises on malformed content. The obligations
//! this module carries are PG-1 to PG-4 in CONTRACT.md, in the full
//! form section, plus the Round 2 run extension the season and the
//! board read. The behavior mirrors the Python module of the same
//! name, which stands as the spec under PORT-1.

use std::path::Path;

use crate::json::{parse_json, JsonValue};
use crate::pynum::{format_seconds, Num};
use crate::pytext::PText;

/// One timed phase of a recorded run.
#[derive(Clone, Debug, PartialEq)]
pub struct Split {
    pub phase: PText,
    pub elapsed_seconds: Num,
}

/// One timed usage read of a recorded run.
#[derive(Clone, Debug, PartialEq)]
pub struct Read {
    pub elapsed_seconds: Num,
    pub usage: Num,
}

/// A recorded run as the loader returns it.
///
/// The usage, budget, and reads fields stand only when the file
/// stated them. The model field stands only on a run the field
/// loader of the season returned.
#[derive(Clone, Debug)]
pub struct Run {
    pub task: PText,
    pub window_seconds: Num,
    pub runner: PText,
    pub recorded_at: PText,
    pub splits: Vec<Split>,
    pub total_seconds: Num,
    pub usage: Option<Num>,
    pub budget: Option<Num>,
    pub reads: Option<Vec<Read>>,
    pub model: Option<PText>,
}

/// Whether a JSON value is a number, as the Python reading counts it.
///
/// A boolean is not a number, and neither is anything else that is
/// not an integer or a float literal.
fn as_number(value: &JsonValue) -> Option<&Num> {
    match value {
        JsonValue::Num(number) => Some(number),
        _ => None,
    }
}

/// Whether a code point is an ASCII digit.
fn is_digit(unit: u32) -> bool {
    (0x30..=0x39).contains(&unit)
}

/// Parse a recorded_at value into its date, when it names a real date.
///
/// The shape is the extended ISO form, four digits, a dash, two
/// digits, a dash, two digits, and the date must stand on the
/// proleptic Gregorian calendar with a year from one to 9999, the
/// range and the calendar the Python date reading enforces.
pub fn parse_recorded_date(text: &PText) -> Option<(i32, u32, u32)> {
    let units = &text.0;
    if units.len() != 10 || units[4] != 0x2D || units[7] != 0x2D {
        return None;
    }
    for (index, &unit) in units.iter().enumerate() {
        if index != 4 && index != 7 && !is_digit(unit) {
            return None;
        }
    }
    let digit = |index: usize| (units[index] - 0x30) as i32;
    let year = digit(0) * 1000 + digit(1) * 100 + digit(2) * 10 + digit(3);
    let month = (digit(5) * 10 + digit(6)) as u32;
    let day = (digit(8) * 10 + digit(9)) as u32;
    if !(1..=9999).contains(&year) || !(1..=12).contains(&month) {
        return None;
    }
    let leap = (year % 4 == 0 && year % 100 != 0) || year % 400 == 0;
    let days_in_month = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        _ => {
            if leap {
                29
            } else {
                28
            }
        }
    };
    if day < 1 || day > days_in_month {
        return None;
    }
    Some((year, month, day))
}

/// The recorded date of a run, for the ghost rule comparisons.
fn recorded_date(run: &Run) -> (i32, u32, u32) {
    // A loaded run always carries a date that parses. A hand built
    // run carrying no real date reads as the earliest date here,
    // where the Python reading of it would raise.
    parse_recorded_date(&run.recorded_at).unwrap_or((0, 0, 0))
}

/// Read a text field of an object, naming every defect it draws.
///
/// Returns the value when it is sound, and nothing when it drew a
/// defect, in the order the Python check reads them.
pub(crate) fn check_text_field(
    data: &JsonValue,
    name: &str,
    defects: &mut Vec<String>,
    prefix: &str,
) -> Option<PText> {
    match data.get(name) {
        None => {
            defects.push(format!("{}missing field {}", prefix, name));
            None
        }
        Some(JsonValue::Str(value)) => {
            if value.is_empty() {
                defects.push(format!("{}field {} is empty", prefix, name));
                None
            } else if value.holds_unpaired_surrogate() {
                defects.push(format!(
                    "{}field {} holds an unpaired surrogate",
                    prefix, name
                ));
                None
            } else {
                Some(value.clone())
            }
        }
        Some(_) => {
            defects.push(format!("{}field {} has the wrong type", prefix, name));
            None
        }
    }
}

/// Validate the splits field and return the clean splits, if any.
fn load_splits(data: &JsonValue, defects: &mut Vec<String>, prefix: &str) -> Option<Vec<Split>> {
    let raw = match data.get("splits") {
        None => {
            defects.push(format!("{}missing field splits", prefix));
            return None;
        }
        Some(JsonValue::Arr(items)) => items,
        Some(_) => {
            defects.push(format!("{}field splits has the wrong type", prefix));
            return None;
        }
    };
    if raw.is_empty() {
        defects.push(format!("{}field splits is empty", prefix));
        return None;
    }
    let zero = Num::from_i64(0);
    let mut clean: Vec<Split> = Vec::new();
    let mut last_valid: Option<Num> = None;
    for (index, entry) in raw.iter().enumerate() {
        if !entry.is_object() {
            defects.push(format!("{}split {} is not an object", prefix, index));
            continue;
        }
        let mut phase: Option<PText> = None;
        match entry.get("phase") {
            None => defects.push(format!("{}split {} is missing field phase", prefix, index)),
            Some(JsonValue::Str(value)) => {
                if value.is_empty() {
                    defects.push(format!("{}split {} field phase is empty", prefix, index));
                } else if value.holds_unpaired_surrogate() {
                    defects.push(format!(
                        "{}split {} field phase holds an unpaired surrogate",
                        prefix, index
                    ));
                } else {
                    phase = Some(value.clone());
                }
            }
            Some(_) => defects.push(format!(
                "{}split {} field phase has the wrong type",
                prefix, index
            )),
        }
        let mut elapsed: Option<Num> = None;
        match entry.get("elapsed_seconds") {
            None => defects.push(format!(
                "{}split {} is missing field elapsed_seconds",
                prefix, index
            )),
            Some(JsonValue::Num(value)) => {
                if !value.is_finite() {
                    defects.push(format!(
                        "{}split {} field elapsed_seconds is not a finite number",
                        prefix, index
                    ));
                } else if value.cmp_py(&zero) == std::cmp::Ordering::Less {
                    defects.push(format!(
                        "{}split {} field elapsed_seconds is negative",
                        prefix, index
                    ));
                } else {
                    let failed_increase = match &last_valid {
                        Some(previous) => value.cmp_py(previous) != std::cmp::Ordering::Greater,
                        None => false,
                    };
                    if failed_increase {
                        defects.push(format!(
                            "{}split {} field elapsed_seconds is not greater than the previous split",
                            prefix, index
                        ));
                    } else {
                        last_valid = Some(value.clone());
                        elapsed = Some(value.clone());
                    }
                }
            }
            Some(_) => defects.push(format!(
                "{}split {} field elapsed_seconds has the wrong type",
                prefix, index
            )),
        }
        if let (Some(phase), Some(elapsed)) = (phase, elapsed) {
            clean.push(Split {
                phase,
                elapsed_seconds: elapsed,
            });
        }
    }
    if clean.len() != raw.len() {
        return None;
    }
    Some(clean)
}

/// Validate one optional usage figure and return it, if sound.
///
/// A budget must be positive and a usage figure must sit at or after
/// zero. An absent field is not a defect here, because the Round 2
/// extension fields are optional on a plain run.
pub(crate) fn load_usage_value(
    data: &JsonValue,
    name: &str,
    defects: &mut Vec<String>,
    prefix: &str,
) -> Option<Num> {
    let value = match data.get(name) {
        None => return None,
        Some(JsonValue::Num(value)) => value,
        Some(_) => {
            defects.push(format!("{}field {} has the wrong type", prefix, name));
            return None;
        }
    };
    if !value.is_finite() {
        defects.push(format!("{}field {} is not a finite number", prefix, name));
        return None;
    }
    let zero = Num::from_i64(0);
    if name == "budget" && value.cmp_py(&zero) != std::cmp::Ordering::Greater {
        defects.push(format!("{}field budget must be positive", prefix));
        return None;
    }
    if name == "usage" && value.cmp_py(&zero) == std::cmp::Ordering::Less {
        defects.push(format!("{}field usage is negative", prefix));
        return None;
    }
    Some(value.clone())
}

/// Validate the reads field and return the clean reads, if any.
pub(crate) fn load_reads(
    data: &JsonValue,
    defects: &mut Vec<String>,
    prefix: &str,
) -> Option<Vec<Read>> {
    let raw = match data.get("reads") {
        None => return None,
        Some(JsonValue::Arr(items)) => items,
        Some(_) => {
            defects.push(format!("{}field reads has the wrong type", prefix));
            return None;
        }
    };
    if raw.is_empty() {
        defects.push(format!("{}field reads is empty", prefix));
        return None;
    }
    let zero = Num::from_i64(0);
    let mut clean: Vec<Read> = Vec::new();
    let mut last_elapsed: Option<Num> = None;
    let mut last_usage: Option<Num> = None;
    for (index, entry) in raw.iter().enumerate() {
        if !entry.is_object() {
            defects.push(format!("{}read {} is not an object", prefix, index));
            continue;
        }
        let mut elapsed: Option<Num> = None;
        match entry.get("elapsed_seconds") {
            None => defects.push(format!(
                "{}read {} is missing field elapsed_seconds",
                prefix, index
            )),
            Some(JsonValue::Num(value)) => {
                if !value.is_finite() {
                    defects.push(format!(
                        "{}read {} field elapsed_seconds is not a finite number",
                        prefix, index
                    ));
                } else if value.cmp_py(&zero) == std::cmp::Ordering::Less {
                    defects.push(format!(
                        "{}read {} field elapsed_seconds is negative",
                        prefix, index
                    ));
                } else {
                    let failed_increase = match &last_elapsed {
                        Some(previous) => value.cmp_py(previous) != std::cmp::Ordering::Greater,
                        None => false,
                    };
                    if failed_increase {
                        defects.push(format!(
                            "{}read {} field elapsed_seconds is not greater than the previous read",
                            prefix, index
                        ));
                    } else {
                        last_elapsed = Some(value.clone());
                        elapsed = Some(value.clone());
                    }
                }
            }
            Some(_) => defects.push(format!(
                "{}read {} field elapsed_seconds has the wrong type",
                prefix, index
            )),
        }
        let mut usage: Option<Num> = None;
        match entry.get("usage") {
            None => defects.push(format!("{}read {} is missing field usage", prefix, index)),
            Some(JsonValue::Num(value)) => {
                if !value.is_finite() {
                    defects.push(format!(
                        "{}read {} field usage is not a finite number",
                        prefix, index
                    ));
                } else if value.cmp_py(&zero) == std::cmp::Ordering::Less {
                    defects.push(format!("{}read {} field usage is negative", prefix, index));
                } else {
                    let decreased = match &last_usage {
                        Some(previous) => value.cmp_py(previous) == std::cmp::Ordering::Less,
                        None => false,
                    };
                    if decreased {
                        defects.push(format!(
                            "{}read {} field usage is less than the previous read",
                            prefix, index
                        ));
                    } else {
                        last_usage = Some(value.clone());
                        usage = Some(value.clone());
                    }
                }
            }
            Some(_) => defects.push(format!(
                "{}read {} field usage has the wrong type",
                prefix, index
            )),
        }
        if let (Some(elapsed_seconds), Some(usage)) = (elapsed, usage) {
            clean.push(Read {
                elapsed_seconds,
                usage,
            });
        }
    }
    if clean.len() != raw.len() {
        return None;
    }
    Some(clean)
}

/// Validate one parsed run object, appending every defect found.
///
/// Returns the clean run, or nothing when this run drew any defect.
/// The prefix stands before every phrase, empty for a single run
/// file and naming the run index inside a field.
pub(crate) fn load_run_data(
    data: &JsonValue,
    defects: &mut Vec<String>,
    prefix: &str,
) -> Option<Run> {
    let before = defects.len();
    let task = check_text_field(data, "task", defects, prefix);
    let runner = check_text_field(data, "runner", defects, prefix);
    let mut window: Option<Num> = None;
    match data.get("window_seconds") {
        None => defects.push(format!("{}missing field window_seconds", prefix)),
        Some(JsonValue::Num(value)) => {
            if !value.is_finite() {
                defects.push(format!(
                    "{}field window_seconds is not a finite number",
                    prefix
                ));
            } else if value.cmp_py(&Num::from_i64(0)) != std::cmp::Ordering::Greater {
                defects.push(format!("{}field window_seconds must be positive", prefix));
            } else {
                window = Some(value.clone());
            }
        }
        Some(_) => defects.push(format!("{}field window_seconds has the wrong type", prefix)),
    }
    let mut recorded_at: Option<PText> = None;
    match data.get("recorded_at") {
        None => defects.push(format!("{}missing field recorded_at", prefix)),
        Some(JsonValue::Str(value)) => {
            if parse_recorded_date(value).is_some() {
                recorded_at = Some(value.clone());
            } else {
                defects.push(format!("{}field recorded_at is not an ISO date", prefix));
            }
        }
        Some(_) => defects.push(format!("{}field recorded_at has the wrong type", prefix)),
    }
    let splits = load_splits(data, defects, prefix);
    if let Some(carried) = data.get("total_seconds") {
        match as_number(carried) {
            None => defects.push(format!("{}field total_seconds has the wrong type", prefix)),
            Some(value) => {
                if !value.is_finite() {
                    defects.push(format!(
                        "{}field total_seconds is not a finite number",
                        prefix
                    ));
                } else if let Some(clean_splits) = &splits {
                    let final_elapsed = &clean_splits[clean_splits.len() - 1].elapsed_seconds;
                    if !value.eq_py(final_elapsed) {
                        defects.push(format!(
                            "{}field total_seconds does not equal the final split",
                            prefix
                        ));
                    }
                }
            }
        }
    }
    let usage = load_usage_value(data, "usage", defects, prefix);
    let budget = load_usage_value(data, "budget", defects, prefix);
    let reads = load_reads(data, defects, prefix);
    let has_reads = data.get("reads").is_some();
    if has_reads {
        if data.get("usage").is_none() {
            defects.push(format!("{}missing field usage", prefix));
        }
        if data.get("budget").is_none() {
            defects.push(format!("{}missing field budget", prefix));
        }
    }
    if defects.len() > before {
        return None;
    }
    let splits = splits.expect("a run with no defects holds clean splits");
    let total_seconds = splits[splits.len() - 1].elapsed_seconds.clone();
    Some(Run {
        task: task.expect("a run with no defects holds a task"),
        window_seconds: window.expect("a run with no defects holds a window"),
        runner: runner.expect("a run with no defects holds a runner"),
        recorded_at: recorded_at.expect("a run with no defects holds a date"),
        splits,
        total_seconds,
        usage: if data.get("usage").is_some() {
            usage
        } else {
            None
        },
        budget: if data.get("budget").is_some() {
            budget
        } else {
            None
        },
        reads: if has_reads { reads } else { None },
        model: None,
    })
}

/// Parse recorded run text into a run, naming every defect found.
///
/// Returns a pair of the run and the defect list. A malformed input
/// returns no run, and this function never raises on bad content.
pub fn load_run_text(text: &str) -> (Option<Run>, Vec<String>) {
    let data = match parse_json(text) {
        Ok(value) => value,
        Err(_) => return (None, vec!["input is not valid JSON".to_string()]),
    };
    if !data.is_object() {
        return (None, vec!["input is not a JSON object".to_string()]);
    }
    let mut defects: Vec<String> = Vec::new();
    let run = load_run_data(&data, &mut defects, "");
    if !defects.is_empty() {
        return (None, defects);
    }
    (run, defects)
}

/// Read a recorded run from disk, with the same contract as text.
pub fn load_run_file(path: &Path) -> (Option<Run>, Vec<String>) {
    match read_text_file(path) {
        None => (None, vec!["file cannot be read".to_string()]),
        Some(text) => load_run_text(&text),
    }
}

/// Read a whole file as UTF-8 text, or nothing when it cannot be read.
///
/// Mirrors the Python file entry points, which name a file that
/// cannot be read or cannot be decoded with the one defect.
pub(crate) fn read_text_file(path: &Path) -> Option<String> {
    let bytes = std::fs::read(path).ok()?;
    String::from_utf8(bytes).ok()
}

/// Return the ghost for one task, or nothing when there are no runs.
///
/// The ghost holds the lowest total, ties on total break toward the
/// earliest recorded date, and full ties keep the input order.
pub fn select_ghost(runs: &[Run]) -> Option<&Run> {
    let mut best: Option<&Run> = None;
    for run in runs {
        let take = match best {
            None => true,
            Some(current) => {
                let by_total = run.total_seconds.cmp_py(&current.total_seconds);
                by_total == std::cmp::Ordering::Less
                    || (by_total == std::cmp::Ordering::Equal
                        && recorded_date(run) < recorded_date(current))
            }
        };
        if take {
            best = Some(run);
        }
    }
    best
}

/// Return the window line for one run.
///
/// The line says the run finished inside its window and names the
/// seconds to spare when the total is at or under the window, and
/// says the run ran past its window and names the overrun when the
/// total is over the window.
pub fn window_line(run: &Run, subject: &str) -> String {
    if run.total_seconds.cmp_py(&run.window_seconds) != std::cmp::Ordering::Greater {
        let spare = format_seconds(&run.window_seconds.sub(&run.total_seconds));
        format!(
            "window, {} run finished inside its window with {} seconds to spare",
            subject, spare
        )
    } else {
        let over = format_seconds(&run.total_seconds.sub(&run.window_seconds));
        format!(
            "window, {} run ran {} seconds past its window",
            subject, over
        )
    }
}

/// Return the race lines for a live run against the ghost.
///
/// Splits pair by position. The window line for the live run stands
/// before the final line, which reports the finish from the two
/// totals. The report is a pure function of the two runs.
pub fn race_report(ghost: &Run, live: &Run) -> Vec<String> {
    let zero = Num::from_i64(0);
    let mut lines: Vec<String> = Vec::new();
    let ghost_splits = &ghost.splits;
    let live_splits = &live.splits;
    let paired = ghost_splits.len().min(live_splits.len());
    for index in 0..paired {
        let ghost_split = &ghost_splits[index];
        let live_split = &live_splits[index];
        if ghost_split.phase != live_split.phase {
            lines.push(format!(
                "split {} phase mismatch, ghost {} and live {}",
                index, ghost_split.phase, live_split.phase
            ));
            continue;
        }
        let difference = ghost_split.elapsed_seconds.sub(&live_split.elapsed_seconds);
        match difference.cmp_py(&zero) {
            std::cmp::Ordering::Greater => lines.push(format!(
                "{} ahead by {} seconds",
                ghost_split.phase,
                format_seconds(&difference)
            )),
            std::cmp::Ordering::Less => lines.push(format!(
                "{} behind by {} seconds",
                ghost_split.phase,
                format_seconds(&difference.neg())
            )),
            std::cmp::Ordering::Equal => lines.push(format!(
                "{} level at {} seconds",
                ghost_split.phase,
                format_seconds(&live_split.elapsed_seconds)
            )),
        }
    }
    if ghost_splits.len() != live_splits.len() {
        lines.push(format!(
            "split counts differ, ghost {} and live {}",
            ghost_splits.len(),
            live_splits.len()
        ));
    }
    lines.push(window_line(live, "the live"));
    let difference = ghost.total_seconds.sub(&live.total_seconds);
    match difference.cmp_py(&zero) {
        std::cmp::Ordering::Greater => lines.push(format!(
            "finish ahead by {} seconds",
            format_seconds(&difference)
        )),
        std::cmp::Ordering::Less => lines.push(format!(
            "finish behind by {} seconds",
            format_seconds(&difference.neg())
        )),
        std::cmp::Ordering::Equal => lines.push(format!(
            "finish level at {} seconds",
            format_seconds(&live.total_seconds)
        )),
    }
    lines
}

/// Check the runner label of one run for a line break.
///
/// The recorded run loader states no line break rule for a runner,
/// so a loaded run can carry a label holding a break, and the
/// readings that print a runner refuse such a label with the one
/// named error of FIX-PG-11 instead of printing it, in the
/// discipline of FIX-PG-12. The check consumes the one line break
/// predicate the loaders consume and stands in one home here for
/// the ghost change report and the command line printings alike.
pub(crate) fn check_runner_label(run: &Run) -> Result<(), String> {
    if run.runner.holds_line_break() {
        return Err("a runner label holds a line break".to_string());
    }
    Ok(())
}

/// Return whether the candidate takes the ghost, with a report line.
///
/// The line names both runners and both totals either way. Handed
/// a run whose runner label holds a line break, the report names
/// that precondition as its error instead of a line, the answer
/// standing on the error channel of FIX-PG-12.
pub fn ghost_change(old_ghost: &Run, candidate: &Run) -> Result<(bool, String), String> {
    check_runner_label(old_ghost)?;
    check_runner_label(candidate)?;
    // PG-4 defines the steal by the PG-2 rule applied to the pair, so
    // the candidate steals exactly when its total, and then its
    // recorded date on a tie, sorts strictly before the standing
    // ghost. Stating the comparison here keeps PG-4 from drifting if
    // select_ghost ever changes how it returns its pick.
    let by_total = candidate.total_seconds.cmp_py(&old_ghost.total_seconds);
    let steals = by_total == std::cmp::Ordering::Less
        || (by_total == std::cmp::Ordering::Equal
            && recorded_date(candidate) < recorded_date(old_ghost));
    let old_total = format_seconds(&old_ghost.total_seconds);
    let new_total = format_seconds(&candidate.total_seconds);
    let line = if steals {
        format!(
            "ghost changes from {} at {} seconds to {} at {} seconds",
            old_ghost.runner, old_total, candidate.runner, new_total
        )
    } else {
        format!(
            "ghost stays with {} at {} seconds against {} at {} seconds",
            old_ghost.runner, old_total, candidate.runner, new_total
        )
    };
    Ok((steals, line))
}
