//! Numbers with the semantics the Python implementation reads by.
//!
//! A JSON number in the Python reading is an int of unbounded size
//! or a float holding an IEEE double, and the two compare and combine
//! under Python rules. This module carries that pair as one type.
//! Comparisons read the exact values, so an int and a float compare
//! as the Python comparison compares them, and figures the readings
//! compute on exact values travel as exact fractions end to end.

use std::cmp::Ordering;

use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{One, Signed, ToPrimitive, Zero};

/// A number as the Python reading holds it, an exact integer or a double.
#[derive(Clone, Debug)]
pub enum Num {
    Int(BigInt),
    Float(f64),
}

impl PartialEq for Num {
    /// Equality as Python reads it, on exact values across the pair.
    fn eq(&self, other: &Num) -> bool {
        self.eq_py(other)
    }
}

impl Num {
    pub fn from_i64(value: i64) -> Num {
        Num::Int(BigInt::from(value))
    }

    pub fn from_f64(value: f64) -> Num {
        Num::Float(value)
    }

    /// Whether the number is finite, as math.isfinite reads it.
    ///
    /// An integer too large to convert to a float counts as not
    /// finite, so a loader names it with the finite number defect
    /// instead of failing on the conversion. The conversion of an
    /// integer overflows exactly when its magnitude reaches the
    /// halfway point between the largest finite double and the next
    /// power of two, which is 2 to the 1024 less 2 to the 970.
    pub fn is_finite(&self) -> bool {
        match self {
            Num::Int(value) => {
                let limit = (BigInt::one() << 1024usize) - (BigInt::one() << 970usize);
                value.abs() < limit
            }
            Num::Float(value) => value.is_finite(),
        }
    }

    /// The exact value of the number as a fraction.
    ///
    /// Every caller reads a value a loader has already found finite,
    /// so the conversion of a float always has an exact fraction.
    pub fn to_rational(&self) -> BigRational {
        match self {
            Num::Int(value) => BigRational::from(value.clone()),
            Num::Float(value) => BigRational::from_float(*value)
                .expect("a finite double always has an exact fraction"),
        }
    }

    /// The value of the number as a double, rounded as Python rounds.
    ///
    /// An integer converts by correct rounding, halves to even. An
    /// integer beyond the double range converts to an infinity here
    /// where Python would raise, and no loader or reading reaches
    /// this conversion with such a value, because the finite check
    /// refuses it first.
    pub fn to_f64(&self) -> f64 {
        match self {
            Num::Int(value) => rational_to_f64(&BigRational::from(value.clone())),
            Num::Float(value) => *value,
        }
    }

    fn class_rank(&self) -> (i32, Option<BigRational>) {
        match self {
            Num::Int(_) => (1, Some(self.to_rational())),
            Num::Float(value) => {
                if value.is_nan() {
                    (3, None)
                } else if *value == f64::INFINITY {
                    (2, None)
                } else if *value == f64::NEG_INFINITY {
                    (0, None)
                } else {
                    (1, Some(self.to_rational()))
                }
            }
        }
    }

    /// Compare two numbers as Python compares them, on exact values.
    pub fn cmp_py(&self, other: &Num) -> Ordering {
        let (rank_a, value_a) = self.class_rank();
        let (rank_b, value_b) = other.class_rank();
        if rank_a != rank_b {
            return rank_a.cmp(&rank_b);
        }
        match (value_a, value_b) {
            (Some(a), Some(b)) => a.cmp(&b),
            _ => Ordering::Equal,
        }
    }

    /// Whether two numbers are equal as Python reads equality.
    pub fn eq_py(&self, other: &Num) -> bool {
        if let (Num::Float(a), Num::Float(b)) = (self, other) {
            if a.is_nan() || b.is_nan() {
                return false;
            }
        }
        self.cmp_py(other) == Ordering::Equal
    }

    /// Subtract as Python subtracts.
    ///
    /// Two integers subtract exactly. Any other pair subtracts as
    /// doubles, with an integer operand converted to a double first.
    pub fn sub(&self, other: &Num) -> Num {
        match (self, other) {
            (Num::Int(a), Num::Int(b)) => Num::Int(a - b),
            _ => Num::Float(self.to_f64() - other.to_f64()),
        }
    }

