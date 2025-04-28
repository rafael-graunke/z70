pub mod types;
use nom::{
    Err as NomErr, IResult, Parser,
    branch::alt,
    bytes::{tag, tag_no_case, take_while},
    character::complete::{newline, space0, space1},
    error::{Error, ErrorKind},
    sequence::terminated,
};
use types::{Instruction, Statement};

fn end_of_statement(input: &str) -> IResult<&str, ()> {
    let (input, _) = space0(input)?;
    let (input, _) = newline(input)?;
    Ok((input, ()))
}

pub fn parse_label(input: &str) -> IResult<&str, Statement> {
    let (input, name) = terminated(
        take_while(|c| ![':', ' ', '\n'].contains(&c)),
        (space0, tag(":"), end_of_statement),
    )
    .parse(input)?;

    Ok((input, Statement::Label(name)))
}

pub fn parse_operation(input: &str) -> IResult<&str, Statement> {
    let (input, instruction) = parse_instruction(input)?;

    Ok((input, Statement::Operation(instruction, None, None)))
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, (_, parsed)) = (
        alt((
            tag_no_case("add"),
            tag_no_case("sub"),
            tag_no_case("cmp"),
            tag_no_case("inc"),
            tag_no_case("dec"),
            tag_no_case("and"),
            tag_no_case("or"),
            tag_no_case("not"),
            tag_no_case("shr"),
            tag_no_case("shl"),
            tag_no_case("jmp"),
            tag_no_case("jz"),
            tag_no_case("js"),
            tag_no_case("jc"),
            tag_no_case("jo"),
            tag_no_case("jp"),
            tag_no_case("mov"),
            tag_no_case("nop"),
        )),
        space1,
    )
        .parse(input)?;

    let (_, op_type) = match parsed {
        "add" => Ok((parsed, Instruction::Add)),
        "sub" => Ok((parsed, Instruction::Sub)),
        "cmp" => Ok((parsed, Instruction::Cmp)),
        "inc" => Ok((parsed, Instruction::Inc)),
        "dec" => Ok((parsed, Instruction::Dec)),
        "and" => Ok((parsed, Instruction::And)),
        "or" => Ok((parsed, Instruction::Or)),
        "not" => Ok((parsed, Instruction::Not)),
        "shr" => Ok((parsed, Instruction::Shr)),
        "shl" => Ok((parsed, Instruction::Shl)),
        "jmp" => Ok((parsed, Instruction::Jmp)),
        "jz" => Ok((parsed, Instruction::Jz)),
        "js" => Ok((parsed, Instruction::Js)),
        "jc" => Ok((parsed, Instruction::Jc)),
        "jo" => Ok((parsed, Instruction::Jo)),
        "jp" => Ok((parsed, Instruction::Jp)),
        "mov" => Ok((parsed, Instruction::Mov)),
        "nop" => Ok((parsed, Instruction::Nop)),
        _ => Err(NomErr::Error(Error::new(
            "Could not find a suitable instruction",
            ErrorKind::NoneOf,
        ))),
    }?;

    
    Ok((input, op_type))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_label() {
        let (_, label) = parse_label("test_label:\n").unwrap();
        assert!(matches!(label, Statement::Label("test_label")));

        let (_, label) = parse_label("test_label   :   \n").unwrap();
        assert!(matches!(label, Statement::Label("test_label")));
    }
}
