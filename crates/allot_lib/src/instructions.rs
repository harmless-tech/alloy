use allot_codegen::RawEnum;
use num_enum::{IntoPrimitive, TryFromPrimitive};

use crate::{Operation, RawType, Register, Type};

#[derive(Clone, Debug, PartialEq, PartialOrd, RawEnum)]
pub enum Instruction {
    /// Does nothing.
    Nop,

    /// Does an operation on register(s).
    Op(Operation, [Register; 2]),
    /// Checks if a register is a type.
    // IsType(Register, RawType),

    /// Moves the value in the second register to the first register.
    Mov(Register, Type),
    /// Copies the value in the second register and puts it in the first
    /// register. Cpy may be faster than move when the amount of data is
    /// small.
    Cpy(Register, Register),

    /// Attempts the cast the value in the register to another type.
    Cast(Register, RawType),

    /// Loads an address value into the register. (Could be simulated by using
    /// Mov)
    Lea(Register, usize),
    /// Jumps to a label, depending on the value in the register.
    Jmp(Option<Register>, Type), // Type = Address || Register
    /// Pops the stack and jumps to that label.
    Ret,

    /// Calls a function, functions get access to registers 5-9, the current
    /// stack frame, and access to the heap.
    Call(String),

    /// Exits the program with the int. (Does nothing on threads currently)
    Exit(Type), // Type = Int32 | Register

    /// Pushes the value in the register onto the stack in the current stack
    /// frame.
    Push(Register),
    /// Pushes a copy of the value in the register onto the stack in the current
    /// stack frame.
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
    /// Pops from the last stack frame and pushes it to the current one.
    /// May fail if the last stack frame is isolated.
    TakeFrom,
    /// Pops from the current stack frame and pushes onto the last stack frame.
    /// May fail if the current stack frame is isolated.
    GiveTo,

    /// Takes the current stack frame (Errors if it is the root stack frame) and
    /// runs it on a new thread starting at the label. Threads have their
    /// own registers and stack frames, the heap is shared between all threads.
    /// Puts its handle into register 5.
    /// To stop the thread use Instruction::Exit.
    ThreadCreate(Type), // Type = Address || Register
    /// Joins a thread and pushes its stack frame. Accepts a pointer to its join
    /// handle. Puts the i32 return value into register 5, pushes the
    /// StackFrame from the thread.
    ThreadJoin(Register),
    /// Asserts that a register is equal to a type. Should only be used in debug
    /// builds of your allot program.
    Assert(Register, Type),

    /// Prints a register. (Debug builds only)
    Dbg(Register),
    /// Prints all registers, stack frames, and the heap. (Debug builds only)
    Dump(u8),
}
