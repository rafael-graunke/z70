pub mod types;
use nom::{
    Err as NomErr, IResult, Parser,
    branch::alt,
    bytes::{complete::take_till, tag, tag_no_case, take, take_while},
    character::complete::{newline, space0, space1},
    combinator::complete,
    error::{Error, ErrorKind},
    multi::many0,
    sequence::{delimited, terminated},
};
use types::{Instruction, Operand, Statement};

pub fn parse_file(input: &str) -> IResult<&str, Vec<Statement>> {
    let (input, statements) = many0(complete(parse_statement)).parse(input)?;

    Ok((input, statements))
}

fn parse_statement(input: &str) -> IResult<&str, Statement> {
    println!("\"{}\"", input);

    let (input, _) = space0(input)?;
    let (input, statement) = alt((parse_operation, parse_label_declaration)).parse(input)?;

    Ok((input, statement))
}

fn end_of_statement(input: &str) -> IResult<&str, ()> {
    let (input, _) = space0(input)?;
    let (input, _) = newline(input)?;
    Ok((input, ()))
}

fn parse_label_declaration(input: &str) -> IResult<&str, Statement> {
    let (input, name) = terminated(
        take_while(|c| ![':', ' ', '\n'].contains(&c)),
        (space0, tag(":"), end_of_statement),
    )
    .parse(input)?;

    Ok((input, Statement::Label(name)))
}

fn parse_operation(input: &str) -> IResult<&str, Statement> {
    let (input, operation) =
        alt((parse_no_inst_op, parse_single_inst_op, parse_dual_inst_op)).parse(input)?;

    Ok((input, operation))
}

fn parse_no_inst_op(input: &str) -> IResult<&str, Statement> {
    let (input, instruction) = parse_instruction(input)?;
    let (input, _) = end_of_statement(input)?;

    Ok((input, Statement::Operation(instruction, None, None)))
}

fn parse_single_inst_op(input: &str) -> IResult<&str, Statement> {
    let (input, instruction) = parse_instruction(input)?;
    let (input, _) = space1(input)?;
    let (input, operand) = parse_operand(input)?;
    let (input, _) = end_of_statement(input)?;

    Ok((input, Statement::Operation(instruction, operand, None)))
}

fn parse_dual_inst_op(input: &str) -> IResult<&str, Statement> {
    let (input, instruction) = parse_instruction(input)?;
    let (input, _) = space1(input)?;
    let (input, first_operand) = parse_operand(input)?;
    let (input, _) = space0(input)?;
    let (input, _) = tag(",").parse(input)?;
    let (input, _) = space0(input)?;
    let (input, second_operand) = parse_operand(input)?;
    let (input, _) = end_of_statement(input)?;

    Ok((
        input,
        Statement::Operation(instruction, first_operand, second_operand),
    ))
}

fn parse_operand(input: &str) -> IResult<&str, Option<Operand>> {
    let result = alt((
        parse_a,
        parse_b,
        parse_i,
        parse_mem,
        parse_const,
        parse_label,
    ))
    .parse(input);

    match result {
        Ok((input, operand)) => Ok((input, Some(operand))),
        Err(_) => Ok((input, None)),
    }
}

fn parse_a(input: &str) -> IResult<&str, Operand> {
    let (input, _) = tag("A").parse(input)?;
    Ok((input, Operand::A))
}

fn parse_b(input: &str) -> IResult<&str, Operand> {
    let (input, _) = tag("B").parse(input)?;
    Ok((input, Operand::B))
}

fn parse_i(input: &str) -> IResult<&str, Operand> {
    let (input, _) = tag("I").parse(input)?;
    Ok((input, Operand::I))
}

fn parse_const(input: &str) -> IResult<&str, Operand> {
    let (input, hex_value) = terminated(take(2u8), tag_no_case("h")).parse(input)?;

    let result = u8::from_str_radix(hex_value, 16);

    match result {
        Ok(value) => Ok((input, Operand::Const(value))),
        Err(_) => Err(NomErr::Error(Error::new(
            "Could not convert string (hex value) to byte",
            ErrorKind::AlphaNumeric,
        ))),
    }
}

fn parse_mem(input: &str) -> IResult<&str, Operand> {
    let (input, operand) = alt((
        delimited(tag("["), parse_i, tag("]")),
        delimited(tag("["), parse_const, tag("]")),
    ))
    .parse(input)?;

    let (_, result) = match operand {
        Operand::I => Ok((operand, Operand::MemI)),
        Operand::Const(value) => Ok((operand, Operand::MemConst(value))),
        _ => Err(NomErr::Error(Error::new(
            "Could not find a suitable memory operand",
            ErrorKind::NoneOf,
        ))),
    }?;

    Ok((input, result))
}

