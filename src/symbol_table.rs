use std::collections::HashMap;

struct SymbolTable {
    table: HashMap<String, usize>,
    variable_cursor: usize,
}

impl SymbolTable {
    fn new() -> Self {
        let mut table = HashMap::new();
        for n in 0..16 {
            table.insert(format!("R{}", n), n);
        }
        table.insert(format!("SP"), 0);
        table.insert(format!("LCL"), 1);
        table.insert(format!("ARG"), 2);
        table.insert(format!("THIS"), 3);
        table.insert(format!("THAT"), 4);

        table.insert(format!("SCREEN"), 16384);
        table.insert(format!("KBD"), 24576);
        SymbolTable {
            table,
            variable_cursor: 16,
        }
    }
    fn insert(&mut self, symbol:String, address:Option<usize>) {
        match address{
            Some(s) =>{
                if symbol.matches(char::is_lowercase).collect::<Vec<&str>>().len() == 0 {
                    println!("{:?}",symbol);
                    println!("{:?}",symbol.matches(char::is_lowercase).collect::<Vec<&str>>());
                    self.table.insert(symbol, s);
                } else{
                    panic!("not allowed lowercase in label symbol..!")
                }
            },
            None => {
                if symbol.matches(char::is_uppercase).collect::<Vec<&str>>().len() == 0 {
                    match self.table.get(symbol.as_str()) {
                        Some(_) => { },
                        None => {
                            self.table.insert(symbol, self.variable_cursor);
                            self.variable_cursor += 1;
                        }
                    }
                }else {
                    panic!("not allowed uppercase in variable symbol..!")
                }
            }
        };
    }
    fn get(&self, symbol:String) -> usize {
        match self.table.get(symbol.as_str()) {
            Some(n) => *n,
            None => 0
        }
    }
}

#[test]
#[should_panic(expected = "not allowed lowercase in label symbol..!")]
fn uppercase_test_panic(){
    let mut s = SymbolTable::new();
    s.insert(format!("loop"), Some(5));
}

#[test]
#[should_panic(expected = "not allowed uppercase in variable symbol..!")]
fn variable_only_allowed_lowercase_test_panic(){
    let mut s = SymbolTable::new();
    s.insert(format!("LOOP"), None);
}

#[test]
fn insert_variable(){
    let mut s = SymbolTable::new();
    s.insert(format!("x"),None);
    assert_eq!(s.get(format!("x")), 16);
    s.insert(format!("x"),None);
    assert_eq!(s.get(format!("x")), 16);

    s.insert(format!("y"),None);
    assert_eq!(s.get(format!("y")), 17);
    s.insert(format!("y"),None);
    assert_eq!(s.get(format!("y")), 17);
}

#[test]
fn insert_label(){
    let mut s = SymbolTable::new();
    s.insert(format!("LOOP"),Some(1));
    assert_eq!(s.get(format!("LOOP")), 1);
    s.insert(format!("LOOP"),Some(1));
    assert_eq!(s.get(format!("LOOP")), 1);

    s.insert(format!("END"),Some(10));
    assert_eq!(s.get(format!("END")), 10);
}