    /// Negate the number.
    pub fn neg(&self) -> Num {
        match self {
            Num::Int(value) => Num::Int(-value.clone()),
            Num::Float(value) => Num::Float(-value),
        }
    }
}

/// Convert an exact fraction to the nearest double, halves to even.
///
/// This is the conversion Python performs for float of a fraction,
/// correctly rounded at every magnitude, subnormal values and the
/// overflow into infinity included.
pub fn rational_to_f64(value: &BigRational) -> f64 {
    let numer = value.numer();
    let denom = value.denom();
    if numer.is_zero() {
        return 0.0;
    }
    let negative = numer.is_negative();
    let p = numer.abs();
    let q = denom.clone();
    let mut e: i64 = p.bits() as i64 - q.bits() as i64;
    let at_least = if e >= 0 {
        p >= (q.clone() << (e as usize))
    } else {
        (p.clone() << ((-e) as usize)) >= q
    };
    if !at_least {
        e -= 1;
    }
    let shift: i64 = if e >= -1022 { 52 - e } else { 1074 };
    let (num, den) = if shift >= 0 {
        (p.clone() << (shift as usize), q.clone())
    } else {
        (p.clone(), q.clone() << ((-shift) as usize))
    };
    let mut m = &num / &den;
    let rem = &num % &den;
    let twice = &rem + &rem;
    match twice.cmp(&den) {
        Ordering::Greater => m += BigInt::one(),
        Ordering::Equal => {
            if (&m % BigInt::from(2)).is_one() {
                m += BigInt::one();
            }
        }
        Ordering::Less => {}
    }
    let bits: u64 = if e >= -1022 {
        let mut e = e;
        if m == (BigInt::one() << 53usize) {
            m >>= 1;
            e += 1;
        }
        if e > 1023 {
            return if negative {
                f64::NEG_INFINITY
            } else {
                f64::INFINITY
            };
        }
        (((e + 1023) as u64) << 52) | (m - (BigInt::one() << 52usize)).to_u64().unwrap()
    } else {
        m.to_u64().unwrap()
    };
    let out = f64::from_bits(bits);
    if negative {
        -out
    } else {
        out
    }
}

/// Round an exact fraction to the nearest integer, halves to even.
///
/// This mirrors Python round on a value at a stated precision, read
/// on the exact value, and it is an odd function, so a negative value
/// rounds as the rounding of its magnitude with the sign restored.
pub fn round_half_even(value: &BigRational) -> BigInt {
    let numer = value.numer();
    let denom = value.denom();
    let magnitude = numer.abs();
    let mut units = &magnitude / denom;
    let rem = &magnitude % denom;
    let twice = &rem + &rem;
    match twice.cmp(denom) {
        Ordering::Greater => units += BigInt::one(),
        Ordering::Equal => {
            if (&units % BigInt::from(2)).is_one() {
                units += BigInt::one();
            }
        }
        Ordering::Less => {}
    }
    if numer.is_negative() {
        -units
    } else {
        units
    }
}

/// Divide two integers the way Python floor division divides them.
///
/// Returns the quotient and the remainder, with the remainder
/// carrying the sign of the divisor, exactly as Python divmod reads.
pub fn floor_divmod(value: &BigInt, divisor: &BigInt) -> (BigInt, BigInt) {
    let quotient = value / divisor;
    let remainder = value % divisor;
    if !remainder.is_zero() && ((remainder.is_negative()) != (divisor.is_negative())) {
        (quotient - BigInt::one(), remainder + divisor.clone())
    } else {
        (quotient, remainder)
    }
}

