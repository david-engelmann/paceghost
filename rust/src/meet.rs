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
//! set. The meet table reads the same walk competitor by
//! competitor, accumulating the lowest standing projection of
//! each competitor, the count of marks after which they held
//! the record, and their standing projection at the close in
//! the same mark loop that emits the record lines, so no second
//! progression stands for the two readings to drift apart on.
//! The reign reading reads the same walk a third way, over the
//! holder the same pass records after each mark, and states the
//! spells, the maximal runs of consecutive marks one competitor
//! held the record after. The days reading reads the same walk
//! a fourth way, over the same trail and the holder and the
//! record figure the pass records at the close of each day, and
//! states the marks and the handovers of each day and the
//! record that stood at its close. The forks reading reads
//! the same walk a fifth way, over the standing events the
//! same pass records for each competitor and the spells of
//! the trail cut by holder, and states one section for each
//! competitor naming every standing it held and the figures
//! of its close. The marks reading reads the same walk a sixth
//! way, over the state the same pass records before and after
//! each mark, and states every mark of the meet in walk order
//! with the standing it took against the state before it. The
//! standings reading reads the same walk a seventh way, over
//! the standing each competitor held at the close of each day
//! and the closing line the pass emits at each day's close,
//! and states every competitor's standing at every close in
//! competitor order with no competitor ranked against another.
//! The traces reading reads the same walk an eighth way, over
//! the same day standings read competitor by competitor, and
//! states every competitor's standing at the close of each
//! day in one section of its own, with the closes at which it
//! held a standing counted at the section's close. The
//! margins reading reads the same walk a ninth way, over the
//! nearest standing the same pass records after each mark,
//! and states every mark with the holder after it, the record
//! after it, the nearest standing held after it by a
//! competitor other than its holder, and the margin by which
//! the record stands ahead of that standing. The spread
//! reading reads the same walk a tenth way, over the
//! farthest standing and the field the same pass records
//! after each mark, and states every mark with the holder
//! after it, the record after it, the count of competitors
//! holding a standing after it, the farthest standing held
//! after it by a competitor other than its holder, and the
//! spread by which that farthest standing stands behind
//! the record. The middle
//! reading reads the same walk an eleventh way, over the
//! middle standing the same pass records after each mark,
//! and states every mark with the holder after it, the
//! record after it, the middle standing held after it by a
//! competitor other than its holder, and the middle margin
//! by which the record stands ahead of that standing. The tails
//! reading reads the same walk a twelfth way, over the
//! lower tail standing and the upper tail standing the
//! same pass records after each mark, and states every
//! mark with the holder after it, the record after it, the
//! lower tail standing held after it by the field behind
//! its holder and the lower tail margin by which the
//! record stands ahead of that standing, and the upper
//! tail standing held after it by that field and the
//! upper tail margin by which the record stands ahead of
//! that standing. Every
//! reading of the walk stands inside the label fencing closure
//! of FIX-PG-11, the one guard pass refusing a runner label
//! that holds a line break after the figures of every day are
//! checked and before any event is collected. The obligations
//! this module carries are PG-13 and PG-14 in CONTRACT.md, in
//! the Round 6 section, PG-15 and PG-16, in the Round 7
//! section, PG-17 and PG-18, in the Round 8 section, PG-19 and
//! PG-20, in the Round 9 section, PG-21 and PG-22 with the
//! FIX-PG-11 amendment, in the Round 10 section, and PG-23
//! and PG-24, in the Round 11 section, PG-25 and PG-26,
//! in the Round 12 section, PG-27 and PG-28, in the
//! Round 13 section, PG-29 and PG-30, in the Round 14
//! section, PG-31 and PG-32, in the Round 15 section,
//! PG-33 and PG-34, in the Round 16 section, and PG-35
//! and PG-36, in the Round 17 section.
//! No Python module stands
//! behind this one, so the contract text is the spec and the
//! pins in the test suite hold it.

use std::path::Path;

use num_bigint::BigInt;
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

/// The close of one day as the days reading states it, the
/// holder and the record figure standing at the close, read
/// off the same state the closing line of the meet report
/// renders, so the two readings of a close cannot drift.
fn day_close(
    holder: Option<usize>,
    standing: &[Option<BigRational>],
) -> Option<(usize, BigRational)> {
    holder.map(|holder| {
        (
            holder,
            standing[holder]
                .clone()
                .expect("the holder stands at the record"),
        )
    })
}

/// The per competitor state the meet table reads off the walk.
///
/// The lowest standing projection stands with the day and the
/// mark at which the competitor first stood at that figure, and
/// the held count is the count of marks after which the
/// competitor was the holder. A competitor holding no
/// projecting read in any day holds no lowest figure, and the
/// two stand and fall together. The standing events are the
/// marks at which one of the competitor's own reads applied,
/// each standing with the day and the mark of the read and the
/// projection it set, and the marks stood at count is the count
/// of marks of the walk from the first standing event to the
/// close of the walk, the first standing event counted, which
/// the forks reading reads beside the events themselves.
struct CompetitorStanding {
    lowest: Option<BigRational>,
    lowest_set: Option<(usize, BigRational)>,
    held: usize,
    standing_events: Vec<(usize, BigRational, BigRational)>,
    marks_stood_at: usize,
}

/// The standing of one mark of the walk against the state
/// that stood before it, as PG-23 states the four standings.
///
/// A mark took the record where the holder changed, naming the
/// holder it took the record from and the figure it displaced,
/// and a mark extended the record where its holder already held
/// it and the figure fell, naming the figure it displaced. A
/// mark stood below in every other case with the holder
/// unchanged, naming the record that stood before it, and that
/// case covers the mark whose own read raises the standing of
/// the competitor already holding the record, so the record
/// after the mark stands above the record before it.
enum MarkStanding {
    Set,
    Took { from: usize, displaced: BigRational },
    Extended { displaced: BigRational },
    Below { standing_record: BigRational },
}

/// One mark of the walk as the marks reading states it.
///
/// The day and the elapsed figure of the mark stand beside the
/// holder after the mark and the record figure after it, and
/// the standing of the mark against the state before it, so
/// the marks reading reads the pass a sixth way off these
/// records alone.
struct MarkRecord {
    day_index: usize,
    mark: BigRational,
    holder: usize,
    record: BigRational,
    standing: MarkStanding,
}

/// The outcome of the one progression walk of a meet.
///
/// The report lines stand beside the per competitor standings
/// the same pass accumulated, so the meet report and the meet
/// table are two readings of one walk and neither walks again.
/// The trail records the holder after each mark in walk order,
/// the day and the mark standing beside the holder, so the
/// reign reading reads the same pass a third way and no walk
/// stands for it either. The day closes record the holder and
/// the record figure at the close of each day in day order,
/// one entry per day, so the days reading reads the same pass
/// a fourth way and no walk stands for it either. The per
/// competitor standings carry the standing events of the same
/// pass beside the table figures, so the forks reading reads
/// the same pass a fifth way and no walk stands for it either.
/// The mark records carry the state before and after each mark
/// beside the holder and the record after it, so the marks
/// reading reads the same pass a sixth way and no walk stands
/// for it either. The day standings record the standing of
/// every competitor at the close of each day in day order,
/// one list of standings per day in competitor order, beside
/// the closing line the pass emits at that close, so the
/// standings reading reads the same pass a seventh way and
/// no walk stands for it either. The traces reading reads
/// the same day standings an eighth way, transposed
/// competitor by competitor, and no walk stands for it
/// either. The nearest standings record, after each mark,
/// the competitor holding the nearest standing after that
/// mark and its figure, one optional entry per mark in walk
/// order beside the mark records, an entry standing empty
/// where no competitor other than the holder held a standing
/// after the mark, so the margins reading reads the same
/// pass a ninth way and no walk stands for it either. The
/// farthest standings record, after each mark, the
/// competitor holding the farthest standing after that
/// mark and its figure, one optional entry per mark in
/// walk order beside the nearest standings, an entry
/// standing empty where no competitor other than the
/// holder held a standing after the mark, and the fields
/// record, after each mark, the count of competitors
/// holding a standing after that mark, the holder
/// counted, one count per mark in walk order beside the
/// farthest standings, so the spread reading reads the
/// same pass a tenth way and no walk stands for it either.
/// The middle standings record, after each mark, the
/// competitor holding the middle standing after that
/// mark and its figure, one optional entry per mark in
/// walk order beside the fields, an entry standing
/// empty where no competitor other than the holder held
/// a standing after the mark, so the middle reading
/// reads the same pass an eleventh way and no walk
/// stands for it either. The tails standings record,
/// after each mark, the lower tail standing and the
/// upper tail standing after that mark, the means of
/// the lowest and the highest figures of the field
/// behind the holder, one optional entry per mark in
/// walk order beside the middle standings, an entry
/// standing empty where no competitor other than the
/// holder held a standing after the mark, so the tails
/// reading reads the same pass a twelfth way and no
/// walk stands for it either. The mean standings
/// record, after each mark, the mean standing after
/// that mark, the mean of the whole field behind the
/// holder, one optional entry per mark in walk order
/// beside the tails standings, an entry standing
/// empty where no competitor other than the holder
/// held a standing after the mark, so the mean
/// reading reads the same pass a thirteenth way and
/// no walk stands for it either. The median
/// standings record, after each mark, the median
/// standing after that mark, the median of the field
/// behind the holder, one optional entry per mark in
/// walk order beside the mean standings, an entry
/// standing empty where no competitor other than the
/// holder held a standing after the mark, so the
/// median reading reads the same pass a fourteenth
/// way and no walk stands for it either. The quartiles
/// standings record, after each mark, the lower
/// quartile standing and the upper quartile standing
/// after that mark, the medians of the lowest and the
/// highest figures of the field behind the holder, one
/// optional entry per mark in walk order beside the
/// median standings, an entry standing empty where no
/// competitor other than the holder held a standing
/// after the mark, so the quartiles reading reads the
/// same pass a fifteenth way and no walk stands for it
/// either.
struct MeetWalk {
    lines: Vec<String>,
    labels: Vec<PText>,
    standings: Vec<CompetitorStanding>,
    close: Vec<Option<BigRational>>,
    marks_walked: usize,
    trail: Vec<(usize, BigRational, usize)>,
    day_closes: Vec<Option<(usize, BigRational)>>,
    marks_detail: Vec<MarkRecord>,
    nearest: Vec<Option<(usize, BigRational)>>,
    farthest: Vec<Option<(usize, BigRational)>>,
    fields: Vec<usize>,
    middle: Vec<Option<(usize, BigRational)>>,
    tails: Vec<Option<(BigRational, BigRational)>>,
    mean: Vec<Option<BigRational>>,
    median: Vec<Option<BigRational>>,
    quartiles: Vec<Option<(BigRational, BigRational)>>,
    day_standings: Vec<Vec<Option<BigRational>>>,
    day_close_lines: Vec<String>,
}

