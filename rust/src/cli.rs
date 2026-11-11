//! The paceghost command line surface.
//!
//! Twenty one commands read recorded runs from local files and
//! print reports. The show command prints one run or its
//! defects, the
//! ghost command prints the standing ghost for a task, and the race
//! command prints a live run raced against a ghost. The season
//! command prints the season report for a field of runs against a
//! ghost, and the board command prints the live board for a run
//! carrying timed reads. The fieldday command prints the stolen
//! record report for a field day of runs inside one window. The
//! ladder command prints the ladder report for a field day read
//! through a list of windows. The rematch command prints the
//! rematch report for two field days read as one progression. The
//! meet command prints the meet report for three or more field
//! days read as one progression. The table command prints the
//! meet table for a meet, the same walk read competitor by
//! competitor. The reign command prints the meet reign for a
//! meet, the same walk read spell by spell. The days command
//! prints the meet days for a meet, the same walk read day by
//! day. The forks command prints the meet forks for a meet,
//! the same walk read as one section standing for each
//! competitor. The marks command prints the meet marks for a
//! meet, the same walk read mark by mark. The standings command
//! prints the meet standings for a meet, the same walk read at
//! the close of each day. The traces command prints the meet
//! traces for a meet, the same walk read competitor by
//! competitor at the close of each day. The margins command
//! prints the meet margins for a meet, the same walk read
//! mark by mark with the field behind each mark. The spread
//! command prints the meet spread for a meet, the same walk
//! read mark by mark with the farthest standing and the
//! field behind each mark. The middle
//! command prints the meet middle for a meet, the same walk
//! read mark by mark with the middle standing behind each
//! mark. The tails
//! command prints the meet tails for a meet, the same walk
//! read mark by mark with the lower tail standing and the
//! upper tail standing behind each mark. The mean
//! command prints the meet mean for a meet, the same walk
//! read mark by mark with the mean standing behind each
//! mark. Nothing here touches the network.
//!
//! The dispatch is a pure function of the argument list and a file
//! reading function, returning the lines each stream carries and
//! the exit status, so the binary in main.rs is a thin shell doing
//! file IO and printing. The argument reading mirrors the Python
//! argparse surface of the Python cli module, exit status two and
//! the help texts included, as probed against that surface. The
//! help and usage layout follows the width argparse reads, the
//! COLUMNS setting when it holds a positive integer, else the
//! terminal width of the output stream, else 80. The arguments
//! arrive as the code points the operating system bytes decode to
//! under the Python surrogateescape reading, so a file name
//! carrying a raw byte the UTF-8 reading cannot place still opens,
//! and an argument echoed into an error line carries the same
//! units the Python stderr carries. The behavior mirrors the
//! Python module, which stands as the spec under PORT-1.

use crate::fieldday::{field_day_report, load_field_day_text, FieldDay};
use crate::ladder::{ladder_report, load_ladder_text, Ladder};
use crate::meet::{
    load_meet_text, meet_days, meet_forks, meet_margins, meet_marks, meet_mean, meet_middle,
    meet_reign, meet_report, meet_spread, meet_standings, meet_table, meet_tails, meet_traces,
    Meet,
};
use crate::pynum::format_seconds;
use crate::pytext::PText;
use crate::rematch::{load_rematch_text, rematch_report, Rematch};
use crate::runs::{check_runner_label, load_run_text, race_report, select_ghost, window_line, Run};
use crate::season::{board_report, load_field_text, season_report};

/// What a command line run prints and how it ends.
pub struct CliOutcome {
    pub stdout: Vec<String>,
    pub stderr: Vec<String>,
    pub code: i32,
}

impl CliOutcome {
    fn new() -> CliOutcome {
        CliOutcome {
            stdout: Vec::new(),
            stderr: Vec::new(),
            code: 0,
        }
    }
}

const COMMANDS: [&str; 21] = [
    "show",
    "ghost",
    "race",
    "season",
    "board",
    "fieldday",
    "ladder",
    "rematch",
    "meet",
    "table",
    "reign",
    "days",
    "forks",
    "marks",
    "standings",
    "traces",
    "margins",
    "spread",
    "middle",
    "tails",
    "mean",
];

const COMMAND_HELPS: [(&str, &str); 21] = [
    ("show", "print one recorded run"),
    ("ghost", "print the standing ghost"),
    ("race", "race a live run against a ghost"),
    ("season", "print the season report for a field of runs"),
    (
        "board",
        "print the live board for a run carrying timed reads",
    ),
    ("fieldday", "print the stolen record report for a field day"),
    (
        "ladder",
        "print the ladder report for a field day and its windows",
    ),
    ("rematch", "print the rematch report for two field days"),
    ("meet", "print the meet report for three or more field days"),
    ("table", "print the meet table for three or more field days"),
    ("reign", "print the meet reign for three or more field days"),
    ("days", "print the meet days for three or more field days"),
    ("forks", "print the meet forks for three or more field days"),
    ("marks", "print the meet marks for three or more field days"),
    (
        "standings",
        "print the meet standings for three or more field days",
    ),
    (
        "traces",
        "print the meet traces for three or more field days",
    ),
    (
        "margins",
        "print the meet margins for three or more field days",
    ),
    (
        "spread",
        "print the meet spread for three or more field days",
    ),
    (
        "middle",
        "print the meet middle for three or more field days",
    ),
    ("tails", "print the meet tails for three or more field days"),
    ("mean", "print the meet mean for three or more field days"),
];

const CHOICES_HEADER: &str =
    "{show,ghost,race,season,board,fieldday,ladder,rematch,meet,table,reign,days,forks,marks,standings,traces,margins,spread,middle,tails,mean}";

/// The static shape of one subcommand.
struct Spec {
    name: &'static str,
    positionals: &'static [&'static str],
    variadic: bool,
}

fn spec_for(command: &str) -> Spec {
    match command {
        "show" => Spec {
            name: "show",
            positionals: &["file"],
            variadic: false,
        },
        "ghost" => Spec {
            name: "ghost",
            positionals: &["files"],
            variadic: true,
        },
        "race" => Spec {
            name: "race",
            positionals: &["ghost_file", "live_file"],
            variadic: false,
        },
        "season" => Spec {
            name: "season",
            positionals: &["field_file", "ghost_file"],
            variadic: false,
        },
        "board" => Spec {
            name: "board",
            positionals: &["file", "ghost_file"],
            variadic: false,
        },
        "fieldday" => Spec {
            name: "fieldday",
            positionals: &["file"],
            variadic: false,
        },
        "rematch" => Spec {
            name: "rematch",
            positionals: &["file"],
            variadic: false,
        },
        "meet" => Spec {
            name: "meet",
            positionals: &["file"],
            variadic: false,
        },
        "table" => Spec {
            name: "table",
            positionals: &["file"],
            variadic: false,
        },
        "reign" => Spec {
            name: "reign",
            positionals: &["file"],
            variadic: false,
        },
        "days" => Spec {
            name: "days",
            positionals: &["file"],
            variadic: false,
        },
        "forks" => Spec {
            name: "forks",
            positionals: &["file"],
            variadic: false,
        },
        "marks" => Spec {
            name: "marks",
            positionals: &["file"],
            variadic: false,
        },
        "standings" => Spec {
            name: "standings",
            positionals: &["file"],
            variadic: false,
        },
        "traces" => Spec {
            name: "traces",
            positionals: &["file"],
            variadic: false,
        },
        "margins" => Spec {
            name: "margins",
            positionals: &["file"],
            variadic: false,
        },
        "spread" => Spec {
            name: "spread",
            positionals: &["file"],
            variadic: false,
        },
        "middle" => Spec {
            name: "middle",
            positionals: &["file"],
            variadic: false,
        },
        "tails" => Spec {
            name: "tails",
            positionals: &["file"],
            variadic: false,
        },
        "mean" => Spec {
            name: "mean",
            positionals: &["file"],
            variadic: false,
        },
        _ => Spec {
            name: "ladder",
            positionals: &["file"],
            variadic: false,
        },
    }
}

