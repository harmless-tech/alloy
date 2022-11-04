use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Operation {
    Prim1(OpPrim1),
    Prim2(OpPrim2),
}
impl From<Operation> for u8 {
    fn from(op: Operation) -> Self {
        match op {
            Operation::Prim1(op) => op.into(),
            Operation::Prim2(op) => (0b10000000) | Into::<u8>::into(op),
        }
    }
}
impl TryFrom<u8> for Operation {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let op = (value & 0b10000000) >> 7;
        let t = value & 0b01111111;

        match op {
            0 => {
                let v = OpPrim1::try_from(t)
                    .map_err(|_| "Could not convert byte to operation prim1.".to_string());
                match v {
                    Ok(p) => Ok(Operation::Prim1(p)),
                    Err(e) => Err(e),
                }
            }
            1 => {
                let v = OpPrim2::try_from(t)
                    .map_err(|_| "Could not convert byte to operation prim2.".to_string());
                match v {
                    Ok(p) => Ok(Operation::Prim2(p)),
                    Err(e) => Err(e),
                }
            }
            _ => Err("Could not convert byte to operation.".to_string()),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
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

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
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
    /// <>
    SameType,
}
