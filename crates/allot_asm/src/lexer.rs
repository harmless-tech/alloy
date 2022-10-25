use allot_lib::{OpPrim1, OpPrim2, Operation, RawInstruction, RawType};
use lazy_regex::{regex, regex_captures};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Token {
    Instruction(RawInstruction),
    Operation(Operation),
    Type(RawType),
    Register(u8),
    Data(String),
}

pub fn lex(program: &str) -> Vec<Token> {
    let mut token_list = Vec::new();

    let lines = program.split('\n');
    for line in lines {
        let line = line.trim();
        let mut line = String::from(line);
        line.push(' ');
        let line = line.as_str();

        let mut index = 0_usize;
        while index < line.len() {
            let s = &line[index..line.len()];

            // Instruction Matching
            if regex!("^nop\\s").is_match(s) {
                token_list.push(Token::Instruction(RawInstruction::Nop));
                index += 3;
            }
            else if regex!("^op\\s").is_match(s) {
                token_list.push(Token::Instruction(RawInstruction::Op));
                index += 2;
            }
            else if regex!("^mov\\s").is_match(s) {
                token_list.push(Token::Instruction(RawInstruction::Mov));
                index += 3;
            }
            else if regex!("^cpy\\s").is_match(s) {
                token_list.push(Token::Instruction(RawInstruction::Cpy));
                index += 3;
            }
            else if regex!("^cast\\s").is_match(s) {
                token_list.push(Token::Instruction(RawInstruction::Cast));
                index += 4;
            }
            else if regex!("^lea\\s").is_match(s) {
                token_list.push(Token::Instruction(RawInstruction::Lea));
                index += 3;
            }
            else if regex!("^jmp\\s").is_match(s) {
                token_list.push(Token::Instruction(RawInstruction::Jmp));
                index += 3;
            }
            else if regex!("^ret\\s").is_match(s) {
                token_list.push(Token::Instruction(RawInstruction::Ret));
                index += 3;
            }
            else if regex!("^call\\s").is_match(s) {
                token_list.push(Token::Instruction(RawInstruction::Call));
                index += 4;
            }
            else if regex!("^exit\\s").is_match(s) {
                token_list.push(Token::Instruction(RawInstruction::Exit));
                index += 4;
            }
            else if regex!("^push\\s").is_match(s) {
                token_list.push(Token::Instruction(RawInstruction::Push));
                index += 4;
            }
            else if regex!("^pushcpy\\s").is_match(s) {
                token_list.push(Token::Instruction(RawInstruction::PushCpy));
                index += 7;
            }
            else if regex!("^pop\\s").is_match(s) {
                token_list.push(Token::Instruction(RawInstruction::Pop));
                index += 3;
            }
            else if regex!("^popmany\\s").is_match(s) {
                token_list.push(Token::Instruction(RawInstruction::PopMany));
                index += 7;
            }
            else if regex!("^stackcpy\\s").is_match(s) {
                token_list.push(Token::Instruction(RawInstruction::StackCpy));
                index += 8;
            }
            else if regex!("^pushframe\\s").is_match(s) {
                token_list.push(Token::Instruction(RawInstruction::PushFrame));
                index += 9;
            }
            else if regex!("^popframe\\s").is_match(s) {
                token_list.push(Token::Instruction(RawInstruction::PopFrame));
                index += 8;
            }
            else if regex!("^takefrom\\s").is_match(s) {
                token_list.push(Token::Instruction(RawInstruction::TakeFrom));
                index += 8;
            }
            else if regex!("^giveto\\s").is_match(s) {
                token_list.push(Token::Instruction(RawInstruction::GiveTo));
                index += 6;
            }
            else if regex!("^threadcreate\\s").is_match(s) {
                token_list.push(Token::Instruction(RawInstruction::ThreadCreate));
                index += 12;
            }
            else if regex!("^threadjoin\\s").is_match(s) {
                token_list.push(Token::Instruction(RawInstruction::ThreadJoin));
                index += 10;
            }
            else if regex!("^assert\\s").is_match(s) {
                token_list.push(Token::Instruction(RawInstruction::Assert));
                index += 6;
            }
            // Operation Matching
            else if regex!("^\\+\\+\\s").is_match(s) {
                token_list.push(Token::Operation(Operation::Prim1(OpPrim1::Increment)));
                index += 2;
            }
            else if regex!("^--\\s").is_match(s) {
                token_list.push(Token::Operation(Operation::Prim1(OpPrim1::Decrement)));
                index += 2;
            }
            else if regex!("^!\\s").is_match(s) {
                token_list.push(Token::Operation(Operation::Prim1(OpPrim1::Not)));
                index += 1;
            }
            else if regex!("^~\\s").is_match(s) {
                token_list.push(Token::Operation(Operation::Prim1(OpPrim1::BitwiseNot)));
                index += 1;
            }
            else if regex!("^\\+\\s").is_match(s) {
                token_list.push(Token::Operation(Operation::Prim2(OpPrim2::Add)));
                index += 1;
            }
            else if regex!("^-\\s").is_match(s) {
                token_list.push(Token::Operation(Operation::Prim2(OpPrim2::Subtract)));
                index += 1;
            }
            else if regex!("^\\*\\s").is_match(s) {
                token_list.push(Token::Operation(Operation::Prim2(OpPrim2::Multiplication)));
                index += 1;
            }
            else if regex!("^/\\s").is_match(s) {
                token_list.push(Token::Operation(Operation::Prim2(OpPrim2::Division)));
                index += 1;
            }
            else if regex!("^%\\s").is_match(s) {
                token_list.push(Token::Operation(Operation::Prim2(OpPrim2::Modulus)));
                index += 1;
            }
            else if regex!("^&&\\s").is_match(s) {
                token_list.push(Token::Operation(Operation::Prim2(OpPrim2::And)));
                index += 2;
            }
            else if regex!("^\\|\\|\\s").is_match(s) {
                token_list.push(Token::Operation(Operation::Prim2(OpPrim2::Or)));
                index += 2;
            }
            else if regex!("^\\^\\^\\s").is_match(s) {
                token_list.push(Token::Operation(Operation::Prim2(OpPrim2::Xor)));
                index += 2;
            }
            else if regex!("^==\\s").is_match(s) {
                token_list.push(Token::Operation(Operation::Prim2(OpPrim2::Equal)));
                index += 2;
            }
            else if regex!("^!=\\s").is_match(s) {
                token_list.push(Token::Operation(Operation::Prim2(OpPrim2::NotEqual)));
                index += 2;
            }
            else if regex!("^>\\s").is_match(s) {
                token_list.push(Token::Operation(Operation::Prim2(OpPrim2::Greater)));
                index += 1;
            }
            else if regex!("^<\\s").is_match(s) {
                token_list.push(Token::Operation(Operation::Prim2(OpPrim2::Less)));
                index += 1;
            }
            else if regex!("^>=\\s").is_match(s) {
                token_list.push(Token::Operation(Operation::Prim2(OpPrim2::GreaterEqual)));
                index += 2;
            }
            else if regex!("^<=\\s").is_match(s) {
                token_list.push(Token::Operation(Operation::Prim2(OpPrim2::LessEqual)));
                index += 2;
            }
            else if regex!("^&\\s").is_match(s) {
                token_list.push(Token::Operation(Operation::Prim2(OpPrim2::BitwiseAnd)));
                index += 1;
            }
            else if regex!("^\\|\\s").is_match(s) {
                token_list.push(Token::Operation(Operation::Prim2(OpPrim2::BitwiseOr)));
                index += 1;
            }
            else if regex!("^\\^\\s").is_match(s) {
                token_list.push(Token::Operation(Operation::Prim2(OpPrim2::BitwiseXor)));
                index += 1;
            }
            else if regex!("^<<\\s").is_match(s) {
                token_list.push(Token::Operation(Operation::Prim2(OpPrim2::ShiftLeft)));
                index += 2;
            }
            else if regex!("^>>\\s").is_match(s) {
                token_list.push(Token::Operation(Operation::Prim2(OpPrim2::ShiftRight)));
                index += 2;
            }
            else if regex!("^<>\\s").is_match(s) {
                token_list.push(Token::Operation(Operation::Prim2(OpPrim2::SameType)));
                index += 2;
            }
            // Type Matching
            else if regex!("^none").is_match(s) {
                token_list.push(Token::Type(RawType::None));
                index += 4;
            }
            else if regex!("^i8").is_match(s) {
                token_list.push(Token::Type(RawType::Int8));
                index += 2;
            }
            else if regex!("^i16").is_match(s) {
                token_list.push(Token::Type(RawType::Int16));
                index += 3;
            }
            else if regex!("^i32").is_match(s) {
                token_list.push(Token::Type(RawType::Int32));
                index += 3;
            }
            else if regex!("^isize").is_match(s) {
                token_list.push(Token::Type(RawType::Int));
                index += 5;
            }
            else if regex!("^i64").is_match(s) {
                token_list.push(Token::Type(RawType::Int64));
                index += 3;
            }
            else if regex!("^i128").is_match(s) {
                token_list.push(Token::Type(RawType::Int128));
                index += 4;
            }
            else if regex!("^u8").is_match(s) {
                token_list.push(Token::Type(RawType::UInt8));
                index += 2;
            }
            else if regex!("^u16").is_match(s) {
                token_list.push(Token::Type(RawType::UInt16));
                index += 3;
            }
            else if regex!("^u32").is_match(s) {
                token_list.push(Token::Type(RawType::UInt32));
                index += 3;
            }
            else if regex!("^usize").is_match(s) {
                token_list.push(Token::Type(RawType::UInt));
                index += 5;
            }
            else if regex!("^u64").is_match(s) {
                token_list.push(Token::Type(RawType::UInt64));
                index += 3;
            }
            else if regex!("^u128").is_match(s) {
                token_list.push(Token::Type(RawType::UInt128));
                index += 4;
            }
            else if regex!("^f32").is_match(s) {
                token_list.push(Token::Type(RawType::Float32));
                index += 3;
            }
            else if regex!("^f64").is_match(s) {
                token_list.push(Token::Type(RawType::Float64));
                index += 3;
            }
            else if regex!("^chr").is_match(s) {
                token_list.push(Token::Type(RawType::Char));
                index += 3;
            }
            else if regex!("^str").is_match(s) {
                token_list.push(Token::Type(RawType::String));
                index += 3;
            }
            else if regex!("^bool").is_match(s) {
                token_list.push(Token::Type(RawType::Boolean));
                index += 4;
            }
            else if regex!("^add").is_match(s) {
                token_list.push(Token::Type(RawType::Address));
                index += 1;
            }
            else if regex!("^reg").is_match(s) {
                token_list.push(Token::Type(RawType::Register));
                index += 1;
            }
            // Register Matching
            else if let Some((_, num)) = regex_captures!("^r([\\d]+)\\s", s) {
                token_list.push(Token::Register(num.parse::<u8>().unwrap()));
                index += 1 + num.len();
            }
            // Data Collecting
            else if regex!("^\\(").is_match(s) {
                let ri = s.rfind(')').unwrap();
                token_list.push(Token::Data(String::from(&s[1..ri])));
                index += ri + 1;
            }
            // Ignore Comments
            else if regex!("^;").is_match(s) {
                break;
            }
            else {
                index += 1;
            }
        }
    }

    token_list.reverse();
    token_list
}
