#![allow(dead_code)] // TODO: Remove?

use std::sync::{Arc, RwLock};

use allot_lib::{Instruction, Register, Type};

use crate::memory::{CrossHeap, Heap, HeapBox, Registers, StackFrame};

mod library;
mod memory;
mod operations;

// TODO: This should reexport allot_lib stuff.

pub struct AllotRuntime {
    current: usize,
    instructions: Arc<Vec<Instruction>>,
    registers: Registers,
    stack_frames: Vec<StackFrame>,
    heap: CrossHeap,
    is_thread: bool,
}
impl AllotRuntime {
    pub fn new(instructions: Vec<Instruction>) -> Self {
        Self {
            instructions: Arc::new(instructions),
            registers: Registers::new(),
            stack_frames: vec![StackFrame::default()],
            heap: Arc::new(RwLock::new(Heap::default())),
            current: 0,
            is_thread: false,
        }
    }

    pub fn new_arc(instructions: Arc<Vec<Instruction>>) -> Self {
        Self {
            instructions,
            registers: Registers::new(),
            stack_frames: vec![StackFrame::default()],
            heap: Arc::new(RwLock::new(Heap::default())),
            current: 0,
            is_thread: false,
        }
    }

    pub fn new_thread(
        instructions: Arc<Vec<Instruction>>,
        stack_frame: StackFrame,
        heap: CrossHeap,
        current: usize,
    ) -> Self {
        Self {
            instructions,
            registers: Registers::new(),
            stack_frames: vec![stack_frame],
            heap,
            current,
            is_thread: true,
        }
    }

    pub fn tick(&mut self) -> Option<i32> {
        let instruction = match self.instructions.get(self.current) {
            None => panic!("There is no instruction at {}", self.current),
            Some(i) => i,
        };
        let mut next = self.current + 1;

        match instruction {
            Instruction::Nop => {}
            Instruction::Op(op, regs) => operations::solve(op, &mut self.registers, regs),
            Instruction::Mov(reg, t) => {
                let val = match t {
                    Type::Register(reg) => self.registers.take(*reg),
                    _ => t.clone(),
                };

                self.registers.insert(*reg, val);
            }
            Instruction::Cpy(reg1, reg2) => {
                let val = self.registers.get(*reg2);
                self.registers.insert(*reg1, val.clone())
            }
            Instruction::Cast(reg, raw) => {
                let val = self.registers.get(*reg);
                let casted = operations::cast(val, *raw);
                self.registers.insert(*reg, casted);
            }
            Instruction::Lea(reg, address) => self.registers.insert(*reg, Type::Address(*address)),
            Instruction::Jmp(opt_reg, t) => {
                let address = AllotRuntime::get_address(t, &mut self.registers);

                let jmp = match opt_reg {
                    None => true,
                    Some(reg) => {
                        let val = self.registers.get_mut(*reg);
                        if let Type::Boolean(i) = val {
                            *i
                        }
                        else {
                            panic!("Jmp requires a Boolean Type.");
                        }
                    }
                };

                if jmp {
                    next = address;
                }
            }
            Instruction::Ret => {
                let val = match self.stack_frames.last_mut() {
                    None => panic!("No stack frames."),
                    Some(frame) => frame.pop(),
                };

                match val {
                    Type::Address(address) => next = address,
                    _ => panic!("Ret popped an non-address type from the stack."),
                }
            }
            Instruction::Call(function) => {
                let r9 = self.registers.clone(Register::R9);
                let stack_frame = self
                    .stack_frames
                    .last_mut()
                    .expect("There was no stack frame to take.");

                let ret = library::call(function.as_str(), r9, stack_frame, &self.heap);
                self.registers.insert(Register::R10, ret);
            }
            Instruction::Exit(t) => {
                let code = AllotRuntime::get_int32(t, &mut self.registers);
                return Some(code);
            }
            Instruction::Push(reg) => {
                let val = self.registers.take(*reg);
                match self.stack_frames.last_mut() {
                    None => panic!("No stack frames."),
                    Some(frame) => {
                        frame.push(val);
                    }
                }
            }
            Instruction::PushCpy(reg) => {
                let val = self.registers.get(*reg);
                match self.stack_frames.last_mut() {
                    None => panic!("No stack frames."),
                    Some(frame) => {
                        frame.push(val.clone());
                    }
                }
            }
            Instruction::Pop(opt_reg) => {
                let val = match self.stack_frames.last_mut() {
                    None => panic!("No stack frames."),
                    Some(frame) => frame.pop(),
                };

                match opt_reg {
                    None => {}
                    Some(reg) => self.registers.insert(*reg, val),
                }
            }
            Instruction::PopMany(t) => {
                let amount = AllotRuntime::get_uint(t, &mut self.registers);

                match self.stack_frames.last_mut() {
                    None => panic!("No stack frames."),
                    Some(frame) => {
                        for _ in 0..amount {
                            frame.pop();
                        }
                    }
                }
            }
            Instruction::StackCpy(reg, t) => {
                let amount = AllotRuntime::get_uint(t, &mut self.registers);

                match self.stack_frames.last_mut() {
                    None => panic!("No stack frames."),
                    Some(frame) => {
                        let t = frame.clone_offset(amount);
                        self.registers.insert(*reg, t);
                    }
                }
            }
            Instruction::PushFrame(b) => self.stack_frames.push(StackFrame::new(*b)),
            Instruction::PopFrame => {
                let val = self.stack_frames.pop();
                if val.is_none() || self.stack_frames.is_empty() {
                    panic!("Could not pop stack frame.");
                }
            }
            Instruction::PushOnto(_) => panic!("Not impl yet!"),
            Instruction::PopInto => panic!("Not impl yet!"),
            Instruction::ThreadCreate(t) => {
                let address = AllotRuntime::get_address(t, &mut self.registers);
                let sf = self.stack_frames.pop();
                if sf.is_none() || self.stack_frames.is_empty() {
                    panic!("Could not pop stack frame for thread.");
                }
                let sf = sf.unwrap();
                let instructions = self.instructions.clone();
                let heap = self.heap.clone();

                let handle = std::thread::spawn(move || {
                    let mut runtime = AllotRuntime::new_thread(instructions, sf, heap, address);
                    (runtime.run(), runtime.take_stack_frame())
                });

                let i = {
                    let mut heap = self.heap.write().unwrap();
                    heap.push(HeapBox::ThreadHandle(Box::new(handle)))
                };
                self.registers.insert(Register::R0, i);
            }
            Instruction::ThreadJoin(reg) => {
                let pointer = match self.registers.get(*reg) {
                    Type::Pointer(p) => p,
                    _ => panic!("ThreadJoin did not receive a pointer type."),
                };

                let handle = {
                    let mut heap = self.heap.write().unwrap();
                    heap.take(*pointer)
                };

                match handle {
                    HeapBox::ThreadHandle(handle) => {
                        let ret = handle.join().expect("Fatal error on thread join.");
                        self.registers.insert(Register::R0, Type::Int32(ret.0));
                        self.stack_frames.push(ret.1);
                    }
                    _ => panic!("ThreadJoin tried to get a ThreadHandle, but the pointer did not lead to one.")
                }
            }
            Instruction::Assert(reg, t) => {
                use allot_lib::OpPrim2;

                // TODO: This is a kinda icky way to do this.
                let val = self.registers.clone(*reg);
                let result = operations::solve_2(&OpPrim2::Equal, val, t.clone());
                if let Type::Boolean(b) = result {
                    // println!("Assert: {:?} is {:?}: {}", reg, t, &b); TODO: Should assert have an output?
                    if !b {
                        return Some(-1);
                    }
                }
                else {
                    return Some(-1);
                }
            }

            // Only available in debug builds.
            #[cfg(debug_assertions)]
            Instruction::Dbg(reg) => {
                println!("Register {:?}", &reg);
                let val = self.registers.get(*reg);
                dbg!(val);
            }
            #[cfg(debug_assertions)]
            Instruction::Dump(opts) => {
                if opts & 0b00000001 != 0 {
                    dbg!(&self.instructions);
                }
                if opts & 0b00000010 != 0 {
                    dbg!(&self.registers);
                }
                if opts & 0b00000100 != 0 {
                    dbg!(&self.stack_frames);
                }
                if opts & 0b00001000 != 0 {
                    dbg!(&self.heap);
                }
            }
        }

        self.current = next;
        None
    }

