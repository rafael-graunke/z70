use crate::parser::{
    parse_file,
    types::{Instruction, Operand, Statement},
};
use std::{
    collections::HashMap,
    fs::File,
    io::{Read, Write},
};

fn assemble_double_operands(
    first: &Operand,
    second: &Operand,
) -> Result<(u8, Option<u8>), &'static str> {
    match (first, second) {
        (Operand::A, Operand::B) => Ok((0x0, None)),
        (Operand::B, Operand::A) => Ok((0x1, None)),
        (Operand::A, Operand::I) => Ok((0x2, None)),
        (Operand::I, Operand::A) => Ok((0x3, None)),
        (Operand::A, Operand::MemI) => Ok((0x4, None)),
        (Operand::MemI, Operand::A) => Ok((0x5, None)),
        (Operand::A, Operand::Const(value)) => Ok((0x6, Some(value.to_owned()))),
        (Operand::B, Operand::Const(value)) => Ok((0x7, Some(value.to_owned()))),
        (Operand::I, Operand::Const(value)) => Ok((0x8, Some(value.to_owned()))),
        (Operand::MemI, Operand::Const(value)) => Ok((0x9, Some(value.to_owned()))),
        (Operand::A, Operand::MemConst(value)) => Ok((0xA, Some(value.to_owned()))),
        (Operand::B, Operand::MemConst(value)) => Ok((0xB, Some(value.to_owned()))),
        (Operand::MemConst(value), Operand::A) => Ok((0xC, Some(value.to_owned()))),
        (Operand::MemConst(value), Operand::B) => Ok((0xD, Some(value.to_owned()))),
        _ => Err("Invalid operand combination"),
    }
}

fn assemble_instruction(instruction: &Instruction) -> u8 {
    match instruction {
        Instruction::Add => 0x0,
        Instruction::Sub => 0x1,
        Instruction::Cmp => 0x2,
        Instruction::Inc => 0x3,
        Instruction::Dec => 0x4,
        Instruction::And => 0x5,
        Instruction::Or => 0x6,
        Instruction::Not => 0x7,
        Instruction::Shr => 0x8,
        Instruction::Shl => 0x9,
        Instruction::Jmp => 0xA0,
        Instruction::Jz => 0xA1,
        Instruction::Js => 0xA2,
        Instruction::Jc => 0xA3,
        Instruction::Jo => 0xA4,
        Instruction::Jp => 0xA5,
        Instruction::Mov => 0xB,
        Instruction::Nop => 0xFF,
    }
}

fn assemble_statement(
    statement: &Statement,
    labels: &HashMap<String, u8>,
    bytes: &mut Vec<u8>,
) -> Result<(), &'static str> {
    match statement {
        Statement::Label(_) => (),

        Statement::Operation(
            instruction @ (Instruction::Add
            | Instruction::Sub
            | Instruction::Cmp
            | Instruction::And
            | Instruction::Or
            | Instruction::Mov),
            Some(a),
            Some(b),
        ) => {
            let mut instruction_byte = assemble_instruction(instruction);
            let (addressing, c) = assemble_double_operands(a, b)?;

            instruction_byte <<= 4;
            instruction_byte += addressing;

            bytes.push(instruction_byte);

            match c {
                Some(value) => bytes.push(value),
                _ => {}
            };
        }

        Statement::Operation(
            instruction @ (Instruction::Jmp
            | Instruction::Jz
            | Instruction::Js
            | Instruction::Jc
            | Instruction::Jo
            | Instruction::Jp),
            Some(Operand::Label(label)),
            None,
        ) => {
            bytes.push(assemble_instruction(instruction));

            let label_address = match labels.get(label.to_owned()) {
                Some(value) => value.to_owned(),
                _ => {
                    println!("{:?}", labels);
                    return Err("Label not found");
                }
            };

            bytes.push(label_address);
        }

        Statement::Operation(
            instruction @ (Instruction::Inc
            | Instruction::Dec
            | Instruction::Not
            | Instruction::Shr
            | Instruction::Shl),
            Some(operand),
            None,
        ) => {
            let mut instruction_byte = assemble_instruction(instruction);
            let addressing = match operand {
                Operand::A => 0x0,
                Operand::B => 0x1,
                Operand::I => 0x2,
                Operand::MemI => 0x4,
                _ => return Err("Operand not valid for single instruction"),
            };
            instruction_byte <<= 4;
            instruction_byte += addressing;
            bytes.push(instruction_byte);
        }

        Statement::Operation(instruction @ Instruction::Nop, None, None) => {
            let instruction_byte = assemble_instruction(instruction);
            bytes.push(instruction_byte);
        },

        _ => return Err("Invalid statement"),
    };

    Ok(())
}

