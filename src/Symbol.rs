use std::collections::HashMap;

#[derive(Debug)]
pub struct SymbolTable {
    table: HashMap<String, usize>,
}

impl SymbolTable {
    pub fn new() -> Self {
        let mut table = HashMap::new();
        for i in 0..16 {
            table.insert(format!("R{}", i), i);
        }
        SymbolTable {
            table
        }
    }
    pub fn get(&self, q: &str) -> Option<&usize> {
        self.table.get(q)
    }
}