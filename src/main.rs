use std::{collections::HashMap, fs};
use token::{parse_line, Instruction};

mod error;
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
fn execute_instruction(instruction: Instruction, variables: &mut HashMap<String, String>) {
    match instruction {
        Instruction::Assign(_, _) => todo!(),
        Instruction::Print(var) => {
            if let Some(value) = variables.get(&var) {
                println!("{}", value);
            } else {
                println!("{}", var); // Print raw message if no variable
            }
        }
        Instruction::WriteFile(_, _) => todo!(),
        Instruction::ReadFile(_, _) => todo!(),
    }
}
