use crate::Type;

#[derive(Debug, Default)]
pub struct StackFrame {
    stack: Vec<Type>,
    /// Cannot access this stack frame from another one.
    isolated: bool,
}
impl StackFrame {
    pub fn new(isolated: bool) -> Self {
        Self {
            stack: Vec::new(),
            isolated,
        }
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