/// Which end of the field behind the holder an extremal
/// selection reads, the lowest figure or the highest.
#[derive(Clone, Copy, PartialEq)]
enum Extremal {
    Lowest,
    Highest,
}

/// The extremal standing after a mark, over the walk's vectors.
///
/// The extremal standing is the standing projection held
/// after the mark by a competitor other than the holder at
/// the end of the field behind the holder the direction
/// names, the lowest figure for the nearest selection and
/// the highest for the farthest, read off the standing and
/// set at vectors the holder selection just read. A tie at
/// that figure breaks to the competitor whose standing was
/// set at the earliest mark, and a tie at one mark breaks
/// to the earlier place in competitor order, the discipline
/// the holder selection applies among the competitors
/// standing at the record figure, read here over the field
/// behind the holder. No competitor other than the holder
/// holding a standing after the mark leaves the extremal
/// standing empty. The two selections share this one scan
/// and keep their own expect strings, passed by the caller,
/// so a broken invariant names the selection that broke it.
fn select_extreme(
    standing: &[Option<BigRational>],
    set_at: &[Option<(usize, BigRational)>],
    holder: usize,
    direction: Extremal,
    expect: &'static str,
) -> Option<(usize, BigRational)> {
    let mut best: Option<usize> = None;
    for (index, figure) in standing.iter().enumerate() {
        if index == holder {
            continue;
        }
        let Some(figure) = figure else {
            continue;
        };
        let take = match best {
            None => true,
            Some(champion) => {
                let champion_figure = standing[champion].as_ref().expect(expect);
                let beyond = match direction {
                    Extremal::Lowest => figure < champion_figure,
                    Extremal::Highest => figure > champion_figure,
                };
                beyond
                    || (figure == champion_figure
                        && (set_at[index].as_ref(), index) < (set_at[champion].as_ref(), champion))
            }
        };
        if take {
            best = Some(index);
        }
    }
    best.map(|index| (index, standing[index].clone().expect(expect)))
}

/// The ordered figures of the field behind the holder,
/// over the walk's vectors.
///
/// The field behind the holder is the standing projections
/// held after the mark by competitors other than the
/// holder, ordered from the lowest figure to the highest,
/// returned as references into the standing vector. The
/// middle selection takes its middle figure from this one
/// collection, the tails selection takes its two means
/// from it, the mean selection takes its mean of the
/// whole field from it, and the median selection takes
/// its middle figure or its two middle figures from
/// it, and the quartiles selection takes its two
/// sub-field medians from it, so the field the five
/// readings state stands in one body and the five
/// cannot be edited apart. The
/// helper guards no invariant and carries no expect
/// string, because it reads no competitor out of the
/// vector; the emptiness of the field is each caller's
/// own case.
fn field_figures(standing: &[Option<BigRational>], holder: usize) -> Vec<&BigRational> {
    let mut figures: Vec<&BigRational> = Vec::new();
    for (index, figure) in standing.iter().enumerate() {
        if index == holder {
            continue;
        }
        if let Some(figure) = figure {
            figures.push(figure);
        }
    }
    figures.sort();
    figures
}

/// The middle standing after a mark, over the walk's vectors.
///
/// The middle standing is the standing at the middle figure
/// after the mark, the middle figure being the figure at the
/// middle place (m + 1) div 2 in the ascending order of the
/// standing projections held after the mark by competitors
/// other than the holder, where m counts those competitors
/// and places count from one. Among the competitors standing
/// at that figure the selection applies the discipline the
/// holder selection applies among the competitors standing
/// at the record figure, the standing set at the earliest
/// mark, and at one mark the earlier place in competitor
/// order. No competitor other than the holder holding a
/// standing after the mark leaves the middle standing empty.
fn select_middle(
    standing: &[Option<BigRational>],
    set_at: &[Option<(usize, BigRational)>],
    holder: usize,
) -> Option<(usize, BigRational)> {
    let figures = field_figures(standing, holder);
    if figures.is_empty() {
        return None;
    }
    let middle_figure = figures[figures.len().div_ceil(2) - 1].clone();
    let mut best: Option<usize> = None;
    for (index, figure) in standing.iter().enumerate() {
        if index == holder {
            continue;
        }
        let Some(figure) = figure else {
            continue;
        };
        if *figure != middle_figure {
            continue;
        }
        let take = match best {
            None => true,
            Some(champion) => {
                (set_at[index].as_ref(), index) < (set_at[champion].as_ref(), champion)
            }
        };
        if take {
            best = Some(index);
        }
    }
    best.map(|index| {
        (
            index,
            standing[index]
                .clone()
                .expect("the middle competitor stands at a figure"),
        )
    })
}

/// The tail standings after a mark, over the walk's vectors.
///
/// The lower tail standing is the mean of the lowest
/// figures of the field behind the holder and the upper
/// tail standing is the mean of the highest figures of
/// that field, the tails taken over the (m + 1) div 2
/// lowest and the (m + 1) div 2 highest of the ordered
/// figures the one collection returns, where m counts the
/// competitors of the field, so the middle figure stands
/// in both tails where m is odd and the two tails stand
/// apart where m is even and past two. A tail standing is
/// a mean of figures and is held by no competitor, so the
/// selection names no holder for either tail and applies
/// no tie discipline. No competitor other than the holder
/// holding a standing after the mark leaves the tail
/// standings empty.
fn select_tails(
    standing: &[Option<BigRational>],
    holder: usize,
) -> Option<(BigRational, BigRational)> {
    let figures = field_figures(standing, holder);
    if figures.is_empty() {
        return None;
    }
    let count = figures.len().div_ceil(2);
    let mean = |part: &[&BigRational]| {
        let mut total = BigRational::from(BigInt::from(0));
        for figure in part {
            total += (*figure).clone();
        }
        total / BigRational::from(BigInt::from(part.len() as i64))
    };
    let lower = mean(&figures[..count]);
    let upper = mean(&figures[figures.len() - count..]);
    Some((lower, upper))
}

/// The mean standing after a mark, over the walk's vectors.
///
/// The mean standing is the mean of the whole field behind
/// the holder, the sum of the ordered figures the one
/// collection returns divided by the count of those
/// figures, where the count is the number of competitors
/// of the field, so where one competitor stands behind the
/// holder the mean is that competitor's figure whole and
/// where two stand behind the holder the mean is the
/// midpoint of their two figures. A mean standing is a
/// mean of figures and is held by no competitor, so the
/// selection names no holder for the mean and applies no
/// tie discipline. No competitor other than the holder
/// holding a standing after the mark leaves the mean
/// standing empty.
fn select_mean(standing: &[Option<BigRational>], holder: usize) -> Option<BigRational> {
    let figures = field_figures(standing, holder);
    if figures.is_empty() {
        return None;
    }
    let mut total = BigRational::from(BigInt::from(0));
    for figure in &figures {
        total += (*figure).clone();
    }
    Some(total / BigRational::from(BigInt::from(figures.len() as i64)))
}

/// The median standing after a mark, over the walk's vectors.
///
/// The median standing is the figure standing at the
/// middle place of the ordered figures the one collection
/// returns where the count of those figures is odd, the
/// middle place being the place (m + 1) div 2 where m
/// counts the competitors of the field and places count
/// from one, and the mean of the two figures standing at
/// the middle places where the count is even, the middle
/// places being the places m div 2 and m div 2 + 1, so
/// where one competitor stands behind the holder the
/// median is that competitor's figure whole, where two
/// stand behind the holder the median is the midpoint of
/// their two figures, where three stand behind the holder
/// the median is the middle of their three figures, and
/// where four stand behind the holder the median is the
/// midpoint of the two middle figures of their four. A
/// median standing is a statistic of the figures of the
/// field and is held by no competitor, so the selection
/// names no holder for the median and applies no tie
/// discipline. No competitor other than the holder
/// holding a standing after the mark leaves the median
/// standing empty.
fn select_median(standing: &[Option<BigRational>], holder: usize) -> Option<BigRational> {
    let figures = field_figures(standing, holder);
    if figures.is_empty() {
        return None;
    }
    if figures.len() % 2 == 1 {
        Some(figures[figures.len() / 2].clone())
    } else {
        let lower = figures[figures.len() / 2 - 1].clone();
        let upper = figures[figures.len() / 2].clone();
        Some((lower + upper) / BigRational::from(BigInt::from(2)))
    }
}