/// One argument of a parser as the help formatter reads it.
struct ActionInfo {
    invocation: String,
    help: Option<&'static str>,
    subactions: Vec<ActionInfo>,
    usage_part: String,
}

/// One parser as the help formatter reads it.
struct ParserInfo {
    prog: &'static str,
    description: Option<&'static str>,
    positionals: Vec<ActionInfo>,
    optionals: Vec<ActionInfo>,
    action_max_length: i64,
}

fn action_max_length(groups: &[&Vec<ActionInfo>]) -> i64 {
    let mut longest = 0i64;
    for group in groups {
        for action in group.iter() {
            longest = longest.max(action.invocation.chars().count() as i64 + 2);
            for sub in &action.subactions {
                longest = longest.max(sub.invocation.chars().count() as i64 + 4);
            }
        }
    }
    longest
}

fn help_action() -> ActionInfo {
    ActionInfo {
        invocation: "-h, --help".to_string(),
        help: Some("show this help message and exit"),
        subactions: Vec::new(),
        usage_part: "[-h]".to_string(),
    }
}

fn top_parser_info() -> ParserInfo {
    let subactions = COMMAND_HELPS
        .iter()
        .map(|(name, help)| ActionInfo {
            invocation: name.to_string(),
            help: Some(help),
            subactions: Vec::new(),
            usage_part: String::new(),
        })
        .collect();
    let choices = ActionInfo {
        invocation: CHOICES_HEADER.to_string(),
        help: None,
        subactions,
        usage_part: format!("{} ...", CHOICES_HEADER),
    };
    let positionals = vec![choices];
    let optionals = vec![help_action()];
    let action_max_length = action_max_length(&[&positionals, &optionals]);
    ParserInfo {
        prog: "paceghost",
        description: Some("Race a recorded run against the ghost of your best run."),
        positionals,
        optionals,
        action_max_length,
    }
}

fn sub_parser_info(spec: &Spec) -> ParserInfo {
    let positionals = spec
        .positionals
        .iter()
        .map(|name| ActionInfo {
            invocation: name.to_string(),
            help: None,
            subactions: Vec::new(),
            usage_part: if spec.variadic {
                format!("{} [{} ...]", name, name)
            } else {
                name.to_string()
            },
        })
        .collect::<Vec<_>>();
    let optionals = vec![help_action()];
    let action_max_length = action_max_length(&[&positionals, &optionals]);
    ParserInfo {
        prog: match spec.name {
            "show" => "paceghost show",
            "ghost" => "paceghost ghost",
            "race" => "paceghost race",
            "season" => "paceghost season",
            "board" => "paceghost board",
            "fieldday" => "paceghost fieldday",
            "rematch" => "paceghost rematch",
            "meet" => "paceghost meet",
            "table" => "paceghost table",
            "reign" => "paceghost reign",
            "days" => "paceghost days",
            "forks" => "paceghost forks",
            "marks" => "paceghost marks",
            "standings" => "paceghost standings",
            "traces" => "paceghost traces",
            "margins" => "paceghost margins",
            "spread" => "paceghost spread",
            "middle" => "paceghost middle",
            "tails" => "paceghost tails",
            "mean" => "paceghost mean",
            _ => "paceghost ladder",
        },
        description: None,
        positionals,
        optionals,
        action_max_length,
    }
}

/// The help formatter, mirroring the argparse formatter of the
/// Python surface at one width.
struct Formatter {
    /// The argparse width, the columns less two.
    width: i64,
}

impl Formatter {
    fn max_help_position(&self) -> i64 {
        (self.width - 20).clamp(4, 24)
    }

    fn help_position(&self, info: &ParserInfo) -> i64 {
        (info.action_max_length + 2).min(self.max_help_position())
    }

    /// The usage text of one parser, post processed the way the
    /// Python formatter post processes its whole help text.
    fn format_usage_text(&self, info: &ParserInfo) -> String {
        finish_help_text(&self.format_usage_raw(info))
    }

    /// The full help text of one parser.
    fn format_help(&self, info: &ParserInfo) -> String {
        let mut text = self.format_usage_raw(info);
        if let Some(description) = info.description {
            text.push_str(&fill_text(description, (self.width).max(11)));
            text.push_str("\n\n");
        }
        for (heading, actions) in [
            ("positional arguments", &info.positionals),
            ("options", &info.optionals),
        ] {
            let mut items = String::new();
            for action in actions {
                items.push_str(&self.format_action(action, 2, info));
            }
            if !items.is_empty() {
                text.push('\n');
                text.push_str(heading);
                text.push_str(":\n");
                text.push_str(&items);
                text.push('\n');
            }
        }
        finish_help_text(&text)
    }

    /// The raw usage text, before the shared post processing.
    fn format_usage_raw(&self, info: &ParserInfo) -> String {
        let prefix = "usage: ";
        let opt_usage = actions_usage(&usage_parts(&info.optionals));
        let pos_usage = actions_usage(&usage_parts(&info.positionals));
        let mut all_parts = usage_parts(&info.optionals);
        all_parts.extend(usage_parts(&info.positionals));
        let action_usage = actions_usage(&all_parts);
        let usage = if action_usage.is_empty() {
            info.prog.to_string()
        } else {
            format!("{} {}", info.prog, action_usage)
        };
        let text_width = self.width;
        if prefix.chars().count() as i64 + usage.chars().count() as i64 <= text_width {
            return format!("{}{}\n\n", prefix, usage);
        }
        let opt_parts = split_usage_parts(&opt_usage);
        let pos_parts = split_usage_parts(&pos_usage);
        let wrapped = if (prefix.chars().count() + info.prog.chars().count()) as f64
            <= 0.75 * text_width as f64
        {
            let indent = " ".repeat(prefix.len() + info.prog.len() + 1);
            let lines: Vec<String> = if !opt_parts.is_empty() {
                let mut first = vec![info.prog.to_string()];
                first.extend(opt_parts.iter().cloned());
                let mut lines = get_lines(&first, &indent, Some(prefix), text_width);
                lines.extend(get_lines(&pos_parts, &indent, None, text_width));
                lines
            } else if !pos_parts.is_empty() {
                let mut first = vec![info.prog.to_string()];
                first.extend(pos_parts.iter().cloned());
                get_lines(&first, &indent, Some(prefix), text_width)
            } else {
                vec![info.prog.to_string()]
            };
            lines.join("\n")
        } else {
            let indent = " ".repeat(prefix.len());
            let mut parts = opt_parts.clone();
            parts.extend(pos_parts.iter().cloned());
            let mut lines = get_lines(&parts, &indent, None, text_width);
            if lines.len() > 1 {
                lines = get_lines(&opt_parts, &indent, None, text_width);
                lines.extend(get_lines(&pos_parts, &indent, None, text_width));
            }
            format!("{}\n{}", info.prog, lines.join("\n"))
        };
        format!("{}{}\n\n", prefix, wrapped)
    }

