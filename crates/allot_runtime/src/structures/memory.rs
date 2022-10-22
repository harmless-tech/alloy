use crate::Type;

#[derive(Debug, Default)]
pub struct StackFrame {
    stack: Vec<Type>,
}
impl StackFrame {
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }
}

#[derive(Debug, Default)]
pub struct Heap {
    //TODO: Heaps only grow right now, is this fine?
    heap: Vec<Type>,
}
impl Heap {
    pub fn new() -> Self {
        Self { heap: Vec::new() }
    }
}
