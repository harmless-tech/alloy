use crate::{
    ArithmeticOperation, BitWiseOperation, LogicOperation, Operation, RawType, Register,
    RelationalOperation, Type,
};

pub trait ByteForm {
    fn to_byte(&self) -> u8;
    fn from_byte(byte: u8) -> Self; // TODO: Result type instead of panic?
}

// Types

impl ByteForm for Type {
    fn to_byte(&self) -> u8 {
        self.to_raw().to_byte()
    }

    fn from_byte(byte: u8) -> Self {
        match byte {
            0 => Type::None,
            1 => Type::Int8(0),
            2 => Type::Int16(0),
            3 => Type::Int32(0),
            4 => Type::Int(0),
            5 => Type::Int64(0),
            6 => Type::Int128(0),
            7 => Type::UInt8(0),
            8 => Type::UInt16(0),
            9 => Type::UInt32(0),
            10 => Type::UInt(0),
            11 => Type::UInt64(0),
            12 => Type::UInt128(0),
            13 => Type::Float32(0.0),
            14 => Type::Float64(0.0),
            15 => Type::Char(' '),
            16 => Type::String(String::new()),
            17 => Type::Boolean(false),
            18 => Type::Pointer(0),
            19 => Type::Address(0),
            20 => Type::Register(Register::R0),
            _ => panic!("Invalid Type byte: {byte}"),
        }
    }
}

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
            _ => panic!("Invalid RawType byte: {byte}"),
        }
    }
}

// Operations

impl ByteForm for Operation {
    fn to_byte(&self) -> u8 {
        match self {
            Operation::Arithmetic(op) => op.to_byte(),
            Operation::BitWise(op) => (1 << 4) | op.to_byte(),
            Operation::Logic(op) => (2 << 4) | op.to_byte(),
            Operation::Relational(op) => (3 << 4) | op.to_byte(),
        }
    }

    fn from_byte(byte: u8) -> Self {
        let op = (byte & 0b11110000) >> 4;
        let t = byte & 0b00001111;

        match op {
            0 => Operation::Arithmetic(ArithmeticOperation::from_byte(t)),
            1 => Operation::BitWise(BitWiseOperation::from_byte(t)),
            2 => Operation::Logic(LogicOperation::from_byte(t)),
            3 => Operation::Relational(RelationalOperation::from_byte(t)),
            _ => panic!("Invalid Operation byte: {byte}"),
        }
    }
}

impl ByteForm for ArithmeticOperation {
    fn to_byte(&self) -> u8 {
        match self {
            ArithmeticOperation::Add => 0,
            ArithmeticOperation::Subtract => 1,
            ArithmeticOperation::Multiplication => 2,
            ArithmeticOperation::Division => 3,
            ArithmeticOperation::Modulus => 4,
            ArithmeticOperation::Increment => 5,
            ArithmeticOperation::Decrement => 6,
        }
    }

    fn from_byte(byte: u8) -> Self {
        match byte {
            0 => ArithmeticOperation::Add,
            1 => ArithmeticOperation::Subtract,
            2 => ArithmeticOperation::Multiplication,
            3 => ArithmeticOperation::Division,
            4 => ArithmeticOperation::Modulus,
            5 => ArithmeticOperation::Increment,
            6 => ArithmeticOperation::Decrement,
            _ => panic!("Invalid ArithmeticOperation byte: {byte}"),
        }
    }
}

impl ByteForm for BitWiseOperation {
    fn to_byte(&self) -> u8 {
        match self {
            BitWiseOperation::And => 0,
            BitWiseOperation::Or => 1,
            BitWiseOperation::Xor => 2,
            BitWiseOperation::Not => 3,
            BitWiseOperation::ShiftLeft(_) => 4,
            BitWiseOperation::ShiftRight(_) => 5,
        }
    }

    fn from_byte(byte: u8) -> Self {
        match byte {
            0 => BitWiseOperation::And,
            1 => BitWiseOperation::Or,
            2 => BitWiseOperation::Xor,
            3 => BitWiseOperation::Not,
            4 => BitWiseOperation::ShiftLeft(0),
            5 => BitWiseOperation::ShiftRight(0),
            _ => panic!("Invalid BitWiseOperation byte: {byte}"),
        }
    }
}

impl ByteForm for LogicOperation {
    fn to_byte(&self) -> u8 {
        match self {
            LogicOperation::Not => 0,
            LogicOperation::And => 1,
            LogicOperation::Or => 2,
        }
    }

    fn from_byte(byte: u8) -> Self {
        match byte {
            0 => LogicOperation::Not,
            1 => LogicOperation::And,
            2 => LogicOperation::Or,
            _ => panic!("Invalid LogicOperation byte: {byte}"),
        }
    }
}

impl ByteForm for RelationalOperation {
    fn to_byte(&self) -> u8 {
        match self {
            RelationalOperation::Equal => 0,
            RelationalOperation::NotEqual => 1,
            RelationalOperation::Greater => 2,
            RelationalOperation::Less => 3,
            RelationalOperation::GreaterEqual => 4,
            RelationalOperation::LessEqual => 5,
        }
    }

    fn from_byte(byte: u8) -> Self {
        match byte {
            0 => RelationalOperation::Equal,
            1 => RelationalOperation::NotEqual,
            2 => RelationalOperation::Greater,
            3 => RelationalOperation::Less,
            4 => RelationalOperation::GreaterEqual,
            5 => RelationalOperation::LessEqual,
            _ => panic!("Invalid RelationalOperation byte: {byte}"),
        }
    }
}
