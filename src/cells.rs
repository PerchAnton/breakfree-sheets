use crate::number::Decimal;
use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};
pub const ROWS: usize = 100;
pub const COLS: usize = 26;
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Address {
    pub col: usize,
    pub row: usize,
}
impl Address {
    pub fn new(col: usize, row: usize) -> Option<Self> {
        (col < COLS && row < ROWS).then_some(Self { col, row })
    }
}
impl FromStr for Address {
    type Err = String;
    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let bytes = text.as_bytes();
        if !(2..=4).contains(&bytes.len())
            || !bytes[0].is_ascii_alphabetic()
            || bytes[1] == b'0'
            || !bytes[1..].iter().all(u8::is_ascii_digit)
        {
            return Err("Reference must be A1 through Z100".into());
        }
        let col = (bytes[0].to_ascii_uppercase() - b'A') as usize;
        let row = text[1..].parse::<usize>().map_err(|_| "Invalid row")?;
        Self::new(col, row - 1).ok_or_else(|| "Reference must be A1 through Z100".into())
    }
}
impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", (b'A' + self.col as u8) as char, self.row + 1)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Kind {
    Text,
    Number,
    Formula,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Cell {
    pub kind: Kind,
    pub source: String,
}
impl Cell {
    pub fn new(kind: Kind, source: impl Into<String>) -> Result<Self, String> {
        let cell = Self {
            kind,
            source: source.into(),
        };
        cell.validate()?;
        Ok(cell)
    }
    pub fn validate(&self) -> Result<(), String> {
        if self.source.chars().count() > 4096 {
            return Err("Cell source exceeds 4096 characters".into());
        }
        if self.kind == Kind::Number {
            Decimal::parse(&self.source)?;
        }
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CalcError {
    pub code: &'static str,
    pub message: String,
}
impl CalcError {
    pub fn new(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Value {
    Number(Decimal),
    Text(String),
    Error(CalcError),
}
impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(n) => n.fmt(f),
            Self::Text(t) => f.write_str(t),
            Self::Error(e) => f.write_str(e.code),
        }
    }
}
