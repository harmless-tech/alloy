use crate::Register;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Type {
    None,

    // Numeric
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int(isize),
    Int64(i64),
    Int128(i128),
    UInt8(u8),
    UInt16(u16),
    UInt32(u32),
    UInt(usize),
    UInt64(u64),
    UInt128(u128),

    Float32(f32),
    Float64(f64),

    // Text
    Char(char),
    String(String),

    // Other
    Boolean(bool),
    Address(usize),
    Pointer(usize),
    Register(Register),
    //TODO: Handle this with library function.
    // Thread(Rc<std::thread::JoinHandle<(Option<i32>, StackFrame)>>), // Exit, StackFrame
}
impl Type {
    pub fn to_raw(&self) -> RawType {
        match self {
            Type::None => RawType::None,
            Type::Int8(_) => RawType::Int8,
            Type::Int16(_) => RawType::Int16,
            Type::Int32(_) => RawType::Int32,
            Type::Int(_) => RawType::Int,
            Type::Int64(_) => RawType::Int64,
            Type::Int128(_) => RawType::Int128,
            Type::UInt8(_) => RawType::UInt8,
            Type::UInt16(_) => RawType::UInt16,
            Type::UInt32(_) => RawType::UInt32,
            Type::UInt(_) => RawType::UInt,
            Type::UInt64(_) => RawType::UInt64,
            Type::UInt128(_) => RawType::UInt128,
            Type::Float32(_) => RawType::Float32,
            Type::Float64(_) => RawType::Float64,
            Type::Char(_) => RawType::Char,
            Type::String(_) => RawType::String,
            Type::Boolean(_) => RawType::Boolean,
            Type::Address(_) => RawType::Address,
            Type::Pointer(_) => RawType::Pointer,
            Type::Register(_) => RawType::Register,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum RawType {
    None,

    // Numeric
    Int8,
    Int16,
    Int32,
    Int,
    Int64,
    Int128,
    UInt8,
    UInt16,
    UInt32,
    UInt,
    UInt64,
    UInt128,

    Float32,
    Float64,

    // Text
    Char,
    String,

    // Other
    Boolean,
    Address,
    Pointer,
    Register,
}
