use allot_lib::{Instruction, Operation, Register, Type};

use crate::{Buffer, ByteForm, BYTECODE_VERSION};

pub fn gen(instructions: Vec<Instruction>) -> Vec<u8> {
    let mut buffer = Buffer::new();
    buffer.write_u64(BYTECODE_VERSION as u64);

    for i in instructions {
        write_instruction(&mut buffer, &i);

        match i {
            Instruction::Nop => {}
            Instruction::Op(v1, v2) => {
                write_op(&mut buffer, &v1);
                write_register(&mut buffer, &v2[0]);
                write_register(&mut buffer, &v2[1]);
            }
            Instruction::Mov(v1, v2) => {
                write_register(&mut buffer, &v1);
                write_type(&mut buffer, &v2);
            }
            Instruction::Cpy(v1, v2) => {
                write_register(&mut buffer, &v1);
                write_register(&mut buffer, &v2);
            }
            Instruction::Cast(v1, v2) => {
                write_register(&mut buffer, &v1);
                buffer.write_u8(v2.to_byte());
            }
            Instruction::Lea(v1, v2) => {
                write_register(&mut buffer, &v1);
                buffer.write_u64(v2 as u64);
            }
            Instruction::Jmp(v1, v2) => {
                match v1 {
                    None => write_register(&mut buffer, &Register::None),
                    Some(v) => write_register(&mut buffer, &v),
                }
                write_type(&mut buffer, &v2);
            }
            Instruction::Ret => {}
            Instruction::Call(v) => buffer.write_string(&v),
            Instruction::Exit(v) => write_type(&mut buffer, &v),
            Instruction::Push(v) => write_register(&mut buffer, &v),
            Instruction::PushCpy(v) => write_register(&mut buffer, &v),
            Instruction::Pop(v) => match v {
                None => write_register(&mut buffer, &Register::None),
                Some(v) => write_register(&mut buffer, &v),
            },
            Instruction::PopMany(v) => write_type(&mut buffer, &v),
            Instruction::StackCpy(v1, v2) => {
                write_register(&mut buffer, &v1);
                write_type(&mut buffer, &v2);
            }
            Instruction::PushFrame(v) => buffer.write_bool(v),
            Instruction::PopFrame => {}
            Instruction::PushOnto(v) => match v {
                None => write_register(&mut buffer, &Register::None),
                Some(v) => write_register(&mut buffer, &v),
            },
            Instruction::PopInto => {}
            Instruction::ThreadCreate(v) => write_type(&mut buffer, &v),
            Instruction::ThreadJoin(v) => write_register(&mut buffer, &v),
            Instruction::Assert(v1, v2) => {
                write_register(&mut buffer, &v1);
                write_type(&mut buffer, &v2);
            }
            #[cfg(debug_assertions)]
            Instruction::Dbg(v) => write_register(&mut buffer, &v),
            #[cfg(debug_assertions)]
            Instruction::Dump(v) => buffer.write_u8(v),
        }
    }

    buffer.0
}

fn write_instruction(buffer: &mut Buffer, i: &Instruction) {
    let b = i.to_raw().to_byte();
    buffer.write_u8(b);
}

fn write_register(buffer: &mut Buffer, r: &Register) {
    let b = r.to_byte();
    buffer.write_u8(b);
}

fn write_type(buffer: &mut Buffer, t: &Type) {
    let b = t.to_raw().to_byte();
    buffer.write_u8(b);

    match t {
        Type::None => {}
        Type::Int8(v) => buffer.write_i8(*v),
        Type::Int16(v) => buffer.write_i16(*v),
        Type::Int32(v) => buffer.write_i32(*v),
        Type::Int(v) => buffer.write_i64(*v as i64),
        Type::Int64(v) => buffer.write_i64(*v),
        Type::Int128(v) => buffer.write_i128(*v),
        Type::UInt8(v) => buffer.write_u8(*v),
        Type::UInt16(v) => buffer.write_u16(*v),
        Type::UInt32(v) => buffer.write_u32(*v),
        Type::UInt(v) => buffer.write_u64(*v as u64),
        Type::UInt64(v) => buffer.write_u64(*v),
        Type::UInt128(v) => buffer.write_u128(*v),
        Type::Float32(v) => buffer.write_f32(*v),
        Type::Float64(v) => buffer.write_f64(*v),
        Type::Char(v) => buffer.write_char(*v),
        Type::String(v) => buffer.write_string(v),
        Type::Boolean(v) => buffer.write_bool(*v),
        Type::Address(v) => buffer.write_u64(*v as u64),
        Type::Pointer(v) => buffer.write_u64(*v as u64),
        Type::Register(v) => write_register(buffer, v),
    }
}

fn write_op(buffer: &mut Buffer, o: &Operation) {
    let b = o.to_byte();
    buffer.write_u8(b);
}
