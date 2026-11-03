//! Text values as sequences of Unicode code points.
//!
//! A Python string is a sequence of code points, and a code point in
//! the surrogate range can stand in one when the JSON reading decodes
//! an unpaired surrogate escape. Keeping text as code point units
//! keeps that case representable in the port, so the loaders can
//! detect it and refuse it with the defect text the contract states,
//! exactly as the Python loaders do.

use std::fmt;

/// A text value held as its sequence of code points.
///
/// A unit in the range D800 to DFFF stands for an unpaired surrogate
/// decoded from a JSON escape. Such a unit never reaches a printer,
/// because every loader that reads text refuses it first.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PText(pub Vec<u32>);

impl PText {
    /// Build a text value from a Rust string.
    pub fn from_text(text: &str) -> PText {
        PText(text.chars().map(|c| c as u32).collect())
    }

    /// Whether the text holds no code points.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Whether any unit is a surrogate code point.
    ///
    /// This mirrors the Python check that encodes the string as
    /// UTF-8 and watches the encode fail, which refuses every
    /// surrogate, paired in the text or not.
    pub fn holds_unpaired_surrogate(&self) -> bool {
        self.0.iter().any(|&unit| (0xD800..=0xDFFF).contains(&unit))
    }

    /// Whether the text holds a line feed or a carriage return.
    pub fn holds_line_break(&self) -> bool {
        self.0.iter().any(|&unit| unit == 0x0A || unit == 0x0D)
    }

    /// Whether this text equals a Rust string, code point for code point.
    pub fn eq_str(&self, other: &str) -> bool {
        self.0.len() == other.chars().count()
            && self
                .0
                .iter()
                .zip(other.chars())
                .all(|(&unit, c)| unit == c as u32)
    }

    /// Render the text as a Rust string for printing.
    ///
    /// Loaded text never holds a surrogate unit, because the loaders
    /// refuse it, so the replacement character fallback for a
    /// surrogate unit is unreachable on any value a report prints.
    pub fn to_display_string(&self) -> String {
        let mut out = String::new();
        for &unit in &self.0 {
            match char::from_u32(unit) {
                Some(c) => out.push(c),
                None => out.push('\u{FFFD}'),
            }
        }
        out
    }
}

impl From<&str> for PText {
    fn from(text: &str) -> PText {
        PText::from_text(text)
    }
}

impl fmt::Display for PText {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_display_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_plain_string_round_trips() {
        let text = PText::from_text("night fork");
        assert_eq!(text.to_display_string(), "night fork");
        assert!(text.eq_str("night fork"));
        assert!(!text.eq_str("night forks"));
        assert!(!text.holds_unpaired_surrogate());
        assert!(!text.holds_line_break());
    }

    #[test]
    fn a_surrogate_unit_is_detected() {
        let text = PText(vec![0x70, 0xD800, 0x6B]);
        assert!(text.holds_unpaired_surrogate());
        assert!(PText(vec![0xDC00]).holds_unpaired_surrogate());
        assert!(PText(vec![0xDFFF]).holds_unpaired_surrogate());
        assert!(!PText(vec![0xD7FF]).holds_unpaired_surrogate());
        assert!(!PText(vec![0xE000]).holds_unpaired_surrogate());
        assert!(!PText(vec![0x10000]).holds_unpaired_surrogate());
    }

    #[test]
    fn line_breaks_are_detected() {
        assert!(PText::from_text("a\nb").holds_line_break());
        assert!(PText::from_text("a\rb").holds_line_break());
        assert!(!PText::from_text("a\tb").holds_line_break());
    }

    #[test]
    fn ordering_is_by_code_point() {
        assert!(PText::from_text("alpha") < PText::from_text("beta"));
        assert!(PText::from_text("beta") < PText::from_text("wick"));
        assert!(PText::from_text("app") < PText::from_text("apple"));
    }
}
