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
        let i = register as usize;
        if i >= 30 {
            panic!("{:?} is not a valid register.", register)
        }
        if let Type::Register(_) = t {
            panic!("Tried to put a Register type into a register.");
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
