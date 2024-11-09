use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, multispace0, not_line_ending, space1},
    sequence::{preceded, separated_pair, tuple},
    IResult,
};

#[derive(Debug)]
#[allow(dead_code)]

pub enum Instruction {
    Assign(String, String),
    Print(String),
    WriteFile(String, String),
    ReadFile(String, String),
}

// Function to parse "assign" command
pub fn parse_assign(input: &str) -> IResult<&str, Instruction> {
    let (input, (_, target, _, val)) = tuple((
        tag("assign"),
        preceded(space1, alphanumeric1),
        space1,
        alt((tag("input"), alphanumeric1)),
    ))(input)?;

    Ok((
        input,
        Instruction::Assign(target.to_string(), val.to_string()),
    ))
}

pub fn parse_print(input: &str) -> IResult<&str, Instruction> {
    // let (input, (_, message)) = separated_pair(tag("print"), multispace0, alpha1)(input)?;
    let (input, (_, message)) = separated_pair(tag("print"), multispace0, not_line_ending)(input)?;
    Ok((input, Instruction::Print(message.to_string())))
}
pub fn parse_line(input: &str) -> IResult<&str, Instruction> {
    // Try each of the parsers, returning the first that succeeds
    let (input, instruction) = alt((parse_assign, parse_print))(input)?;

    Ok((input, instruction))
}
