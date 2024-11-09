use std::{
    collections::HashMap,
    io::{stdin, stdout, Write},
};

use crate::token::Instruction;

pub fn execute_instruction(instruction: Instruction, variables: &mut HashMap<String, String>) {
    match instruction {
        Instruction::Assign(var, value) => {
            if value == "input" {
                // Handle user input assignment
                let mut input = String::new();
                let _ = stdout().flush();
                stdin().read_line(&mut input).unwrap();

                variables.insert(var, input.trim().to_string());
            } else {
                // Handle normal value assignment
                variables.insert(var, value);
            }
        }
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
