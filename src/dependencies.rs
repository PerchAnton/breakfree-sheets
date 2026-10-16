use crate::cells::Address;
use std::collections::{BTreeMap, BTreeSet};
#[derive(Default, Debug)]
pub struct Graph {
    pub dependencies: BTreeMap<Address, BTreeSet<Address>>,
}
impl Graph {
    pub fn order(&self) -> (Vec<Address>, BTreeSet<Address>) {
        let mut degree = BTreeMap::new();
        let mut dependents: BTreeMap<Address, BTreeSet<Address>> = BTreeMap::new();
        for (cell, refs) in &self.dependencies {
            let known: Vec<_> = refs
                .iter()
                .filter(|r| self.dependencies.contains_key(r))
                .copied()
                .collect();
            degree.insert(*cell, known.len());
            for reference in known {
                dependents.entry(reference).or_default().insert(*cell);
            }
        }
        let mut ready: BTreeSet<_> = degree
            .iter()
            .filter(|(_, d)| **d == 0)
            .map(|(c, _)| *c)
            .collect();
        let mut ordered = Vec::new();
        while let Some(cell) = ready.pop_first() {
            ordered.push(cell);
            if let Some(items) = dependents.get(&cell) {
                for next in items {
                    let value = degree.get_mut(next).unwrap();
                    *value -= 1;
                    if *value == 0 {
                        ready.insert(*next);
                    }
                }
            }
        }
        let done: BTreeSet<_> = ordered.iter().copied().collect();
        let blocked = self
            .dependencies
            .keys()
            .filter(|c| !done.contains(c))
            .copied()
            .collect();
        (ordered, blocked)
    }
}