fn first_pass(statements: &Vec<Statement>, labels: &mut HashMap<String, u8>) {
    let mut addr: u8 = 0;

    for st in statements {
        match st {
            Statement::Operation(_, _, Some(Operand::MemConst(_)) | Some(Operand::Const(_))) => {
                addr += 2;
            }
            Statement::Operation(_, Some(Operand::MemConst(_)) | Some(Operand::Const(_)), _) => {
                addr += 2;
            }
            Statement::Operation(Instruction::Jmp, Some(Operand::Label(_)), _) => {
                addr += 2;
            }
            Statement::Label(label) => {
                labels.insert(label.to_string(), addr);
                addr += 1;
            }
            _ => {
                addr += 1;
            }
        };
    }
}

fn second_pass(
    statements: &Vec<Statement>,
    labels: &HashMap<String, u8>,
    bytes: &mut Vec<u8>,
) -> Result<(), &'static str> {
    for st in statements {
        assemble_statement(st, &labels, bytes)?;
    }

    Ok(())
}

pub fn assemble_file(source: &mut File, target: &str) -> Result<(), &'static str> {
    let mut asm: String = String::new();
    source.read_to_string(&mut asm).unwrap();

    let (_, statements) = parse_file(asm.as_str()).unwrap();

    let mut bytes = Vec::<u8>::new();
    let mut labels = HashMap::<String, u8>::new();

    first_pass(&statements, &mut labels);

    second_pass(&statements, &labels, &mut bytes)?;
    let mut file = File::create(target).unwrap();
    file.write_all(&bytes).unwrap();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assemble_invalid_statements() {
        let mut bytes = Vec::<u8>::new();
        let labels = HashMap::<String, u8>::new();

        assert!(matches!(
            assemble_statement(
                // Invalid operands "add A, A"
                &Statement::Operation(Instruction::Add, Some(Operand::A), Some(Operand::A)),
                &labels,
                &mut bytes
            ),
            Err(_)
        ));

        assert!(matches!(
            assemble_statement(
                // Invalid memory2memory operands "mov [I], [80H]"
                &Statement::Operation(
                    Instruction::Mov,
                    Some(Operand::MemI),
                    Some(Operand::MemConst(0x80))
                ),
                &labels,
                &mut bytes
            ),
            Err(_)
        ));

        assert!(matches!(
            assemble_statement(
                // Subtle invalid operands "mov I, [80H]"
                &Statement::Operation(
                    Instruction::Mov,
                    Some(Operand::I),
                    Some(Operand::MemConst(0x80))
                ),
                &labels,
                &mut bytes
            ),
            Err(_)
        ));

        assert!(matches!(
            assemble_statement(
                // Non-existent label "jmp LOOP"
                &Statement::Operation(Instruction::Jmp, Some(Operand::Label("LOOP")), None),
                &labels,
                &mut bytes
            ),
            Err("Label not found")
        ));
    }

    #[test]
    fn test_assemble_valid_statements() {
        let mut bytes = Vec::<u8>::new();
        let labels = HashMap::<String, u8>::new();

        assert!(matches!(
            assemble_statement(
                // mov A, [80H]
                &Statement::Operation(
                    Instruction::Mov,
                    Some(Operand::A),
                    Some(Operand::MemConst(0x80))
                ),
                &labels,
                &mut bytes
            ),
            Ok(())
        ));
        assert_eq!(bytes, vec![0xBA, 0x80]);

        bytes.clear();

        assert!(matches!(
            assemble_statement(
                // nop
                &Statement::Operation(
                    Instruction::Nop, None, None
                ),
                &labels,
                &mut bytes
            ),
            Ok(())
        ));
        assert_eq!(bytes, vec![0xFF]);
    }

    #[test]
    fn test_assemble_label_declaration() {
        let mut bytes = Vec::<u8>::new();
        let mut labels = HashMap::<String, u8>::new();

        let statements = &vec![
            Statement::Label("LOOP"),
            Statement::Operation(Instruction::Jmp, Some(Operand::Label("LOOP")), None),
        ];
        first_pass(statements, &mut labels);
        assert!(matches!(
            second_pass(statements, &labels, &mut bytes),
            Ok(())
        ));
        assert_eq!(bytes, vec![0xA0, 0x00]);

        bytes.clear();

        let statements = &vec![
            Statement::Operation(
                Instruction::Add,
                Some(Operand::A),
                Some(Operand::Const(0x10)),
            ), // 2 bytes
            Statement::Operation(Instruction::Inc, Some(Operand::A), None), // 1 bytes
            Statement::Label("LOOP"), // should map to address 0x3
            Statement::Operation(Instruction::Jmp, Some(Operand::Label("LOOP")), None),
        ];
        first_pass(statements, &mut labels);
        assert!(matches!(
            second_pass(statements, &labels, &mut bytes),
            Ok(())
        ));
        assert_eq!(bytes, vec![0x06, 0x10, 0x30, 0xA0, 0x03]);
    }
}