fn parse_label(input: &str) -> IResult<&str, Operand> {
    let (input, label) = take_till(|c| [' ', '\n'].contains(&c)).parse(input)?;

    Ok((input, Operand::Label(label)))
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, parsed) = alt((
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
    ))
    .parse(input)?;

    let (_, op_type) = match parsed.to_lowercase().as_str() {
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
    fn test_parse_label_declaration() {
        assert!(matches!(
            parse_label_declaration("test_label:\n"),
            Ok((_, Statement::Label("test_label")))
        ));
        assert!(matches!(
            parse_label_declaration("test_label   :   \n"),
            Ok((_, Statement::Label("test_label")))
        ));
        assert!(matches!(parse_label_declaration("test_label"), Err(_)));
        assert!(matches!(parse_label_declaration("test_label:"), Err(_)));
    }

    #[test]
    fn test_parse_instruction() {
        assert!(matches!(parse_instruction("NA "), Err(_)));
        assert!(matches!(
            parse_instruction("mov"),
            Ok((_, Instruction::Mov))
        ));
        assert!(matches!(
            parse_instruction("mov "),
            Ok((_, Instruction::Mov))
        ));
        assert!(matches!(
            parse_instruction("MOV"),
            Ok((_, Instruction::Mov))
        ));
        assert!(matches!(
            parse_instruction("ADD"),
            Ok((_, Instruction::Add))
        ));
        assert!(matches!(
            parse_instruction("SUB"),
            Ok((_, Instruction::Sub))
        ));
    }

    #[test]
    fn test_parse_operand() {
        assert!(matches!(parse_operand("A"), Ok((_, Some(Operand::A)))));
        assert!(matches!(parse_operand("B"), Ok((_, Some(Operand::B)))));
        assert!(matches!(parse_operand("I"), Ok((_, Some(Operand::I)))));
        assert!(matches!(
            parse_operand("10H"),
            Ok((_, Some(Operand::Const(0x10))))
        ));
        assert!(matches!(
            parse_operand("[20H]"),
            Ok((_, Some(Operand::MemConst(0x20))))
        ));
        assert!(matches!(parse_operand("[I]"), Ok((_, Some(Operand::MemI)))));
        assert!(matches!(
            parse_operand("test_label \n"),
            Ok((_, Some(Operand::Label("test_label"))))
        ));
    }

    #[test]
    fn test_parse_operation() {
        assert!(matches!(
            parse_operation("mov A, B\n"),
            Ok((
                _,
                Statement::Operation(Instruction::Mov, Some(Operand::A), Some(Operand::B))
            ))
        ));
        assert!(matches!(
            parse_operation("inc I\n"),
            Ok((
                _,
                Statement::Operation(Instruction::Inc, Some(Operand::I), None)
            ))
        ));
        assert!(matches!(
            parse_operation("jmp LOOP\n"),
            Ok((
                _,
                Statement::Operation(Instruction::Jmp, Some(Operand::Label("LOOP")), None)
            ))
        ));
        assert!(matches!(
            parse_operation("nop\n"),
            Ok((_, Statement::Operation(Instruction::Nop, None, None)))
        ));
    }

    #[test]
    fn test_parse_statement() {
        let (remainder, statement) = parse_statement("mov A, B\n LOOP: \n dec A\n").unwrap();
        assert_eq!(remainder, " LOOP: \n dec A\n");
        assert_eq!(
            statement,
            Statement::Operation(Instruction::Mov, Some(Operand::A), Some(Operand::B))
        );

        let (remainder, statement) = parse_statement(remainder).unwrap();
        assert_eq!(remainder, " dec A\n");
        assert_eq!(statement, Statement::Label("LOOP"));

        let (remainder, statement) = parse_statement(remainder).unwrap();
        assert_eq!(remainder, "");
        assert_eq!(
            statement,
            Statement::Operation(Instruction::Dec, Some(Operand::A), None)
        );
    }

    #[test]
    fn test_parse_file() {
        let (_, vector) = parse_file("mov A, 80H\n LOOP: \n dec A\n jmp LOOP\n").unwrap();

        assert_eq!(
            vector,
            Vec::from([
                Statement::Operation(
                    Instruction::Mov,
                    Some(Operand::A),
                    Some(Operand::Const(0x80))
                ),
                Statement::Label("LOOP"),
                Statement::Operation(Instruction::Dec, Some(Operand::A), None),
                Statement::Operation(Instruction::Jmp, Some(Operand::Label("LOOP")), None),
            ])
        )
    }
}