    pub fn run(&mut self) -> i32 {
        let mut code = self.tick();
        while code.is_none() {
            code = self.tick();
        }
        code.unwrap()
    }

    pub fn take_stack_frame(&mut self) -> StackFrame {
        self.stack_frames.pop().expect("No stack frames to take.")
    }
}
impl AllotRuntime {
    #[inline]
    fn get_uint(t: &Type, registers: &mut Registers) -> usize {
        match t {
            Type::UInt(i) => *i,
            Type::Register(reg) => match registers.get(*reg) {
                Type::UInt(i) => *i,
                _ => panic!("Register did not hold a UInt type."),
            },
            _ => panic!("Type was not a UInt or Register."),
        }
    }

    #[inline]
    fn get_int32(t: &Type, registers: &mut Registers) -> i32 {
        match t {
            Type::Int32(i) => *i,
            Type::Register(reg) => match registers.get(*reg) {
                Type::Int32(i) => *i,
                _ => panic!("Register did not hold a Int32 type."),
            },
            _ => panic!("Type was not a Int32 or Register."),
        }
    }

    #[inline]
    fn get_address(t: &Type, registers: &mut Registers) -> usize {
        match t {
            Type::Address(i) => *i,
            Type::Register(reg) => match registers.get(*reg) {
                Type::Address(i) => *i,
                _ => panic!("Register did not hold a Label type."),
            },
            _ => panic!("Type was not a Label or Register."),
        }
    }

    #[inline]
    fn get_pointer(t: &Type, registers: &mut Registers) -> usize {
        match t {
            Type::Pointer(i) => *i,
            Type::Register(reg) => match registers.get(*reg) {
                Type::Pointer(i) => *i,
                _ => panic!("Register did not hold a Label type."),
            },
            _ => panic!("Type was not a Label or Register."),
        }
    }
}