/// The repr Python prints for a float, in the range format_seconds reads.
///
/// format_seconds asks for the repr only of a nonzero double whose
/// magnitude rounds to zero at six places, so the magnitude is below
/// one part in two million and Python always renders the repr in
/// scientific notation, with the shortest digits that round trip, an
/// exponent sign, and at least two exponent digits.
pub fn py_float_repr(value: f64) -> String {
    let text = format!("{:e}", value);
    let (mantissa, exponent) = text
        .split_once('e')
        .expect("lower exp always names an exponent");
    let exponent: i32 = exponent.parse().expect("the exponent is a small integer");
    let sign = if exponent < 0 { "-" } else { "+" };
    format!("{}e{}{:02}", mantissa, sign, exponent.abs())
}

/// Render a number of seconds the way format_seconds renders it.
///
/// Values render exactly at up to six decimal places, read on the
/// exact double by round half to even at the sixth place. A nonzero
/// value that rounds to zero at that precision renders at full
/// precision instead, so a lead never prints as zero.
pub fn format_seconds_f64(value: f64) -> String {
    if !value.is_finite() {
        // Python raises on a non finite figure here, and no loader or
        // reading hands one over, so this arm never renders.
        return value.to_string();
    }
    let scaled = BigRational::from_float(value).expect("a finite double has an exact fraction")
        * BigRational::from(BigInt::from(1_000_000));
    let units = round_half_even(&scaled);
    if units.is_zero() {
        if value != 0.0 {
            return py_float_repr(value);
        }
        return "0".to_string();
    }
    let negative = units.is_negative();
    let magnitude = units.abs();
    let whole = &magnitude / BigInt::from(1_000_000);
    let part = (&magnitude % BigInt::from(1_000_000)).to_u64().unwrap();
    let sign = if negative { "-" } else { "" };
    if part == 0 {
        format!("{}{}", sign, whole)
    } else {
        let digits = format!("{:06}", part);
        format!("{}{}.{}", sign, whole, digits.trim_end_matches('0'))
    }
}

/// Render a number of seconds held as a Num.
pub fn format_seconds(value: &Num) -> String {
    format_seconds_f64(value.to_f64())
}

