#[derive(PartialEq, Debug)]
pub enum Statement<'a > {
    Label(&'a str),
    Operation(Instruction, Option<Operand<'a>>, Option<Operand<'a>>)
}

#[derive(PartialEq, Debug)]
pub enum Instruction {
    Add,
    Sub,
    Cmp,
    Inc,
    Dec,
    And,
    Or,
    Not,
    Shr,
    Shl,
    Jmp,
    Jz,
    Js,
    Jc,
    Jo,
    Jp,
    Mov,
    Nop
}

#[derive(PartialEq, Debug)]
pub enum Operand<'a> {
    Label(&'a str),
    A,
    B,
    I,
    MemI,
    Const(u8),
    MemConst(u8)
}
