use std::fs::File;
use std::io::{BufRead, BufReader, Error, Read};

pub struct Parser {
    file: BufReader<File>,
    current: Option<String>,
}

impl Parser {
    pub fn new(filename: &str) -> Result<Self, Error> {
        let f = File::open(filename)?;
        let f = BufReader::new(f);
        Ok(Parser {
            file: f,
            current: None,
        })
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
                None => break,
            }
        }
        None
    }
    pub fn command_type(&self) -> String {
        if self.current.as_ref().unwrap().starts_with("@") {
            String::from("A_COMMAND")
        } else if self.current.as_ref().unwrap().starts_with("(") {
            String::from("L_COMMAND")
        } else if self.current.as_ref().unwrap().chars().all(|c| {
            c.is_alphabetic()
                || c == '-'
                || c == '+'
                || c == '='
                || c == '|'
                || c == '!'
                || c == '&'
        }) {
            String::from("C_COMMAND")
        } else {
            panic!("not valid command")
        }
    }
    pub fn symbol(&self) -> String {
        // return @i -> i @1 -> 1 (loop) -> loop
        if self.command_type() == "A_COMMAND" || self.command_type() == "L_COMMAND" {
            let mut s = String::from(self.current.as_ref().unwrap());
            s.retain(|c| c != '@' && c != '(' && c != ')');
            return s;
        }
        String::from("")
    }
    pub fn dest(&self) -> String {
        if self.command_type() == "C_COMMAND" && self.current.as_ref().unwrap().contains("=") {
            let r = self
                .current
                .as_ref()
                .unwrap()
                .splitn(2, '=')
                .collect::<Vec<&str>>();
            let mut result: usize = 0;
            return String::from(r[0]);
        }
        String::from("")
    }
    pub fn comp(&self) -> String {
        if self.command_type() == "C_COMMAND" {
            // dest=comp;jmp
            let mut command = String::new();
            if self.current.as_ref().unwrap().contains('=') {
                let rest = self
                    .current
                    .as_ref()
                    .unwrap()
                    .splitn(2, '=')
                    .collect::<Vec<&str>>();
                command.push_str(rest[1]);
            }
            if command.contains(';') {
                let r = command.splitn(2, ';').collect::<Vec<&str>>();
                return String::from(r[0]);
            }
            return String::from(command);
        }
        String::from("")
    }
    pub fn jump() {}
}

#[test]
fn comp_test() {
    let mut p = Parser::new("./Add.asm").unwrap();
    p.advance();
    p.advance();
    assert_eq!(p.comp(), String::from("A"));
    p.advance();
    p.advance();
    assert_eq!(p.comp(), String::from("D+A"));
    p.advance();
    p.advance();
    assert_eq!(p.comp(), String::from("D"));
}

#[test]
fn dest_test() {
    let mut p = Parser::new("./Add.asm").unwrap();
    p.advance();
    assert_eq!(p.dest(), String::from(""));
    p.advance();
    // D=A -> 010
    assert_eq!(p.dest(), String::from("D"));
    p.advance();
    p.advance();
    //D=D+A -> 010
    assert_eq!(p.dest(), String::from("D"));
    p.advance();
    p.advance();
    //M=D -> 001
    assert_eq!(p.dest(), String::from("M"));
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

#[test]
fn symbol_test() {
    let mut p = Parser::new("./Add.asm").unwrap();
    p.advance();
    assert_eq!(p.symbol(), String::from("2"));
    p.advance();
    assert_eq!(p.symbol(), String::from(""));
}