/// The quartile standings after a mark, over the walk's vectors.
///
/// The lower quartile standing is the median of the
/// lowest figures of the field behind the holder and
/// the upper quartile standing is the median of the
/// highest figures of that field, the lowest and the
/// highest counting the (m + 1) div 2 figures where m
/// counts the competitors of the field, the median of
/// a part being the figure standing at its middle
/// place where the part counts odd and the mean of
/// the two figures standing at its middle places
/// where the part counts even, so where one competitor
/// stands behind the holder each quartile is that
/// competitor's figure whole, where two stand behind
/// the holder the lower quartile is the lower of their
/// two figures and the upper quartile is the higher,
/// where three stand behind the holder the lower
/// quartile is the mean of the two lowest figures and
/// the upper quartile is the mean of the two highest,
/// where four stand behind the holder the lower
/// quartile is the mean of the two lowest figures and
/// the upper quartile is the mean of the two highest,
/// the two sub-fields standing apart, and where five
/// stand behind the holder the lower quartile is the
/// middle figure of the three lowest figures and the
/// upper quartile is the middle figure of the three
/// highest. A quartile standing is a statistic of the
/// figures of a sub-field and is held by no
/// competitor, so the selection names no holder for
/// either quartile and applies no tie discipline. No
/// competitor other than the holder holding a standing
/// after the mark leaves the quartile standings empty.
fn select_quartiles(
    standing: &[Option<BigRational>],
    holder: usize,
) -> Option<(BigRational, BigRational)> {
    let figures = field_figures(standing, holder);
    if figures.is_empty() {
        return None;
    }
    let count = figures.len().div_ceil(2);
    let median = |part: &[&BigRational]| {
        if part.len() % 2 == 1 {
            part[part.len() / 2].clone()
        } else {
            let lower = part[part.len() / 2 - 1].clone();
            let upper = part[part.len() / 2].clone();
            (lower + upper) / BigRational::from(BigInt::from(2))
        }
    };
    let lower = median(&figures[..count]);
    let upper = median(&figures[figures.len() - count..]);
    Some((lower, upper))
}