    /// One action and its subactions, formatted for a help section.
    fn format_action(&self, action: &ActionInfo, current_indent: i64, info: &ParserInfo) -> String {
        let help_position = self.help_position(info);
        let indent = " ".repeat(current_indent as usize);
        let header = &action.invocation;
        let mut action_header = if action.help.is_none() {
            format!("{}{}\n", indent, header)
        } else if header.chars().count() as i64 <= help_position - current_indent - 2 {
            let action_width = help_position - current_indent - 2;
            format!(
                "{}{:<width$}  ",
                indent,
                header,
                width = action_width as usize
            )
        } else {
            format!("{}{}\n", indent, header)
        };
        match action.help {
            Some(help) if !help.trim().is_empty() => {
                let help_width = (self.width - help_position).max(11);
                let help_lines = wrap_lines(help, help_width);
                let indent_first = if action_header.ends_with('\n') {
                    help_position
                } else {
                    0
                };
                for (index, line) in help_lines.iter().enumerate() {
                    let line_indent = if index == 0 {
                        indent_first
                    } else {
                        help_position
                    };
                    action_header.push_str(&" ".repeat(line_indent as usize));
                    action_header.push_str(line);
                    action_header.push('\n');
                }
            }
            _ => {
                if !action_header.ends_with('\n') {
                    action_header.push('\n');
                }
            }
        }
        for sub in &action.subactions {
            action_header.push_str(&self.format_action(sub, current_indent + 2, info));
        }
        action_header
    }
}

fn usage_parts(actions: &[ActionInfo]) -> Vec<String> {
    actions
        .iter()
        .map(|action| action.usage_part.clone())
        .collect()
}

/// Join the usage parts of a set of actions the way argparse joins
/// them, with its bracket spacing cleanups.
fn actions_usage(parts: &[String]) -> String {
    let mut text = parts.join(" ");
    text = text.replace("[ ", "[").replace("( ", "(");
    text = text.replace(" ]", "]").replace(" )", ")");
    let mut cleaned = String::new();
    let chars: Vec<char> = text.chars().collect();
    let mut index = 0usize;
    while index < chars.len() {
        let c = chars[index];
        if c == '[' || c == '(' {
            let mut look = index + 1;
            while look < chars.len() && chars[look] == ' ' {
                look += 1;
            }
            if look < chars.len() && (chars[look] == ']' || chars[look] == ')') {
                index = look + 1;
                continue;
            }
        }
        cleaned.push(c);
        index += 1;
    }
    cleaned.trim().to_string()
}

/// Split a usage string into the parts the argparse wrapping reads,
/// bracketed runs standing as one part each.
fn split_usage_parts(text: &str) -> Vec<String> {
    let chars: Vec<char> = text.chars().collect();
    let mut parts = Vec::new();
    let mut index = 0usize;
    while index < chars.len() {
        if chars[index].is_whitespace() {
            index += 1;
            continue;
        }
        if chars[index] == '(' || chars[index] == '[' {
            let open = chars[index];
            let close = if open == '(' { ')' } else { ']' };
            let mut end = index;
            let mut matched = false;
            loop {
                if end >= chars.len() || chars[end] != open {
                    break;
                }
                match chars[end + 1..].iter().position(|&c| c == close) {
                    Some(relative) => {
                        end = end + 1 + relative + 1;
                        if end < chars.len() && chars[end] == open {
                            continue;
                        }
                        matched = true;
                        break;
                    }
                    None => break,
                }
            }
            if matched && (end >= chars.len() || chars[end].is_whitespace()) {
                parts.push(chars[index..end].iter().collect());
                index = end;
                continue;
            }
        }
        let start = index;
        while index < chars.len() && !chars[index].is_whitespace() {
            index += 1;
        }
        parts.push(chars[start..index].iter().collect());
    }
    parts
}

/// Wrap usage parts into lines the way the argparse usage wrapping
/// wraps them, the first line losing its indent to the prefix.
fn get_lines(parts: &[String], indent: &str, prefix: Option<&str>, text_width: i64) -> Vec<String> {
    let indent_length = indent.chars().count() as i64;
    let mut lines: Vec<Vec<String>> = Vec::new();
    let mut line: Vec<String> = Vec::new();
    let mut line_len = match prefix {
        Some(prefix) => prefix.chars().count() as i64 - 1,
        None => indent_length - 1,
    };
    for part in parts {
        let part_len = part.chars().count() as i64;
        if line_len + 1 + part_len > text_width && !line.is_empty() {
            lines.push(std::mem::take(&mut line));
            line_len = indent_length - 1;
        }
        line.push(part.clone());
        line_len += part_len + 1;
    }
    if !line.is_empty() {
        lines.push(line);
    }
    let mut out: Vec<String> = lines
        .iter()
        .map(|line| format!("{}{}", indent, line.join(" ")))
        .collect();
    if prefix.is_some() && !out.is_empty() {
        out[0] = out[0][indent.len()..].to_string();
    }
    out
}

/// Collapse the runs of three or more newlines the Python formatter
/// collapses, then strip the text to one trailing newline.
fn finish_help_text(text: &str) -> String {
    let mut collapsed = String::new();
    let mut run = 0usize;
    for c in text.chars() {
        if c == '\n' {
            run += 1;
            if run <= 2 {
                collapsed.push(c);
            }
        } else {
            run = 0;
            collapsed.push(c);
        }
    }
    let stripped = collapsed.trim_matches('\n');
    format!("{}\n", stripped)
}

