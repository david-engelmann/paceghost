//! A JSON reading that mirrors the Python json module where the
//! loaders depend on its semantics.
//!
//! The semantics mirrored here were each probed against the Python
//! reading on this machine and are pinned by the unit tests below.
//! A repeated object key keeps its last value. A number is classified
//! from its raw literal, so an integer literal stands as an exact
//! integer, a float literal stands as the double it rounds to, an
//! integer literal of more than 4300 digits fails the reading as the
//! Python integer string limit fails it, and a float literal that
//! overflows stands as an infinity. The NaN and Infinity literals
//! parse, as the Python reading accepts them. A string is a sequence
//! of code points, a high surrogate escape immediately followed by a
//! low surrogate escape combines into the one code point above the
//! basic multilingual plane, and any other surrogate stands alone as
//! its own unit, so an unpaired surrogate stays representable and the
//! loaders can refuse it. Unescaped control characters fail the
//! reading, leading byte order marks fail it, and content after the
//! top level value fails it. Nesting deeper than the Python reading
//! on this machine reads through the command line, 9997 open
//! containers, fails the reading, which the loaders name with their
//! not valid JSON defect, exactly as they name the RecursionError
//! the Python reading raises there.

use num_bigint::BigInt;

use crate::pynum::Num;
use crate::pytext::PText;

/// The most open containers the reading accepts at once.
///
/// The Python reading raises on nesting past a depth its own call
/// stack sets, so the depth moves with the caller. Measured through
/// the console entry point on this machine, the reading loads a
/// document holding 9997 containers open at once and raises on the
/// 9998th, for bare arrays, for bare objects, and for mixed nesting
/// counted the same way. Inside another process the figure moves,
/// 9979 under the test runner on this machine, and that variance is
/// a runtime artifact. The port pins the command line figure of
/// PORT-8, because the command line is the surface whose behavior
/// the port ships. The corpus pin, input nested 100000 deep draws
/// the not valid JSON defect, holds under either figure.
const MAX_OPEN_CONTAINERS: usize = 9997;

/// The most digits an integer literal may carry.
///
/// Probed against the Python reading, whose integer string
/// conversion limit stands at 4300 digits.
const MAX_INTEGER_DIGITS: usize = 4300;

/// A parsed JSON value.
#[derive(Clone, Debug)]
pub enum JsonValue {
    Null,
    Bool(bool),
    Num(Num),
    Str(PText),
    Arr(Vec<JsonValue>),
    Obj(Vec<(PText, JsonValue)>),
}

/// Dropping a deeply nested value must not recurse, because the
/// reading accepts nesting thousands of containers deep and a
/// recursive drop overflows the stack the Python reading survives
/// on its deferred deallocation. The drop drains nested containers
/// into a work list and empties each value before it falls, so
/// every value the drop glue itself releases is shallow.
impl Drop for JsonValue {
    fn drop(&mut self) {
        let mut work: Vec<JsonValue> = Vec::new();
        self.drain_children(&mut work);
        while let Some(mut value) = work.pop() {
            value.drain_children(&mut work);
        }
    }
}

impl JsonValue {
    /// Move a value's nested containers into the work list, leaving
    /// the value itself holding only empty containers and scalars.
    fn drain_children(&mut self, work: &mut Vec<JsonValue>) {
        match self {
            JsonValue::Arr(items) => work.append(items),
            JsonValue::Obj(pairs) => {
                work.extend(pairs.drain(..).map(|pair| pair.1));
            }
            _ => {}
        }
    }

    /// Read a field of an object, the last value standing for a key.
    ///
    /// The loaders only ever look fields up by name, so pairs stand
    /// in document order and the lookup walks them in reverse.
    pub fn get(&self, key: &str) -> Option<&JsonValue> {
        match self {
            JsonValue::Obj(pairs) => pairs
                .iter()
                .rev()
                .find(|pair| pair.0.eq_str(key))
                .map(|pair| &pair.1),
            _ => None,
        }
    }

    /// Whether the value is an object.
    pub fn is_object(&self) -> bool {
        matches!(self, JsonValue::Obj(_))
    }
}

