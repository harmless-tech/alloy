use std::{
    collections::BTreeMap,
    os::raw::c_void,
    sync::{Arc, Mutex},
};

use allot_lib::Type;

pub type CrossHeap = Arc<Mutex<Heap>>; // TODO: Each thread should handle its own heap, add a way to send info to other
                                       // threads.

#[derive(Debug, Default)]
pub struct Heap {
    heap: BTreeMap<usize, *const c_void>,
    heap_pointer: usize,
}
impl Heap {
    pub fn new() -> Self {
        Self {
            heap: BTreeMap::new(),
            heap_pointer: 0,
        }
    }

    pub fn cross_new() -> CrossHeap {
        Arc::new(Mutex::new(Heap::default()))
    }

    pub fn push<T>(&mut self, t: T) -> Type {
        let raw = Box::into_raw(Box::new(t));
        let ptr: *const c_void = unsafe { std::mem::transmute(raw) };

        let i = self.heap_pointer;
        self.heap_pointer += 1;

        self.heap.insert(i, ptr);
        Type::Pointer(i)
    }

    // TODO: Need a way to type check?
    pub fn take<T>(&mut self, pointer: usize) -> Box<T> {
        match self.heap.remove(&pointer) {
            None => panic!("Pointer does not point to anything in the heap."),
            Some(ptr) => {
                let pointer: *mut T = unsafe { std::mem::transmute(ptr) };
                unsafe { Box::from_raw(pointer) }
            }
        }
    }

    pub fn update<T>(&mut self, pointer: usize, t: T) {
        let raw = Box::into_raw(Box::new(t));
        let ptr: *mut c_void = unsafe { std::mem::transmute(raw) };

        self.heap.insert(pointer, ptr);
    }

    /// Frees the memory at the current pointer. Does nothing if the pointer
    /// points to nothing.
    pub fn free(&mut self, pointer: usize) {
        self.heap.remove(&pointer);
    }
}
unsafe impl Send for Heap {}
