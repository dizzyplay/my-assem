use std::env;

mod parser;

fn main() -> Result<(),std::io::Error> {
    // read file
    let file_name = env::args().collect::<Vec<String>>();
    println!("{:?}",file_name[1].as_str());
    let mut parser = parser::Parser::new(file_name[1].as_str())?;
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


