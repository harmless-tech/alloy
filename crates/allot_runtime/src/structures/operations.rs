#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Operation {
    Arithmetic(ArithmeticOperation),
    BitWise(BitWiseOperation),
    Logic(LogicOperation),
    Relational(RelationalOperation),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ArithmeticOperation {
    Add,            // +
    Subtract,       // -
    Multiplication, // *
    Division,       // /
    Modulus,        // %
    Increment,      // ++
    Decrement,      // --
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BitWiseOperation {
    And,               // &
    Or,                // |
    Xor,               // ^
    Not,               // ~
    ShiftLeft(usize),  // << u64
    ShiftRight(usize), // >> u64
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LogicOperation {
    Not, // !
    And, // &&
    Or,  // ||
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RelationalOperation {
    Equal,        // ==
    NotEqual,     // !=
    Greater,      // >
    Less,         // <
    GreaterEqual, // >=
    LessEqual,    // <=
}