/// Collapse every run of whitespace to one space and strip the
/// ends, the normalization the Python formatter applies before it
/// wraps a help or description text.
fn normalize_whitespace(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Wrap a help text at one width the way argparse wraps it.
fn wrap_lines(text: &str, width: i64) -> Vec<String> {
    textwrap_wrap(&normalize_whitespace(text), width)
}

/// Fill a description text at one width the way argparse fills it.
fn fill_text(text: &str, width: i64) -> String {
    textwrap_wrap(&normalize_whitespace(text), width).join("\n")
}

/// Split a whitespace normalized text into the chunks the Python
/// text wrapping reads, words standing whole and a hyphen break
/// standing after a hyphen a letter precedes and a letter follows.
fn split_chunks(text: &str) -> Vec<String> {
    let mut chunks = Vec::new();
    let mut word = String::new();
    let chars: Vec<char> = text.chars().collect();
    let mut index = 0usize;
    let flush_word = |word: &mut String, chunks: &mut Vec<String>| {
        if !word.is_empty() {
            chunks.push(std::mem::take(word));
        }
    };
    while index < chars.len() {
        let c = chars[index];
        if c.is_whitespace() {
            flush_word(&mut word, &mut chunks);
            chunks.push(" ".to_string());
            index += 1;
            continue;
        }
        word.push(c);
        if c == '-'
            && index + 1 < chars.len()
            && !chars[index + 1].is_whitespace()
            && word[..word.len() - 1]
                .chars()
                .any(|before| before.is_alphanumeric() || before == '_')
        {
            flush_word(&mut word, &mut chunks);
        }
        index += 1;
    }
    flush_word(&mut word, &mut chunks);
    chunks
}

/// Wrap a text into lines of at most the width, the greedy fill of
/// the Python text wrapping with its long word splitting.
fn textwrap_wrap(text: &str, width: i64) -> Vec<String> {
    let chunks = split_chunks(text);
    let mut stack: Vec<String> = chunks.into_iter().rev().collect();
    let mut lines: Vec<String> = Vec::new();
    let char_len = |text: &str| text.chars().count() as i64;
    while !stack.is_empty() {
        let mut cur_line: Vec<String> = Vec::new();
        let mut cur_len: i64 = 0;
        if !lines.is_empty() {
            if let Some(last) = stack.last() {
                if last.trim().is_empty() {
                    stack.pop();
                }
            }
        }
        while let Some(next) = stack.last() {
            let next_len = char_len(next);
            if cur_len + next_len <= width {
                cur_line.push(stack.pop().expect("the stack holds the chunk"));
                cur_len += next_len;
            } else {
                break;
            }
        }
        if let Some(next) = stack.last().cloned() {
            if char_len(&next) > width {
                let space_left = if width < 1 { 1 } else { width - cur_len };
                let mut end = space_left;
                if char_len(&next) > space_left {
                    let head: String = next.chars().take(space_left.max(0) as usize).collect();
                    if let Some(hyphen) = head.rfind('-') {
                        let hyphen_index = head[..hyphen].chars().count() as i64;
                        if hyphen_index > 0 && head[..hyphen].chars().any(|c| c != '-') {
                            end = hyphen_index + 1;
                        }
                    }
                }
                let head: String = next.chars().take(end.max(0) as usize).collect();
                let tail: String = next.chars().skip(end.max(0) as usize).collect();
                cur_line.push(head);
                *stack.last_mut().expect("the stack holds the chunk") = tail;
            }
        }
        if let Some(last) = cur_line.last() {
            if last.trim().is_empty() {
                cur_line.pop();
            }
        }
        if !cur_line.is_empty() {
            lines.push(cur_line.concat());
        }
    }
    lines
}

/// Read an integer the way the Python int reading reads a setting,
/// surrounding whitespace and one sign allowed and single
/// underscores standing between digits.
fn parse_python_int(raw: &str) -> Option<i64> {
    let text = raw.trim();
    let (sign, digits) = match text.strip_prefix('-') {
        Some(rest) => (-1i64, rest),
        None => (1i64, text.strip_prefix('+').unwrap_or(text)),
    };
    if digits.is_empty() {
        return None;
    }
    let mut value: i64 = 0;
    let mut previous_was_digit = false;
    let mut previous_was_underscore = false;
    let mut seen_digit = false;
    for byte in digits.bytes() {
        match byte {
            b'0'..=b'9' => {
                value = value
                    .checked_mul(10)
                    .and_then(|v| v.checked_add((byte - b'0') as i64))
                    .unwrap_or(i64::MAX / 4);
                previous_was_digit = true;
                previous_was_underscore = false;
                seen_digit = true;
            }
            b'_' => {
                if !previous_was_digit {
                    return None;
                }
                previous_was_digit = false;
                previous_was_underscore = true;
            }
            _ => return None,
        }
    }
    if previous_was_underscore || !seen_digit {
        return None;
    }
    Some(sign * value)
}

/// The width of the output stream terminal, when it answers.
#[cfg(unix)]
fn stdout_terminal_columns() -> Option<i64> {
    let mut size: libc::winsize = unsafe { std::mem::zeroed() };
    let result = unsafe { libc::ioctl(libc::STDOUT_FILENO, libc::TIOCGWINSZ, &mut size) };
    if result != 0 {
        return None;
    }
    let columns = size.ws_col as i64;
    Some(if columns == 0 { 80 } else { columns })
}

/// The width of the output stream terminal, when it answers.
#[cfg(not(unix))]
fn stdout_terminal_columns() -> Option<i64> {
    None
}

/// The columns the Python terminal size reading reads on this
/// machine, the COLUMNS setting when it holds a positive integer,
/// else the terminal width of the output stream, else 80.
fn terminal_columns() -> i64 {
    if let Ok(raw) = std::env::var("COLUMNS") {
        if let Some(value) = parse_python_int(&raw) {
            if value > 0 {
                return value;
            }
        }
    }
    stdout_terminal_columns().unwrap_or(80)
}

/// Decode one operating system argument the way the Python
/// surrogateescape reading decodes it, each byte the UTF-8 reading
/// cannot place standing as the unit 0xDC00 past its value.
pub fn decode_os_arg(bytes: &[u8]) -> PText {
    let mut units: Vec<u32> = Vec::new();
    let mut index = 0usize;
    while index < bytes.len() {
        let lead = bytes[index];
        let width = match lead {
            0x00..=0x7F => 1usize,
            0xC2..=0xDF => 2,
            0xE0..=0xEF => 3,
            0xF0..=0xF4 => 4,
            _ => 0,
        };
        if width == 1 {
            units.push(lead as u32);
            index += 1;
            continue;
        }
        if width > 1 && index + width <= bytes.len() {
            let seq = &bytes[index..index + width];
            let continuations = seq[1..].iter().all(|b| (0x80..=0xBF).contains(b));
            let second_in_range = match lead {
                0xE0 => seq[1] >= 0xA0,
                0xED => seq[1] <= 0x9F,
                0xF0 => seq[1] >= 0x90,
                0xF4 => seq[1] <= 0x8F,
                _ => true,
            };
            if continuations && second_in_range {
                let unit = match width {
                    2 => ((lead as u32 & 0x1F) << 6) | (seq[1] as u32 & 0x3F),
                    3 => {
                        ((lead as u32 & 0x0F) << 12)
                            | ((seq[1] as u32 & 0x3F) << 6)
                            | (seq[2] as u32 & 0x3F)
                    }
                    _ => {
                        ((lead as u32 & 0x07) << 18)
                            | ((seq[1] as u32 & 0x3F) << 12)
                            | ((seq[2] as u32 & 0x3F) << 6)
                            | (seq[3] as u32 & 0x3F)
                    }
                };
                units.push(unit);
                index += width;
                continue;
            }
        }
        units.push(0xDC00 + lead as u32);
        index += 1;
    }
    PText(units)
}

/// Encode one argument back to the operating system bytes the
/// surrogateescape reading round trips, each escape unit standing
/// as the one byte it was decoded from.
pub fn encode_os_arg(text: &PText) -> Vec<u8> {
    let mut bytes = Vec::new();
    for &unit in &text.0 {
        if (0xDC80..=0xDCFF).contains(&unit) {
            bytes.push((unit - 0xDC00) as u8);
        } else if let Some(c) = char::from_u32(unit) {
            let mut buffer = [0u8; 4];
            bytes.extend_from_slice(c.encode_utf8(&mut buffer).as_bytes());
        }
    }
    bytes
}

/// Whether a code point prints as itself under the Python reading,
/// the categories that never print standing apart from the rest.
fn py_printable(unit: u32) -> bool {
    if unit == 0x20 {
        return true;
    }
    if unit > 0xFFFF && (unit & 0xFFFF) >= 0xFFFE {
        return false;
    }
    !matches!(unit,
        0x00..=0x1F
        | 0x7F..=0x9F
        | 0xD800..=0xDFFF
        | 0x2028
        | 0x2029
        | 0xA0
        | 0x1680
        | 0x2000..=0x200A
        | 0x202F
        | 0x205F
        | 0x3000
        | 0xAD
        | 0x600..=0x605
        | 0x61C
        | 0x6DD
        | 0x70F
        | 0x890..=0x891
        | 0x8E2
        | 0x180E
        | 0x200B..=0x200F
        | 0x202A..=0x202E
        | 0x2060..=0x2064
        | 0x2066..=0x206F
        | 0xFEFF
        | 0xFFF9..=0xFFFB
        | 0xE000..=0xF8FF
        | 0xF0000..=0xFFFFD
        | 0x100000..=0x10FFFD
        | 0xFDD0..=0xFDEF
        | 0xFFFE
        | 0xFFFF)
}

/// Quote a token the way the Python repr quotes it in an error
/// line, a unit that does not print standing as its escape text.
fn py_repr(token: &PText) -> String {
    let units = &token.0;
    let quote = if units.contains(&(b'\'' as u32)) && !units.contains(&(b'"' as u32)) {
        '"'
    } else {
        '\''
    };
    let mut out = String::new();
    out.push(quote);
    for &unit in units {
        match unit {
            0x5C => out.push_str("\\\\"),
            0x0A => out.push_str("\\n"),
            0x0D => out.push_str("\\r"),
            0x09 => out.push_str("\\t"),
            unit if unit == quote as u32 => {
                out.push('\\');
                out.push(quote);
            }
            unit if py_printable(unit) => {
                if let Some(c) = char::from_u32(unit) {
                    out.push(c);
                }
            }
            unit if unit < 0x100 => out.push_str(&format!("\\x{:02x}", unit)),
            unit if unit <= 0xFFFF => out.push_str(&format!("\\u{:04x}", unit)),
            unit => out.push_str(&format!("\\U{:08x}", unit)),
        }
    }
    out.push(quote);
    out
}

/// Render a token the way the Python error stream renders it, the
/// units standing as themselves and each surrogate unit standing
/// as the literal escape text its backslashreplace encoding writes.
fn stderr_text(token: &PText) -> String {
    let mut out = String::new();
    for &unit in &token.0 {
        if (0xD800..=0xDFFF).contains(&unit) {
            out.push_str(&format!("\\u{:04x}", unit));
        } else if let Some(c) = char::from_u32(unit) {
            out.push(c);
        }
    }
    out
}

/// Whether a token reads as the help flag where it stands.
///
/// Argparse fires the help action when it meets the token during
/// its left to right scan, and it accepts the unambiguous long
/// abbreviations of the one long option it knows.
fn is_help_token(token: &PText) -> bool {
    token.eq_str("-h")
        || (token.0.len() >= 3
            && token.0[0] == '-' as u32
            && token.0[1] == '-' as u32
            && "--help"
                .chars()
                .map(|c| c as u32)
                .collect::<Vec<u32>>()
                .starts_with(&token.0))
}

/// The explicit argument of a help flag written with an equals sign.
fn help_explicit_argument(token: &PText) -> Option<PText> {
    let split = token.0.iter().position(|&unit| unit == '=' as u32)?;
    let head = PText(token.0[..split].to_vec());
    let units_ok = head.0.len() >= 3
        && head.0[0] == '-' as u32
        && head.0[1] == '-' as u32
        && "--help"
            .chars()
            .map(|c| c as u32)
            .collect::<Vec<u32>>()
            .starts_with(&head.0);
    if units_ok {
        Some(PText(token.0[split + 1..].to_vec()))
    } else {
        None
    }
}

/// Whether a token reads as an option rather than a positional value.
///
/// A lone dash is a positional value, and the double dash separator
/// is handled where it stands.
fn is_option_like(token: &PText) -> bool {
    token.0.len() > 1 && token.0[0] == '-' as u32
}

fn fail_with(out: CliOutcome, usage_text: &str, error: String) -> CliOutcome {
    let mut out = out;
    let mut lines: Vec<String> = usage_text.lines().map(|line| line.to_string()).collect();
    lines.push(error);
    out.stderr = lines;
    out.code = 2;
    out
}

fn text_lines(text: &str) -> Vec<String> {
    text.lines().map(|line| line.to_string()).collect()
}

/// Run the paceghost command line and return what it prints.
///
/// The reader returns the text of a file, or nothing when the file
/// cannot be read, which the loaders name with their one defect.
/// The width the help layout reads comes from the terminal reading
/// of the Python surface, as the binary meets it.
pub fn dispatch(argv: &[PText], read_file: &dyn Fn(&[u8]) -> Option<String>) -> CliOutcome {
    dispatch_with_columns(argv, read_file, terminal_columns())
}

/// Run the paceghost command line at a stated number of columns.
///
/// The pins drive the layout at stated widths through this entry,
/// against the layouts the Python surface prints at those widths.
pub fn dispatch_with_columns(
    argv: &[PText],
    read_file: &dyn Fn(&[u8]) -> Option<String>,
    columns: i64,
) -> CliOutcome {
    let formatter = Formatter { width: columns - 2 };
    let top = top_parser_info();
    let main_usage = formatter.format_usage_text(&top);
    let mut out = CliOutcome::new();
    let mut extras: Vec<PText> = Vec::new();
    let mut index = 0usize;
    // The top level scan reads extras and fires help where it stands,
    // and the first positional token is the command.
    let command: &'static str = loop {
        if index >= argv.len() {
            return fail_with(
                out,
                &main_usage,
                "paceghost: error: the following arguments are required: command".to_string(),
            );
        }
        let token = &argv[index];
        if is_help_token(token) {
            out.stdout = text_lines(&formatter.format_help(&top));
            return out;
        }
        if let Some(value) = help_explicit_argument(token) {
            return fail_with(
                out,
                &main_usage,
                format!(
                    "paceghost: error: argument -h/--help: ignored explicit argument {}",
                    py_repr(&value)
                ),
            );
        }
        if token.eq_str("--") {
            if index + 1 >= argv.len() {
                // A separator with nothing after it leaves the command
                // unseen, and the Python reads the command as missing.
                return fail_with(
                    out,
                    &main_usage,
                    "paceghost: error: the following arguments are required: command".to_string(),
                );
            }
            // With a token after it the separator itself stands as
            // the value the subparsers action reads first, exactly as
            // the Python pattern for that action takes it.
            return fail_with(
                out,
                &main_usage,
                format!(
                    "paceghost: error: argument command: invalid choice: {} (choose from 'show', 'ghost', 'race', 'season', 'board', 'fieldday', 'ladder', 'rematch', 'meet', 'table', 'reign', 'days', 'forks', 'marks', 'standings', 'traces', 'margins', 'spread', 'middle', 'tails', 'mean')",
                    py_repr(token)
                ),
            );
        }
        if is_option_like(token) {
            extras.push(token.clone());
            index += 1;
            continue;
        }
        match COMMANDS.iter().find(|name| token.eq_str(name)) {
            Some(name) => {
                index += 1;
                break name;
            }
            None => {
                return fail_with(
                    out,
                    &main_usage,
                    format!(
                        "paceghost: error: argument command: invalid choice: {} (choose from 'show', 'ghost', 'race', 'season', 'board', 'fieldday', 'ladder', 'rematch', 'meet', 'table', 'reign', 'days', 'forks', 'marks', 'standings', 'traces', 'margins', 'spread', 'middle', 'tails', 'mean')",
                        py_repr(token)
                    ),
                );
            }
        }
    };
    let spec = spec_for(command);
    let sub = sub_parser_info(&spec);
    let sub_usage = formatter.format_usage_text(&sub);
    // The subcommand scan fills its positionals, fires help where it
    // stands, and sets every other token aside as an extra.
    let mut positionals: Vec<PText> = Vec::new();
    let mut positional_only = false;
    while index < argv.len() {
        let token = &argv[index];
        if !positional_only {
            if is_help_token(token) {
                out.stdout = text_lines(&formatter.format_help(&sub));
                return out;
            }
            if let Some(value) = help_explicit_argument(token) {
                return fail_with(
                    out,
                    &sub_usage,
                    format!(
                        "paceghost {}: error: argument -h/--help: ignored explicit argument {}",
                        spec.name,
                        py_repr(&value)
                    ),
                );
            }
            if token.eq_str("--") {
                positional_only = true;
                index += 1;
                continue;
            }
            if is_option_like(token) {
                extras.push(token.clone());
                index += 1;
                continue;
            }
        }
        if spec.variadic || positionals.len() < spec.positionals.len() {
            positionals.push(token.clone());
        } else {
            extras.push(token.clone());
        }
        index += 1;
    }
    if positionals.len() < spec.positionals.len() {
        let missing = &spec.positionals[positionals.len()..];
        return fail_with(
            out,
            &sub_usage,
            format!(
                "paceghost {}: error: the following arguments are required: {}",
                spec.name,
                missing.join(", ")
            ),
        );
    }
    if !extras.is_empty() {
        let rendered: Vec<String> = extras.iter().map(stderr_text).collect();
        return fail_with(
            out,
            &main_usage,
            format!(
                "paceghost: error: unrecognized arguments: {}",
                rendered.join(" ")
            ),
        );
    }
    run_command(&spec, &positionals, read_file, &mut out);
    out
}

