use crate::{HeapType, Register, Type};

pub struct Registers(Vec<Type>);
impl Registers {
    pub fn new() -> Self {
        let registers = (0..30).map(|_i| Type::None).collect();
        Self(registers)
    }

    pub fn get(&self, register: Register) -> &Type {
        match self.0.get(register as usize) {
            None => panic!("{:?} is not a valid register.", register),
            Some(i) => i,
        }
    }

    pub fn get_mut(&mut self, register: Register) -> &mut Type {
        match self.0.get_mut(register as usize) {
            None => panic!("{:?} is not a valid register.", register),
            Some(i) => i,
        }
    }

    pub fn insert(&mut self, register: Register, t: Type) {
        let i = register as usize;
        if i >= 30 {
            panic!("{:?} is not a valid register.", register)
        }

        self.0.remove(i);
        self.0.insert(i, t);
    }

    pub fn own(&mut self, register: Register) -> Type {
        let i = register as usize;
        if i >= 30 {
            panic!("{:?} is not a valid register.", register)
        }

        let element = self.0.remove(i);
        self.0.insert(i, Type::None);

        element
    }

    pub fn copy(&mut self, register: Register) -> Type {
        let r = self.get(register);
        r.copy()
    }
}
impl Default for Registers {
    fn default() -> Self {
        Self::new()
    }
}

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
    heap: Vec<HeapType>,
}
impl Heap {
    pub fn new() -> Self {
        Self { heap: Vec::new() }
    }
}
