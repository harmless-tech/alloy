#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Operation {
    Prim1(OpPrim1),
    Prim2(OpPrim2),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OpPrim1 {
    /// ++
    Increment,
    /// --
    Decrement,
    /// !
    Not,
    /// ~
    BitwiseNot,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OpPrim2 {
    /// +
    Add,
    /// -
    Subtract,
    /// *
    Multiplication,
    /// /
    Division,
    /// %
    Modulus,
    /// &&
    And,
    /// ||
    Or,
    /// ^^
    Xor,
    /// ==
    Equal,
    /// !=
    NotEqual,
    /// >
    Greater,
    /// <
    Less,
    /// >=
    GreaterEqual,
    /// <=
    LessEqual,
    /// &
    BitwiseAnd,
    ///  |
    BitwiseOr,
    /// ^
    BitwiseXor,
    /// <<
    ShiftLeft,
    /// >>
    ShiftRight,
    /// == for type
    SameType,
}