/// Return the summary lines the show command prints for one run.
///
/// Handed a run whose runner label holds a line break, the lines
/// come back as the named error of FIX-PG-12 instead, because the
/// recorded run loader states no line break rule for a runner and
/// the summary prints the runner. The same pass refuses the
/// fields the summary prints that a hand built run can carry a
/// break in past its loader, under FIX-PG-14, checked after the
/// runner in the order the summary prints them, the task, then
/// the recorded_at, then the split phases in split order, each
/// in its named error, because the summary prints each of them
/// inside a line of its own.
pub fn run_summary_lines(run: &Run) -> Result<Vec<String>, String> {
    check_runner_label(run)?;
    if run.task.holds_line_break() {
        return Err("a task holds a line break".to_string());
    }
    if run.recorded_at.holds_line_break() {
        return Err("a recorded_at holds a line break".to_string());
    }
    for split in &run.splits {
        if split.phase.holds_line_break() {
            return Err("a split phase holds a line break".to_string());
        }
    }
    let mut lines = vec![
        format!("task {}", run.task),
        format!("runner {}", run.runner),
        format!("recorded_at {}", run.recorded_at),
        format!("window_seconds {}", format_seconds(&run.window_seconds)),
        format!("total_seconds {}", format_seconds(&run.total_seconds)),
        format!("splits {}", run.splits.len()),
    ];
    for split in &run.splits {
        lines.push(format!(
            "split {} at {} seconds",
            split.phase,
            format_seconds(&split.elapsed_seconds)
        ));
    }
    lines.push(window_line(run, "this"));
    Ok(lines)
}