/// Render a number of seconds held as an exact fraction.
///
/// Python reads float of the fraction inside format_seconds, so the
/// fraction converts to the nearest double first.
pub fn format_seconds_fraction(value: &BigRational) -> String {
    format_seconds_f64(rational_to_f64(value))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn frac(numer: i64, denom: i64) -> BigRational {
        BigRational::new(BigInt::from(numer), BigInt::from(denom))
    }

    #[test]
    fn an_integer_is_finite_inside_the_double_range() {
        assert!(Num::from_i64(1800).is_finite());
        let huge = Num::Int(BigInt::from(10).pow(400));
        assert!(!huge.is_finite());
        let near_top = Num::Int((BigInt::one() << 1023usize) + BigInt::one());
        assert!(near_top.is_finite());
    }

    #[test]
    fn a_float_is_finite_when_the_double_is() {
        assert!(Num::from_f64(2.5).is_finite());
        assert!(!Num::from_f64(f64::NAN).is_finite());
        assert!(!Num::from_f64(f64::INFINITY).is_finite());
        assert!(!Num::from_f64(f64::NEG_INFINITY).is_finite());
    }

    #[test]
    fn integers_and_floats_compare_on_exact_values() {
        assert_eq!(
            Num::from_i64(90).cmp_py(&Num::from_f64(90.0)),
            Ordering::Equal
        );
        let two_53_plus_1 = Num::Int((BigInt::one() << 53usize) + BigInt::one());
        let two_53 = Num::from_f64(9007199254740992.0);
        assert_eq!(two_53_plus_1.cmp_py(&two_53), Ordering::Greater);
        assert_eq!(
            Num::from_f64(0.1).cmp_py(&Num::Int(BigInt::from(0))),
            Ordering::Greater
        );
        assert!(Num::from_i64(90).eq_py(&Num::from_f64(90.0)));
        assert!(!Num::from_f64(f64::NAN).eq_py(&Num::from_f64(f64::NAN)));
    }

    #[test]
    fn subtraction_matches_python_arithmetic() {
        match Num::from_i64(1500).sub(&Num::from_i64(1320)) {
            Num::Int(value) => assert_eq!(value, BigInt::from(180)),
            other => panic!("expected an integer difference, got {:?}", other),
        }
        match Num::from_f64(1.0000004).sub(&Num::from_f64(1.0)) {
            Num::Float(value) => assert_eq!(value, 1.0000004f64 - 1.0),
            other => panic!("expected a float difference, got {:?}", other),
        }
        match Num::from_i64(5).sub(&Num::from_f64(2.5)) {
            Num::Float(value) => assert_eq!(value, 2.5),
            other => panic!("expected a float difference, got {:?}", other),
        }
    }

    #[test]
    fn rational_to_f64_rounds_like_python() {
        assert_eq!(rational_to_f64(&frac(1, 3)), 1.0f64 / 3.0);
        assert_eq!(rational_to_f64(&frac(-1, 3)), -1.0f64 / 3.0);
        assert_eq!(rational_to_f64(&frac(1, 2)), 0.5);
        let two_53_plus_1 = BigRational::from((BigInt::one() << 53usize) + BigInt::one());
        assert_eq!(rational_to_f64(&two_53_plus_1), 9007199254740992.0);
        let sub_half = BigRational::new(BigInt::one(), BigInt::one() << 1075usize);
        assert_eq!(rational_to_f64(&sub_half), 0.0);
        let sub_three = BigRational::new(BigInt::from(3), BigInt::one() << 1075usize);
        assert_eq!(rational_to_f64(&sub_three), 1e-323);
        let sub_five = BigRational::new(BigInt::from(5), BigInt::one() << 1075usize);
        assert_eq!(rational_to_f64(&sub_five), 1e-323);
        let sub_one = BigRational::new(BigInt::one(), BigInt::one() << 1074usize);
        assert_eq!(rational_to_f64(&sub_one), 5e-324);
        let huge = BigRational::from(BigInt::from(10).pow(400));
        assert_eq!(rational_to_f64(&huge), f64::INFINITY);
    }

    #[test]
    fn the_repr_of_a_small_double_matches_python() {
        assert_eq!(
            py_float_repr(4.000000413701855e-07),
            "4.000000413701855e-07"
        );
        assert_eq!(py_float_repr(1e-07), "1e-07");
        assert_eq!(py_float_repr(2.5e-07), "2.5e-07");
        assert_eq!(py_float_repr(5e-324), "5e-324");
        assert_eq!(py_float_repr(-3e-07), "-3e-07");
    }

    #[test]
    fn format_seconds_matches_the_python_renders() {
        assert_eq!(format_seconds_f64(0.0), "0");
        assert_eq!(format_seconds_f64(-0.0), "0");
        assert_eq!(format_seconds_f64(2700.0), "2700");
        assert_eq!(format_seconds_f64(2.5), "2.5");
        assert_eq!(format_seconds_f64(0.49999999999999994), "0.5");
        assert_eq!(
            format_seconds_f64(4.000000413701855e-07),
            "4.000000413701855e-07"
        );
        assert_eq!(format_seconds_f64(1e-07), "1e-07");
        assert_eq!(format_seconds_f64(30000000.0), "30000000");
        assert_eq!(format_seconds_f64(88.57142857142857), "88.571429");
        assert_eq!(format_seconds_f64(0.1 + 0.2), "0.3");
        assert_eq!(format_seconds_f64(1234567.8910112), "1234567.891011");
        assert_eq!(format_seconds_f64(60.00000000000001), "60");
        assert_eq!(format_seconds_f64(771.4285714285714), "771.428571");
        assert_eq!(format_seconds(&Num::from_i64(1320)), "1320");
        assert_eq!(format_seconds_fraction(&frac(540000, 7)), "77142.857143");
    }

    #[test]
    fn floor_divmod_matches_python() {
        assert_eq!(
            floor_divmod(&BigInt::from(7), &BigInt::from(2)),
            (BigInt::from(3), BigInt::from(1))
        );
        assert_eq!(
            floor_divmod(&BigInt::from(-7), &BigInt::from(2)),
            (BigInt::from(-4), BigInt::from(1))
        );
        assert_eq!(
            floor_divmod(&BigInt::from(-6), &BigInt::from(3)),
            (BigInt::from(-2), BigInt::from(0))
        );
    }
}
