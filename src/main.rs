use interpreter::execute_instruction;
use std::{collections::HashMap, fs};
use token::parse_line;

mod error;
mod interpreter;
mod token;
fn main() {
    let mut variables: HashMap<String, String> = HashMap::new();

    let data = fs::read_to_string("program.txt").unwrap();
    for line in data.lines() {
        let result = parse_line(line);
        match result {
            Ok((_, instruction)) => execute_instruction(instruction, &mut variables),
            Err(e) => eprintln!("{:?}", e),
        }
    }
}
