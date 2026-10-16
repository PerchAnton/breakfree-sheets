use crate::{
    calculation::Calculator,
    cells::{Address, Cell, Value},
    workbook::Workbook,
};
use std::collections::BTreeMap;
#[derive(Clone)]
struct Change {
    address: Address,
    before: Option<Cell>,
    after: Option<Cell>,
}
pub struct Editor {
    pub book: Workbook,
    pub values: BTreeMap<Address, Value>,
    calculator: Calculator,
    undo: Vec<Vec<Change>>,
    redo: Vec<Vec<Change>>,
    saved: Workbook,
}
impl Default for Editor {
    fn default() -> Self {
        Self::new(Workbook::default())
    }
}
impl Editor {
    pub fn new(book: Workbook) -> Self {
        let mut calculator = Calculator::default();
        let values = calculator.calculate(&book);
        Self {
            saved: book.clone(),
            book,
            values,
            calculator,
            undo: Vec::new(),
            redo: Vec::new(),
        }
    }
    pub fn set(&mut self, address: Address, cell: Option<Cell>) -> Result<(), String> {
        self.apply(vec![(address, cell)])
    }
    pub fn apply(&mut self, edits: Vec<(Address, Option<Cell>)>) -> Result<(), String> {
        let mut candidate = self.book.clone();
        for (address, cell) in &edits {
            candidate.set(*address, cell.clone())?;
        }
        let changes: Vec<_> = edits
            .into_iter()
            .filter_map(|(address, after)| {
                let before = self.book.get(address).cloned();
                (before != after).then_some(Change {
                    address,
                    before,
                    after,
                })
            })
            .collect();
        if changes.is_empty() {
            return Ok(());
        }
        self.book = candidate;
        self.undo.push(changes);
        if self.undo.len() > 500 {
            self.undo.remove(0);
        }
        self.redo.clear();
        self.recalculate();
        Ok(())
    }
    pub fn recalculate(&mut self) {
        self.values = self.calculator.calculate(&self.book);
    }
    pub fn dirty(&self) -> bool {
        self.book != self.saved
    }
    pub fn mark_saved(&mut self) {
        self.saved = self.book.clone();
    }
    pub fn can_undo(&self) -> bool {
        !self.undo.is_empty()
    }
    pub fn can_redo(&self) -> bool {
        !self.redo.is_empty()
    }
    pub fn undo(&mut self) {
        if let Some(changes) = self.undo.pop() {
            for change in &changes {
                self.book
                    .set(change.address, change.before.clone())
                    .unwrap();
            }
            self.redo.push(changes);
            self.recalculate();
        }
    }
    pub fn redo(&mut self) {
        if let Some(changes) = self.redo.pop() {
            for change in &changes {
                self.book.set(change.address, change.after.clone()).unwrap();
            }
            self.undo.push(changes);
            self.recalculate();
        }
    }
}
