//! The season over a recorded field and the live board, for paceghost.
//!
//! The field loader never raises on malformed content and names
//! every defect it finds, reusing the PG-1 phrases under a run index
//! prefix. The season report and the board are pure functions of
//! their inputs, and every usage figure is computed on exact values
//! and rendered under the rule FIX-PG-7 states. The obligations this
//! module carries are PG-5 and PG-6 in CONTRACT.md, in the Round 2
//! section. The behavior mirrors the Python module of the same name,
//! which stands as the spec under PORT-1.

use std::cmp::Ordering;
use std::path::Path;

use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{One, Zero};

use crate::json::{parse_json, JsonValue};
use crate::pynum::{floor_divmod, format_seconds, Num};
use crate::pytext::PText;
use crate::runs::{check_text_field, load_run_data, read_text_file, Run};

/// Return the count of units a fraction rounds to at a precision.
///
/// The count is taken on the exact value, and an exact half rounds
/// up. Usage figures are never negative, so the rule needs no sign
/// case.
fn round_half_up_units(fraction: &BigRational, places: u32) -> BigInt {
    let scaled = fraction.clone() * BigRational::from(BigInt::from(10).pow(places));
    let units = floor_divmod(scaled.numer(), scaled.denom()).0;
    let rest = scaled - BigRational::from(units.clone());
    if rest.numer() * BigInt::from(2) >= *rest.denom() {
        units + BigInt::one()
    } else {
        units
    }
}

/// Render a count of units at a precision in plain decimal.
fn render_units(units: &BigInt, places: u32) -> String {
    let scale = BigInt::from(10).pow(places);
    let (whole, part) = floor_divmod(units, &scale);
    if part.is_zero() {
        return format!("{}", whole);
    }
    let part = part.to_string();
    let padded = format!("{:0>width$}", part, width = places as usize);
    format!("{}.{}", whole, padded.trim_end_matches('0'))
}

/// Return the exact decimal expansion of a fraction, or none.
///
/// The expansion terminates exactly when the denominator of the
/// fraction in its lowest terms carries no prime factor other than
/// two and five.
fn exact_expansion(fraction: &BigRational) -> Option<String> {
    let mut denominator = fraction.denom().clone();
    let two = BigInt::from(2);
    let five = BigInt::from(5);
    let mut twos: u32 = 0;
    while (&denominator % &two).is_zero() {
        denominator /= &two;
        twos += 1;
    }
    let mut fives: u32 = 0;
    while (&denominator % &five).is_zero() {
        denominator /= &five;
        fives += 1;
    }
    if !denominator.is_one() {
        return None;
    }
    let places = twos.max(fives);
    let units = (fraction.clone() * BigRational::from(BigInt::from(10).pow(places))).to_integer();
    Some(render_units(&units, places))
}

/// Render a usage figure from its exact value, never through a float.
///
/// A figure that needs six or fewer decimal places renders exactly.
/// A figure that needs more renders rounded at six decimal places,
/// with an exact half at the seventh place rounding up on the exact
/// value. A nonzero figure that rounds to zero at six places renders
/// its exact decimal expansion when that expansion terminates, and
/// otherwise renders rounded at twelve decimal places with halves
/// rounding up. Every figure renders in plain decimal notation and
/// never in scientific notation. The rule is FIX-PG-7 in
/// CONTRACT.md, amending PG-5 and PG-6.
pub fn format_usage(value: &BigRational) -> String {
    let scaled = value.clone() * BigRational::from(BigInt::from(1_000_000));
    if scaled.is_integer() {
        return render_units(&scaled.to_integer(), 6);
    }
    let bound = BigRational::new(BigInt::one(), BigInt::from(2_000_000));
    if !value.is_zero() && value < &bound {
        if let Some(expansion) = exact_expansion(value) {
            return expansion;
        }
        return render_units(&round_half_up_units(value, 12), 12);
    }
    render_units(&round_half_up_units(value, 6), 6)
}

