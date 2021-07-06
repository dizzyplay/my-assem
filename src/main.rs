use std::env;
use std::fs::{OpenOptions};
use std::io::Write;

mod parser;
mod code;
mod symbol_table;

fn main() -> Result<(), std::io::Error> {
    // read file
    let file_name = env::args().collect::<Vec<String>>();
    let file_name = file_name[1].as_str();
    println!("Assembling -> {}", file_name);
    let mut parser = parser::Parser::new(file_name)?;
    let mut one_pass_parser = parser::Parser::new(file_name)?;
    let mut hack = String::new();

    let mut symbol_table = symbol_table::SymbolTable::new();
    let mut line: usize = 1;
    loop {
        match one_pass_parser.advance() {
            Some(_) => {
                if one_pass_parser.command_type() == "L_COMMAND" {
                    symbol_table.insert(one_pass_parser.symbol(), Some(line + 1));
                } else if one_pass_parser.command_type() == "A_COMMAND" && one_pass_parser.symbol().chars().all(|c|{
                    if c.is_lowercase() || c == '_'  {
                        return true;
                    }
                    false
                }) {
                    symbol_table.insert(one_pass_parser.symbol(), None);
                }
                line += 1;
            }
            None => break
        }
    }
    loop {
        let mut one_line_assembly = String::new();
        match parser.advance() {
            Some(_) => {
                if parser.command_type() == "A_COMMAND" {
                    let variable:usize;
                    if parser.symbol().chars().all(char::is_numeric) {
                        // 숫자면 바로 2진수로
                        variable = parser.symbol().parse::<usize>().unwrap();
                    }else {
                        variable = symbol_table.get(parser.symbol());
                    }
                    one_line_assembly.push_str("0");
                    one_line_assembly.push_str(format!("{:0>15b}", variable).as_str());
                } else if parser.command_type() == "C_COMMAND" {
                    let comp = code::comp(parser.comp());
                    let dest = code::dest(parser.dest());
                    let jump = code::jump(parser.jump());
                    one_line_assembly.push_str("111");
                    one_line_assembly.push_str(comp.as_str());
                    one_line_assembly.push_str(dest.as_str());
                    one_line_assembly.push_str(jump.as_str());
                } else if parser.command_type() == "L_COMMAND" {
                    // 심볼테이블에서 찾기
                    continue;
                }
            }
            None => break
        }
        hack.push_str(one_line_assembly.as_str());
        hack.push_str("\n");
    }
    let split_file_name = file_name.splitn(2, ".").collect::<Vec<&str>>();
    let mut hack_file_name = String::from(split_file_name[0]);
    hack_file_name.push_str(".hack");

    let mut f = OpenOptions::new().create(true).truncate(true).write(true).open(hack_file_name).unwrap();
    f.write(hack.as_bytes()).unwrap();
    Ok(())
}


