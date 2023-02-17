use std::thread::JoinHandle;

#[doc(hidden)]
pub use allot_lib::*;
use allot_lib::{Instruction, OpPrim2, Register, Type};

use crate::{library, memory::StackFrame, operations, AllotRuntime};

impl AllotRuntime {
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
}
