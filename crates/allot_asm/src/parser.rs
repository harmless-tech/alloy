use allot_lib::{Instruction, RawInstruction, RawType, Register, Type};

use crate::lexer::Token;

pub fn parse(tokens: Vec<Token>) -> Vec<Instruction> {
    let mut p = Parser::new(tokens);
    p.parse();

    p.instructions
}

struct Parser {
    tokens: Vec<Token>,
    instructions: Vec<Instruction>,
}
impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            instructions: Vec::new(),
        }
    }

    fn parse(&mut self) {
        while !self.tokens.is_empty() {
            let t = self.tokens.pop().expect("No token?");

            match t {
                Token::Instruction(i) => self.parse_instruction(i),
                _ => panic!("Parse Error: Expected instruction."),
            }
        }
    }

    fn parse_instruction(&mut self, i: RawInstruction) {
        match i {
            RawInstruction::Nop => self.instructions.push(Instruction::Nop),
            RawInstruction::Op => match (self.tokens.pop(), self.tokens.pop(), self.tokens.pop()) {
                (Some(t1), Some(t2), Some(t3)) => match (t1, t2, t3) {
                    (Token::Operation(op), Token::Register(r1), Token::Register(r2)) => {
                        self.instructions.push(Instruction::Op(
                            op,
                            [Register::cast(r1), Register::cast(r2)],
                        ))
                    }
                    _ => panic!("Parse Error: Unexpected token."),
                },
                _ => panic!("No token?"),
            },
            RawInstruction::Mov => match (self.tokens.pop(), self.tokens.pop()) {
                (Some(t1), Some(t2)) => match (t1, t2) {
                    (Token::Register(r), Token::Type(t)) => {
                        self.instructions.push(Instruction::Mov(
                            Register::cast(r),
                            Parser::parse_type(t, self.tokens.pop().unwrap()),
                        ))
                    }
                    _ => panic!("Parse Error: Unexpected token."),
                },
                _ => panic!("No token?"),
            },
            RawInstruction::Cpy => match (self.tokens.pop(), self.tokens.pop()) {
                (Some(t1), Some(t2)) => match (t1, t2) {
                    (Token::Register(r1), Token::Register(r2)) => self
                        .instructions
                        .push(Instruction::Cpy(Register::cast(r1), Register::cast(r2))),
                    _ => panic!("Parse Error: Unexpected token."),
                },
                _ => panic!("No token?"),
            },
            RawInstruction::Cast => match (self.tokens.pop(), self.tokens.pop()) {
                (Some(t1), Some(t2)) => match (t1, t2) {
                    (Token::Register(r), Token::Type(t)) => self
                        .instructions
                        .push(Instruction::Cast(Register::cast(r), t)),
                    _ => panic!("Parse Error: Unexpected token."),
                },
                _ => panic!("No token?"),
            },
            RawInstruction::Lea => match (self.tokens.pop(), self.tokens.pop()) {
                (Some(t1), Some(t2)) => match (t1, t2) {
                    (Token::Register(r), Token::Data(d)) => self.instructions.push(
                        Instruction::Lea(Register::cast(r), d.parse::<usize>().unwrap()),
                    ),
                    _ => panic!("Parse Error: Expected data."),
                },
                _ => panic!("No token?"),
            },
            RawInstruction::Jmp => match (self.tokens.pop(), self.tokens.pop()) {
                (Some(t1), Some(t2)) => match (t1, t2) {
                    (Token::Register(r), Token::Type(t)) => {
                        let r = Register::cast(r);
                        let r = match r {
                            Register::None => None,
                            _ => Some(r),
                        };
                        self.instructions.push(Instruction::Jmp(
                            r,
                            Parser::parse_type(t, self.tokens.pop().unwrap()),
                        ))
                    }
                    _ => panic!("Parse Error: Unexpected token."),
                },
                _ => panic!("No token?"),
            },
            RawInstruction::Ret => self.instructions.push(Instruction::Ret),
            RawInstruction::Call => match self.tokens.pop() {
                None => panic!("No token?"),
                Some(t) => match t {
                    Token::Data(d) => self.instructions.push(Instruction::Call(d)),
                    _ => panic!("Parse Error: Expected data."),
                },
            },
            RawInstruction::Exit => match self.tokens.pop() {
                None => panic!("No token?"),
                Some(t) => match t {
                    Token::Type(t) => self.instructions.push(Instruction::Exit(
                        Parser::parse_type(t, self.tokens.pop().unwrap()),
                    )),
                    _ => panic!("Parse Error: Expected type."),
                },
            },
            RawInstruction::Push => match self.tokens.pop() {
                None => panic!("No token?"),
                Some(t) => match t {
                    Token::Register(r) => {
                        self.instructions.push(Instruction::Push(Register::cast(r)))
                    }
                    _ => panic!("Parse Error: Expected register."),
                },
            },
            RawInstruction::PushCpy => match self.tokens.pop() {
                None => panic!("No token?"),
                Some(t) => match t {
                    Token::Register(r) => self
                        .instructions
                        .push(Instruction::PushCpy(Register::cast(r))),
                    _ => panic!("Parse Error: Expected register."),
                },
            },
            RawInstruction::Pop => match self.tokens.pop() {
                None => panic!("No token?"),
                Some(t) => match t {
                    Token::Register(r) => {
                        let r = Register::cast(r);
                        let r = match r {
                            Register::None => None,
                            _ => Some(r),
                        };
                        self.instructions.push(Instruction::Pop(r))
                    }
                    _ => panic!("Parse Error: Expected register."),
                },
            },
            RawInstruction::PopMany => match self.tokens.pop() {
                None => panic!("No token?"),
                Some(t) => match t {
                    Token::Type(t) => {
                        self.instructions
                            .push(Instruction::PopMany(Parser::parse_type(
                                t,
                                self.tokens.pop().unwrap(),
                            )))
                    }
                    _ => panic!("Parse Error: Expected type."),
                },
            },
            RawInstruction::StackCpy => match (self.tokens.pop(), self.tokens.pop()) {
                (Some(t1), Some(t2)) => match (t1, t2) {
                    (Token::Register(r), Token::Type(t)) => {
                        self.instructions.push(Instruction::StackCpy(
                            Register::cast(r),
                            Parser::parse_type(t, self.tokens.pop().unwrap()),
                        ))
                    }
                    _ => panic!("Parse Error: Unexpected token."),
                },
                _ => panic!("No token?"),
            },
            RawInstruction::PushFrame => match self.tokens.pop() {
                None => panic!("No token?"),
                Some(t) => match t {
                    Token::Data(d) => self
                        .instructions
                        .push(Instruction::PushFrame(d.parse::<bool>().unwrap())),
                    _ => panic!("Parse Error: Expected data."),
                },
            },
            RawInstruction::PopFrame => self.instructions.push(Instruction::PopFrame),
            RawInstruction::TakeFrom => self.instructions.push(Instruction::TakeFrom),
            RawInstruction::GiveTo => self.instructions.push(Instruction::GiveTo),
            RawInstruction::ThreadCreate => {
                match self.tokens.pop() {
                    None => panic!("No token?"),
                    Some(t) => {
                        match t {
                            Token::Type(t) => self.instructions.push(Instruction::ThreadCreate(
                                Parser::parse_type(t, self.tokens.pop().unwrap()),
                            )),
                            _ => panic!("Parse Error: Expected type."),
                        }
                    }
                }
            }
            RawInstruction::ThreadJoin => match self.tokens.pop() {
                None => panic!("No token?"),
                Some(t) => match t {
                    Token::Register(r) => self
                        .instructions
                        .push(Instruction::ThreadJoin(Register::cast(r))),
                    _ => panic!("Parse Error: Expected register."),
                },
            },
            RawInstruction::Assert => match (self.tokens.pop(), self.tokens.pop()) {
                (Some(t1), Some(t2)) => match (t1, t2) {
                    (Token::Register(r), Token::Type(t)) => {
                        self.instructions.push(Instruction::Assert(
                            Register::cast(r),
                            Parser::parse_type(t, self.tokens.pop().unwrap()),
                        ))
                    }
                    _ => panic!("Parse Error: Unexpected token."),
                },
                _ => panic!("No token?"),
            },
            #[cfg(debug_assertions)]
            _ => panic!("Unsupported instruction."),
        }
    }

    fn parse_type(t: RawType, next: Token) -> Type {
        match (t, next) {
            (RawType::None, Token::Data(_)) => Type::None,
            (RawType::Int8, Token::Data(d)) => Type::Int8(d.parse::<i8>().unwrap()),
            (RawType::Int16, Token::Data(d)) => Type::Int16(d.parse::<i16>().unwrap()),
            (RawType::Int32, Token::Data(d)) => Type::Int32(d.parse::<i32>().unwrap()),
            (RawType::Int, Token::Data(d)) => Type::Int(d.parse::<isize>().unwrap()),
            (RawType::Int64, Token::Data(d)) => Type::Int64(d.parse::<i64>().unwrap()),
            (RawType::Int128, Token::Data(d)) => Type::Int128(d.parse::<i128>().unwrap()),
            (RawType::UInt8, Token::Data(d)) => Type::UInt8(d.parse::<u8>().unwrap()),
            (RawType::UInt16, Token::Data(d)) => Type::UInt16(d.parse::<u16>().unwrap()),
            (RawType::UInt32, Token::Data(d)) => Type::UInt32(d.parse::<u32>().unwrap()),
            (RawType::UInt, Token::Data(d)) => Type::UInt(d.parse::<usize>().unwrap()),
            (RawType::UInt64, Token::Data(d)) => Type::UInt64(d.parse::<u64>().unwrap()),
            (RawType::UInt128, Token::Data(d)) => Type::UInt128(d.parse::<u128>().unwrap()),
            (RawType::Float32, Token::Data(d)) => Type::Float32(d.parse::<f32>().unwrap()),
            (RawType::Float64, Token::Data(d)) => Type::Float64(d.parse::<f64>().unwrap()),
            (RawType::Char, Token::Data(d)) => Type::Char(d.parse::<char>().unwrap()),
            (RawType::String, Token::Data(d)) => Type::String(d),
            (RawType::Boolean, Token::Data(d)) => Type::Boolean(d.parse::<bool>().unwrap()),
            (RawType::Address, Token::Data(d)) => Type::Address(d.parse::<usize>().unwrap()),
            (RawType::Register, Token::Data(d)) => {
                Type::Register(Register::cast(d.parse::<u8>().unwrap()))
            }
            _ => panic!("Parser Error: Unsupported type or next token was not a data token."),
        }
    }
}
