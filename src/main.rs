use std::env;
use std::fs::{OpenOptions};
use std::io::Write;

mod parser;
mod code;

fn main() -> Result<(),std::io::Error> {
    // read file
    let file_name = env::args().collect::<Vec<String>>();
    let file_name = file_name[1].as_str();
    println!("Assembling -> {}",file_name);
    let mut parser = parser::Parser::new(file_name)?;
    let mut hack = String::new();
    loop{
        let mut one_line_assembly = String::new();
        match parser.advance() {
            Some(_) => {
                if parser.command_type() == "A_COMMAND" {
                    // 숫자면 바로 2진수로
                    one_line_assembly.push_str("0");
                    one_line_assembly.push_str(format!("{:0>15b}", parser.symbol().parse::<usize>().unwrap()).as_str());
                }else if parser.command_type() == "C_COMMAND" {
                    let comp = code::comp(parser.comp());
                    let dest = code::dest(parser.dest());
                    let jump = code::jump(parser.jump());
                    one_line_assembly.push_str("111");
                    one_line_assembly.push_str(comp.as_str());
                    one_line_assembly.push_str(dest.as_str());
                    one_line_assembly.push_str(jump.as_str());
                }else if parser.command_type() == "L_COMMAND" {
                    // 심볼테이블에서 찾기
                }
            },
            None => break
        }

        hack.push_str(one_line_assembly.as_str());
        hack.push_str("\n");
    }
    let split_file_name = file_name.splitn(2,".").collect::<Vec<&str>>();
    let mut hack_file_name = String::new();
    hack_file_name.push_str(split_file_name[0]);
    hack_file_name.push_str(".hack");

    let mut f = OpenOptions::new().create(true).write(true).open(hack_file_name).unwrap();
    f.write(hack.as_bytes()).unwrap();
    Ok(())
}