fn load_run(
    path: &PText,
    read_file: &dyn Fn(&[u8]) -> Option<String>,
) -> (Option<Run>, Vec<String>) {
    match read_file(&encode_os_arg(path)) {
        None => (None, vec!["file cannot be read".to_string()]),
        Some(text) => load_run_text(&text),
    }
}

fn print_defects(out: &mut CliOutcome, heading: &str, defects: &[String]) {
    out.stdout.push(heading.to_string());
    for defect in defects {
        out.stdout.push(format!("defect {}", defect));
    }
}

fn load_or_print_defects(
    path: &PText,
    read_file: &dyn Fn(&[u8]) -> Option<String>,
    out: &mut CliOutcome,
) -> Option<Run> {
    let (run, defects) = load_run(path, read_file);
    if !defects.is_empty() {
        print_defects(out, "invalid recorded run", &defects);
        return None;
    }
    run
}

fn run_command(
    spec: &Spec,
    positionals: &[PText],
    read_file: &dyn Fn(&[u8]) -> Option<String>,
    out: &mut CliOutcome,
) {
    match spec.name {
        "show" => command_show(&positionals[0], read_file, out),
        "ghost" => command_ghost(positionals, read_file, out),
        "race" => command_race(&positionals[0], &positionals[1], read_file, out),
        "season" => command_season(&positionals[0], &positionals[1], read_file, out),
        "board" => command_board(&positionals[0], &positionals[1], read_file, out),
        "fieldday" => command_fieldday(&positionals[0], read_file, out),
        "rematch" => command_rematch(&positionals[0], read_file, out),
        "meet" => command_meet(&positionals[0], read_file, out),
        "table" => command_table(&positionals[0], read_file, out),
        "reign" => command_reign(&positionals[0], read_file, out),
        "days" => command_days(&positionals[0], read_file, out),
        "forks" => command_forks(&positionals[0], read_file, out),
        "marks" => command_marks(&positionals[0], read_file, out),
        "standings" => command_standings(&positionals[0], read_file, out),
        "traces" => command_traces(&positionals[0], read_file, out),
        "margins" => command_margins(&positionals[0], read_file, out),
        "spread" => command_spread(&positionals[0], read_file, out),
        "middle" => command_middle(&positionals[0], read_file, out),
        "tails" => command_tails(&positionals[0], read_file, out),
        "mean" => command_mean(&positionals[0], read_file, out),
        _ => command_ladder(&positionals[0], read_file, out),
    }
}

fn command_show(file: &PText, read_file: &dyn Fn(&[u8]) -> Option<String>, out: &mut CliOutcome) {
    let run = match load_or_print_defects(file, read_file, out) {
        None => {
            out.code = 1;
            return;
        }
        Some(run) => run,
    };
    match run_summary_lines(&run) {
        Ok(lines) => out.stdout.extend(lines),
        Err(error) => {
            out.stdout.push(error);
            out.code = 1;
        }
    }
}

