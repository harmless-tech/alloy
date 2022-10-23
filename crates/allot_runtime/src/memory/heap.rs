use std::{
    collections::BTreeMap,
    sync::{Arc, RwLock},
    thread,
};

use allot_lib::Type;

use crate::StackFrame;

pub type CrossHeap = Arc<RwLock<Heap>>;

/// HeapTypes can only be managed by function calls.
#[derive(Debug)]
pub enum HeapBox {
    None,
    Type(Box<Type>),
    ThreadHandle(Box<thread::JoinHandle<(i32, StackFrame)>>),
    // File
    // Vec
    // Tuple?
    // HashSet
    // HashMap
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

    pub fn take(&mut self, pointer: usize) -> HeapBox {
        match self.heap.remove(&pointer) {
            None => panic!("Pointer does not point to anything in the heap."),
            Some(i) => i,
        }
    }

    /// Frees the memory at the current pointer. Does nothing if the pointer points to nothing.
    pub fn free(&mut self, pointer: usize) {
        self.heap.remove(&pointer);
    }
}
