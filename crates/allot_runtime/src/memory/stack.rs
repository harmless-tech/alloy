use allot_lib::Type;

#[derive(Debug, Default)]
pub struct StackFrame {
    stack: Vec<Type>,
    // Cannot access this stack frame from another one.
    // isolated: bool, TODO: Enable this when transferring between stack frames is possible.
}
impl StackFrame {
    pub fn new(_isolated: bool) -> Self {
        Self {
            stack: Vec::new(),
            // isolated,
        }
    }

    pub fn from(stack_frame: &mut Self) -> Self {
        let stack: Vec<Type> = stack_frame
            .stack
            .drain(0..stack_frame.stack.len())
            .collect();
        Self {
            stack,
            // isolated: false,
        }
    }

    pub fn push(&mut self, t: Type) {
        self.stack.push(t);
    }

    pub fn pop(&mut self) -> Type {
        match self.stack.pop() {
            None => panic!("Tried to pop from stack but it was empty."),
            Some(v) => v,
        }
    }

    pub fn clone_offset(&self, offset: usize) -> Type {
        let offset = self.stack.len() - offset - 1;
        match self.stack.get(offset) {
            None => panic!("There is no item on the stack at {offset}."),
            Some(item) => item.clone(),
        }
    }
}