fn command_ghost(
    files: &[PText],
    read_file: &dyn Fn(&[u8]) -> Option<String>,
    out: &mut CliOutcome,
) {
    let mut runs: Vec<Run> = Vec::new();
    let mut failed = false;
    for path in files {
        match load_or_print_defects(path, read_file, out) {
            None => failed = true,
            Some(run) => runs.push(run),
        }
    }
    if failed {
        out.code = 1;
        return;
    }
    let share_one_task = runs.iter().all(|run| run.task == runs[0].task);
    if !share_one_task {
        out.stdout.push("runs do not share one task".to_string());
        out.code = 1;
        return;
    }
    let ghost = select_ghost(&runs).expect("the ghost command reads at least one run");
    if let Err(error) = check_runner_label(ghost) {
        out.stdout.push(error);
        out.code = 1;
        return;
    }
    out.stdout.push(format!("ghost for task {}", ghost.task));
    out.stdout.push(format!("runner {}", ghost.runner));
    out.stdout
        .push(format!("recorded_at {}", ghost.recorded_at));
    out.stdout.push(format!(
        "total_seconds {}",
        format_seconds(&ghost.total_seconds)
    ));
}

fn command_race(
    ghost_file: &PText,
    live_file: &PText,
    read_file: &dyn Fn(&[u8]) -> Option<String>,
    out: &mut CliOutcome,
) {
    let ghost = load_or_print_defects(ghost_file, read_file, out);
    let live = load_or_print_defects(live_file, read_file, out);
    let (ghost, live) = match (ghost, live) {
        (Some(ghost), Some(live)) => (ghost, live),
        _ => {
            out.code = 1;
            return;
        }
    };
    if ghost.task != live.task {
        out.stdout.push("runs do not share one task".to_string());
        out.code = 1;
        return;
    }
    out.stdout.extend(race_report(&ghost, &live));
}

fn load_field_or_print_defects(
    path: &PText,
    read_file: &dyn Fn(&[u8]) -> Option<String>,
    out: &mut CliOutcome,
) -> Option<Vec<Run>> {
    let (runs, defects) = match read_file(&encode_os_arg(path)) {
        None => (None, vec!["file cannot be read".to_string()]),
        Some(text) => load_field_text(&text),
    };
    if !defects.is_empty() {
        print_defects(out, "invalid field", &defects);
        return None;
    }
    runs
}

fn command_season(
    field_file: &PText,
    ghost_file: &PText,
    read_file: &dyn Fn(&[u8]) -> Option<String>,
    out: &mut CliOutcome,
) {
    let runs = match load_field_or_print_defects(field_file, read_file, out) {
        None => {
            out.code = 1;
            return;
        }
        Some(runs) => runs,
    };
    let ghost = match load_or_print_defects(ghost_file, read_file, out) {
        None => {
            out.code = 1;
            return;
        }
        Some(ghost) => ghost,
    };
    let ghost_usage = match &ghost.usage {
        None => {
            out.stdout.push("ghost record carries no usage".to_string());
            out.code = 1;
            return;
        }
        Some(usage) => usage.clone(),
    };
    if runs[0].task != ghost.task {
        out.stdout.push("runs do not share one task".to_string());
        out.code = 1;
        return;
    }
    out.stdout.extend(season_report(&runs, &ghost_usage));
}

fn command_board(
    file: &PText,
    ghost_file: &PText,
    read_file: &dyn Fn(&[u8]) -> Option<String>,
    out: &mut CliOutcome,
) {
    let run = load_or_print_defects(file, read_file, out);
    let ghost = load_or_print_defects(ghost_file, read_file, out);
    let (run, ghost) = match (run, ghost) {
        (Some(run), Some(ghost)) => (run, ghost),
        _ => {
            out.code = 1;
            return;
        }
    };
    if run.reads.is_none() {
        out.stdout.push("run carries no timed reads".to_string());
        out.code = 1;
        return;
    }
    let ghost_usage = match &ghost.usage {
        None => {
            out.stdout.push("ghost record carries no usage".to_string());
            out.code = 1;
            return;
        }
        Some(usage) => usage.clone(),
    };
    if run.task != ghost.task {
        out.stdout.push("runs do not share one task".to_string());
        out.code = 1;
        return;
    }
    out.stdout.extend(board_report(&run, &ghost_usage));
}

fn load_field_day_or_print_defects(
    path: &PText,
    read_file: &dyn Fn(&[u8]) -> Option<String>,
    out: &mut CliOutcome,
) -> Option<FieldDay> {
    let (day, defects) = match read_file(&encode_os_arg(path)) {
        None => (None, vec!["file cannot be read".to_string()]),
        Some(text) => load_field_day_text(&text),
    };
    if !defects.is_empty() {
        print_defects(out, "invalid field day", &defects);
        return None;
    }
    day
}

fn command_fieldday(
    file: &PText,
    read_file: &dyn Fn(&[u8]) -> Option<String>,
    out: &mut CliOutcome,
) {
    let day = match load_field_day_or_print_defects(file, read_file, out) {
        None => {
            out.code = 1;
            return;
        }
        Some(day) => day,
    };
    let lines =
        field_day_report(&day).expect("a loaded field day satisfies the report preconditions");
    out.stdout.extend(lines);
}

fn load_ladder_or_print_defects(
    path: &PText,
    read_file: &dyn Fn(&[u8]) -> Option<String>,
    out: &mut CliOutcome,
) -> Option<Ladder> {
    let (ladder, defects) = match read_file(&encode_os_arg(path)) {
        None => (None, vec!["file cannot be read".to_string()]),
        Some(text) => load_ladder_text(&text),
    };
    if !defects.is_empty() {
        print_defects(out, "invalid ladder", &defects);
        return None;
    }
    ladder
}

fn command_ladder(file: &PText, read_file: &dyn Fn(&[u8]) -> Option<String>, out: &mut CliOutcome) {
    let ladder = match load_ladder_or_print_defects(file, read_file, out) {
        None => {
            out.code = 1;
            return;
        }
        Some(ladder) => ladder,
    };
    let lines = ladder_report(&ladder).expect("a loaded ladder satisfies the report preconditions");
    out.stdout.extend(lines);
}

fn load_rematch_or_print_defects(
    path: &PText,
    read_file: &dyn Fn(&[u8]) -> Option<String>,
    out: &mut CliOutcome,
) -> Option<Rematch> {
    let (rematch, defects) = match read_file(&encode_os_arg(path)) {
        None => (None, vec!["file cannot be read".to_string()]),
        Some(text) => load_rematch_text(&text),
    };
    if !defects.is_empty() {
        print_defects(out, "invalid rematch", &defects);
        return None;
    }
    rematch
}

fn command_rematch(
    file: &PText,
    read_file: &dyn Fn(&[u8]) -> Option<String>,
    out: &mut CliOutcome,
) {
    let rematch = match load_rematch_or_print_defects(file, read_file, out) {
        None => {
            out.code = 1;
            return;
        }
        Some(rematch) => rematch,
    };
    let lines =
        rematch_report(&rematch).expect("a loaded rematch satisfies the report preconditions");
    out.stdout.extend(lines);
}