/// Parse field text into its runs, naming every defect found.
///
/// Returns a pair of the run list and the defect list. A malformed
/// input returns no runs, and this function never raises on bad
/// content. Each clean run carries its model field.
pub fn load_field_text(text: &str) -> (Option<Vec<Run>>, Vec<String>) {
    let data = match parse_json(text) {
        Ok(value) => value,
        Err(_) => return (None, vec!["input is not valid JSON".to_string()]),
    };
    if !data.is_object() {
        return (None, vec!["input is not a JSON object".to_string()]);
    }
    let raw = match data.get("runs") {
        None => return (None, vec!["missing field runs".to_string()]),
        Some(JsonValue::Arr(items)) => items,
        Some(_) => return (None, vec!["field runs has the wrong type".to_string()]),
    };
    if raw.is_empty() {
        return (None, vec!["field runs is empty".to_string()]);
    }
    let mut defects: Vec<String> = Vec::new();
    let mut runs: Vec<Run> = Vec::new();
    let mut field_task: Option<PText> = None;
    for (index, entry) in raw.iter().enumerate() {
        let prefix = format!("run {} ", index);
        if !entry.is_object() {
            defects.push(format!("{}is not an object", prefix));
            continue;
        }
        let run = load_run_data(entry, &mut defects, &prefix);
        let model = check_text_field(entry, "model", &mut defects, &prefix);
        for name in ["usage", "budget"] {
            let phrase = format!("{}missing field {}", prefix, name);
            if entry.get(name).is_none() && !defects.contains(&phrase) {
                defects.push(phrase);
            }
        }
        let mut throwaway: Vec<String> = Vec::new();
        let task = check_text_field(entry, "task", &mut throwaway, "");
        if let Some(task) = task {
            match &field_task {
                None => field_task = Some(task),
                Some(standing) => {
                    if task != *standing {
                        defects.push(format!(
                            "{}field task does not match the field task",
                            prefix
                        ));
                    }
                }
            }
        }
        if let (Some(mut run), Some(model)) = (run, model) {
            run.model = Some(model);
            runs.push(run);
        }
    }
    if !defects.is_empty() {
        return (None, defects);
    }
    (Some(runs), defects)
}

/// Read a field from disk, with the same contract as text.
pub fn load_field_file(path: &Path) -> (Option<Vec<Run>>, Vec<String>) {
    match read_text_file(path) {
        None => (None, vec!["file cannot be read".to_string()]),
        Some(text) => load_field_text(&text),
    }
}

/// Return the season lines for a field against one ghost usage.
///
/// Blocks order by wins descending, then total usage ascending, then
/// model name. A run wins when it finished inside its own window on
/// less usage than the ghost.
pub fn season_report(runs: &[Run], ghost_usage: &Num) -> Vec<String> {
    let ghost_usage = ghost_usage.to_rational();
    let mut models: Vec<PText> = Vec::new();
    let mut grouped: Vec<Vec<&Run>> = Vec::new();
    for run in runs {
        let model = run
            .model
            .as_ref()
            .expect("a field run carries its model")
            .clone();
        match models.iter().position(|standing| *standing == model) {
            Some(position) => grouped[position].push(run),
            None => {
                models.push(model);
                grouped.push(vec![run]);
            }
        }
    }
    struct Row {
        model: PText,
        raced: usize,
        inside_window: usize,
        inside_budget: usize,
        total_usage: BigRational,
        wins: usize,
    }
    let mut rows: Vec<Row> = Vec::new();
    for (model, model_runs) in models.into_iter().zip(grouped) {
        let raced = model_runs.len();
        let mut inside_window = 0usize;
        let mut inside_budget = 0usize;
        let mut wins = 0usize;
        let mut total_usage = BigRational::from(BigInt::zero());
        for run in model_runs {
            let usage = run
                .usage
                .as_ref()
                .expect("a field run carries a usage figure")
                .to_rational();
            total_usage += usage.clone();
            let in_window = run.total_seconds.cmp_py(&run.window_seconds) != Ordering::Greater;
            if in_window {
                inside_window += 1;
            }
            let budget = run
                .budget
                .as_ref()
                .expect("a field run carries a budget")
                .to_rational();
            if usage <= budget {
                inside_budget += 1;
            }
            if in_window && usage < ghost_usage {
                wins += 1;
            }
        }
        rows.push(Row {
            model,
            raced,
            inside_window,
            inside_budget,
            total_usage,
            wins,
        });
    }
    rows.sort_by(|a, b| {
        b.wins
            .cmp(&a.wins)
            .then_with(|| a.total_usage.cmp(&b.total_usage))
            .then_with(|| a.model.cmp(&b.model))
    });
    let mut lines = vec![format!("season for task {}", runs[0].task)];
    for row in rows {
        lines.push(format!("model {}", row.model));
        lines.push(format!("runs raced {}", row.raced));
        lines.push(format!("inside window {}", row.inside_window));
        lines.push(format!("inside budget {}", row.inside_budget));
        lines.push(format!("total usage {}", format_usage(&row.total_usage)));
        let mean = row.total_usage.clone() / BigRational::from(BigInt::from(row.raced as i64));
        lines.push(format!("mean usage {}", format_usage(&mean)));
        lines.push(format!(
            "head to head against the ghost, wins {} of {}",
            row.wins, row.raced
        ));
    }
    lines
}