/// The reading failed. Every failure names the same loader defect.
#[derive(Debug)]
pub struct ParseError;

struct Parser {
    chars: Vec<char>,
    pos: usize,
}

enum Container {
    Arr(Vec<JsonValue>),
    Obj(Vec<(PText, JsonValue)>, PText),
}

/// Parse JSON text the way the Python json module reads it.
///
/// Returns the value, or the parse error the loaders name with their
/// not valid JSON defect.
pub fn parse_json(text: &str) -> Result<JsonValue, ParseError> {
    let mut parser = Parser {
        chars: text.chars().collect(),
        pos: 0,
    };
    let value = parser.parse_document()?;
    parser.skip_ws();
    if parser.pos != parser.chars.len() {
        return Err(ParseError);
    }
    Ok(value)
}

impl Parser {
    fn peek(&self) -> Option<char> {
        self.chars.get(self.pos).copied()
    }

    fn bump(&mut self) -> Option<char> {
        let current = self.peek();
        if current.is_some() {
            self.pos += 1;
        }
        current
    }

    fn skip_ws(&mut self) {
        while matches!(self.peek(), Some(' ' | '\t' | '\n' | '\r')) {
            self.pos += 1;
        }
    }

    fn expect_literal(&mut self, literal: &str) -> Result<(), ParseError> {
        for expected in literal.chars() {
            if self.bump() != Some(expected) {
                return Err(ParseError);
            }
        }
        Ok(())
    }

