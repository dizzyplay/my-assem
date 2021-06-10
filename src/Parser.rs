use std::fs::{OpenOptions, File};
use std::io::{Error, Read, ErrorKind, BufReader, BufRead};

pub struct Parser{
    file:BufReader<File>,
    current:Option<String>
}

impl Parser {
    pub fn new(filename:&str) -> Result<Self,Error> {
        let f = File::open(filename)?;
        let f = BufReader::new(f);
        Ok(
            Parser{
                file:f,
                current:None
            }
        )
    }

    pub fn advance(&mut self)->Option<String>{
        loop{
            match self.file.by_ref().lines().next(){
                Some(s) => {
                    let s = s.unwrap();
                    if s.starts_with("//") || s.is_empty() {
                        continue
                    }else {
                        self.current = Some(s.clone());
                        return Some(s.clone());
                    }
                },
                None => break
            }
        }
        None
        // let s = self.file.by_ref().lines().next();
        // match s {
        //     Some(s) => {
        //         self.current = Option::from(s.unwrap());
        //         Some(String::from(&self.current.clone().unwrap()))
        //     },
        //     None => None
        // }
    }
    pub fn command_type(&self)->String{
        if self.current.as_ref().unwrap().starts_with("@") {
            String::from("A_COMMAND")
        }else if self.current.as_ref().unwrap().starts_with("("){
            String::from("L_COMMAND")
        }else {
            String::from("C_COMMAND")
        }
    }
    pub fn symbol(){

    }
    pub fn dest(){

    }
    pub fn comp(){

    }
    pub fn jump(){

    }
}

#[test]
fn command_test(){
    let mut p = Parser::new("./Add.asm").unwrap();
    loop {
        match p.advance() {
            Some(s) => {
                println!("{}",s);
                println!("{}",p.command_type());
            },
            None => break
        }
    }
    // p.advance();
    // let c = p.command_type();
    // println!("{}",c);
    // assert_eq!(c,String::from("A_COMMAND"));
}