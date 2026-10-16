use crate::{
    cells::{Address, CalcError, Kind, Value},
    dependencies::Graph,
    formula::{self, Expression, Instruction},
    number::Decimal,
    workbook::Workbook,
};
use std::collections::BTreeMap;
fn numeric(value: Option<&Value>) -> Result<Decimal, CalcError> {
    match value {
        None => Ok(Decimal::zero()),
        Some(Value::Number(n)) => Ok(n.clone()),
        Some(Value::Text(_)) => Err(CalcError::new(
            "#VALUE!",
            "Text is not implicitly converted to a number",
        )),
        Some(Value::Error(e)) => Err(e.clone()),
    }
}
fn evaluate(expr: &Expression, values: &BTreeMap<Address, Value>) -> Value {
    let mut stack: Vec<Result<Decimal, CalcError>> = Vec::new();
    for instruction in &expr.instructions {
        match instruction {
            Instruction::Number(n) => stack.push(Ok(n.clone())),
            Instruction::Reference(cell) => stack.push(numeric(values.get(cell))),
            Instruction::Negate => {
                let value = stack.pop().unwrap();
                stack.push(value.map(|n| n.negate()));
            }
            Instruction::Binary(op) => {
                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();
                stack.push(left.and_then(|l| {
                    right.and_then(|r| {
                        l.operation(*op, &r)
                            .map_err(|code| CalcError::new(code, "Arithmetic failed"))
                    })
                }));
            }
        }
    }
    match stack.pop().unwrap() {
        Ok(n) => Value::Number(n),
        Err(e) => Value::Error(e),
    }
}
#[derive(Default)]
pub struct Calculator {
    pub graph: Graph,
}
impl Calculator {
    pub fn calculate(&mut self, book: &Workbook) -> BTreeMap<Address, Value> {
        let mut compiled = BTreeMap::new();
        self.graph.dependencies.clear();
        for (address, cell) in book.cells() {
            let mut refs = Default::default();
            if cell.kind == Kind::Formula {
                let expression = formula::parse(&cell.source);
                if let Ok(expr) = &expression {
                    refs = expr.references.clone();
                }
                compiled.insert(*address, expression);
            }
            self.graph.dependencies.insert(*address, refs);
        }
        let (order, blocked) = self.graph.order();
        let mut values = BTreeMap::new();
        for address in order {
            let cell = book.get(address).unwrap();
            let value = match cell.kind {
                Kind::Text => Value::Text(cell.source.clone()),
                Kind::Number => Value::Number(
                    Decimal::parse(&cell.source).expect("model validates numeric source"),
                ),
                Kind::Formula => match &compiled[&address] {
                    Ok(expr) => evaluate(expr, &values),
                    Err(e) => Value::Error(e.clone()),
                },
            };
            values.insert(address, value);
        }
        for address in blocked {
            values.insert(
                address,
                Value::Error(CalcError::new(
                    "#CYCLE!",
                    "Cell participates in or depends on a cycle",
                )),
            );
        }
        values
    }
}
