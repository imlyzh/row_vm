use std::io::{stdin, stdout, Write};

use minparser::pest_parser::parse;



pub fn main() {
    println!("wellcome to row vm parse repl");
    loop {
        print!(">>> ");
        stdout().flush().unwrap();
        let mut buf = "".to_owned();
        stdin().read_line(&mut buf).unwrap();
        match parse(&buf) {
            Ok(expr) => println!("> {:?}", expr),
            Err(err) => println!("error: {}", err),
        }
    }
}