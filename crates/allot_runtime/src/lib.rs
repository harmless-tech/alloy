#![allow(dead_code)] // TODO: Remove?

extern crate core;

#[cfg(feature = "bytecode")]
mod bytecode;
mod library;
pub mod structures;
#[cfg(test)]
mod tests;
mod traits;

#[cfg(feature = "bytecode")]
pub use bytecode::from_bytecode;
#[cfg(feature = "bytecode_gen")]
pub use bytecode::to_bytecode;

use std::sync::{Arc, RwLock};
use structures::*;

pub struct AllotRuntime {
    current: usize,
    instructions: Vec<Instruction>,
    labels: Vec<usize>,
    registers: Registers,
    stack_frames: Vec<StackFrame>,
    heap: Arc<RwLock<Heap>>,
    is_thread: bool,
}
impl AllotRuntime {
    pub fn new(instructions: Vec<Instruction>, labels: Vec<usize>) -> Self {
        Self {
            instructions,
            labels,
            registers: Registers::new(),
            stack_frames: vec![StackFrame::default()],
            heap: Arc::new(RwLock::new(Heap::default())),
            current: 0,
            is_thread: false,
        }
    }

    pub fn new_thread(instructions: Vec<Instruction>, labels: Vec<usize>) -> Self {
        let mut s = AllotRuntime::new(instructions, labels);
        s.is_thread = true;

        s
    }

    pub fn tick(&mut self) -> Option<i32> {
        let instruction = match self.instructions.get(self.current) {
            None => panic!("There is no instruction at {}", self.current),
            Some(i) => i,
        };

        match instruction {
            Instruction::Nop => {}
            Instruction::Op(op, regs) => {
                op.resolve(&mut self.registers, regs);
            }
            Instruction::Mov(reg, t) => {
                let val = match t {
                    Type::Pointer(_) | Type::Label(_) | Type::Address(_) | Type::Thread(_) => {
                        panic!("Attempted to move impossible type into a register.")
                    }
                    Type::Register(reg) => self.registers.own(*reg),
                    _ => t.copy(),
                };

                self.registers.insert(*reg, val);
            }
            Instruction::Cpy(reg1, reg2) => {
                let val = self.registers.get(*reg2);
                self.registers.insert(*reg1, val.copy())
            }
            Instruction::Cast(_, _) => {}
            Instruction::Lea(_) => {}
            Instruction::Jmp(_, _) => {}
            Instruction::Ret => {}
            Instruction::Call(_) => {}
            Instruction::Exit(t) => {
                let i = match t {
                    Type::Int32(i) => *i,
                    Type::Register(reg) => {
                        let val = self.registers.get_mut(*reg);
                        if let Type::Int32(i) = val {
                            *i
                        }
                        else {
                            panic!("Exit requires a Int32 Type.");
                        }
                    }
                    _ => panic!("Exit requires a Int32 Type."),
                };
                return Some(i);
            }
            Instruction::Push(_) => {}
            Instruction::Pop(_) => {}
            Instruction::PushFrame(_) => {}
            Instruction::PopFrame => {}
            Instruction::PushOnto(_) => {}
            Instruction::PopInto => {}
            Instruction::ThreadStart(_) => {}
            Instruction::ThreadJoin(_) => {}

            // Only available in debug builds.
            #[cfg(debug_assertions)]
            Instruction::Assert(reg, t) => {
                // TODO: This is a kinda icky way to do this.
                let val = self.registers.own(*reg);
                let result = OpPrim2::Equal.resolve(val, t.copy());
                if let Type::Boolean(b) = result {
                    if !b {
                        return Some(-1);
                    }
                }
                else {
                    return Some(-1);
                }
            }
            #[cfg(debug_assertions)]
            Instruction::Dbg(reg) => {
                println!("Register {:?}", &reg);
                let val = self.registers.get_mut(*reg);
                dbg!(val);
            }
            #[cfg(debug_assertions)]
            Instruction::Dump => {}
        }

        // TODO: Instructions

        self.current += 1;
        None
    }

    pub fn run(&mut self) -> i32 {
        let mut code = self.tick();
        while code.is_none() {
            code = self.tick();
        }
        code.unwrap()
    }

    // #[inline]
    // fn get_register(&mut self, register: Register) -> &mut Type {
    //     match self.registers.get_mut(register as usize) {
    //         None => panic!("{:?} is not a valid register.", register),
    //         Some(i) => i,
    //     }
    // }
}
/// Instructions Impl.
impl AllotRuntime {
    // #[inline]
    // fn i_exit(&mut self, t: &Type) -> i32 {
    //     match t {
    //         Type::Int32(i) => *i,
    //         Type::Register(reg) => {
    //             let val = self.registers.get_mut(reg.clone());
    //             if let Type::Int32(i) = val {
    //                 i.clone()
    //             }
    //             else {
    //                 panic!("Exit requires a Int32 Type.");
    //             }
    //         }
    //         _ => panic!("Exit requires a Int32 Type.")
    //     }
    // }
}
