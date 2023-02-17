use std::{sync::Arc, thread::JoinHandle};

#[doc(hidden)]
pub use allot_lib::*;
use allot_lib::{Instruction, Register, Type};

use crate::memory::{CrossHeap, Heap, Registers, StackFrame};

mod library;
mod memory;
mod operations;

pub struct AllotRuntime {
    pub current: usize,
    pub instructions: Arc<Vec<Instruction>>,
    pub registers: Registers,
    pub stack_frames: Vec<StackFrame>,
    pub heap: CrossHeap,
}
impl AllotRuntime {
    pub fn new(instructions: Vec<Instruction>) -> Self {
        Self {
            instructions: Arc::new(instructions),
            registers: Registers::new(),
            stack_frames: vec![StackFrame::default()],
            heap: Heap::cross_new(),
            current: 0,
        }
    }

    pub fn new_arc(instructions: Arc<Vec<Instruction>>) -> Self {
        Self {
            instructions,
            registers: Registers::new(),
            stack_frames: vec![StackFrame::default()],
            heap: Heap::cross_new(),
            current: 0,
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
                let stack_frame = self
                    .stack_frames
                    .last_mut()
                    .expect("There was no stack frame to take.");

                let ret = library::call(
                    function.as_str(),
                    (
                        self.registers.get(Register::R5),
                        self.registers.get(Register::R6),
                        self.registers.get(Register::R7),
                        self.registers.get(Register::R8),
                        self.registers.get(Register::R9),
                    ),
                    stack_frame,
                    &mut self.heap,
                );

                if let Some(t) = ret.0 {
                    self.registers.insert(Register::R5, t)
                }
                if let Some(t) = ret.1 {
                    self.registers.insert(Register::R6, t)
                }
                if let Some(t) = ret.2 {
                    self.registers.insert(Register::R7, t)
                }
                if let Some(t) = ret.3 {
                    self.registers.insert(Register::R8, t)
                }
                if let Some(t) = ret.4 {
                    self.registers.insert(Register::R9, t)
                }
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
            Instruction::TakeFrom => panic!("Not impl yet!"),
            Instruction::GiveTo => panic!("Not impl yet!"),
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
                    let mut heap = self.heap.lock().unwrap();
                    heap.push(handle)
                };
                self.registers.insert(Register::R5, i);
            }
            Instruction::ThreadJoin(reg) => {
                let pointer = match self.registers.get(*reg) {
                    Type::Pointer(p) => p,
                    _ => panic!("ThreadJoin did not receive a pointer type."),
                };

                let handle = {
                    let mut heap = self.heap.lock().unwrap();
                    heap.take::<JoinHandle<(i32, StackFrame)>>(*pointer)
                };

                let ret = handle.join().expect("Fatal error on thread join.");
                self.registers.insert(Register::R5, Type::Int32(ret.0));
                self.stack_frames.push(ret.1);
            }
            Instruction::Assert(reg, t) => {
                // TODO: This is a kinda icky way to do this. Maybe check type first, then do
                // Equal?
                let val = self.registers.clone(*reg);
                let result = operations::solve_2(&OpPrim2::Equal, val, t.clone());
                if let Type::Boolean(b) = result {
                    if !b {
                        return Some(-1);
                    }
                }
                else {
                    return Some(-1);
                }
            }

            // Only runs in debug builds.
            #[allow(unused_variables)]
            Instruction::Dbg(reg) => {
                #[cfg(debug_assertions)]
                {
                    println!("Register {:?}", &reg);
                    let val = self.registers.get(*reg);
                    dbg!(val);
                }
            }
            #[allow(unused_variables)]
            Instruction::Dump(opts) => {
                #[cfg(debug_assertions)]
                {
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
}
