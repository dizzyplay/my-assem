use std::collections::HashMap;

mod Symbol{
    use std::collections::HashMap;
    #[derive(Debug)]
    pub struct SymbolTable {
        table:HashMap<String,usize>
    }

    impl SymbolTable {
        pub fn new()->Self{
            let mut table = HashMap::new();
            for i in 0..15 {
                table.insert(format!("R{}",i),i);
            }
            SymbolTable{
                table
            }
        }
        pub fn get(&self, q:&str) -> Some(usize) {
            self.table.get(q)
        }
    }
}

fn main() {
    let mut symbol_table = Symbol::SymbolTable::new();

    println!("{:?}",symbol_table.get("R15"));
}


