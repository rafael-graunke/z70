pub enum Statement<'a > {
    Label(&'a str),
    Operation(OperationType, Option<Operand<'a>>, Option<Operand<'a>>)
}

pub enum OperationType {
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

pub enum Operand<'a> {
    Label(&'a str),
    A,
    B,
    I,
    MemI,
    Const(u8),
    MemConst(u8)
}
