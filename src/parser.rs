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
                    let mut s = s.unwrap();
                    if s.starts_with("//") || s.is_empty() {
                        continue;
                    } else {
                        s.retain(|c|!c.is_whitespace());
                        let rest = s.splitn(2, "//").collect::<Vec<&str>>();
                        self.current = Some(rest[0].to_string().clone());
                        return Some(rest[0].to_string());
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
            c.is_alphanumeric()
                || c == '-'
                || c == '+'
                || c == '='
                || c == '|'
                || c == '!'
                || c == '&'
                || c == ';'
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
            let r = self.current.as_ref().unwrap().splitn(2,'=').collect::<Vec<&str>>();
            return String::from(r[0]);
        }
        String::from("")
    }
    pub fn comp(&self) -> String{
        if self.command_type() == "C_COMMAND" {
            if let Some(rest) = self.current.as_ref().unwrap().splitn(2,'=').last() {
                let result = rest.splitn(2,';').collect::<Vec<&str>>();
                return String::from(result[0]);
            }
        }
        String::from("")
    }
    pub fn jump(&self) -> String {
        if self.command_type() == "C_COMMAND" {
            if self.current.as_ref().unwrap().contains(";") {
                if let Some(rest) = self.current.as_ref().unwrap().splitn(2, ";").last(){
                    return String::from(rest);
                }
            }
        }
        String::from("")
    }
}

#[test]
fn dest_comp_jump_test(){
    let mut p = Parser::new("./Add.asm").unwrap();
    p.advance();
    assert_eq!(p.dest(), String::from(""));
    assert_eq!(p.comp(), String::from(""));
    assert_eq!(p.jump(), String::from(""));

    p.advance();
    // D=A
    assert_eq!(p.dest(), String::from("D"));
    assert_eq!(p.comp(), String::from("A"));
    assert_eq!(p.jump(), String::from(""));
    p.advance();
    p.advance();
    //D=D+A
    assert_eq!(p.dest(), String::from("D"));
    assert_eq!(p.comp(), String::from("D+A"));
    assert_eq!(p.jump(), String::from(""));
    p.advance();
    p.advance();
    //M=D
    assert_eq!(p.dest(), String::from("M"));
    assert_eq!(p.comp(), String::from("D"));
    assert_eq!(p.jump(), String::from(""));
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
