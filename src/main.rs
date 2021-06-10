use std::collections::HashMap;
use std::env;
use std::fs::OpenOptions;

mod Parser;

fn main() -> Result<(),std::io::Error> {
    // read file
    let file_name = env::args().collect::<Vec<String>>();
    println!("{:?}",file_name[1].as_str());
    let mut parser = Parser::Parser::new(file_name[1].as_str())?;
    loop{
        match parser.advance() {
            Some(s) => {
                println!("{:?}",s)
            },
            None => break
        }
    }
    Ok(())
}


// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/06/add/Add.asm

// Computes R0 = 2 + 3  (R0 refers to RAM[0])

// @2
// D=A
// @3
// D=D+A
// @0
// M=D