    fn parse_document(&mut self) -> Result<JsonValue, ParseError> {
        let mut stack: Vec<Container> = Vec::new();
        'value: loop {
            self.skip_ws();
            let head = self.peek().ok_or(ParseError)?;
            let mut value = match head {
                '{' => {
                    if stack.len() >= MAX_OPEN_CONTAINERS {
                        return Err(ParseError);
                    }
                    self.pos += 1;
                    self.skip_ws();
                    if self.peek() == Some('}') {
                        self.pos += 1;
                        JsonValue::Obj(Vec::new())
                    } else {
                        let key = self.parse_key()?;
                        stack.push(Container::Obj(Vec::new(), key));
                        continue 'value;
                    }
                }
                '[' => {
                    if stack.len() >= MAX_OPEN_CONTAINERS {
                        return Err(ParseError);
                    }
                    self.pos += 1;
                    self.skip_ws();
                    if self.peek() == Some(']') {
                        self.pos += 1;
                        JsonValue::Arr(Vec::new())
                    } else {
                        stack.push(Container::Arr(Vec::new()));
                        continue 'value;
                    }
                }
                '"' => JsonValue::Str(self.parse_string()?),
                't' => {
                    self.expect_literal("true")?;
                    JsonValue::Bool(true)
                }
                'f' => {
                    self.expect_literal("false")?;
                    JsonValue::Bool(false)
                }
                'n' => {
                    self.expect_literal("null")?;
                    JsonValue::Null
                }
                'N' => {
                    self.expect_literal("NaN")?;
                    JsonValue::Num(Num::from_f64(f64::NAN))
                }
                'I' => {
                    self.expect_literal("Infinity")?;
                    JsonValue::Num(Num::from_f64(f64::INFINITY))
                }
                '-' => {
                    if self.rest_starts_with("Infinity") {
                        self.pos += 1 + "Infinity".len();
                        JsonValue::Num(Num::from_f64(f64::NEG_INFINITY))
                    } else {
                        self.parse_number()?
                    }
                }
                '0'..='9' => self.parse_number()?,
                _ => return Err(ParseError),
            };
            // Deliver the value to the container that holds it, and
            // keep delivering while containers close around it.
            loop {
                match stack.pop() {
                    None => return Ok(value),
                    Some(Container::Arr(mut items)) => {
                        items.push(value);
                        self.skip_ws();
                        match self.bump() {
                            Some(',') => {
                                stack.push(Container::Arr(items));
                                continue 'value;
                            }
                            Some(']') => {
                                value = JsonValue::Arr(items);
                            }
                            _ => return Err(ParseError),
                        }
                    }
                    Some(Container::Obj(mut pairs, key)) => {
                        pairs.push((key, value));
                        self.skip_ws();
                        match self.bump() {
                            Some(',') => {
                                let key = self.parse_key()?;
                                stack.push(Container::Obj(pairs, key));
                                continue 'value;
                            }
                            Some('}') => {
                                value = JsonValue::Obj(pairs);
                            }
                            _ => return Err(ParseError),
                        }
                    }
                }
            }
        }
    }

    fn rest_starts_with(&self, literal: &str) -> bool {
        // The parser stands on a minus sign, and the literal follows it.
        let start = self.pos + 1;
        literal
            .chars()
            .enumerate()
            .all(|(offset, expected)| self.chars.get(start + offset) == Some(&expected))
    }

    fn parse_key(&mut self) -> Result<PText, ParseError> {
        self.skip_ws();
        if self.peek() != Some('"') {
            return Err(ParseError);
        }
        let key = self.parse_string()?;
        self.skip_ws();
        if self.bump() != Some(':') {
            return Err(ParseError);
        }
        Ok(key)
    }

    fn parse_string(&mut self) -> Result<PText, ParseError> {
        // The caller stands on the opening quote.
        self.pos += 1;
        let mut units: Vec<u32> = Vec::new();
        loop {
            let current = self.bump().ok_or(ParseError)?;
            match current {
                '"' => return Ok(PText(units)),
                '\\' => {
                    let escape = self.bump().ok_or(ParseError)?;
                    match escape {
                        '"' => units.push(0x22),
                        '\\' => units.push(0x5C),
                        '/' => units.push(0x2F),
                        'b' => units.push(0x08),
                        'f' => units.push(0x0C),
                        'n' => units.push(0x0A),
                        'r' => units.push(0x0D),
                        't' => units.push(0x09),
                        'u' => {
                            let unit = self.parse_hex4()?;
                            if (0xD800..=0xDBFF).contains(&unit) {
                                match self.take_low_surrogate() {
                                    Some(low) => units
                                        .push(0x10000 + ((unit - 0xD800) << 10) + (low - 0xDC00)),
                                    None => units.push(unit),
                                }
                            } else {
                                units.push(unit);
                            }
                        }
                        _ => return Err(ParseError),
                    }
                }
                current if (current as u32) < 0x20 => return Err(ParseError),
                current => units.push(current as u32),
            }
        }
    }

    fn parse_hex4(&mut self) -> Result<u32, ParseError> {
        let mut value: u32 = 0;
        for _ in 0..4 {
            let current = self.bump().ok_or(ParseError)?;
            let digit = current.to_digit(16).ok_or(ParseError)?;
            value = value * 16 + digit;
        }
        Ok(value)
    }

    fn hex4_at(&self, start: usize) -> Option<u32> {
        let mut value: u32 = 0;
        for offset in 0..4 {
            let current = self.chars.get(start + offset)?;
            value = value * 16 + current.to_digit(16)?;
        }
        Some(value)
    }

    fn take_low_surrogate(&mut self) -> Option<u32> {
        if self.chars.get(self.pos) != Some(&'\\') || self.chars.get(self.pos + 1) != Some(&'u') {
            return None;
        }
        let low = self.hex4_at(self.pos + 2)?;
        if (0xDC00..=0xDFFF).contains(&low) {
            self.pos += 6;
            Some(low)
        } else {
            None
        }
    }

    fn parse_number(&mut self) -> Result<JsonValue, ParseError> {
        let start = self.pos;
        if self.peek() == Some('-') {
            self.pos += 1;
        }
        match self.peek() {
            Some('0') => {
                self.pos += 1;
            }
            Some('1'..='9') => {
                while matches!(self.peek(), Some('0'..='9')) {
                    self.pos += 1;
                }
            }
            _ => return Err(ParseError),
        }
        let mut is_float = false;
        if self.peek() == Some('.') {
            is_float = true;
            self.pos += 1;
            if !matches!(self.peek(), Some('0'..='9')) {
                return Err(ParseError);
            }
            while matches!(self.peek(), Some('0'..='9')) {
                self.pos += 1;
            }
        }
        if matches!(self.peek(), Some('e' | 'E')) {
            is_float = true;
            self.pos += 1;
            if matches!(self.peek(), Some('+' | '-')) {
                self.pos += 1;
            }
            if !matches!(self.peek(), Some('0'..='9')) {
                return Err(ParseError);
            }
            while matches!(self.peek(), Some('0'..='9')) {
                self.pos += 1;
            }
        }
        let literal: String = self.chars[start..self.pos].iter().collect();
        if !is_float {
            let digits = literal.strip_prefix('-').unwrap_or(&literal);
            if digits.len() > MAX_INTEGER_DIGITS {
                return Err(ParseError);
            }
            let magnitude = BigInt::parse_bytes(digits.as_bytes(), 10).ok_or(ParseError)?;
            let value = if literal.starts_with('-') {
                -magnitude
            } else {
                magnitude
            };
            return Ok(JsonValue::Num(Num::Int(value)));
        }
        let value: f64 = literal.parse().map_err(|_| ParseError)?;
        Ok(JsonValue::Num(Num::Float(value)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn number(text: &str) -> Num {
        let value = parse_json(text).unwrap();
        match &value {
            JsonValue::Num(number) => number.clone(),
            other => panic!("expected a number from {:?}, got {:?}", text, other),
        }
    }

    #[test]
    fn a_repeated_object_key_keeps_its_last_value() {
        let value = parse_json("{\"a\": 1, \"a\": 2}").unwrap();
        match value.get("a") {
            Some(JsonValue::Num(Num::Int(n))) => assert_eq!(*n, BigInt::from(2)),
            other => panic!("expected the last value, got {:?}", other),
        }
    }

    #[test]
    fn an_integer_literal_too_large_for_a_double_still_parses() {
        let text = format!("1{}", "0".repeat(400));
        let value = number(&text);
        assert!(matches!(value, Num::Int(_)));
        assert!(!value.is_finite());
    }

    #[test]
    fn an_integer_literal_past_the_digit_limit_fails_the_reading() {
        let at_limit = format!("1{}", "0".repeat(4299));
        assert!(parse_json(&at_limit).is_ok());
        let past_limit = format!("1{}", "0".repeat(4300));
        assert!(parse_json(&past_limit).is_err());
    }

    #[test]
    fn a_float_literal_that_overflows_stands_as_an_infinity() {
        match number("1e400") {
            Num::Float(value) => assert_eq!(value, f64::INFINITY),
            other => panic!("expected an infinity, got {:?}", other),
        }
        match number("-1e400") {
            Num::Float(value) => assert_eq!(value, f64::NEG_INFINITY),
            other => panic!("expected an infinity, got {:?}", other),
        }
        let long_literal = format!("{}.0", "1".repeat(5000));
        match number(&long_literal) {
            Num::Float(value) => assert_eq!(value, f64::INFINITY),
            other => panic!("expected an infinity, got {:?}", other),
        }
    }

    #[test]
    fn the_constant_literals_parse_as_python_accepts_them() {
        match number("NaN") {
            Num::Float(value) => assert!(value.is_nan()),
            other => panic!("expected a NaN, got {:?}", other),
        }
        match number("Infinity") {
            Num::Float(value) => assert_eq!(value, f64::INFINITY),
            other => panic!("expected an infinity, got {:?}", other),
        }
        match number("-Infinity") {
            Num::Float(value) => assert_eq!(value, f64::NEG_INFINITY),
            other => panic!("expected an infinity, got {:?}", other),
        }
    }

    #[test]
    fn a_paired_surrogate_escape_combines_into_one_code_point() {
        let value = parse_json("\"\\ud800\\udc00\"").unwrap();
        match &value {
            JsonValue::Str(text) => {
                assert_eq!(text.0, vec![0x10000]);
                assert!(!text.holds_unpaired_surrogate());
            }
            other => panic!("expected a string, got {:?}", other),
        }
    }

    #[test]
    fn unpaired_surrogates_stand_as_their_own_units() {
        let cases: Vec<(&str, Vec<u32>)> = vec![
            ("\"\\ud800x\"", vec![0xD800, 0x78]),
            ("\"\\udc00\\ud800\"", vec![0xDC00, 0xD800]),
            ("\"\\ud800\\ud800\"", vec![0xD800, 0xD800]),
            ("\"a\\udc00\"", vec![0x61, 0xDC00]),
        ];
        for (text, expected) in cases {
            match parse_json(text) {
                Ok(JsonValue::Str(ref parsed)) => {
                    assert_eq!(parsed.0, expected, "for input {}", text);
                    assert!(parsed.holds_unpaired_surrogate());
                }
                other => panic!("expected a string from {}, got {:?}", text, other),
            }
        }
    }

    #[test]
    fn the_number_grammar_refuses_what_python_refuses() {
        for bad in ["01", "1.", ".5", "+1", "1e", "1e+", "-", "--1", "0x1"] {
            assert!(parse_json(bad).is_err(), "accepted {}", bad);
        }
        for good in [
            "0",
            "-0",
            "0.5",
            "-0.5",
            "1e5",
            "1E5",
            "1e+5",
            "1e-5",
            "123456789",
        ] {
            assert!(parse_json(good).is_ok(), "refused {}", good);
        }
    }

    #[test]
    fn malformed_inputs_fail_the_reading() {
        for bad in [
            "",
            "{",
            "[",
            "{\"a\": 1} trailing",
            "\u{feff}{}",
            "\"a\tb\"",
            "'single'",
            "[1,]",
            "{,}",
            "{\"a\" 1}",
            "{\"a\": }",
            "tru",
            "nul",
            "\"unterminated",
            "\"bad \\q escape\"",
        ] {
            assert!(parse_json(bad).is_err(), "accepted {:?}", bad);
        }
    }

    #[test]
    fn scalars_and_empty_containers_parse() {
        assert!(matches!(parse_json("null").unwrap(), JsonValue::Null));
        assert!(matches!(
            parse_json(" true ").unwrap(),
            JsonValue::Bool(true)
        ));
        assert!(matches!(
            parse_json("{}").unwrap(),
            JsonValue::Obj(ref pairs) if pairs.is_empty()
        ));
        assert!(matches!(
            parse_json("[ ]").unwrap(),
            JsonValue::Arr(ref items) if items.is_empty()
        ));
        assert!(matches!(
            parse_json("{\"splits\": \"x\"}").unwrap(),
            JsonValue::Obj(_)
        ));
    }

    #[test]
    fn nesting_at_the_probed_depth_loads_and_past_it_fails() {
        let deep_ok = format!("{}{}", "[".repeat(9997), "]".repeat(9997));
        assert!(parse_json(&deep_ok).is_ok());
        let deep_fail = format!("{}{}", "[".repeat(9998), "]".repeat(9998));
        assert!(parse_json(&deep_fail).is_err());
        let pinned = format!("{}{}", "[".repeat(100000), "]".repeat(100000));
        assert!(parse_json(&pinned).is_err());
        let objects_ok = format!("{}1{}", "{\"a\":".repeat(9997), "}".repeat(9997));
        assert!(parse_json(&objects_ok).is_ok());
        let objects_fail = format!("{}1{}", "{\"a\":".repeat(9998), "}".repeat(9998));
        assert!(parse_json(&objects_fail).is_err());
        let mixed_ok = format!("{}1{}", "[{\"a\":".repeat(4998), "}]".repeat(4998));
        assert!(parse_json(&mixed_ok).is_ok());
        let mixed_fail = format!("{}1{}", "[{\"a\":".repeat(4999), "}]".repeat(4999));
        assert!(parse_json(&mixed_fail).is_err());
        let momentary_ok = format!("{}{}{}", "[".repeat(9996), "[]", "]".repeat(9996));
        assert!(parse_json(&momentary_ok).is_ok());
        let momentary_fail = format!("{}{}{}", "[".repeat(9997), "[]", "]".repeat(9997));
        assert!(parse_json(&momentary_fail).is_err());
    }

    #[test]
    fn a_string_with_an_escaped_control_character_parses() {
        let value = parse_json("\"a\\tb\"").unwrap();
        match &value {
            JsonValue::Str(text) => assert!(text.eq_str("a\tb")),
            other => panic!("expected a string, got {:?}", other),
        }
    }
}
