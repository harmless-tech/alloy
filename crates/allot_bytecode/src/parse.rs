use allot_lib::{
    Instruction, Instruction::Cast, Operation, RawInstruction, RawType, Register, Type,
};

use crate::{Buffer, ByteForm, BYTECODE_VERSION};

pub fn parse(bytes: Vec<u8>) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    let mut buffer = Buffer::with(bytes);

    let bytecode_version = buffer.read_u64() as usize;
    if bytecode_version != BYTECODE_VERSION {
        panic!("Give bytecode version is not equal to current bytecode version.");
        // TODO: Return an error instead.
    }

    while !buffer.is_empty() {
        instructions.push(read_instruction(&mut buffer));
    }

    instructions
}

fn read_instruction(buffer: &mut Buffer) -> Instruction {
    let raw = RawInstruction::from_byte(buffer.read_u8());
    match raw {
        RawInstruction::Nop => Instruction::Nop,
        RawInstruction::Op => Instruction::Op(
            read_op(buffer),
            [read_register(buffer), read_register(buffer)],
        ),
        RawInstruction::Mov => Instruction::Mov(read_register(buffer), read_type(buffer)),
        RawInstruction::Cpy => Instruction::Cpy(read_register(buffer), read_register(buffer)),
        RawInstruction::Cast => Cast(read_register(buffer), RawType::from_byte(buffer.read_u8())),
        RawInstruction::Lea => Instruction::Lea(read_register(buffer), buffer.read_u64() as usize),
        RawInstruction::Jmp => {
            let reg = read_register(buffer);
            let reg = match reg {
                Register::None => None,
                _ => Some(reg),
            };
            Instruction::Jmp(reg, read_type(buffer))
        }
        RawInstruction::Ret => Instruction::Ret,
        RawInstruction::Call => Instruction::Call(buffer.read_string()),
        RawInstruction::Exit => Instruction::Exit(read_type(buffer)),
        RawInstruction::Push => Instruction::Push(read_register(buffer)),
        RawInstruction::PushCpy => Instruction::PushCpy(read_register(buffer)),
        RawInstruction::Pop => {
            let reg = read_register(buffer);
            let reg = match reg {
                Register::None => None,
                _ => Some(reg),
            };
            Instruction::Pop(reg)
        }
        RawInstruction::PopMany => Instruction::PopMany(read_type(buffer)),
        RawInstruction::StackCpy => Instruction::StackCpy(read_register(buffer), read_type(buffer)),
        RawInstruction::PushFrame => Instruction::PushFrame(buffer.read_bool()),
        RawInstruction::PopFrame => Instruction::PopFrame,
        RawInstruction::PushOnto => {
            let reg = read_register(buffer);
            let reg = match reg {
                Register::None => None,
                _ => Some(reg),
            };
            Instruction::PushOnto(reg)
        }
        RawInstruction::PopInto => Instruction::PopInto,
        RawInstruction::ThreadCreate => Instruction::ThreadCreate(read_type(buffer)),
        RawInstruction::ThreadJoin => Instruction::ThreadJoin(read_register(buffer)),
        RawInstruction::Assert => Instruction::Assert(read_register(buffer), read_type(buffer)),
        #[cfg(debug_assertions)]
        RawInstruction::Dbg => Instruction::Dbg(read_register(buffer)),
        #[cfg(debug_assertions)]
        RawInstruction::Dump => Instruction::Dump(buffer.read_u8()),
    }
}

fn read_register(buffer: &mut Buffer) -> Register {
    Register::from_byte(buffer.read_u8())
}

fn read_type(buffer: &mut Buffer) -> Type {
    let raw = RawType::from_byte(buffer.read_u8());
    match raw {
        RawType::None => Type::None,
        RawType::Int8 => Type::Int8(buffer.read_i8()),
        RawType::Int16 => Type::Int16(buffer.read_i16()),
        RawType::Int32 => Type::Int32(buffer.read_i32()),
        RawType::Int => Type::Int(buffer.read_i64() as isize),
        RawType::Int64 => Type::Int64(buffer.read_i64()),
        RawType::Int128 => Type::Int128(buffer.read_i128()),
        RawType::UInt8 => Type::UInt8(buffer.read_u8()),
        RawType::UInt16 => Type::UInt16(buffer.read_u16()),
        RawType::UInt32 => Type::UInt32(buffer.read_u32()),
        RawType::UInt => Type::UInt(buffer.read_u64() as usize),
        RawType::UInt64 => Type::UInt64(buffer.read_u64()),
        RawType::UInt128 => Type::UInt128(buffer.read_u128()),
        RawType::Float32 => Type::Float32(buffer.read_f32()),
        RawType::Float64 => Type::Float64(buffer.read_f64()),
        RawType::Char => Type::Char(buffer.read_char()),
        RawType::String => Type::String(buffer.read_string()),
        RawType::Boolean => Type::Boolean(buffer.read_bool()),
        RawType::Address => Type::Address(buffer.read_u64() as usize),
        RawType::Pointer => Type::Pointer(buffer.read_u64() as usize),
        RawType::Register => Type::Register(read_register(buffer)),
    }
}

fn read_op(buffer: &mut Buffer) -> Operation {
    Operation::from_byte(buffer.read_u8())
}
