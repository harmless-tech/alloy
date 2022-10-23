use std::{
    collections::BTreeMap,
    sync::{Arc, RwLock},
};

// TODO: Split.
use allot_lib::{Register, Type};

#[derive(Debug)]
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
        // TODO: Block inserting Register type.
        let i = register as usize;
        if i >= 30 {
            panic!("{:?} is not a valid register.", register)
        }

        self.0.remove(i);
        self.0.insert(i, t);
    }

    pub fn take(&mut self, register: Register) -> Type {
        let i = register as usize;
        if i >= 30 {
            panic!("{:?} is not a valid register.", register)
        }

        let element = self.0.remove(i);
        self.0.insert(i, Type::None);

        element
    }

    pub fn clone(&mut self, register: Register) -> Type {
        let r = self.get(register);
        r.clone()
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

pub type CrossHeap = Arc<RwLock<Heap>>;

/// HeapTypes can only be managed by function calls.
#[derive(Debug)]
pub enum HeapBox {
    None,
    Type(Box<Type>),
    // File
    // Vec
    // Tuple?
    // HashSet
    // HashMap
    // ThreadHandle
}

#[derive(Debug, Default)]
pub struct Heap {
    heap: BTreeMap<usize, HeapBox>,
    heap_pointer: usize,
}
impl Heap {
    pub fn new() -> Self {
        Self {
            heap: BTreeMap::new(),
            heap_pointer: 0,
        }
    }

    pub fn push(&mut self, t: HeapBox) -> Type {
        self.heap.insert(self.heap_pointer, t);
        self.heap_pointer += 1;

        Type::Pointer(self.heap_pointer - 1)
    }

    pub fn get(&mut self, pointer: usize) -> &HeapBox {
        match self.heap.get(&pointer) {
            None => panic!(
                "Tried to get an item on the heap at {pointer}, but there was nothing there."
            ),
            Some(item) => item,
        }
    }

    pub fn get_mut(&mut self, pointer: usize) -> &mut HeapBox {
        match self.heap.get_mut(&pointer) {
            None => panic!(
                "Tried to get_mut an item on the heap at {pointer}, but there was nothing there."
            ),
            Some(item) => item,
        }
    }

    /// Frees the memory at the current pointer. Does nothing if the pointer points to nothing.
    pub fn free(&mut self, pointer: usize) {
        self.heap.remove(&pointer);
    }
}
