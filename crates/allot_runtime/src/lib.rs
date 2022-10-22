#![allow(dead_code)] // TODO: Remove?

use std::sync::{Arc, RwLock};

#[cfg(feature = "bytecode")]
pub use bytecode::from_bytecode;
#[cfg(feature = "bytecode_gen")]
pub use bytecode::to_bytecode;
use structures::*;

#[cfg(feature = "bytecode")]
mod bytecode;
mod library;
pub mod structures;
mod traits;

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
        let mut next = self.current + 1;

        match instruction {
            Instruction::Nop => {}
            Instruction::Op(op, regs) => {
                op.solve(&mut self.registers, regs);
            }
            Instruction::Mov(reg, t) => {
                let val = match t {
                    Type::Pointer(_) | Type::Label(_) | Type::Address(_) | Type::Thread(_) => {
                        panic!("Attempted to move impossible type into a register.")
                    }
                    Type::Register(reg) => self.registers.take(*reg),
                    _ => t.clone(),
                };

                self.registers.insert(*reg, val);
            }
            Instruction::Cpy(reg1, reg2) => {
                let val = self.registers.get(*reg2);
                self.registers.insert(*reg1, val.clone())
            }
            Instruction::Cast(_, _) => {}
            Instruction::Lea(reg, label) => {
                let val = match self.labels.get(*label) {
                    None => panic!("Label {label} was not found."),
                    Some(l) => *l,
                };
                self.registers.insert(*reg, Type::Label(val));
            }
            Instruction::Jmp(opt_reg, label) => {
                let label = match label {
                    Type::Label(l) => *l,
                    Type::Register(reg) => {
                        let val = self.registers.get_mut(*reg);
                        if let Type::Label(i) = val {
                            *i
                        }
                        else {
                            panic!("Jmp requires a Label Type.");
                        }
                    }
                    _ => panic!("Jmp requires a Label Type."),
                };

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
                    next = label;
                }
            }
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
            Instruction::PopMany(_) => {}
            Instruction::StackCpy(_, _) => {}
            Instruction::PushFrame(b) => self.stack_frames.push(StackFrame::new(*b)),
            Instruction::PopFrame => {
                let val = self.stack_frames.pop();
                if val.is_none() || self.stack_frames.is_empty() {
                    panic!("Could not pop stack frame.");
                }
            }
            Instruction::PushOnto(_) => {}
            Instruction::PopInto => {}
            Instruction::ThreadStart(_) => {}
            Instruction::ThreadJoin(_) => {}
            Instruction::Assert(reg, t) => {
                // TODO: This is a kinda icky way to do this.
                let val = self.registers.clone(*reg);
                let result = OpPrim2::Equal.solve(val, t.clone());
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
                let val = self.registers.get_mut(*reg);
                dbg!(val);
            }
            #[cfg(debug_assertions)]
            Instruction::Dump => {}
        }

        // TODO: Instructions

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
}
