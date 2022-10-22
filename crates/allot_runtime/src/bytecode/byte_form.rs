use crate::{OpPrim1, OpPrim2, Operation, RawType, Register};

pub trait ByteForm {
    fn to_byte(&self) -> u8;
    fn from_byte(byte: u8) -> Self; // TODO: Result type instead of panic?
}

// Registers

impl ByteForm for Register {
    fn to_byte(&self) -> u8 {
        match self {
            Register::R0 => 0,
            Register::R1 => 1,
            Register::R2 => 2,
            Register::R3 => 3,
            Register::R4 => 4,
            Register::R5 => 5,
            Register::R6 => 6,
            Register::R7 => 7,
            Register::R8 => 8,
            Register::R9 => 9,
            Register::R10 => 10,
            Register::R11 => 11,
            Register::R12 => 12,
            Register::R13 => 13,
            Register::R14 => 14,
            Register::R15 => 15,
            Register::R16 => 16,
            Register::R17 => 17,
            Register::R18 => 18,
            Register::R19 => 19,
            Register::R20 => 20,
            Register::R21 => 21,
            Register::R22 => 22,
            Register::R23 => 23,
            Register::R24 => 24,
            Register::R25 => 25,
            Register::R26 => 26,
            Register::R27 => 27,
            Register::R28 => 28,
            Register::R29 => 29,
            Register::None => 255,
        }
    }

    fn from_byte(byte: u8) -> Self {
        match byte {
            0 => Register::R0,
            1 => Register::R1,
            2 => Register::R2,
            3 => Register::R3,
            4 => Register::R4,
            5 => Register::R5,
            6 => Register::R6,
            7 => Register::R7,
            8 => Register::R8,
            9 => Register::R9,
            10 => Register::R10,
            11 => Register::R11,
            12 => Register::R12,
            13 => Register::R13,
            14 => Register::R14,
            15 => Register::R15,
            16 => Register::R16,
            17 => Register::R17,
            18 => Register::R18,
            19 => Register::R19,
            20 => Register::R20,
            21 => Register::R21,
            22 => Register::R22,
            23 => Register::R23,
            24 => Register::R24,
            25 => Register::R25,
            26 => Register::R26,
            27 => Register::R27,
            28 => Register::R28,
            29 => Register::R29,
            255 => Register::None,
            _ => panic!("Invalid Register byte: {byte}"),
        }
    }
}

// Types

// TODO: Is this needed for types?
// impl ByteForm for Type {
//     fn to_byte(&self) -> u8 {
//         self.to_raw().to_byte()
//     }
//
//     fn from_byte(byte: u8) -> Self {
//         match byte {
//             0 => Type::None,
//             1 => Type::Int8(0),
//             2 => Type::Int16(0),
//             3 => Type::Int32(0),
//             4 => Type::Int(0),
//             5 => Type::Int64(0),
//             6 => Type::Int128(0),
//             7 => Type::UInt8(0),
//             8 => Type::UInt16(0),
//             9 => Type::UInt32(0),
//             10 => Type::UInt(0),
//             11 => Type::UInt64(0),
//             12 => Type::UInt128(0),
//             13 => Type::Float32(0.0),
//             14 => Type::Float64(0.0),
//             15 => Type::Char(' '),
//             16 => Type::String(String::new()),
//             17 => Type::Boolean(false),
//             18 => Type::Pointer(0),
//             19 => Type::Label(0),
//             20 => Type::Register(Register::None),
//             // 21 => Type::Thread(),
//             _ => panic!("Invalid Type byte: {byte}"),
//         }
//     }
// }

impl ByteForm for RawType {
    fn to_byte(&self) -> u8 {
        match self {
            RawType::None => 0,
            RawType::Int8 => 1,
            RawType::Int16 => 2,
            RawType::Int32 => 3,
            RawType::Int => 4,
            RawType::Int64 => 5,
            RawType::Int128 => 6,
            RawType::UInt8 => 7,
            RawType::UInt16 => 8,
            RawType::UInt32 => 9,
            RawType::UInt => 10,
            RawType::UInt64 => 11,
            RawType::UInt128 => 12,
            RawType::Float32 => 13,
            RawType::Float64 => 14,
            RawType::Char => 15,
            RawType::String => 16,
            RawType::Boolean => 17,
            RawType::Pointer => 18,
            RawType::Address => 19,
            RawType::Register => 20,
            RawType::Thread => 21,
        }
    }

