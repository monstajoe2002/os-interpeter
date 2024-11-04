use std::collections::HashMap;

use crate::token::Instruction;

pub fn execute_instruction(instruction: Instruction, variables: &mut HashMap<String, String>) {
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
