pub use memory::*;
pub use operations::*;
pub use types::*;

mod memory;
mod operations;
mod types;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[repr(u8)]
pub enum Register {
    R0 = 0,
    R1 = 1,
    R2 = 2,
    R3 = 3,
    R4 = 4,
    R5 = 5,
    R6 = 6,
    R7 = 7,
    R8 = 8,
    R9 = 9,
    R10 = 10,
    R11 = 11,
    R12 = 12,
    R13 = 13,
    R14 = 14,
    R15 = 15,
    R16 = 16,
    R17 = 17,
    R18 = 18,
    R19 = 19,
    R20 = 20,
    R21 = 21,
    R22 = 22,
    R23 = 23,
    R24 = 24,
    R25 = 25,
    R26 = 26,
    R27 = 27,
    R28 = 28,
    R29 = 29,

    None = 255,
}

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
    Cast(RawType, Register),

    /// Loads a label value into the register.
    Lea(Register, usize),
    /// Jumps to a label, depending on the value in the register.
    Jmp(Option<Register>, Type), // Type = Label || Register
    /// Pops the stack and jumps to that label.
    Ret,

    /// Calls a function, functions get the current stack frame, read-only access to the number of args in register 9, write-only access to register 0, and access to the heap.
    Call(String),

    /// Exits the program with the int.
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
    ThreadStart(Type), // Type = Label || Register
    /// Joins a thread and pushes its stack frame.
    ThreadJoin(Register),

    /// Asserts that a register is equal to a type. Should only be used in debug builds of your allot program.
    Assert(Register, Type),

    #[cfg(debug_assertions)]
    /// Prints a register. (Debug builds only)
    Dbg(Register),

    #[cfg(debug_assertions)]
    /// Prints all registers, stack frames, and the heap. (Debug builds only)
    Dump,
}

// TODO: RawInstruction enum like RawType?
