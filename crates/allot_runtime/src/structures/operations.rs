use crate::{Register, Registers, Type};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Operation {
    Prim1(OpPrim1),
    Prim2(OpPrim2),
}
impl Operation {
    pub fn solve(&self, registers: &mut Registers, regs: &[Register; 2]) {
        match self {
            Operation::Prim1(op) => {
                let v = registers.take(regs[0]);
                let t = op.solve(v);
                registers.insert(regs[0], t);
            }
            Operation::Prim2(op) => {
                let v1 = registers.clone(regs[0]);
                let v2 = registers.clone(regs[1]);
                let t = op.solve(v1, v2);
                registers.insert(regs[0], t);
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OpPrim1 {
    Increment,  // ++
    Decrement,  // --
    Not,        // !
    BitwiseNot, // ~
}
impl OpPrim1 {
    pub fn solve(&self, t: Type) -> Type {
        match self {
            OpPrim1::Increment => match t {
                Type::Int8(v) => Type::Int8(v + 1),
                Type::Int16(v) => Type::Int16(v + 1),
                Type::Int32(v) => Type::Int32(v + 1),
                Type::Int(v) => Type::Int(v + 1),
                Type::Int64(v) => Type::Int64(v + 1),
                Type::Int128(v) => Type::Int128(v + 1),
                Type::UInt8(v) => Type::UInt8(v + 1),
                Type::UInt16(v) => Type::UInt16(v + 1),
                Type::UInt32(v) => Type::UInt32(v + 1),
                Type::UInt(v) => Type::UInt(v + 1),
                Type::UInt64(v) => Type::UInt64(v + 1),
                Type::UInt128(v) => Type::UInt128(v + 1),
                Type::Float32(v) => Type::Float32(v + 1.0),
                Type::Float64(v) => Type::Float64(v + 1.0),
                Type::Char(v) => Type::Char(char::from_u32(v as u32 + 1).unwrap()),
                _ => panic!("Increment only works on number types."),
            },
            OpPrim1::Decrement => match t {
                Type::Int8(v) => Type::Int8(v - 1),
                Type::Int16(v) => Type::Int16(v - 1),
                Type::Int32(v) => Type::Int32(v - 1),
                Type::Int(v) => Type::Int(v - 1),
                Type::Int64(v) => Type::Int64(v - 1),
                Type::Int128(v) => Type::Int128(v - 1),
                Type::UInt8(v) => Type::UInt8(v - 1),
                Type::UInt16(v) => Type::UInt16(v - 1),
                Type::UInt32(v) => Type::UInt32(v - 1),
                Type::UInt(v) => Type::UInt(v - 1),
                Type::UInt64(v) => Type::UInt64(v - 1),
                Type::UInt128(v) => Type::UInt128(v - 1),
                Type::Float32(v) => Type::Float32(v - 1.0),
                Type::Float64(v) => Type::Float64(v - 1.0),
                Type::Char(v) => Type::Char(char::from_u32(v as u32 - 1).unwrap()),
                _ => panic!("Decrement only works on number types."),
            },
            OpPrim1::Not => match t {
                Type::Boolean(v) => Type::Boolean(!v),
                _ => panic!("Not only works on boolean type."),
            },
            OpPrim1::BitwiseNot => match t {
                Type::Int8(v) => Type::Int8(!v),
                Type::Int16(v) => Type::Int16(!v),
                Type::Int32(v) => Type::Int32(!v),
                Type::Int(v) => Type::Int(!v),
                Type::Int64(v) => Type::Int64(!v),
                Type::Int128(v) => Type::Int128(!v),
                Type::UInt8(v) => Type::UInt8(!v),
                Type::UInt16(v) => Type::UInt16(!v),
                Type::UInt32(v) => Type::UInt32(!v),
                Type::UInt(v) => Type::UInt(!v),
                Type::UInt64(v) => Type::UInt64(!v),
                Type::UInt128(v) => Type::UInt128(!v),
                _ => panic!("BitwiseNot only works on int number types."),
            },
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OpPrim2 {
    Add,            // +
    Subtract,       // -
    Multiplication, // *
    Division,       // /
    Modulus,        // %
    And,            // &&
    Or,             // ||
    Xor,            // ^^
    Equal,          // ==
    NotEqual,       // !=
    Greater,        // >
    Less,           // <
    GreaterEqual,   // >=
    LessEqual,      // <=
    BitwiseAnd,     // &
    BitwiseOr,      // |
    BitwiseXor,     // ^
    ShiftLeft,      // <<
    ShiftRight,     // >>
}
impl OpPrim2 {
    pub fn solve(&self, t1: Type, t2: Type) -> Type {
        match self {
            OpPrim2::Add => match (t1, t2) {
                (Type::Int8(v1), Type::Int8(v2)) => Type::Int8(v1 + v2),
                (Type::Int16(v1), Type::Int16(v2)) => Type::Int16(v1 + v2),
                (Type::Int32(v1), Type::Int32(v2)) => Type::Int32(v1 + v2),
                (Type::Int(v1), Type::Int(v2)) => Type::Int(v1 + v2),
                (Type::Int64(v1), Type::Int64(v2)) => Type::Int64(v1 + v2),
                (Type::Int128(v1), Type::Int128(v2)) => Type::Int128(v1 + v2),
                (Type::UInt8(v1), Type::UInt8(v2)) => Type::UInt8(v1 + v2),
                (Type::UInt16(v1), Type::UInt16(v2)) => Type::UInt16(v1 + v2),
                (Type::UInt32(v1), Type::UInt32(v2)) => Type::UInt32(v1 + v2),
                (Type::UInt(v1), Type::UInt(v2)) => Type::UInt(v1 + v2),
                (Type::UInt64(v1), Type::UInt64(v2)) => Type::UInt64(v1 + v2),
                (Type::UInt128(v1), Type::UInt128(v2)) => Type::UInt128(v1 + v2),
                (Type::Float32(v1), Type::Float32(v2)) => Type::Float32(v1 + v2),
                (Type::Float64(v1), Type::Float64(v2)) => Type::Float64(v1 + v2),
                (Type::Char(v1), Type::Char(v2)) => {
                    Type::Char(char::from_u32(v1 as u32 + v2 as u32).unwrap())
                }
                (Type::String(mut v1), Type::String(v2)) => Type::String({
                    v1.push_str(v2.as_str());
                    v1
                }),
                _ => panic!("Add only works on two of the same number types and two strings."),
            },
            OpPrim2::Subtract => match (t1, t2) {
                (Type::Int8(v1), Type::Int8(v2)) => Type::Int8(v1 - v2),
                (Type::Int16(v1), Type::Int16(v2)) => Type::Int16(v1 - v2),
                (Type::Int32(v1), Type::Int32(v2)) => Type::Int32(v1 - v2),
                (Type::Int(v1), Type::Int(v2)) => Type::Int(v1 - v2),
                (Type::Int64(v1), Type::Int64(v2)) => Type::Int64(v1 - v2),
                (Type::Int128(v1), Type::Int128(v2)) => Type::Int128(v1 - v2),
                (Type::UInt8(v1), Type::UInt8(v2)) => Type::UInt8(v1 - v2),
                (Type::UInt16(v1), Type::UInt16(v2)) => Type::UInt16(v1 - v2),
                (Type::UInt32(v1), Type::UInt32(v2)) => Type::UInt32(v1 - v2),
                (Type::UInt(v1), Type::UInt(v2)) => Type::UInt(v1 - v2),
                (Type::UInt64(v1), Type::UInt64(v2)) => Type::UInt64(v1 - v2),
                (Type::UInt128(v1), Type::UInt128(v2)) => Type::UInt128(v1 - v2),
                (Type::Float32(v1), Type::Float32(v2)) => Type::Float32(v1 - v2),
                (Type::Float64(v1), Type::Float64(v2)) => Type::Float64(v1 - v2),
                (Type::Char(v1), Type::Char(v2)) => {
                    Type::Char(char::from_u32(v1 as u32 - v2 as u32).unwrap())
                }
                _ => panic!("Subtract only works on two of the same number types."),
            },
            OpPrim2::Multiplication => match (t1, t2) {
                (Type::Int8(v1), Type::Int8(v2)) => Type::Int8(v1 * v2),
                (Type::Int16(v1), Type::Int16(v2)) => Type::Int16(v1 * v2),
                (Type::Int32(v1), Type::Int32(v2)) => Type::Int32(v1 * v2),
                (Type::Int(v1), Type::Int(v2)) => Type::Int(v1 * v2),
                (Type::Int64(v1), Type::Int64(v2)) => Type::Int64(v1 * v2),
                (Type::Int128(v1), Type::Int128(v2)) => Type::Int128(v1 * v2),
                (Type::UInt8(v1), Type::UInt8(v2)) => Type::UInt8(v1 * v2),
                (Type::UInt16(v1), Type::UInt16(v2)) => Type::UInt16(v1 * v2),
                (Type::UInt32(v1), Type::UInt32(v2)) => Type::UInt32(v1 * v2),
                (Type::UInt(v1), Type::UInt(v2)) => Type::UInt(v1 * v2),
                (Type::UInt64(v1), Type::UInt64(v2)) => Type::UInt64(v1 * v2),
                (Type::UInt128(v1), Type::UInt128(v2)) => Type::UInt128(v1 * v2),
                (Type::Float32(v1), Type::Float32(v2)) => Type::Float32(v1 * v2),
                (Type::Float64(v1), Type::Float64(v2)) => Type::Float64(v1 * v2),
                (Type::Char(v1), Type::Char(v2)) => {
                    Type::Char(char::from_u32(v1 as u32 * v2 as u32).unwrap())
                }
                _ => panic!("Multiplication only works on two of the same number types."),
            },
            OpPrim2::Division => match (t1, t2) {
                (Type::Int8(v1), Type::Int8(v2)) => Type::Int8(v1 / v2),
                (Type::Int16(v1), Type::Int16(v2)) => Type::Int16(v1 / v2),
                (Type::Int32(v1), Type::Int32(v2)) => Type::Int32(v1 / v2),
                (Type::Int(v1), Type::Int(v2)) => Type::Int(v1 / v2),
                (Type::Int64(v1), Type::Int64(v2)) => Type::Int64(v1 / v2),
                (Type::Int128(v1), Type::Int128(v2)) => Type::Int128(v1 / v2),
                (Type::UInt8(v1), Type::UInt8(v2)) => Type::UInt8(v1 / v2),
                (Type::UInt16(v1), Type::UInt16(v2)) => Type::UInt16(v1 / v2),
                (Type::UInt32(v1), Type::UInt32(v2)) => Type::UInt32(v1 / v2),
                (Type::UInt(v1), Type::UInt(v2)) => Type::UInt(v1 / v2),
                (Type::UInt64(v1), Type::UInt64(v2)) => Type::UInt64(v1 / v2),
                (Type::UInt128(v1), Type::UInt128(v2)) => Type::UInt128(v1 / v2),
                (Type::Float32(v1), Type::Float32(v2)) => Type::Float32(v1 / v2),
                (Type::Float64(v1), Type::Float64(v2)) => Type::Float64(v1 / v2),
                (Type::Char(v1), Type::Char(v2)) => {
                    Type::Char(char::from_u32(v1 as u32 / v2 as u32).unwrap())
                }
                _ => panic!("Division only works on two of the same number types."),
            },
            OpPrim2::Modulus => match (t1, t2) {
                (Type::Int8(v1), Type::Int8(v2)) => Type::Int8(v1 % v2),
                (Type::Int16(v1), Type::Int16(v2)) => Type::Int16(v1 % v2),
                (Type::Int32(v1), Type::Int32(v2)) => Type::Int32(v1 % v2),
                (Type::Int(v1), Type::Int(v2)) => Type::Int(v1 % v2),
                (Type::Int64(v1), Type::Int64(v2)) => Type::Int64(v1 % v2),
                (Type::Int128(v1), Type::Int128(v2)) => Type::Int128(v1 % v2),
                (Type::UInt8(v1), Type::UInt8(v2)) => Type::UInt8(v1 % v2),
                (Type::UInt16(v1), Type::UInt16(v2)) => Type::UInt16(v1 % v2),
                (Type::UInt32(v1), Type::UInt32(v2)) => Type::UInt32(v1 % v2),
                (Type::UInt(v1), Type::UInt(v2)) => Type::UInt(v1 % v2),
                (Type::UInt64(v1), Type::UInt64(v2)) => Type::UInt64(v1 % v2),
                (Type::UInt128(v1), Type::UInt128(v2)) => Type::UInt128(v1 % v2),
                (Type::Float32(v1), Type::Float32(v2)) => Type::Float32(v1 % v2),
                (Type::Float64(v1), Type::Float64(v2)) => Type::Float64(v1 % v2),
                (Type::Char(v1), Type::Char(v2)) => {
                    Type::Char(char::from_u32(v1 as u32 % v2 as u32).unwrap())
                }
                _ => panic!("Modulus only works on two of the same number types."),
            },
            OpPrim2::And => match (t1, t2) {
                (Type::Boolean(v1), Type::Boolean(v2)) => Type::Boolean(v1 && v2),
                _ => panic!("Add only works on two of the same boolean type."),
            },
            OpPrim2::Or => match (t1, t2) {
                (Type::Boolean(v1), Type::Boolean(v2)) => Type::Boolean(v1 || v2),
                _ => panic!("Or only works on two of the same boolean type."),
            },
            OpPrim2::Xor => match (t1, t2) {
                (Type::Boolean(v1), Type::Boolean(v2)) => Type::Boolean(v1 ^ v2),
                _ => panic!("Xor only works on two of the same boolean type."),
            },
            OpPrim2::Equal => match (t1, t2) {
                (Type::None, Type::None) => Type::Boolean(true),
                (Type::Int8(v1), Type::Int8(v2)) => Type::Boolean(v1 == v2),
                (Type::Int16(v1), Type::Int16(v2)) => Type::Boolean(v1 == v2),
                (Type::Int32(v1), Type::Int32(v2)) => Type::Boolean(v1 == v2),
                (Type::Int(v1), Type::Int(v2)) => Type::Boolean(v1 == v2),
                (Type::Int64(v1), Type::Int64(v2)) => Type::Boolean(v1 == v2),
                (Type::Int128(v1), Type::Int128(v2)) => Type::Boolean(v1 == v2),
                (Type::UInt8(v1), Type::UInt8(v2)) => Type::Boolean(v1 == v2),
                (Type::UInt16(v1), Type::UInt16(v2)) => Type::Boolean(v1 == v2),
                (Type::UInt32(v1), Type::UInt32(v2)) => Type::Boolean(v1 == v2),
                (Type::UInt(v1), Type::UInt(v2)) => Type::Boolean(v1 == v2),
                (Type::UInt64(v1), Type::UInt64(v2)) => Type::Boolean(v1 == v2),
                (Type::UInt128(v1), Type::UInt128(v2)) => Type::Boolean(v1 == v2),
                (Type::Float32(v1), Type::Float32(v2)) => Type::Boolean(v1 == v2),
                (Type::Float64(v1), Type::Float64(v2)) => Type::Boolean(v1 == v2),
                (Type::Char(v1), Type::Char(v2)) => Type::Boolean(v1 == v2),
                (Type::String(v1), Type::String(v2)) => Type::Boolean(v1.eq(&v2)),
                (Type::Boolean(v1), Type::Boolean(v2)) => Type::Boolean(v1 == v2),
                _ => panic!("Equal only works on two of the same primitive types."),
            },
            OpPrim2::NotEqual => match (t1, t2) {
                (Type::None, Type::None) => Type::Boolean(false),
                (Type::Int8(v1), Type::Int8(v2)) => Type::Boolean(v1 != v2),
                (Type::Int16(v1), Type::Int16(v2)) => Type::Boolean(v1 != v2),
                (Type::Int32(v1), Type::Int32(v2)) => Type::Boolean(v1 != v2),
                (Type::Int(v1), Type::Int(v2)) => Type::Boolean(v1 != v2),
                (Type::Int64(v1), Type::Int64(v2)) => Type::Boolean(v1 != v2),
                (Type::Int128(v1), Type::Int128(v2)) => Type::Boolean(v1 != v2),
                (Type::UInt8(v1), Type::UInt8(v2)) => Type::Boolean(v1 != v2),
                (Type::UInt16(v1), Type::UInt16(v2)) => Type::Boolean(v1 != v2),
                (Type::UInt32(v1), Type::UInt32(v2)) => Type::Boolean(v1 != v2),
                (Type::UInt(v1), Type::UInt(v2)) => Type::Boolean(v1 != v2),
                (Type::UInt64(v1), Type::UInt64(v2)) => Type::Boolean(v1 != v2),
                (Type::UInt128(v1), Type::UInt128(v2)) => Type::Boolean(v1 != v2),
                (Type::Float32(v1), Type::Float32(v2)) => Type::Boolean(v1 != v2),
                (Type::Float64(v1), Type::Float64(v2)) => Type::Boolean(v1 != v2),
                (Type::Char(v1), Type::Char(v2)) => Type::Boolean(v1 != v2),
                (Type::String(v1), Type::String(v2)) => Type::Boolean(v1.ne(&v2)),
                (Type::Boolean(v1), Type::Boolean(v2)) => Type::Boolean(v1 != v2),
                _ => panic!("NotEqual only works on two of the same primitive types."),
            },
            OpPrim2::Greater => match (t1, t2) {
                (Type::Int8(v1), Type::Int8(v2)) => Type::Boolean(v1 > v2),
                (Type::Int16(v1), Type::Int16(v2)) => Type::Boolean(v1 > v2),
                (Type::Int32(v1), Type::Int32(v2)) => Type::Boolean(v1 > v2),
                (Type::Int(v1), Type::Int(v2)) => Type::Boolean(v1 > v2),
                (Type::Int64(v1), Type::Int64(v2)) => Type::Boolean(v1 > v2),
                (Type::Int128(v1), Type::Int128(v2)) => Type::Boolean(v1 > v2),
                (Type::UInt8(v1), Type::UInt8(v2)) => Type::Boolean(v1 > v2),
                (Type::UInt16(v1), Type::UInt16(v2)) => Type::Boolean(v1 > v2),
                (Type::UInt32(v1), Type::UInt32(v2)) => Type::Boolean(v1 > v2),
                (Type::UInt(v1), Type::UInt(v2)) => Type::Boolean(v1 > v2),
                (Type::UInt64(v1), Type::UInt64(v2)) => Type::Boolean(v1 > v2),
                (Type::UInt128(v1), Type::UInt128(v2)) => Type::Boolean(v1 > v2),
                (Type::Float32(v1), Type::Float32(v2)) => Type::Boolean(v1 > v2),
                (Type::Float64(v1), Type::Float64(v2)) => Type::Boolean(v1 > v2),
                (Type::Char(v1), Type::Char(v2)) => Type::Boolean(v1 > v2),
                (Type::String(v1), Type::String(v2)) => Type::Boolean(v1.cmp(&v2).is_lt()),
                _ => panic!("Greater only works on two of the same number/string types."),
            },
            OpPrim2::Less => match (t1, t2) {
                (Type::Int8(v1), Type::Int8(v2)) => Type::Boolean(v1 < v2),
                (Type::Int16(v1), Type::Int16(v2)) => Type::Boolean(v1 < v2),
                (Type::Int32(v1), Type::Int32(v2)) => Type::Boolean(v1 < v2),
                (Type::Int(v1), Type::Int(v2)) => Type::Boolean(v1 < v2),
                (Type::Int64(v1), Type::Int64(v2)) => Type::Boolean(v1 < v2),
                (Type::Int128(v1), Type::Int128(v2)) => Type::Boolean(v1 < v2),
                (Type::UInt8(v1), Type::UInt8(v2)) => Type::Boolean(v1 < v2),
                (Type::UInt16(v1), Type::UInt16(v2)) => Type::Boolean(v1 < v2),
                (Type::UInt32(v1), Type::UInt32(v2)) => Type::Boolean(v1 < v2),
                (Type::UInt(v1), Type::UInt(v2)) => Type::Boolean(v1 < v2),
                (Type::UInt64(v1), Type::UInt64(v2)) => Type::Boolean(v1 < v2),
                (Type::UInt128(v1), Type::UInt128(v2)) => Type::Boolean(v1 < v2),
                (Type::Float32(v1), Type::Float32(v2)) => Type::Boolean(v1 < v2),
                (Type::Float64(v1), Type::Float64(v2)) => Type::Boolean(v1 < v2),
                (Type::Char(v1), Type::Char(v2)) => Type::Boolean(v1 < v2),
                (Type::String(v1), Type::String(v2)) => Type::Boolean(v1.cmp(&v2).is_gt()),
                _ => panic!("Less only works on two of the same number/string types."),
            },
            OpPrim2::GreaterEqual => match (t1, t2) {
                (Type::Int8(v1), Type::Int8(v2)) => Type::Boolean(v1 >= v2),
                (Type::Int16(v1), Type::Int16(v2)) => Type::Boolean(v1 >= v2),
                (Type::Int32(v1), Type::Int32(v2)) => Type::Boolean(v1 >= v2),
                (Type::Int(v1), Type::Int(v2)) => Type::Boolean(v1 >= v2),
                (Type::Int64(v1), Type::Int64(v2)) => Type::Boolean(v1 >= v2),
                (Type::Int128(v1), Type::Int128(v2)) => Type::Boolean(v1 >= v2),
                (Type::UInt8(v1), Type::UInt8(v2)) => Type::Boolean(v1 >= v2),
                (Type::UInt16(v1), Type::UInt16(v2)) => Type::Boolean(v1 >= v2),
                (Type::UInt32(v1), Type::UInt32(v2)) => Type::Boolean(v1 >= v2),
                (Type::UInt(v1), Type::UInt(v2)) => Type::Boolean(v1 >= v2),
                (Type::UInt64(v1), Type::UInt64(v2)) => Type::Boolean(v1 >= v2),
                (Type::UInt128(v1), Type::UInt128(v2)) => Type::Boolean(v1 >= v2),
                (Type::Float32(v1), Type::Float32(v2)) => Type::Boolean(v1 >= v2),
                (Type::Float64(v1), Type::Float64(v2)) => Type::Boolean(v1 >= v2),
                (Type::Char(v1), Type::Char(v2)) => Type::Boolean(v1 >= v2),
                (Type::String(v1), Type::String(v2)) => Type::Boolean(v1.cmp(&v2).is_le()),
                _ => panic!("GreaterEqual only works on two of the same number/string types."),
            },
            OpPrim2::LessEqual => match (t1, t2) {
                (Type::Int8(v1), Type::Int8(v2)) => Type::Boolean(v1 <= v2),
                (Type::Int16(v1), Type::Int16(v2)) => Type::Boolean(v1 <= v2),
                (Type::Int32(v1), Type::Int32(v2)) => Type::Boolean(v1 <= v2),
                (Type::Int(v1), Type::Int(v2)) => Type::Boolean(v1 <= v2),
                (Type::Int64(v1), Type::Int64(v2)) => Type::Boolean(v1 <= v2),
                (Type::Int128(v1), Type::Int128(v2)) => Type::Boolean(v1 <= v2),
                (Type::UInt8(v1), Type::UInt8(v2)) => Type::Boolean(v1 <= v2),
                (Type::UInt16(v1), Type::UInt16(v2)) => Type::Boolean(v1 <= v2),
                (Type::UInt32(v1), Type::UInt32(v2)) => Type::Boolean(v1 <= v2),
                (Type::UInt(v1), Type::UInt(v2)) => Type::Boolean(v1 <= v2),
                (Type::UInt64(v1), Type::UInt64(v2)) => Type::Boolean(v1 <= v2),
                (Type::UInt128(v1), Type::UInt128(v2)) => Type::Boolean(v1 <= v2),
                (Type::Float32(v1), Type::Float32(v2)) => Type::Boolean(v1 <= v2),
                (Type::Float64(v1), Type::Float64(v2)) => Type::Boolean(v1 <= v2),
                (Type::Char(v1), Type::Char(v2)) => Type::Boolean(v1 <= v2),
                (Type::String(v1), Type::String(v2)) => Type::Boolean(v1.cmp(&v2).is_ge()),
                _ => panic!("LessEqual only works on two of the same number/string types."),
            },
            OpPrim2::BitwiseAnd => match (t1, t2) {
                (Type::Int8(v1), Type::Int8(v2)) => Type::Int8(v1 & v2),
                (Type::Int16(v1), Type::Int16(v2)) => Type::Int16(v1 & v2),
                (Type::Int32(v1), Type::Int32(v2)) => Type::Int32(v1 & v2),
                (Type::Int(v1), Type::Int(v2)) => Type::Int(v1 & v2),
                (Type::Int64(v1), Type::Int64(v2)) => Type::Int64(v1 & v2),
                (Type::Int128(v1), Type::Int128(v2)) => Type::Int128(v1 & v2),
                (Type::UInt8(v1), Type::UInt8(v2)) => Type::UInt8(v1 & v2),
                (Type::UInt16(v1), Type::UInt16(v2)) => Type::UInt16(v1 & v2),
                (Type::UInt32(v1), Type::UInt32(v2)) => Type::UInt32(v1 & v2),
                (Type::UInt(v1), Type::UInt(v2)) => Type::UInt(v1 & v2),
                (Type::UInt64(v1), Type::UInt64(v2)) => Type::UInt64(v1 & v2),
                (Type::UInt128(v1), Type::UInt128(v2)) => Type::UInt128(v1 & v2),
                (Type::Char(v1), Type::Char(v2)) => {
                    Type::Char(char::from_u32(v1 as u32 & v2 as u32).unwrap())
                }
                _ => panic!("BitwiseAnd only works on an int number type and an UInt."),
            },
            OpPrim2::BitwiseOr => match (t1, t2) {
                (Type::Int8(v1), Type::Int8(v2)) => Type::Int8(v1 | v2),
                (Type::Int16(v1), Type::Int16(v2)) => Type::Int16(v1 | v2),
                (Type::Int32(v1), Type::Int32(v2)) => Type::Int32(v1 | v2),
                (Type::Int(v1), Type::Int(v2)) => Type::Int(v1 | v2),
                (Type::Int64(v1), Type::Int64(v2)) => Type::Int64(v1 | v2),
                (Type::Int128(v1), Type::Int128(v2)) => Type::Int128(v1 | v2),
                (Type::UInt8(v1), Type::UInt8(v2)) => Type::UInt8(v1 | v2),
                (Type::UInt16(v1), Type::UInt16(v2)) => Type::UInt16(v1 | v2),
                (Type::UInt32(v1), Type::UInt32(v2)) => Type::UInt32(v1 | v2),
                (Type::UInt(v1), Type::UInt(v2)) => Type::UInt(v1 | v2),
                (Type::UInt64(v1), Type::UInt64(v2)) => Type::UInt64(v1 | v2),
                (Type::UInt128(v1), Type::UInt128(v2)) => Type::UInt128(v1 | v2),
                (Type::Char(v1), Type::Char(v2)) => {
                    Type::Char(char::from_u32(v1 as u32 | v2 as u32).unwrap())
                }
                _ => panic!("BitwiseOr only works on an int number type and an UInt."),
            },
            OpPrim2::BitwiseXor => match (t1, t2) {
                (Type::Int8(v1), Type::Int8(v2)) => Type::Int8(v1 ^ v2),
                (Type::Int16(v1), Type::Int16(v2)) => Type::Int16(v1 ^ v2),
                (Type::Int32(v1), Type::Int32(v2)) => Type::Int32(v1 ^ v2),
                (Type::Int(v1), Type::Int(v2)) => Type::Int(v1 ^ v2),
                (Type::Int64(v1), Type::Int64(v2)) => Type::Int64(v1 ^ v2),
                (Type::Int128(v1), Type::Int128(v2)) => Type::Int128(v1 ^ v2),
                (Type::UInt8(v1), Type::UInt8(v2)) => Type::UInt8(v1 ^ v2),
                (Type::UInt16(v1), Type::UInt16(v2)) => Type::UInt16(v1 ^ v2),
                (Type::UInt32(v1), Type::UInt32(v2)) => Type::UInt32(v1 ^ v2),
                (Type::UInt(v1), Type::UInt(v2)) => Type::UInt(v1 ^ v2),
                (Type::UInt64(v1), Type::UInt64(v2)) => Type::UInt64(v1 ^ v2),
                (Type::UInt128(v1), Type::UInt128(v2)) => Type::UInt128(v1 ^ v2),
                (Type::Char(v1), Type::Char(v2)) => {
                    Type::Char(char::from_u32(v1 as u32 ^ v2 as u32).unwrap())
                }
                _ => panic!("BitwiseXor only works on an int number type and an UInt."),
            },
            OpPrim2::ShiftLeft => match (t1, t2) {
                (Type::Int8(v1), Type::UInt(v2)) => Type::Int8(v1 << v2),
                (Type::Int16(v1), Type::UInt(v2)) => Type::Int16(v1 << v2),
                (Type::Int32(v1), Type::UInt(v2)) => Type::Int32(v1 << v2),
                (Type::Int(v1), Type::UInt(v2)) => Type::Int(v1 << v2),
                (Type::Int64(v1), Type::UInt(v2)) => Type::Int64(v1 << v2),
                (Type::Int128(v1), Type::UInt(v2)) => Type::Int128(v1 << v2),
                (Type::UInt8(v1), Type::UInt(v2)) => Type::UInt8(v1 << v2),
                (Type::UInt16(v1), Type::UInt(v2)) => Type::UInt16(v1 << v2),
                (Type::UInt32(v1), Type::UInt(v2)) => Type::UInt32(v1 << v2),
                (Type::UInt(v1), Type::UInt(v2)) => Type::UInt(v1 << v2),
                (Type::UInt64(v1), Type::UInt(v2)) => Type::UInt64(v1 << v2),
                (Type::UInt128(v1), Type::UInt(v2)) => Type::UInt128(v1 << v2),
                (Type::Char(v1), Type::UInt(v2)) => {
                    Type::Char(char::from_u32((v1 as u32) << (v2 as usize)).unwrap())
                }
                _ => panic!("ShiftLeft only works on an int number type and an UInt."),
            },
            OpPrim2::ShiftRight => match (t1, t2) {
                (Type::Int8(v1), Type::UInt(v2)) => Type::Int8(v1 >> v2),
                (Type::Int16(v1), Type::UInt(v2)) => Type::Int16(v1 >> v2),
                (Type::Int32(v1), Type::UInt(v2)) => Type::Int32(v1 >> v2),
                (Type::Int(v1), Type::UInt(v2)) => Type::Int(v1 >> v2),
                (Type::Int64(v1), Type::UInt(v2)) => Type::Int64(v1 >> v2),
                (Type::Int128(v1), Type::UInt(v2)) => Type::Int128(v1 >> v2),
                (Type::UInt8(v1), Type::UInt(v2)) => Type::UInt8(v1 >> v2),
                (Type::UInt16(v1), Type::UInt(v2)) => Type::UInt16(v1 >> v2),
                (Type::UInt32(v1), Type::UInt(v2)) => Type::UInt32(v1 >> v2),
                (Type::UInt(v1), Type::UInt(v2)) => Type::UInt(v1 >> v2),
                (Type::UInt64(v1), Type::UInt(v2)) => Type::UInt64(v1 >> v2),
                (Type::UInt128(v1), Type::UInt(v2)) => Type::UInt128(v1 >> v2),
                (Type::Char(v1), Type::UInt(v2)) => {
                    Type::Char(char::from_u32((v1 as u32) >> (v2 as usize)).unwrap())
                }
                _ => panic!("ShiftRight only works on an int number type and an UInt."),
            },
        }
    }
}
