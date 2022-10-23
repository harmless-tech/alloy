use crate::{Operation, RawType, Register, Type};

#[derive(Debug)]
pub enum Instruction {
    /// Does nothing.
    Nop,

    /// Does an operation on register(s).
    Op(Operation, [Register; 2]),

    /// Moves the value in the second register to the first register.
    Mov(Register, Type),
    /// Copies the value in the second register and puts it in the first register.
    /// May fail if the type cannot be copied.
    Cpy(Register, Register),

    /// Attempts the cast the value in the register to another type.
    Cast(Register, RawType),

    /// Loads an address value into the register. (Could be simulated by using Mov)
    Lea(Register, usize),
    /// Jumps to a label, depending on the value in the register.
    Jmp(Option<Register>, Type), // Type = Label || Register
    /// Pops the stack and jumps to that label.
    Ret,

    /// Calls a function, functions get access to the current stack frame, read-only access to register 9, write-only access to register 10, and access to the heap.
    Call(String),

    /// Exits the program with the int. (Does nothing on threads currently)
    Exit(Type), // Type = Int32 | Register

    /// Pushes the value in the register onto the stack in the current stack frame.
    Push(Register),
    /// Pushes a copy of the value in the register onto the stack in the current stack frame.
    PushCpy(Register),
    /// Pops the value on top of the stack into register or gets rid of it.
    Pop(Option<Register>),
    /// Pops many values off of the stack, cannot store any of them.
    PopMany(Type), // Type = None | UInt | Register
    /// Copies an item at the stack offset into a register.
    StackCpy(Register, Type), // Type = None | UInt | Register
    /// Pushes a new stack frame.
    PushFrame(bool),
    /// Pops the top stack frame. Errors if it is the root stack frame.
    PopFrame,
    /// Pushes onto the last stack frame. If no register is listed, then it pops from the current stack frame.
    /// May fail if the current stack frame is isolated.
    PushOnto(Option<Register>),
    /// Pops from the last stack frame and pushes it to the current one.
    /// May fail if the last stack frame is isolated.
    PopInto,

    /// Takes the current stack frame (Errors if it is the root stack frame) and runs it on a new thread starting at the label.
    /// Threads have their own registers and stack frames, the heap is shared between all threads.
    /// Puts its handle into register 0.
    /// To stop the thread use Instruction::Exit.
    ThreadCreate(Type), // Type = Address || Register
    /// Joins a thread and pushes its stack frame. Accepts a pointer to its join handle.
    /// Puts the i32 return value into register 0, pushes the StackFrame from the thread.
    ThreadJoin(Register),
    /// Asserts that a register is equal to a type. Should only be used in debug builds of your allot program.
    Assert(Register, Type),

    #[cfg(debug_assertions)]
    /// Prints a register. (Debug builds only)
    Dbg(Register),
    #[cfg(debug_assertions)]
    /// Prints all registers, stack frames, and the heap. (Debug builds only)
    Dump(u8),
}
impl Instruction {
    pub fn to_raw(&self) -> RawInstruction {
        match self {
            Instruction::Nop => RawInstruction::Nop,
            Instruction::Op(_, _) => RawInstruction::Op,
            Instruction::Mov(_, _) => RawInstruction::Mov,
            Instruction::Cpy(_, _) => RawInstruction::Cpy,
            Instruction::Cast(_, _) => RawInstruction::Cast,
            Instruction::Lea(_, _) => RawInstruction::Lea,
            Instruction::Jmp(_, _) => RawInstruction::Jmp,
            Instruction::Ret => RawInstruction::Ret,
            Instruction::Call(_) => RawInstruction::Call,
            Instruction::Exit(_) => RawInstruction::Exit,
            Instruction::Push(_) => RawInstruction::Push,
            Instruction::PushCpy(_) => RawInstruction::PushCpy,
            Instruction::Pop(_) => RawInstruction::Pop,
            Instruction::PopMany(_) => RawInstruction::PopMany,
            Instruction::StackCpy(_, _) => RawInstruction::StackCpy,
            Instruction::PushFrame(_) => RawInstruction::PushFrame,
            Instruction::PopFrame => RawInstruction::PopFrame,
            Instruction::PushOnto(_) => RawInstruction::PushOnto,
            Instruction::PopInto => RawInstruction::PopInto,
            Instruction::ThreadCreate(_) => RawInstruction::ThreadCreate,
            Instruction::ThreadJoin(_) => RawInstruction::ThreadJoin,
            Instruction::Assert(_, _) => RawInstruction::Assert,
            #[cfg(debug_assertions)]
            Instruction::Dbg(_) => RawInstruction::Dbg,
            #[cfg(debug_assertions)]
            Instruction::Dump(_) => RawInstruction::Dump,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RawInstruction {
    Nop,
    Op,
    Mov,
    Cpy,
    Cast,
    Lea,
    Jmp,
    Ret,
    Call,
    Exit,
    Push,
    PushCpy,
    Pop,
    PopMany,
    StackCpy,
    PushFrame,
    PopFrame,
    PushOnto,
    PopInto,
    ThreadCreate,
    ThreadJoin,
    Assert,
    #[cfg(debug_assertions)]
    Dbg,
    #[cfg(debug_assertions)]
    Dump,
}