fn ghost_verdict(figure: &BigRational, ghost_usage: &BigRational) -> &'static str {
    match figure.cmp(ghost_usage) {
        Ordering::Less => "inside the ghost usage",
        Ordering::Greater => "past the ghost usage",
        Ordering::Equal => "level with the ghost usage",
    }
}

fn budget_verdict(figure: &BigRational, budget: &BigRational) -> &'static str {
    if figure <= budget {
        "inside the budget"
    } else {
        "past the budget"
    }
}

/// Return the live board lines for a run carrying timed reads.
///
/// The run carries reads, usage, and budget fields, and the ghost
/// carries a usage field. Each read projects the final usage from
/// the rate so far, and the final line sets the actual usage beside
/// the last projection and says whether the board called it.
pub fn board_report(run: &Run, ghost_usage: &Num) -> Vec<String> {
    let ghost_usage = ghost_usage.to_rational();
    let budget = run
        .budget
        .as_ref()
        .expect("a board run carries a budget")
        .to_rational();
    let window = run.window_seconds.to_rational();
    let reads = run.reads.as_ref().expect("a board run carries timed reads");
    let mut lines: Vec<String> = Vec::new();
    let mut last: Option<(BigRational, &'static str, &'static str)> = None;
    for (index, read) in reads.iter().enumerate() {
        let usage = read.usage.to_rational();
        let head = format!(
            "read {} at {} seconds, usage {}",
            index,
            format_seconds(&read.elapsed_seconds),
            format_usage(&usage)
        );
        let elapsed = read.elapsed_seconds.to_rational();
        if elapsed.is_zero() {
            lines.push(format!("{}, projects no figure", head));
            continue;
        }
        let projection = usage * &window / elapsed;
        let gv = ghost_verdict(&projection, &ghost_usage);
        let bv = budget_verdict(&projection, &budget);
        lines.push(format!(
            "{}, projection {}, {}, {}",
            head,
            format_usage(&projection),
            gv,
            bv
        ));
        last = Some((projection, gv, bv));
    }
    let actual = run
        .usage
        .as_ref()
        .expect("a board run carries a usage figure")
        .to_rational();
    let gv = ghost_verdict(&actual, &ghost_usage);
    let bv = budget_verdict(&actual, &budget);
    match last {
        None => lines.push(format!(
            "finish, actual usage {}, no projection was made, {}, {}",
            format_usage(&actual),
            gv,
            bv
        )),
        Some((projection, last_gv, last_bv)) => {
            let judgement = if last_gv == gv && last_bv == bv {
                "the board called it"
            } else {
                "the board missed it"
            };
            lines.push(format!(
                "finish, actual usage {} against a last projection of {}, {}, {}, {}",
                format_usage(&actual),
                format_usage(&projection),
                gv,
                bv,
                judgement
            ));
        }
    }
    lines
}
