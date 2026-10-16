use crate::cells::{Address, Cell};
use std::collections::BTreeMap;
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Workbook {
    cells: BTreeMap<Address, Cell>,
}
impl Workbook {
    pub fn cells(&self) -> &BTreeMap<Address, Cell> {
        &self.cells
    }
    pub fn get(&self, address: Address) -> Option<&Cell> {
        self.cells.get(&address)
    }
    pub fn set(&mut self, address: Address, cell: Option<Cell>) -> Result<(), String> {
        if Address::new(address.col, address.row).is_none() {
            return Err("Address outside A1:Z100".into());
        }
        if let Some(cell) = cell {
            cell.validate()?;
            self.cells.insert(address, cell);
        } else {
            self.cells.remove(&address);
        }
        Ok(())
    }
}