/// Walk the record progression of one loaded meet once.
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
/// marks of each day the closing line of that day stands among
/// the report lines, and the report closes with the finish line,
/// the line naming the day and the mark at which the figure the
/// holder stands at was set, and the count of days read. In the
/// same pass the walk accumulates, per competitor, the lowest
/// standing projection held after any mark with the day and mark
/// at which it was first stood at, and the count of marks after
/// which the competitor was the holder, and it records the
/// holder after each mark in the trail the reign reading reads
/// and the holder and the record figure at the close of each
/// day in the day closes the days reading reads, and the
/// standing events of each competitor, the marks at which one
/// of its own reads applied with the projection each set, in
/// the standing events the forks reading reads, and it records
/// against each mark the holder and the record figure after it
/// and the standing of the mark against the state that stood
/// before it, in the mark records the marks reading reads, and
/// at the close of each day it records the standing of every
/// competitor and the closing line it emits at that close, in
/// the day standings and the day closing lines the standings
/// reading reads, and after the holder selection at each mark
/// it records the nearest standing after the mark, the lowest
/// standing then held by a competitor other than the holder,
/// a tie at that figure breaking to the competitor whose
/// standing was set at the earliest mark and then to the
/// earlier competitor place, in the nearest standings the
/// margins reading reads, and it records the farthest
/// standing after the mark, the highest standing then held
/// by a competitor other than the holder, a tie at that
/// figure breaking to the competitor whose standing was
/// set at the earliest mark and then to the earlier
/// competitor place, and the field after the mark, the
/// count of competitors then holding a standing, the
/// holder counted, in the farthest standings and the
/// fields the spread reading reads, and it records the
/// middle standing after the mark, the standing at the
/// middle figure of the field behind the holder, a tie
/// at that figure breaking to the competitor whose
/// standing was set at the earliest mark and then to the
/// earlier competitor place, in the middle standings the
/// middle reading reads, and it records the lower tail
/// standing and the upper tail standing after the mark,
/// the means of the lowest and the highest figures of
/// the field behind the holder, in the tails standings
/// the tails reading reads, and it records the mean
/// standing after the mark, the mean of the whole field
/// behind the holder, in the mean standings the mean
/// reading reads, and it records the median standing
/// after the mark, the median of the field behind the
/// holder, in the median standings the median reading
/// reads, and it records the lower quartile standing
/// and the upper quartile standing after the mark,
/// the medians of the lowest and the highest figures
/// of the field behind the holder, in the quartiles
/// standings the quartiles reading reads. Handed a
/// meet value built
/// by hand holding no days, or more days than the ordinal table
/// names, the walk names that precondition as its error, and
/// handed a day carrying a figure its loader would refuse as not
/// finite it names the broken precondition as its error, the
/// days checked in list order, in the discipline of FIX-PG-8 and
/// FIX-PG-10, and never fails bare.
fn meet_walk(meet: &Meet) -> Result<MeetWalk, String> {
    if meet.days.is_empty() {
        return Err("the meet holds no days".to_string());
    }
    if meet.days.len() > ORDINALS.len() {
        return Err("the meet holds more days than the ordinal table names".to_string());
    }
    for day in &meet.days {
        check_finite_figures(&day.window_seconds, &day.runs)?;
    }
    for day in &meet.days {
        for run in &day.runs {
            if run.runner.holds_line_break() {
                return Err("a runner label holds a line break".to_string());
            }
        }
    }
    let CollectedEvents {
        labels,
        events,
        marks,
    } = collect_events(meet.days.iter());
    let marks_walked = marks.len();
    let mut lines = vec![format!("meet, {} days", meet.days.len())];
    let mut standing: Vec<Option<BigRational>> = vec![None; labels.len()];
    let mut set_at: Vec<Option<(usize, BigRational)>> = vec![None; labels.len()];
    let mut lowest: Vec<Option<BigRational>> = vec![None; labels.len()];
    let mut lowest_set: Vec<Option<(usize, BigRational)>> = vec![None; labels.len()];
    let mut held: Vec<usize> = vec![0; labels.len()];
    let mut standing_events: Vec<Vec<(usize, BigRational, BigRational)>> =
        vec![Vec::new(); labels.len()];
    let mut stood_from: Vec<Option<usize>> = vec![None; labels.len()];
    let mut trail: Vec<(usize, BigRational, usize)> = Vec::new();
    let mut day_closes: Vec<Option<(usize, BigRational)>> = Vec::new();
    let mut marks_detail: Vec<MarkRecord> = Vec::new();
    let mut nearest: Vec<Option<(usize, BigRational)>> = Vec::new();
    let mut farthest: Vec<Option<(usize, BigRational)>> = Vec::new();
    let mut fields: Vec<usize> = Vec::new();
    let mut middle: Vec<Option<(usize, BigRational)>> = Vec::new();
    let mut tails: Vec<Option<(BigRational, BigRational)>> = Vec::new();
    let mut mean: Vec<Option<BigRational>> = Vec::new();
    let mut median: Vec<Option<BigRational>> = Vec::new();
    let mut quartiles: Vec<Option<(BigRational, BigRational)>> = Vec::new();
    let mut day_standings: Vec<Vec<Option<BigRational>>> = Vec::new();
    let mut day_close_lines: Vec<String> = Vec::new();
    let mut previous: Option<(usize, BigRational)> = None;
    let mut holder: Option<usize> = None;
    let mut emitted: Option<(usize, BigRational)> = None;
    let mut closed_day = 0usize;
    for (position, (day_index, mark)) in marks.iter().enumerate() {
        while closed_day < *day_index {
            let emitted_line = close_line(closed_day, holder, &standing, &labels);
            lines.push(emitted_line.clone());
            day_close_lines.push(emitted_line);
            day_standings.push(standing.clone());
            day_closes.push(day_close(holder, &standing));
            closed_day += 1;
        }
        for competitor in 0..labels.len() {
            if let Some(projection) = events[competitor].get(&(*day_index, mark.clone())) {
                standing[competitor] = Some(projection.clone());
                set_at[competitor] = Some((*day_index, mark.clone()));
                standing_events[competitor].push((*day_index, mark.clone(), projection.clone()));
                if stood_from[competitor].is_none() {
                    stood_from[competitor] = Some(position);
                }
            }
        }
        for competitor in 0..labels.len() {
            if let Some(figure) = &standing[competitor] {
                let is_lowest = match &lowest[competitor] {
                    None => true,
                    Some(current) => figure < current,
                };
                if is_lowest {
                    lowest[competitor] = Some(figure.clone());
                    lowest_set[competitor] = Some((*day_index, mark.clone()));
                }
            }
        }
        let (record, new_holder) = match select_holder(&standing, &set_at, holder) {
            Some(pair) => pair,
            None => continue,
        };
        holder = Some(new_holder);
        held[new_holder] += 1;
        trail.push((*day_index, mark.clone(), new_holder));
        let mark_standing = match previous {
            None => MarkStanding::Set,
            Some((before_holder, before_record)) if new_holder != before_holder => {
                MarkStanding::Took {
                    from: before_holder,
                    displaced: before_record,
                }
            }
            Some((_, before_record)) if record < before_record => MarkStanding::Extended {
                displaced: before_record,
            },
            Some((_, before_record)) => MarkStanding::Below {
                standing_record: before_record,
            },
        };
        marks_detail.push(MarkRecord {
            day_index: *day_index,
            mark: mark.clone(),
            holder: new_holder,
            record: record.clone(),
            standing: mark_standing,
        });
        // The nearest selection, the lowest standing held
        // behind the holder, its discipline stated once at
        // the shared scan and kept in its own expect string.
        nearest.push(select_extreme(
            &standing,
            &set_at,
            new_holder,
            Extremal::Lowest,
            "the nearest competitor stands at a figure",
        ));
        // The farthest selection, the highest standing held
        // behind the holder, its discipline stated once at
        // the shared scan and kept in its own expect string.
        farthest.push(select_extreme(
            &standing,
            &set_at,
            new_holder,
            Extremal::Highest,
            "the farthest competitor stands at a figure",
        ));
        fields.push(standing.iter().filter(|held| held.is_some()).count());
        middle.push(select_middle(&standing, &set_at, new_holder));
        tails.push(select_tails(&standing, new_holder));
        mean.push(select_mean(&standing, new_holder));
        median.push(select_median(&standing, new_holder));
        quartiles.push(select_quartiles(&standing, new_holder));
        previous = Some((new_holder, record.clone()));
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
        let emitted_line = close_line(closed_day, holder, &standing, &labels);
        lines.push(emitted_line.clone());
        day_close_lines.push(emitted_line);
        day_standings.push(standing.clone());
        day_closes.push(day_close(holder, &standing));
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
    let standings = (0..labels.len())
        .map(|competitor| CompetitorStanding {
            lowest: lowest[competitor].clone(),
            lowest_set: lowest_set[competitor].clone(),
            held: held[competitor],
            standing_events: standing_events[competitor].clone(),
            marks_stood_at: match stood_from[competitor] {
                Some(position) => marks_walked - position,
                None => 0,
            },
        })
        .collect();
    Ok(MeetWalk {
        lines,
        labels,
        standings,
        close: standing,
        marks_walked,
        trail,
        day_closes,
        marks_detail,
        nearest,
        farthest,
        fields,
        middle,
        tails,
        mean,
        median,
        quartiles,
        day_standings,
        day_close_lines,
    })
}

/// Return the meet lines for one loaded meet.
///
/// The report is the chronological reading of the one walk, the
/// record lines at the marks where the holder or the record
/// figure changes and the closing lines the walk emits.
pub fn meet_report(meet: &Meet) -> Result<Vec<String>, String> {
    Ok(meet_walk(meet)?.lines)
}

/// Return the meet table lines for one loaded meet.
///
/// The table reads the one walk competitor by competitor, in
/// the competitor order of the walk. One line stands for each
/// competitor, naming their lowest standing projection and the
/// day and mark at which its figure was set, the count of marks
/// after which they held the record, and their standing
/// projection at the close, and a competitor holding no
/// projecting read in any day stands in the none forms with a
/// held count of zero. The closing line counts the competitors,
/// the competitors who held the record after some mark, and
/// the marks the walk walked. No competitor is ranked or
/// compared against another, and the order is the competitor
/// order of the walk alone.
pub fn meet_table(meet: &Meet) -> Result<Vec<String>, String> {
    let walk = meet_walk(meet)?;
    let mut lines = vec![format!("meet table, {} days", meet.days.len())];
    for (competitor, label) in walk.labels.iter().enumerate() {
        let standing = &walk.standings[competitor];
        match (&standing.lowest, &standing.lowest_set, &walk.close[competitor]) {
            (Some(lowest), Some((set_day, set_mark)), Some(close)) => lines.push(format!(
                "competitor {}, lowest standing projection {}, set on the {} day at {} seconds, record held after {} marks, standing projection {} at the close",
                label,
                format_usage(lowest),
                ORDINALS[*set_day],
                format_seconds_fraction(set_mark),
                standing.held,
                format_usage(close)
            )),
            _ => lines.push(format!(
                "competitor {}, no standing projection at any mark, record held after {} marks, no standing projection at the close",
                label, standing.held
            )),
        }
    }
    let holders = walk.standings.iter().filter(|s| s.held > 0).count();
    lines.push(format!(
        "table counts, competitors {}, record holders {}, marks walked {}",
        walk.labels.len(),
        holders,
        walk.marks_walked
    ));
    Ok(lines)
}

/// One spell of the reign reading.
///
/// A spell is a maximal run of consecutive marks of the walk
/// after which the same competitor was the holder. The first
/// and last marks stand as the day index and the elapsed figure
/// of the marks that bound the run, and the length is the count
/// of its marks.
struct Spell {
    holder: usize,
    first: (usize, BigRational),
    last: (usize, BigRational),
    length: usize,
}

/// Aggregate the trail of one walk into its spells.
///
/// The spells stand in walk order, one for each maximal run of
/// consecutive marks one competitor held the record after. The
/// reign reading states the list whole and the forks reading
/// cuts it by holder, and the two read this one aggregation so
/// their spells cannot drift apart.
fn collect_spells(trail: &[(usize, BigRational, usize)]) -> Vec<Spell> {
    let mut spells: Vec<Spell> = Vec::new();
    for (day_index, mark, holder) in trail {
        match spells.last_mut() {
            Some(spell) if spell.holder == *holder => {
                spell.last = (*day_index, mark.clone());
                spell.length += 1;
            }
            _ => spells.push(Spell {
                holder: *holder,
                first: (*day_index, mark.clone()),
                last: (*day_index, mark.clone()),
                length: 1,
            }),
        }
    }
    spells
}

/// Return the reign lines for one loaded meet.
///
/// The reading reads the trail of the one walk, the holder the
/// walk recorded after each mark, and states the spells in
/// walk order, one line for each maximal run of consecutive
/// marks one competitor held the record after, naming the
/// competitor and the first and last marks of the spell and
/// its length in marks. The closing line counts the marks the
/// walk walked, the spells, and the competitors holding at
/// least one spell, and names the longest spell and the
/// competitor who held it, the earliest such spell standing
/// where two or more spells share the greatest length. A meet
/// in which no read projects holds no marks and no spells,
/// and its closing line names that no spell stood. No
/// competitor is ranked or compared against another beyond
/// the longest spell the closing line names.
pub fn meet_reign(meet: &Meet) -> Result<Vec<String>, String> {
    let walk = meet_walk(meet)?;
    let spells = collect_spells(&walk.trail);
    let mut lines = vec![format!("meet reign, {} days", meet.days.len())];
    for spell in &spells {
        lines.push(format!(
            "spell, {}, first mark at {} seconds on the {} day, last mark at {} seconds on the {} day, {} marks",
            walk.labels[spell.holder],
            format_seconds_fraction(&spell.first.1),
            ORDINALS[spell.first.0],
            format_seconds_fraction(&spell.last.1),
            ORDINALS[spell.last.0],
            spell.length
        ));
    }
    if spells.is_empty() {
        lines.push(format!(
            "reign counts, marks walked {}, spells 0, holders 0, no spell stood",
            walk.marks_walked
        ));
    } else {
        let mut longest = &spells[0];
        for spell in &spells[1..] {
            if spell.length > longest.length {
                longest = spell;
            }
        }
        let mut holders: Vec<usize> = Vec::new();
        for spell in &spells {
            if !holders.contains(&spell.holder) {
                holders.push(spell.holder);
            }
        }
        lines.push(format!(
            "reign counts, marks walked {}, spells {}, holders {}, longest spell {} marks held by {}",
            walk.marks_walked,
            spells.len(),
            holders.len(),
            longest.length,
            walk.labels[longest.holder]
        ));
    }
    Ok(lines)
}

/// Return the days lines for one loaded meet.
///
/// The reading reads the trail and the day closes of the one
/// walk, and states one line per day of the meet in day order,
/// naming the count of marks of the walk standing in that day,
/// the count of handovers counted in that day, a handover
/// being a mark whose holder differs from the holder after the
/// mark before it in the walk and counted in the day of its
/// own mark, and the record at the close of the day and the
/// holder who holds it, or that no record stood at that close
/// where none stands. The closing line counts the days, the
/// marks the walk walked, and the handovers over the whole
/// meet, and the handovers equal the spells of the reign
/// reading minus one where a spell stands. No competitor is
/// ranked or compared against another, and the days stand in
/// day order alone.
pub fn meet_days(meet: &Meet) -> Result<Vec<String>, String> {
    let walk = meet_walk(meet)?;
    let mut marks: Vec<usize> = vec![0; meet.days.len()];
    let mut handovers: Vec<usize> = vec![0; meet.days.len()];
    let mut previous: Option<usize> = None;
    for (day_index, _mark, holder) in &walk.trail {
        marks[*day_index] += 1;
        if matches!(previous, Some(before) if before != *holder) {
            handovers[*day_index] += 1;
        }
        previous = Some(*holder);
    }
    let mut lines = vec![format!("meet days, {} days", meet.days.len())];
    for (day_index, close) in walk.day_closes.iter().enumerate() {
        let close_text = match close {
            Some((holder, record)) => format!(
                "record at the close held by {} at a projection of {}",
                walk.labels[*holder],
                format_usage(record)
            ),
            None => "no record stood at the close".to_string(),
        };
        lines.push(format!(
            "day, the {} day, marks {}, handovers {}, {}",
            ORDINALS[day_index], marks[day_index], handovers[day_index], close_text
        ));
    }
    lines.push(format!(
        "days counts, days {}, marks walked {}, handovers {}",
        meet.days.len(),
        walk.marks_walked,
        handovers.iter().sum::<usize>()
    ));
    Ok(lines)
}

/// Return the forks lines for one loaded meet.
///
/// The reading reads the standing events and the trail of the
/// one walk competitor by competitor, in the competitor order
/// of the walk, and states one section for each competitor,
/// naming every standing event of the competitor in walk order
/// with the projection the event set and the day and mark at
/// which it was set, then each spell of the competitor in the
/// order the spells stand in the reign reading, and closes the
/// section with the count of days whose runs name the
/// competitor, the count of marks the competitor stood at, the
/// count of its spells, and the standing projection it holds
/// at the close, or that it holds none. The closing line counts
/// the competitors, the marks stood at summed over the
/// competitors, which is the marks walked counted once for
/// each competitor holding a standing after each mark, and the
/// spells over the whole meet, which is the spells count of
/// the reign reading. No competitor is ranked or compared
/// against another, and the competitors stand in the
/// competitor order of the walk alone.
pub fn meet_forks(meet: &Meet) -> Result<Vec<String>, String> {
    let walk = meet_walk(meet)?;
    let spells = collect_spells(&walk.trail);
    let mut lines = vec![format!(
        "meet forks, {} days, {} competitors",
        meet.days.len(),
        walk.labels.len()
    )];
    let mut total_marks_stood_at = 0usize;
    for (competitor, label) in walk.labels.iter().enumerate() {
        lines.push(format!("competitor {}", label));
        let standing = &walk.standings[competitor];
        for (day_index, mark, figure) in &standing.standing_events {
            lines.push(format!(
                "standing, at a projection of {}, set at {} seconds on the {} day",
                format_usage(figure),
                format_seconds_fraction(mark),
                ORDINALS[*day_index]
            ));
        }
        let own_spells: Vec<&Spell> = spells
            .iter()
            .filter(|spell| spell.holder == competitor)
            .collect();
        for spell in &own_spells {
            lines.push(format!(
                "spell, first mark at {} seconds on the {} day, last mark at {} seconds on the {} day, {} marks",
                format_seconds_fraction(&spell.first.1),
                ORDINALS[spell.first.0],
                format_seconds_fraction(&spell.last.1),
                ORDINALS[spell.last.0],
                spell.length
            ));
        }
        let days_stood_in = meet
            .days
            .iter()
            .filter(|day| day.runs.iter().any(|run| run.runner == *label))
            .count();
        total_marks_stood_at += standing.marks_stood_at;
        let close_text = match &walk.close[competitor] {
            Some(close) => format!("standing projection {} at the close", format_usage(close)),
            None => "no standing projection at the close".to_string(),
        };
        lines.push(format!(
            "close, days stood in {}, marks stood at {}, spells held {}, {}",
            days_stood_in,
            standing.marks_stood_at,
            own_spells.len(),
            close_text
        ));
    }
    lines.push(format!(
        "forks counts, competitors {}, marks stood at {}, spells {}",
        walk.labels.len(),
        total_marks_stood_at,
        spells.len()
    ));
    Ok(lines)
}

/// Return the marks lines for one loaded meet.
///
/// The reading reads the mark records of the one walk, and
/// states one section for each day of the meet in meet order,
/// every day holding a section, and inside a section one line
/// for each mark of the walk standing in that day, in walk
/// order, naming the place of the mark in the whole walk, the
/// holder after the mark and the record figure after it, in
/// the mark form of the meet report, and the standing of the
/// mark in one of the four phrases of PG-24, every phrase but
/// the first naming the state that stood before the mark. The
/// closing line counts the marks the walk walked, the record
/// marks, the marks that set, took, or extended the record,
/// and the days. The marks that set or took the record equal
/// the spells of the reign reading, because every spell opens
/// on such a mark and no extension opens one, and the marks
/// walked equal the sum of the per day mark counts of the days
/// reading, because every mark stands in exactly one day. No
/// competitor is ranked or compared against another beyond the
/// holder each mark line names.
pub fn meet_marks(meet: &Meet) -> Result<Vec<String>, String> {
    let walk = meet_walk(meet)?;
    let mut sections: Vec<Vec<(usize, &MarkRecord)>> = vec![Vec::new(); meet.days.len()];
    for (index, record) in walk.marks_detail.iter().enumerate() {
        sections[record.day_index].push((index + 1, record));
    }
    let mut lines = vec![format!("meet marks, {} days", meet.days.len())];
    let mut record_marks = 0usize;
    for (day_index, section) in sections.iter().enumerate() {
        lines.push(format!(
            "day, the {} day, marks {}",
            ORDINALS[day_index],
            section.len()
        ));
        for (place, record) in section {
            let phrase = match &record.standing {
                MarkStanding::Set => "set the standing record".to_string(),
                MarkStanding::Took { from, displaced } => format!(
                    "took the record from {}, displacing a projection of {}",
                    walk.labels[*from],
                    format_usage(displaced)
                ),
                MarkStanding::Extended { displaced } => format!(
                    "extended its own record, displacing a projection of {}",
                    format_usage(displaced)
                ),
                MarkStanding::Below { standing_record } => format!(
                    "stood below the standing record of {}",
                    format_usage(standing_record)
                ),
            };
            if !matches!(record.standing, MarkStanding::Below { .. }) {
                record_marks += 1;
            }
            lines.push(format!(
                "mark {}, at {} seconds on the {} day, {} at a projection of {}, {}",
                place,
                format_seconds_fraction(&record.mark),
                ORDINALS[record.day_index],
                walk.labels[record.holder],
                format_usage(&record.record),
                phrase
            ));
        }
    }
    lines.push(format!(
        "marks counts, marks walked {}, record marks {}, days {}",
        walk.marks_walked,
        record_marks,
        meet.days.len()
    ));
    Ok(lines)
}

/// Return the meet standings lines for one loaded meet.
///
/// The standings reading reads the one walk a seventh way.
/// One section stands for each day of the meet in day order,
/// and each section names the standing of every competitor at
/// the close of that day in competitor order, in the close
/// forms of PG-16, a competitor holding a figure standing at
/// it and a competitor holding none standing in the none form,
/// and it closes with the closing line the walk emitted at
/// the close of that day, the line the meet report prints,
/// byte for byte. The reading opens by naming the meet, the
/// count of its days, and the count of its competitors, and
/// closes by counting the days, the competitors, and the
/// standings held, the sum over the sections of the
/// competitors holding a standing at a close. The competitor
/// order never changes across the sections, so a later section
/// reads against an earlier one competitor by competitor, and
/// a competitor's standing at a later close is its figure at
/// that close or the standing it already held, never an
/// invented one, under the discipline PG-26 states. No
/// competitor is ranked or compared against another, and the
/// order is the competitor order of the walk alone.
pub fn meet_standings(meet: &Meet) -> Result<Vec<String>, String> {
    let walk = meet_walk(meet)?;
    let mut lines = vec![format!(
        "meet standings, {} days, {} competitors",
        meet.days.len(),
        walk.labels.len()
    )];
    let mut standings_held = 0usize;
    for (day_index, day) in walk.day_standings.iter().enumerate() {
        let held_here = day.iter().filter(|standing| standing.is_some()).count();
        standings_held += held_here;
        lines.push(format!(
            "day, the {} day, standings {}",
            ORDINALS[day_index], held_here
        ));
        for (competitor, standing) in day.iter().enumerate() {
            match standing {
                Some(figure) => lines.push(format!(
                    "competitor {}, standing projection {} at the close",
                    walk.labels[competitor],
                    format_usage(figure)
                )),
                None => lines.push(format!(
                    "competitor {}, no standing projection at the close",
                    walk.labels[competitor]
                )),
            }
        }
        lines.push(walk.day_close_lines[day_index].clone());
    }
    lines.push(format!(
        "standings counts, days {}, competitors {}, standings held {}",
        meet.days.len(),
        walk.labels.len(),
        standings_held
    ));
    Ok(lines)
}

/// Return the meet traces lines for one loaded meet.
///
/// The traces reading reads the one walk an eighth way, the
/// standings reading transposed. One section stands for each
/// competitor in the competitor order of the walk, opening as
/// the forks sections open, and inside a section one line
/// stands for each day of the meet in day order, naming the
/// standing of that competitor at the close of that day in the
/// close forms of PG-26, every day holding a line in every
/// section. The section closes by counting the closes at which
/// the competitor held a standing and naming the standing it
/// holds at the close of the last day in the same two forms,
/// the figure form standing exactly where that count is not
/// zero, because a standing once set is never cleared. The
/// reading opens by naming the meet, the count of its days,
/// and the count of its competitors, and closes by counting
/// the competitors, the days, and the standings held, the sum
/// over the sections of the closes at which a competitor held
/// a standing, which is the standings held of the standings
/// reading over the same meet. The reading computes no figure
/// the standings reading does not already state. No competitor
/// is ranked or compared against another, and the order is the
/// competitor order of the walk alone.
pub fn meet_traces(meet: &Meet) -> Result<Vec<String>, String> {
    let walk = meet_walk(meet)?;
    let mut lines = vec![format!(
        "meet traces, {} days, {} competitors",
        meet.days.len(),
        walk.labels.len()
    )];
    let mut standings_held = 0usize;
    for (competitor, label) in walk.labels.iter().enumerate() {
        lines.push(format!("competitor {}", label));
        let mut held = 0usize;
        for (day_index, day) in walk.day_standings.iter().enumerate() {
            match &day[competitor] {
                Some(figure) => {
                    held += 1;
                    lines.push(format!(
                        "the {} day, standing projection {} at the close",
                        ORDINALS[day_index],
                        format_usage(figure)
                    ));
                }
                None => lines.push(format!(
                    "the {} day, no standing projection at the close",
                    ORDINALS[day_index]
                )),
            }
        }
        standings_held += held;
        let close_text = match &walk.close[competitor] {
            Some(close) => format!("standing projection {} at the close", format_usage(close)),
            None => "no standing projection at the close".to_string(),
        };
        lines.push(format!(
            "close, standings held at {} closes, {}",
            held, close_text
        ));
    }
    lines.push(format!(
        "traces counts, competitors {}, days {}, standings held {}",
        walk.labels.len(),
        meet.days.len(),
        standings_held
    ));
    Ok(lines)
}

/// Return the meet margins lines for one loaded meet.
///
/// The margins reading reads the one walk a ninth way. One
/// section stands for each day of the meet in meet order,
/// every day holding a section, and inside a section one
/// line stands for each mark of the walk standing in that
/// day, in walk order, naming the place of the mark in the
/// whole walk, the holder after the mark and the record
/// figure after it, in the mark form of the marks reading
/// as far as the record figure, and then the nearest
/// standing after the mark, the lowest standing projection
/// held after it by a competitor other than the holder, a
/// tie at that figure standing with the competitor whose
/// standing was set at the earliest mark and the earlier
/// competitor place standing where their standings were
/// set at one mark, and the margin by which the record
/// stands ahead of that standing, or the words and no
/// other standing where no competitor other than the
/// holder holds a standing after the mark. The margin is
/// computed on exact values before any rendering, so it
/// is never negative, and it stands at zero exactly where
/// another competitor stands at the record figure. The
/// reading closes by counting the marks the walk walked,
/// the marks with another standing, and the days, and
/// naming the closest margin and the earliest mark
/// standing at it, the closest margin taken over the
/// marks and never over the competitors, or by stating
/// that no other standing stood at any mark where none
/// did. No competitor is ranked or compared against
/// another beyond the holder and the nearest standing
/// each mark line names.
pub fn meet_margins(meet: &Meet) -> Result<Vec<String>, String> {
    let walk = meet_walk(meet)?;
    let mut sections: Vec<Vec<(usize, &MarkRecord)>> = vec![Vec::new(); meet.days.len()];
    for (index, record) in walk.marks_detail.iter().enumerate() {
        sections[record.day_index].push((index + 1, record));
    }
    let mut lines = vec![format!("meet margins, {} days", meet.days.len())];
    let mut with_another = 0usize;
    let mut closest: Option<(BigRational, &MarkRecord)> = None;
    for (day_index, section) in sections.iter().enumerate() {
        lines.push(format!(
            "day, the {} day, marks {}",
            ORDINALS[day_index],
            section.len()
        ));
        for (place, record) in section {
            let head = format!(
                "mark {}, at {} seconds on the {} day, {} at a projection of {}",
                place,
                format_seconds_fraction(&record.mark),
                ORDINALS[record.day_index],
                walk.labels[record.holder],
                format_usage(&record.record)
            );
            match &walk.nearest[place - 1] {
                Some((competitor, figure)) => {
                    let margin = figure.clone() - record.record.clone();
                    with_another += 1;
                    let nearer = match &closest {
                        None => true,
                        Some((smallest, _)) => margin < *smallest,
                    };
                    if nearer {
                        closest = Some((margin.clone(), *record));
                    }
                    lines.push(format!(
                        "{}, nearest standing {} held by {}, margin {}",
                        head,
                        format_usage(figure),
                        walk.labels[*competitor],
                        format_usage(&margin)
                    ));
                }
                None => lines.push(format!("{}, and no other standing", head)),
            }
        }
    }
    match closest {
        Some((margin, record)) => lines.push(format!(
            "margins counts, marks walked {}, marks with another standing {}, days {}, closest margin {} at {} seconds on the {} day",
            walk.marks_walked,
            with_another,
            meet.days.len(),
            format_usage(&margin),
            format_seconds_fraction(&record.mark),
            ORDINALS[record.day_index]
        )),
        None => lines.push(format!(
            "margins counts, marks walked {}, marks with another standing {}, days {}, and, no other standing at any mark",
            walk.marks_walked,
            with_another,
            meet.days.len()
        )),
    }
    Ok(lines)
}

/// Return the meet spread lines for one loaded meet.
///
/// The spread reading reads the one walk a tenth way. One
/// section stands for each day of the meet in meet order,
/// every day holding a section, and inside a section one
/// line stands for each mark of the walk standing in that
/// day, in walk order, naming the place of the mark in the
/// whole walk, the holder after the mark and the record
/// figure after it, in the mark form of the marks reading
/// as far as the record figure, and then the field after
/// the mark, the count of competitors holding a standing
/// after it with the holder counted, and the farthest
/// standing after the mark, the highest standing
/// projection held after it by a competitor other than
/// the holder, a tie at that figure standing with the
/// competitor whose standing was set at the earliest
/// mark and the earlier competitor place standing where
/// their standings were set at one mark, and the spread
/// by which that farthest standing stands behind the
/// record, or the words and no other standing where no
/// competitor other than the holder holds a standing
/// after the mark. The spread is computed on exact
/// values before any rendering, so it is never
/// negative, and it stands at zero exactly where every
/// competitor holding a standing stands at the record
/// figure. The reading closes by counting the marks the
/// walk walked, the marks with another standing, and
/// the days, and naming the fullest field and the
/// earliest mark standing at it, the fullest field
/// taken over the marks and never over the
/// competitors, or by stating that no other standing
/// stood at any mark where none did. No competitor is
/// ranked or compared against another beyond the
/// holder and the farthest standing each mark line
/// names.
pub fn meet_spread(meet: &Meet) -> Result<Vec<String>, String> {
    let walk = meet_walk(meet)?;
    let mut sections: Vec<Vec<(usize, &MarkRecord)>> = vec![Vec::new(); meet.days.len()];
    for (index, record) in walk.marks_detail.iter().enumerate() {
        sections[record.day_index].push((index + 1, record));
    }
    let mut lines = vec![format!("meet spread, {} days", meet.days.len())];
    let mut with_another = 0usize;
    let mut fullest: Option<(usize, &MarkRecord)> = None;
    for (day_index, section) in sections.iter().enumerate() {
        lines.push(format!(
            "day, the {} day, marks {}",
            ORDINALS[day_index],
            section.len()
        ));
        for (place, record) in section {
            let field = walk.fields[place - 1];
            let head = format!(
                "mark {}, at {} seconds on the {} day, {} at a projection of {}, field of {}",
                place,
                format_seconds_fraction(&record.mark),
                ORDINALS[record.day_index],
                walk.labels[record.holder],
                format_usage(&record.record),
                field
            );
            match &walk.farthest[place - 1] {
                Some((competitor, figure)) => {
                    let spread = figure.clone() - record.record.clone();
                    with_another += 1;
                    let fuller = match &fullest {
                        None => true,
                        Some((greatest, _)) => field > *greatest,
                    };
                    if fuller {
                        fullest = Some((field, *record));
                    }
                    lines.push(format!(
                        "{}, farthest standing {} held by {}, spread {}",
                        head,
                        format_usage(figure),
                        walk.labels[*competitor],
                        format_usage(&spread)
                    ));
                }
                None => lines.push(format!("{}, and no other standing", head)),
            }
        }
    }
    match fullest {
        Some((field, record)) => lines.push(format!(
            "spread counts, marks walked {}, marks with another standing {}, days {}, fullest field {} at {} seconds on the {} day",
            walk.marks_walked,
            with_another,
            meet.days.len(),
            field,
            format_seconds_fraction(&record.mark),
            ORDINALS[record.day_index]
        )),
        None => lines.push(format!(
            "spread counts, marks walked {}, marks with another standing {}, days {}, and, no other standing at any mark",
            walk.marks_walked,
            with_another,
            meet.days.len()
        )),
    }
    Ok(lines)
}

/// Return the meet middle lines for one loaded meet.
///
/// The middle reading reads the one walk an eleventh way.
/// One section stands for each day of the meet in meet
/// order, every day holding a section, and inside a
/// section one line stands for each mark of the walk
/// standing in that day, in walk order, naming the place
/// of the mark in the whole walk, the holder after the
/// mark and the record figure after it, in the mark form
/// of the marks reading as far as the record figure, and
/// then the middle standing after the mark, the standing
/// at the middle figure of the field behind the holder,
/// the figure at the middle place (m + 1) div 2 in the
/// ascending order of the standings held after the mark
/// by competitors other than the holder, a tie at that
/// figure standing with the competitor whose standing
/// was set at the earliest mark and the earlier
/// competitor place standing where their standings were
/// set at one mark, and the middle margin by which the
/// record stands ahead of that standing, or the words
/// and no other standing where no competitor other than
/// the holder holds a standing after the mark. The
/// middle margin is computed on exact values before any
/// rendering, so it is never negative, and it stands at
/// zero exactly where the middle standing stands at the
/// record figure. The reading closes by counting the
/// marks the walk walked, the marks with another
/// standing, and the days, and naming the closest middle
/// margin and the earliest mark standing at it, the
/// closest middle margin taken over the marks and never
/// over the competitors, or by stating that no other
/// standing stood at any mark where none did. No
/// competitor is ranked or compared against another
/// beyond the holder and the middle standing each mark
/// line names.
pub fn meet_middle(meet: &Meet) -> Result<Vec<String>, String> {
    let walk = meet_walk(meet)?;
    let mut sections: Vec<Vec<(usize, &MarkRecord)>> = vec![Vec::new(); meet.days.len()];
    for (index, record) in walk.marks_detail.iter().enumerate() {
        sections[record.day_index].push((index + 1, record));
    }
    let mut lines = vec![format!("meet middle, {} days", meet.days.len())];
    let mut with_another = 0usize;
    let mut closest: Option<(BigRational, &MarkRecord)> = None;
    for (day_index, section) in sections.iter().enumerate() {
        lines.push(format!(
            "day, the {} day, marks {}",
            ORDINALS[day_index],
            section.len()
        ));
        for (place, record) in section {
            let head = format!(
                "mark {}, at {} seconds on the {} day, {} at a projection of {}",
                place,
                format_seconds_fraction(&record.mark),
                ORDINALS[record.day_index],
                walk.labels[record.holder],
                format_usage(&record.record)
            );
            match &walk.middle[place - 1] {
                Some((competitor, figure)) => {
                    let margin = figure.clone() - record.record.clone();
                    with_another += 1;
                    let nearer = match &closest {
                        None => true,
                        Some((smallest, _)) => margin < *smallest,
                    };
                    if nearer {
                        closest = Some((margin.clone(), *record));
                    }
                    lines.push(format!(
                        "{}, middle standing {} held by {}, middle margin {}",
                        head,
                        format_usage(figure),
                        walk.labels[*competitor],
                        format_usage(&margin)
                    ));
                }
                None => lines.push(format!("{}, and no other standing", head)),
            }
        }
    }
    match closest {
        Some((margin, record)) => lines.push(format!(
            "middle counts, marks walked {}, marks with another standing {}, days {}, closest middle margin {} at {} seconds on the {} day",
            walk.marks_walked,
            with_another,
            meet.days.len(),
            format_usage(&margin),
            format_seconds_fraction(&record.mark),
            ORDINALS[record.day_index]
        )),
        None => lines.push(format!(
            "middle counts, marks walked {}, marks with another standing {}, days {}, and, no other standing at any mark",
            walk.marks_walked,
            with_another,
            meet.days.len()
        )),
    }
    Ok(lines)
}

/// Return the meet tails lines for one loaded meet.
///
/// The tails reading reads the one walk a twelfth way.
/// One section stands for each day of the meet in meet
/// order, every day holding a section, and inside a
/// section one line stands for each mark of the walk
/// standing in that day, in walk order, naming the place
/// of the mark in the whole walk, the holder after the
/// mark and the record figure after it, in the mark form
/// of the marks reading as far as the record figure, and
/// then the lower tail standing after the mark, the mean
/// of the lowest (m + 1) div 2 figures of the field
/// behind the holder, where m counts the competitors of
/// that field, and the lower tail margin by which the
/// record stands ahead of that standing, and the upper
/// tail standing after the mark, the mean of the highest
/// (m + 1) div 2 figures of that field, and the upper
/// tail margin by which the record stands ahead of that
/// standing, or the words and no other standing where no
/// competitor other than the holder holds a standing
/// after the mark. A tail standing is a mean of figures
/// and is held by no competitor, so no mark line names a
/// holder for either tail. The tail margins are computed
/// on exact values before any rendering, so neither is
/// ever negative, and each stands at zero exactly where
/// every figure of its half of the field stands at the
/// record figure. The reading closes by counting the
/// marks the walk walked, the marks with another
/// standing, and the days, and naming the closest lower
/// tail margin and the earliest mark standing at it and
/// the closest upper tail margin and the earliest mark
/// standing at it, the closest tail margins taken over
/// the marks and never over the competitors, or by
/// stating that no other standing stood at any mark
/// where none did. No competitor is ranked or compared
/// against another beyond the holder each mark line
/// names.
pub fn meet_tails(meet: &Meet) -> Result<Vec<String>, String> {
    let walk = meet_walk(meet)?;
    let mut sections: Vec<Vec<(usize, &MarkRecord)>> = vec![Vec::new(); meet.days.len()];
    for (index, record) in walk.marks_detail.iter().enumerate() {
        sections[record.day_index].push((index + 1, record));
    }
    let mut lines = vec![format!("meet tails, {} days", meet.days.len())];
    let mut with_another = 0usize;
    let mut closest_lower: Option<(BigRational, &MarkRecord)> = None;
    let mut closest_upper: Option<(BigRational, &MarkRecord)> = None;
    for (day_index, section) in sections.iter().enumerate() {
        lines.push(format!(
            "day, the {} day, marks {}",
            ORDINALS[day_index],
            section.len()
        ));
        for (place, record) in section {
            let head = format!(
                "mark {}, at {} seconds on the {} day, {} at a projection of {}",
                place,
                format_seconds_fraction(&record.mark),
                ORDINALS[record.day_index],
                walk.labels[record.holder],
                format_usage(&record.record)
            );
            match &walk.tails[place - 1] {
                Some((lower, upper)) => {
                    let lower_margin = lower.clone() - record.record.clone();
                    let upper_margin = upper.clone() - record.record.clone();
                    with_another += 1;
                    let nearer_lower = match &closest_lower {
                        None => true,
                        Some((smallest, _)) => lower_margin < *smallest,
                    };
                    if nearer_lower {
                        closest_lower = Some((lower_margin.clone(), *record));
                    }
                    let nearer_upper = match &closest_upper {
                        None => true,
                        Some((smallest, _)) => upper_margin < *smallest,
                    };
                    if nearer_upper {
                        closest_upper = Some((upper_margin.clone(), *record));
                    }
                    lines.push(format!(
                        "{}, lower tail standing {}, lower tail margin {}, upper tail standing {}, upper tail margin {}",
                        head,
                        format_usage(lower),
                        format_usage(&lower_margin),
                        format_usage(upper),
                        format_usage(&upper_margin)
                    ));
                }
                None => lines.push(format!("{}, and no other standing", head)),
            }
        }
    }
    match (closest_lower, closest_upper) {
        (Some((lower_margin, lower_record)), Some((upper_margin, upper_record))) => {
            lines.push(format!(
                "tails counts, marks walked {}, marks with another standing {}, days {}, closest lower tail margin {} at {} seconds on the {} day, closest upper tail margin {} at {} seconds on the {} day",
                walk.marks_walked,
                with_another,
                meet.days.len(),
                format_usage(&lower_margin),
                format_seconds_fraction(&lower_record.mark),
                ORDINALS[lower_record.day_index],
                format_usage(&upper_margin),
                format_seconds_fraction(&upper_record.mark),
                ORDINALS[upper_record.day_index]
            ))
        }
        _ => lines.push(format!(
            "tails counts, marks walked {}, marks with another standing {}, days {}, and, no other standing at any mark",
            walk.marks_walked,
            with_another,
            meet.days.len()
        )),
    }
    Ok(lines)
}

/// Return the meet mean lines for one loaded meet.
///
/// The mean reading reads the one walk a thirteenth way.
/// One section stands for each day of the meet in meet
/// order, every day holding a section, and inside a
/// section one line stands for each mark of the walk
/// standing in that day, in walk order, naming the place
/// of the mark in the whole walk, the holder after the
/// mark and the record figure after it, in the mark form
/// of the marks reading as far as the record figure, and
/// then the mean standing after the mark, the mean of the
/// whole field behind the holder, and the mean margin by
/// which the record stands ahead of that standing, or the
/// words and no other standing where no competitor other
/// than the holder holds a standing after the mark. A
/// mean standing is a mean of figures and is held by no
/// competitor, so no mark line names a holder for the
/// mean. The mean margin is computed on exact values
/// before any rendering, so it is never negative, and it
/// stands at zero exactly where every figure of the field
/// stands at the record figure. The reading closes by
/// counting the marks the walk walked, the marks with
/// another standing, and the days, and naming the closest
/// mean margin and the earliest mark standing at it, the
/// closest mean margin taken over the marks and never
/// over the competitors, or by stating that no other
/// standing stood at any mark where none did. No
/// competitor is ranked or compared against another
/// beyond the holder each mark line names.
pub fn meet_mean(meet: &Meet) -> Result<Vec<String>, String> {
    let walk = meet_walk(meet)?;
    let mut sections: Vec<Vec<(usize, &MarkRecord)>> = vec![Vec::new(); meet.days.len()];
    for (index, record) in walk.marks_detail.iter().enumerate() {
        sections[record.day_index].push((index + 1, record));
    }
    let mut lines = vec![format!("meet mean, {} days", meet.days.len())];
    let mut with_another = 0usize;
    let mut closest: Option<(BigRational, &MarkRecord)> = None;
    for (day_index, section) in sections.iter().enumerate() {
        lines.push(format!(
            "day, the {} day, marks {}",
            ORDINALS[day_index],
            section.len()
        ));
        for (place, record) in section {
            let head = format!(
                "mark {}, at {} seconds on the {} day, {} at a projection of {}",
                place,
                format_seconds_fraction(&record.mark),
                ORDINALS[record.day_index],
                walk.labels[record.holder],
                format_usage(&record.record)
            );
            match &walk.mean[place - 1] {
                Some(figure) => {
                    let margin = figure.clone() - record.record.clone();
                    with_another += 1;
                    let nearer = match &closest {
                        None => true,
                        Some((smallest, _)) => margin < *smallest,
                    };
                    if nearer {
                        closest = Some((margin.clone(), *record));
                    }
                    lines.push(format!(
                        "{}, mean standing {}, mean margin {}",
                        head,
                        format_usage(figure),
                        format_usage(&margin)
                    ));
                }
                None => lines.push(format!("{}, and no other standing", head)),
            }
        }
    }
    match closest {
        Some((margin, record)) => lines.push(format!(
            "mean counts, marks walked {}, marks with another standing {}, days {}, closest mean margin {} at {} seconds on the {} day",
            walk.marks_walked,
            with_another,
            meet.days.len(),
            format_usage(&margin),
            format_seconds_fraction(&record.mark),
            ORDINALS[record.day_index]
        )),
        None => lines.push(format!(
            "mean counts, marks walked {}, marks with another standing {}, days {}, and, no other standing at any mark",
            walk.marks_walked,
            with_another,
            meet.days.len()
        )),
    }
    Ok(lines)
}

/// Return the meet median lines for one loaded meet.
///
/// The median reading reads the one walk a fourteenth
/// way. One section stands for each day of the meet in
/// meet order, every day holding a section, and inside
/// a section one line stands for each mark of the walk
/// standing in that day, in walk order, naming the
/// place of the mark in the whole walk, the holder
/// after the mark and the record figure after it, in
/// the mark form of the marks reading as far as the
/// record figure, and then the median standing after
/// the mark, the median of the field behind the
/// holder, and the median margin by which the record
/// stands ahead of that standing, or the words and no
/// other standing where no competitor other than the
/// holder holds a standing after the mark. A median
/// standing is a statistic of the figures of the field
/// and is held by no competitor, so no mark line names
/// a holder for the median. The median margin is
/// computed on exact values before any rendering, so
/// it is never negative, and it stands at zero exactly
/// where every figure standing at a middle place of
/// the field stands at the record figure. The reading
/// closes by counting the marks the walk walked, the
/// marks with another standing, and the days, and
/// naming the closest median margin and the earliest
/// mark standing at it, the closest median margin
/// taken over the marks and never over the competitors,
/// or by stating that no other standing stood at any
/// mark where none did. No competitor is ranked or
/// compared against another beyond the holder each
/// mark line names.
pub fn meet_median(meet: &Meet) -> Result<Vec<String>, String> {
    let walk = meet_walk(meet)?;
    let mut sections: Vec<Vec<(usize, &MarkRecord)>> = vec![Vec::new(); meet.days.len()];
    for (index, record) in walk.marks_detail.iter().enumerate() {
        sections[record.day_index].push((index + 1, record));
    }
    let mut lines = vec![format!("meet median, {} days", meet.days.len())];
    let mut with_another = 0usize;
    let mut closest: Option<(BigRational, &MarkRecord)> = None;
    for (day_index, section) in sections.iter().enumerate() {
        lines.push(format!(
            "day, the {} day, marks {}",
            ORDINALS[day_index],
            section.len()
        ));
        for (place, record) in section {
            let head = format!(
                "mark {}, at {} seconds on the {} day, {} at a projection of {}",
                place,
                format_seconds_fraction(&record.mark),
                ORDINALS[record.day_index],
                walk.labels[record.holder],
                format_usage(&record.record)
            );
            match &walk.median[place - 1] {
                Some(figure) => {
                    let margin = figure.clone() - record.record.clone();
                    with_another += 1;
                    let nearer = match &closest {
                        None => true,
                        Some((smallest, _)) => margin < *smallest,
                    };
                    if nearer {
                        closest = Some((margin.clone(), *record));
                    }
                    lines.push(format!(
                        "{}, median standing {}, median margin {}",
                        head,
                        format_usage(figure),
                        format_usage(&margin)
                    ));
                }
                None => lines.push(format!("{}, and no other standing", head)),
            }
        }
    }
    match closest {
        Some((margin, record)) => lines.push(format!(
            "median counts, marks walked {}, marks with another standing {}, days {}, closest median margin {} at {} seconds on the {} day",
            walk.marks_walked,
            with_another,
            meet.days.len(),
            format_usage(&margin),
            format_seconds_fraction(&record.mark),
            ORDINALS[record.day_index]
        )),
        None => lines.push(format!(
            "median counts, marks walked {}, marks with another standing {}, days {}, and, no other standing at any mark",
            walk.marks_walked,
            with_another,
            meet.days.len()
        )),
    }
    Ok(lines)
}

/// Return the meet quartiles lines for one loaded meet.
///
/// The quartiles reading reads the one walk a fifteenth
/// way. One section stands for each day of the meet in
/// meet order, every day holding a section, and inside
/// a section one line stands for each mark of the walk
/// standing in that day, in walk order, naming the
/// place of the mark in the whole walk, the holder
/// after the mark and the record figure after it, in
/// the mark form of the marks reading as far as the
/// record figure, and then the lower quartile standing
/// after the mark, the median of the lowest figures of
/// the field behind the holder, and the lower quartile
/// margin by which the record stands ahead of that
/// standing, and the upper quartile standing after the
/// mark, the median of the highest figures of that
/// field, and the upper quartile margin by which the
/// record stands ahead of that standing, or the words
/// and no other standing where no competitor other than
/// the holder holds a standing after the mark. A
/// quartile standing is a statistic of the figures of
/// a sub-field and is held by no competitor, so no mark
/// line names a holder for either quartile. Each
/// quartile margin is computed on exact values before
/// any rendering, so it is never negative, and it
/// stands at zero exactly where every figure standing
/// at a middle place of its sub-field stands at the
/// record figure. The reading closes by counting the
/// marks the walk walked, the marks with another
/// standing, and the days, and naming the closest
/// lower quartile margin and the closest upper
/// quartile margin and the earliest mark standing at
/// each, the closest margins taken over the marks and
/// never over the competitors, or by stating that no
/// other standing stood at any mark where none did. No
/// competitor is ranked or compared against another
/// beyond the holder each mark line names.
pub fn meet_quartiles(meet: &Meet) -> Result<Vec<String>, String> {
    let walk = meet_walk(meet)?;
    let mut sections: Vec<Vec<(usize, &MarkRecord)>> = vec![Vec::new(); meet.days.len()];
    for (index, record) in walk.marks_detail.iter().enumerate() {
        sections[record.day_index].push((index + 1, record));
    }
    let mut lines = vec![format!("meet quartiles, {} days", meet.days.len())];
    let mut with_another = 0usize;
    let mut closest_lower: Option<(BigRational, &MarkRecord)> = None;
    let mut closest_upper: Option<(BigRational, &MarkRecord)> = None;
    for (day_index, section) in sections.iter().enumerate() {
        lines.push(format!(
            "day, the {} day, marks {}",
            ORDINALS[day_index],
            section.len()
        ));
        for (place, record) in section {
            let head = format!(
                "mark {}, at {} seconds on the {} day, {} at a projection of {}",
                place,
                format_seconds_fraction(&record.mark),
                ORDINALS[record.day_index],
                walk.labels[record.holder],
                format_usage(&record.record)
            );
            match &walk.quartiles[place - 1] {
                Some((lower, upper)) => {
                    let lower_margin = lower.clone() - record.record.clone();
                    let upper_margin = upper.clone() - record.record.clone();
                    with_another += 1;
                    let nearer_lower = match &closest_lower {
                        None => true,
                        Some((smallest, _)) => lower_margin < *smallest,
                    };
                    if nearer_lower {
                        closest_lower = Some((lower_margin.clone(), *record));
                    }
                    let nearer_upper = match &closest_upper {
                        None => true,
                        Some((smallest, _)) => upper_margin < *smallest,
                    };
                    if nearer_upper {
                        closest_upper = Some((upper_margin.clone(), *record));
                    }
                    lines.push(format!(
                        "{}, lower quartile standing {}, lower quartile margin {}, upper quartile standing {}, upper quartile margin {}",
                        head,
                        format_usage(lower),
                        format_usage(&lower_margin),
                        format_usage(upper),
                        format_usage(&upper_margin)
                    ));
                }
                None => lines.push(format!("{}, and no other standing", head)),
            }
        }
    }
    match (closest_lower, closest_upper) {
        (Some((lower_margin, lower_record)), Some((upper_margin, upper_record))) => {
            lines.push(format!(
                "quartiles counts, marks walked {}, marks with another standing {}, days {}, closest lower quartile margin {} at {} seconds on the {} day, closest upper quartile margin {} at {} seconds on the {} day",
                walk.marks_walked,
                with_another,
                meet.days.len(),
                format_usage(&lower_margin),
                format_seconds_fraction(&lower_record.mark),
                ORDINALS[lower_record.day_index],
                format_usage(&upper_margin),
                format_seconds_fraction(&upper_record.mark),
                ORDINALS[upper_record.day_index]
            ))
        }
        _ => lines.push(format!(
            "quartiles counts, marks walked {}, marks with another standing {}, days {}, and, no other standing at any mark",
            walk.marks_walked,
            with_another,
            meet.days.len()
        )),
    }
    Ok(lines)
}
