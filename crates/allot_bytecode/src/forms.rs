use allot_lib::{OpPrim1, OpPrim2, Operation, RawInstruction, RawType, Register};

pub trait ByteForm {
    fn to_byte(&self) -> u8;
    fn from_byte(byte: u8) -> Self; // TODO: Result type instead of panic?
}

// Instructions

impl ByteForm for RawInstruction {
    fn to_byte(&self) -> u8 {
        match self {
            RawInstruction::Nop => 0,
            RawInstruction::Op => 1,
            RawInstruction::Mov => 2,
            RawInstruction::Cpy => 3,
            RawInstruction::Cast => 4,
            RawInstruction::Lea => 5,
            RawInstruction::Jmp => 6,
            RawInstruction::Ret => 7,
            RawInstruction::Call => 8,
            RawInstruction::Exit => 9,
            RawInstruction::Push => 10,
            RawInstruction::PushCpy => 11,
            RawInstruction::Pop => 12,
            RawInstruction::PopMany => 13,
            RawInstruction::StackCpy => 14,
            RawInstruction::PushFrame => 15,
            RawInstruction::PopFrame => 16,
            RawInstruction::TakeFrom => 17,
            RawInstruction::GiveTo => 18,
            RawInstruction::ThreadCreate => 19,
            RawInstruction::ThreadJoin => 20,
            RawInstruction::Assert => 21,
            #[cfg(debug_assertions)]
            RawInstruction::Dbg => 128,
            #[cfg(debug_assertions)]
            RawInstruction::Dump => 129,
        }
    }

    fn from_byte(byte: u8) -> Self {
        match byte {
            0 => RawInstruction::Nop,
            1 => RawInstruction::Op,
            2 => RawInstruction::Mov,
            3 => RawInstruction::Cpy,
            4 => RawInstruction::Cast,
            5 => RawInstruction::Lea,
            6 => RawInstruction::Jmp,
            7 => RawInstruction::Ret,
            8 => RawInstruction::Call,
            9 => RawInstruction::Exit,
            10 => RawInstruction::Push,
            11 => RawInstruction::PushCpy,
            12 => RawInstruction::Pop,
            13 => RawInstruction::PopMany,
            14 => RawInstruction::StackCpy,
            15 => RawInstruction::PushFrame,
            16 => RawInstruction::PopFrame,
            17 => RawInstruction::TakeFrom,
            18 => RawInstruction::GiveTo,
            19 => RawInstruction::ThreadCreate,
            20 => RawInstruction::ThreadJoin,
            21 => RawInstruction::Assert,
            #[cfg(debug_assertions)]
            128 => RawInstruction::Dbg,
            #[cfg(debug_assertions)]
            129 => RawInstruction::Dump,
            _ => panic!("Invalid Instruction byte: {byte}"),
        }
    }
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
        Register::cast(byte)
    }
}

// Types

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
            RawType::Address => 18,
            RawType::Pointer => 19,
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
            18 => RawType::Address,
            19 => RawType::Pointer,
            20 => RawType::Register,
            _ => panic!("Invalid Type byte: {byte}"),
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
            OpPrim2::SameType => 19,
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
            19 => OpPrim2::SameType,
            _ => panic!("Invalid OpPrim2 byte: {byte}"),
        }
    }
}
