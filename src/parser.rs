use std::fs::{File};
use std::io::{Error, Read, BufReader, BufRead};

pub struct Parser {
    file: BufReader<File>,
    current: Option<String>,
}

impl Parser {
    pub fn new(filename: &str) -> Result<Self, Error> {
        let f = File::open(filename)?;
        let f = BufReader::new(f);
        Ok(
            Parser {
                file: f,
                current: None,
            }
        )
    }

    pub fn advance(&mut self) -> Option<String> {
        loop {
            match self.file.by_ref().lines().next() {
                Some(s) => {
                    let s = s.unwrap();
                    if s.starts_with("//") || s.is_empty() {
                        continue;
                    } else {
                        self.current = Some(String::from(s.clone().trim_start()));
                        return Some(s.clone());
                    }
                }
                None => break
            }
        }
        None
    }
    pub fn command_type(&self) -> String {
        if self.current.as_ref().unwrap().starts_with("@") {
            String::from("A_COMMAND")
        } else if self.current.as_ref().unwrap().starts_with("(") {
            String::from("L_COMMAND")
        } else if self.current.as_ref().unwrap().chars().all(
            |c| c.is_alphabetic() || c == '-' || c == '+' || c == '=' || c == '|' || c == '!' || c == '&'
        ) {
            String::from("C_COMMAND")
        } else {
            panic!("not valid command")
        }
    }
    pub fn symbol() {}
    pub fn dest() {}
    pub fn comp() {}
    pub fn jump() {}
}

#[test]
fn command_test() {
    let mut p = Parser::new("./Add.asm").unwrap();
    p.advance();
    assert_eq!(p.command_type(), String::from("A_COMMAND"));
    p.advance();
    assert_eq!(p.command_type(), String::from("C_COMMAND"));
    p.advance();
    assert_eq!(p.command_type(), String::from("A_COMMAND"));
    p.advance();
    assert_eq!(p.command_type(), String::from("C_COMMAND"));
    p.advance();
    assert_eq!(p.command_type(), String::from("A_COMMAND"));
    p.advance();
    assert_eq!(p.command_type(), String::from("C_COMMAND"));
}