    fn from_byte(byte: u8) -> Self {
        match byte {
            0 => RawType::None,
            1 => RawType::Int8,
            2 => RawType::Int16,
            3 => RawType::Int32,
            4 => RawType::Int,
            5 => RawType::Int64,
            6 => RawType::Int128,
            7 => RawType::UInt8,
            8 => RawType::UInt16,
            9 => RawType::UInt32,
            10 => RawType::UInt,
            11 => RawType::UInt64,
            12 => RawType::UInt128,
            13 => RawType::Float32,
            14 => RawType::Float64,
            15 => RawType::Char,
            16 => RawType::String,
            17 => RawType::Boolean,
            18 => RawType::Pointer,
            19 => RawType::Address,
            20 => RawType::Register,
            21 => RawType::Thread,
            _ => panic!("Invalid RawType byte: {byte}"),
        }
    }
}

// Operations

impl ByteForm for Operation {
    fn to_byte(&self) -> u8 {
        match self {
            Operation::Prim1(op) => op.to_byte(),
            Operation::Prim2(op) => (0b10000000) | op.to_byte(),
        }
    }

    fn from_byte(byte: u8) -> Self {
        let op = (byte & 0b10000000) >> 7;
        let t = byte & 0b01111111;

        match op {
            0 => Operation::Prim1(OpPrim1::from_byte(t)),
            1 => Operation::Prim2(OpPrim2::from_byte(t)),
            _ => panic!("Invalid Operation byte: {byte}"),
        }
    }
}

impl ByteForm for OpPrim1 {
    fn to_byte(&self) -> u8 {
        match self {
            OpPrim1::Increment => 0,
            OpPrim1::Decrement => 1,
            OpPrim1::Not => 2,
            OpPrim1::BitwiseNot => 3,
        }
    }

    fn from_byte(byte: u8) -> Self {
        match byte {
            0 => OpPrim1::Increment,
            1 => OpPrim1::Decrement,
            2 => OpPrim1::Not,
            3 => OpPrim1::BitwiseNot,
            _ => panic!("Invalid OpPrim1 byte: {byte}"),
        }
    }
}

impl ByteForm for OpPrim2 {
    fn to_byte(&self) -> u8 {
        match self {
            OpPrim2::Add => 0,
            OpPrim2::Subtract => 1,
            OpPrim2::Multiplication => 2,
            OpPrim2::Division => 3,
            OpPrim2::Modulus => 4,
            OpPrim2::And => 5,
            OpPrim2::Or => 6,
            OpPrim2::Xor => 7,
            OpPrim2::Equal => 8,
            OpPrim2::NotEqual => 9,
            OpPrim2::Greater => 10,
            OpPrim2::Less => 11,
            OpPrim2::GreaterEqual => 12,
            OpPrim2::LessEqual => 13,
            OpPrim2::BitwiseAnd => 14,
            OpPrim2::BitwiseOr => 15,
            OpPrim2::BitwiseXor => 16,
            OpPrim2::ShiftLeft => 17,
            OpPrim2::ShiftRight => 18,
        }
    }

    fn from_byte(byte: u8) -> Self {
        match byte {
            0 => OpPrim2::Add,
            1 => OpPrim2::Subtract,
            2 => OpPrim2::Multiplication,
            3 => OpPrim2::Division,
            4 => OpPrim2::Modulus,
            5 => OpPrim2::And,
            6 => OpPrim2::Or,
            7 => OpPrim2::Xor,
            8 => OpPrim2::Equal,
            9 => OpPrim2::NotEqual,
            10 => OpPrim2::Greater,
            11 => OpPrim2::Less,
            12 => OpPrim2::GreaterEqual,
            13 => OpPrim2::LessEqual,
            14 => OpPrim2::BitwiseAnd,
            15 => OpPrim2::BitwiseOr,
            16 => OpPrim2::BitwiseXor,
            17 => OpPrim2::ShiftLeft,
            18 => OpPrim2::ShiftRight,
            _ => panic!("Invalid OpPrim2 byte: {byte}"),
        }
    }
}
