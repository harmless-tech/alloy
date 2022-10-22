use crate::Type;

#[derive(Debug)]
pub struct StackFrame {
    pub stack: Vec<Type>,
    /// Isolated StackFrames do not use variables from StackFrames below it.
    pub isolated: bool,
}
impl StackFrame {}

#[derive(Debug)]
pub struct Heap {}