fn load_meet_or_print_defects(
    path: &PText,
    read_file: &dyn Fn(&[u8]) -> Option<String>,
    out: &mut CliOutcome,
) -> Option<Meet> {
    let (meet, defects) = match read_file(&encode_os_arg(path)) {
        None => (None, vec!["file cannot be read".to_string()]),
        Some(text) => load_meet_text(&text),
    };
    if !defects.is_empty() {
        print_defects(out, "invalid meet", &defects);
        return None;
    }
    meet
}

fn command_meet(file: &PText, read_file: &dyn Fn(&[u8]) -> Option<String>, out: &mut CliOutcome) {
    let meet = match load_meet_or_print_defects(file, read_file, out) {
        None => {
            out.code = 1;
            return;
        }
        Some(meet) => meet,
    };
    let lines = meet_report(&meet).expect("a loaded meet satisfies the report preconditions");
    out.stdout.extend(lines);
}

fn command_table(file: &PText, read_file: &dyn Fn(&[u8]) -> Option<String>, out: &mut CliOutcome) {
    let meet = match load_meet_or_print_defects(file, read_file, out) {
        None => {
            out.code = 1;
            return;
        }
        Some(meet) => meet,
    };
    let lines = meet_table(&meet).expect("a loaded meet satisfies the table preconditions");
    out.stdout.extend(lines);
}

fn command_reign(file: &PText, read_file: &dyn Fn(&[u8]) -> Option<String>, out: &mut CliOutcome) {
    let meet = match load_meet_or_print_defects(file, read_file, out) {
        None => {
            out.code = 1;
            return;
        }
        Some(meet) => meet,
    };
    let lines = meet_reign(&meet).expect("a loaded meet satisfies the reign preconditions");
    out.stdout.extend(lines);
}

fn command_days(file: &PText, read_file: &dyn Fn(&[u8]) -> Option<String>, out: &mut CliOutcome) {
    let meet = match load_meet_or_print_defects(file, read_file, out) {
        None => {
            out.code = 1;
            return;
        }
        Some(meet) => meet,
    };
    let lines = meet_days(&meet).expect("a loaded meet satisfies the days preconditions");
    out.stdout.extend(lines);
}

fn command_forks(file: &PText, read_file: &dyn Fn(&[u8]) -> Option<String>, out: &mut CliOutcome) {
    let meet = match load_meet_or_print_defects(file, read_file, out) {
        None => {
            out.code = 1;
            return;
        }
        Some(meet) => meet,
    };
    let lines = meet_forks(&meet).expect("a loaded meet satisfies the forks preconditions");
    out.stdout.extend(lines);
}

fn command_marks(file: &PText, read_file: &dyn Fn(&[u8]) -> Option<String>, out: &mut CliOutcome) {
    let meet = match load_meet_or_print_defects(file, read_file, out) {
        None => {
            out.code = 1;
            return;
        }
        Some(meet) => meet,
    };
    let lines = meet_marks(&meet).expect("a loaded meet satisfies the marks preconditions");
    out.stdout.extend(lines);
}

fn command_standings(
    file: &PText,
    read_file: &dyn Fn(&[u8]) -> Option<String>,
    out: &mut CliOutcome,
) {
    let meet = match load_meet_or_print_defects(file, read_file, out) {
        None => {
            out.code = 1;
            return;
        }
        Some(meet) => meet,
    };
    let lines = meet_standings(&meet).expect("a loaded meet satisfies the standings preconditions");
    out.stdout.extend(lines);
}

fn command_traces(file: &PText, read_file: &dyn Fn(&[u8]) -> Option<String>, out: &mut CliOutcome) {
    let meet = match load_meet_or_print_defects(file, read_file, out) {
        None => {
            out.code = 1;
            return;
        }
        Some(meet) => meet,
    };
    let lines = meet_traces(&meet).expect("a loaded meet satisfies the traces preconditions");
    out.stdout.extend(lines);
}

fn command_margins(
    file: &PText,
    read_file: &dyn Fn(&[u8]) -> Option<String>,
    out: &mut CliOutcome,
) {
    let meet = match load_meet_or_print_defects(file, read_file, out) {
        None => {
            out.code = 1;
            return;
        }
        Some(meet) => meet,
    };
    let lines = meet_margins(&meet).expect("a loaded meet satisfies the margins preconditions");
    out.stdout.extend(lines);
}

fn command_spread(file: &PText, read_file: &dyn Fn(&[u8]) -> Option<String>, out: &mut CliOutcome) {
    let meet = match load_meet_or_print_defects(file, read_file, out) {
        None => {
            out.code = 1;
            return;
        }
        Some(meet) => meet,
    };
    let lines = meet_spread(&meet).expect("a loaded meet satisfies the spread preconditions");
    out.stdout.extend(lines);
}

fn command_middle(file: &PText, read_file: &dyn Fn(&[u8]) -> Option<String>, out: &mut CliOutcome) {
    let meet = match load_meet_or_print_defects(file, read_file, out) {
        None => {
            out.code = 1;
            return;
        }
        Some(meet) => meet,
    };
    let lines = meet_middle(&meet).expect("a loaded meet satisfies the middle preconditions");
    out.stdout.extend(lines);
}

fn command_tails(file: &PText, read_file: &dyn Fn(&[u8]) -> Option<String>, out: &mut CliOutcome) {
    let meet = match load_meet_or_print_defects(file, read_file, out) {
        None => {
            out.code = 1;
            return;
        }
        Some(meet) => meet,
    };
    let lines = meet_tails(&meet).expect("a loaded meet satisfies the tails preconditions");
    out.stdout.extend(lines);
}

fn command_mean(file: &PText, read_file: &dyn Fn(&[u8]) -> Option<String>, out: &mut CliOutcome) {
    let meet = match load_meet_or_print_defects(file, read_file, out) {
        None => {
            out.code = 1;
            return;
        }
        Some(meet) => meet,
    };
    let lines = meet_mean(&meet).expect("a loaded meet satisfies the mean preconditions");
    out.stdout.extend(lines);
}

#[cfg(test)]
mod tests {
    use super::parse_python_int;

    #[test]
    fn the_columns_integer_reading_matches_the_python_int_reading() {
        assert_eq!(parse_python_int("120"), Some(120));
        assert_eq!(parse_python_int(" 90 "), Some(90));
        assert_eq!(parse_python_int("+95"), Some(95));
        assert_eq!(parse_python_int("1_00"), Some(100));
        assert_eq!(parse_python_int("0"), Some(0));
        assert_eq!(parse_python_int("-5"), Some(-5));
        assert_eq!(parse_python_int("abc"), None);
        assert_eq!(parse_python_int("9 0"), None);
        assert_eq!(parse_python_int("1_"), None);
        assert_eq!(parse_python_int("_1"), None);
        assert_eq!(parse_python_int("1__0"), None);
        assert_eq!(parse_python_int(""), None);
        assert_eq!(parse_python_int("-"), None);
    }
}
