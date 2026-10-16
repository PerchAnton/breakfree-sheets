//! Iterative restricted parser; never dispatches document text as executable code.
use crate::{
    cells::{Address, CalcError},
    number::Decimal,
};
use std::collections::BTreeSet;
#[derive(Clone, Debug)]
pub enum Instruction {
    Number(Decimal),
    Reference(Address),
    Binary(char),
    Negate,
}
#[derive(Clone, Debug)]
pub struct Expression {
    pub instructions: Vec<Instruction>,
    pub references: BTreeSet<Address>,
}
fn syntax(message: &str) -> CalcError {
    CalcError::new("#SYNTAX!", message)
}
fn priority(op: char) -> u8 {
    match op {
        '+' | '-' => 1,
        '*' | '/' => 2,
        '~' => 3,
        _ => 0,
    }
}
fn emit(op: char) -> Instruction {
    if op == '~' {
        Instruction::Negate
    } else {
        Instruction::Binary(op)
    }
}
pub fn parse(source: &str) -> Result<Expression, CalcError> {
    if source.chars().count() > 1024 {
        return Err(CalcError::new("#LIMIT!", "Formula exceeds 1024 characters"));
    }
    let text = source
        .strip_prefix('=')
        .ok_or_else(|| syntax("Formula must start with ="))?
        .trim();
    let bytes = text.as_bytes();
    let mut pos = 0;
    let mut tokens = 0;
    let mut output = Vec::new();
    let mut operators = Vec::new();
    let mut references = BTreeSet::new();
    let mut need_value = true;
    while pos < bytes.len() {
        let ch = text[pos..].chars().next().unwrap();
        if ch.is_whitespace() {
            pos += ch.len_utf8();
            continue;
        }
        tokens += 1;
        if tokens > 512 {
            return Err(CalcError::new("#LIMIT!", "Formula exceeds 512 tokens"));
        }
        if ch.is_ascii_digit() || ch == '.' {
            if ch == '.' && !bytes.get(pos + 1).is_some_and(u8::is_ascii_digit) {
                return Err(syntax("A decimal point needs digits"));
            }
            if !need_value {
                return Err(syntax("Expected an operator between values"));
            }
            let start = pos;
            while pos < bytes.len() && bytes[pos].is_ascii_digit() {
                pos += 1;
            }
            if pos < bytes.len() && bytes[pos] == b'.' {
                pos += 1;
                while pos < bytes.len() && bytes[pos].is_ascii_digit() {
                    pos += 1;
                }
            }
            let numeric =
                Decimal::parse(&text[start..pos]).map_err(|e| CalcError::new("#NUM!", e))?;
            output.push(Instruction::Number(numeric));
            need_value = false;
        } else if ch.is_ascii_alphabetic() {
            if !need_value {
                return Err(syntax("Expected an operator"));
            }
            let start = pos;
            while pos < bytes.len() && bytes[pos].is_ascii_alphabetic() {
                pos += 1;
            }
            let digits_start = pos;
            while pos < bytes.len() && bytes[pos].is_ascii_digit() {
                pos += 1;
            }
            if digits_start == pos {
                return Err(syntax("Functions and names are unsupported"));
            }
            let address = text[start..pos]
                .parse::<Address>()
                .map_err(|e| CalcError::new("#REF!", e))?;
            references.insert(address);
            output.push(Instruction::Reference(address));
            need_value = false;
        } else {
            pos += ch.len_utf8();
            match ch {
                '(' if need_value => operators.push(ch),
                ')' if !need_value => {
                    while operators.last().is_some_and(|op| *op != '(') {
                        output.push(emit(operators.pop().unwrap()));
                    }
                    if operators.pop() != Some('(') {
                        return Err(syntax("Unmatched closing parenthesis"));
                    }
                }
                '-' if need_value => operators.push('~'),
                '+' | '-' | '*' | '/' if !need_value => {
                    while operators
                        .last()
                        .is_some_and(|op| *op != '(' && priority(*op) >= priority(ch))
                    {
                        output.push(emit(operators.pop().unwrap()));
                    }
                    operators.push(ch);
                    need_value = true;
                }
                _ => return Err(syntax("Malformed or unsupported formula syntax")),
            }
        }
    }
    if need_value {
        return Err(syntax("Formula is empty or ends with an operator"));
    }
    while let Some(op) = operators.pop() {
        if op == '(' {
            return Err(syntax("Unmatched opening parenthesis"));
        }
        output.push(emit(op));
    }
    Ok(Expression {
        instructions: output,
        references,
    })
}
