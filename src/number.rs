//! Bounded decimal floating point, rounded from exact rationals after each operation.
use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{One, Signed, Zero};
use std::{cmp::Ordering, str::FromStr};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Decimal {
    coefficient: BigInt,
    exponent: i32,
}
fn ten(power: u32) -> BigInt {
    BigInt::from(10).pow(power)
}
impl Decimal {
    pub fn zero() -> Self {
        Self {
            coefficient: BigInt::zero(),
            exponent: 0,
        }
    }
    pub fn parse(source: &str) -> Result<Self, String> {
        if source.is_empty() || source.len() > 4096 {
            return Err("Invalid decimal length".into());
        }
        let unsigned = source.strip_prefix(['-', '+']).unwrap_or(source);
        let mut dot = false;
        let mut fraction = 0;
        let mut digits = String::new();
        for ch in unsigned.chars() {
            match ch {
                '.' if !dot => dot = true,
                '0'..='9' => {
                    digits.push(ch);
                    if dot {
                        fraction += 1;
                    }
                }
                _ => {
                    return Err(
                        "Use decimal digits and a dot; no grouping or exponent notation".into(),
                    );
                }
            }
        }
        if digits.is_empty() {
            return Err("A decimal needs digits".into());
        }
        let significant = digits.trim_start_matches('0');
        let count = significant.len().max(1);
        let adjusted = if significant.is_empty() {
            -fraction
        } else {
            significant.len() as i32 - fraction - 1
        };
        if count > 28 || !(-999..=999).contains(&adjusted) {
            return Err("At most 28 significant digits; adjusted exponent -999 through 999".into());
        }
        let mut coefficient = BigInt::from_str(&digits).map_err(|_| "Invalid decimal")?;
        if source.starts_with('-') {
            coefficient = -coefficient;
        }
        Ok(Self::normalized(coefficient, -fraction))
    }
    fn normalized(mut coefficient: BigInt, mut exponent: i32) -> Self {
        if coefficient.is_zero() {
            return Self::zero();
        }
        while (&coefficient % 10u32).is_zero() {
            coefficient /= 10u32;
            exponent += 1;
        }
        Self {
            coefficient,
            exponent,
        }
    }
    fn rational(&self) -> BigRational {
        if self.exponent >= 0 {
            BigRational::from_integer(&self.coefficient * ten(self.exponent as u32))
        } else {
            BigRational::new(self.coefficient.clone(), ten((-self.exponent) as u32))
        }
    }
    fn rounded(value: BigRational) -> Result<Self, &'static str> {
        if value.is_zero() {
            return Ok(Self::zero());
        }
        let negative = value.is_negative();
        let numerator = value.numer().abs();
        let denominator = value.denom();
        let mut adjusted =
            numerator.to_str_radix(10).len() as i32 - denominator.to_str_radix(10).len() as i32;
        let too_small = if adjusted >= 0 {
            numerator < denominator * ten(adjusted as u32)
        } else {
            &numerator * ten((-adjusted) as u32) < *denominator
        };
        if too_small {
            adjusted -= 1;
        }
        if adjusted > 999 {
            return Err("#NUM!");
        }
        let exponent = (adjusted - 27).max(-1026);
        let (n, d) = if exponent >= 0 {
            (numerator, denominator * ten(exponent as u32))
        } else {
            (numerator * ten((-exponent) as u32), denominator.clone())
        };
        let mut quotient = &n / &d;
        let remainder = &n % &d;
        if adjusted < -999 && !remainder.is_zero() {
            return Err("#NUM!");
        }
        match (&remainder * 2u32).cmp(&d) {
            Ordering::Greater => quotient += 1u32,
            Ordering::Equal if (&quotient % 2u32) == BigInt::one() => quotient += 1u32,
            _ => {}
        }
        if negative {
            quotient = -quotient;
        }
        let result = Self::normalized(quotient, exponent);
        if !result.is_zero() && result.adjusted() > 999 {
            return Err("#NUM!");
        }
        Ok(result)
    }
    fn adjusted(&self) -> i32 {
        self.coefficient.abs().to_str_radix(10).len() as i32 + self.exponent - 1
    }
    pub fn is_zero(&self) -> bool {
        self.coefficient.is_zero()
    }
    pub fn negate(&self) -> Self {
        Self {
            coefficient: -&self.coefficient,
            exponent: self.exponent,
        }
    }
    pub fn operation(&self, op: char, right: &Self) -> Result<Self, &'static str> {
        if op == '/' && right.is_zero() {
            return Err("#DIV/0!");
        }
        let (left, right) = (self.rational(), right.rational());
        let exact = match op {
            '+' => left + right,
            '-' => left - right,
            '*' => left * right,
            '/' => left / right,
            _ => return Err("#SYNTAX!"),
        };
        Self::rounded(exact)
    }
}
impl std::fmt::Display for Decimal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_zero() {
            return f.write_str("0");
        }
        let sign = if self.coefficient.is_negative() {
            "-"
        } else {
            ""
        };
        let digits = self.coefficient.abs().to_str_radix(10);
        let adjusted = self.adjusted();
        if !(-6..28).contains(&adjusted) {
            let rest = if digits.len() > 1 {
                format!(".{}", &digits[1..])
            } else {
                String::new()
            };
            return write!(f, "{sign}{}{rest}E{adjusted:+}", &digits[..1]);
        }
        let point = digits.len() as i32 + self.exponent;
        let body = if point <= 0 {
            format!("0.{}{}", "0".repeat((-point) as usize), digits)
        } else if point >= digits.len() as i32 {
            format!("{}{}", digits, "0".repeat(point as usize - digits.len()))
        } else {
            format!(
                "{}.{}",
                &digits[..point as usize],
                &digits[point as usize..]
            )
        };
        write!(f, "{sign}{body}")
    }
}
