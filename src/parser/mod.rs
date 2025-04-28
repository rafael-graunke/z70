pub mod types;

use nom::{
    IResult, Parser,
    bytes::{tag, take_while},
    character::complete::{newline, space0},
    sequence::terminated,
};
use types::Statement;

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
    todo!()
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
