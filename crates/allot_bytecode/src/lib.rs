use std::mem::size_of;

#[cfg(feature = "forms")]
pub use forms::*;
#[cfg(feature = "gen")]
pub use gen::gen;
#[cfg(feature = "parse")]
pub use parse::parse;

#[cfg(feature = "gen")]
mod gen;
#[cfg(feature = "parse")]
mod parse;

/// For now the layout of allot files the BYTECODE_VERSION, then just a linear list of instructions.
// TODO: Allow some data about the program to be stored.

pub const BYTECODE_VERSION: usize = 0;

#[derive(Clone, Debug, Default)]
pub struct Buffer(Vec<u8>);
impl Buffer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with(vec: Vec<u8>) -> Self {
        Self(vec)
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    // Write

    pub fn write_i8(&mut self, data: i8) {
        let buffer = &(data).to_le_bytes();
        self.0.extend_from_slice(buffer);
    }

    pub fn write_i16(&mut self, data: i16) {
        let buffer = &(data).to_le_bytes();
        self.0.extend_from_slice(buffer);
    }

    pub fn write_i32(&mut self, data: i32) {
        let buffer = &(data).to_le_bytes();
        self.0.extend_from_slice(buffer);
    }

    pub fn write_i64(&mut self, data: i64) {
        let buffer = &(data).to_le_bytes();
        self.0.extend_from_slice(buffer);
    }

    pub fn write_i128(&mut self, data: i128) {
        let buffer = &(data).to_le_bytes();
        self.0.extend_from_slice(buffer);
    }

    pub fn write_u8(&mut self, data: u8) {
        let buffer = &(data).to_le_bytes();
        self.0.extend_from_slice(buffer);
    }

    pub fn write_u16(&mut self, data: u16) {
        let buffer = &(data).to_le_bytes();
        self.0.extend_from_slice(buffer);
    }

    pub fn write_u32(&mut self, data: u32) {
        let buffer = &(data).to_le_bytes();
        self.0.extend_from_slice(buffer);
    }

    pub fn write_u64(&mut self, data: u64) {
        let buffer = &(data).to_le_bytes();
        self.0.extend_from_slice(buffer);
    }

    pub fn write_u128(&mut self, data: u128) {
        let buffer = &(data).to_le_bytes();
        self.0.extend_from_slice(buffer);
    }

    pub fn write_f32(&mut self, data: f32) {
        let buffer = &(data).to_le_bytes();
        self.0.extend_from_slice(buffer);
    }

    pub fn write_f64(&mut self, data: f64) {
        let buffer = &(data).to_le_bytes();
        self.0.extend_from_slice(buffer);
    }

    pub fn write_char(&mut self, data: char) {
        let buffer = &(data as u32).to_le_bytes();
        self.0.extend_from_slice(buffer);
    }

    pub fn write_str(&mut self, data: &str) {
        let buffer = data.as_bytes();

        self.write_u64(data.len() as u64);
        self.0.extend_from_slice(buffer);
    }

    pub fn write_string(&mut self, data: &String) {
        let buffer = data.as_bytes();

        self.write_u64(data.len() as u64);
        self.0.extend_from_slice(buffer);
    }

    pub fn write_bool(&mut self, data: bool) {
        let buffer = &(data as u8).to_le_bytes();
        self.0.extend_from_slice(buffer);
    }

    // Read

    pub fn read_i8(&mut self) -> i8 {
        let v: [u8; size_of::<i8>()] = self
            .0
            .drain(0..size_of::<i8>())
            .collect::<Vec<u8>>()
            .try_into()
            .unwrap();
        i8::from_le_bytes(v)
    }

    pub fn read_i16(&mut self) -> i16 {
        let v: [u8; size_of::<i16>()] = self
            .0
            .drain(0..size_of::<i16>())
            .collect::<Vec<u8>>()
            .try_into()
            .unwrap();
        i16::from_le_bytes(v)
    }

    pub fn read_i32(&mut self) -> i32 {
        let v: [u8; size_of::<i32>()] = self
            .0
            .drain(0..size_of::<i32>())
            .collect::<Vec<u8>>()
            .try_into()
            .unwrap();
        i32::from_le_bytes(v)
    }

    pub fn read_i64(&mut self) -> i64 {
        let v: [u8; size_of::<i64>()] = self
            .0
            .drain(0..size_of::<i64>())
            .collect::<Vec<u8>>()
            .try_into()
            .unwrap();
        i64::from_le_bytes(v)
    }

    pub fn read_i128(&mut self) -> i128 {
        let v: [u8; size_of::<i128>()] = self
            .0
            .drain(0..size_of::<i128>())
            .collect::<Vec<u8>>()
            .try_into()
            .unwrap();
        i128::from_le_bytes(v)
    }

    pub fn read_u8(&mut self) -> u8 {
        let v: [u8; size_of::<u8>()] = self
            .0
            .drain(0..size_of::<u8>())
            .collect::<Vec<u8>>()
            .try_into()
            .unwrap();
        u8::from_le_bytes(v)
    }

    pub fn read_u16(&mut self) -> u16 {
        let v: [u8; size_of::<u16>()] = self
            .0
            .drain(0..size_of::<u16>())
            .collect::<Vec<u8>>()
            .try_into()
            .unwrap();
        u16::from_le_bytes(v)
    }

    pub fn read_u32(&mut self) -> u32 {
        let v: [u8; size_of::<u32>()] = self
            .0
            .drain(0..size_of::<u32>())
            .collect::<Vec<u8>>()
            .try_into()
            .unwrap();
        u32::from_le_bytes(v)
    }

    pub fn read_u64(&mut self) -> u64 {
        let v: [u8; size_of::<u64>()] = self
            .0
            .drain(0..size_of::<u64>())
            .collect::<Vec<u8>>()
            .try_into()
            .unwrap();
        u64::from_le_bytes(v)
    }

    pub fn read_u128(&mut self) -> u128 {
        let v: [u8; size_of::<u128>()] = self
            .0
            .drain(0..size_of::<u128>())
            .collect::<Vec<u8>>()
            .try_into()
            .unwrap();
        u128::from_le_bytes(v)
    }

    pub fn read_f32(&mut self) -> f32 {
        let v: [u8; size_of::<f32>()] = self
            .0
            .drain(0..size_of::<f32>())
            .collect::<Vec<u8>>()
            .try_into()
            .unwrap();
        f32::from_le_bytes(v)
    }

    pub fn read_f64(&mut self) -> f64 {
        let v: [u8; size_of::<f64>()] = self
            .0
            .drain(0..size_of::<f64>())
            .collect::<Vec<u8>>()
            .try_into()
            .unwrap();
        f64::from_le_bytes(v)
    }

    pub fn read_char(&mut self) -> char {
        let num = self.read_u32();
        char::from_u32(num).unwrap()
    }

    pub fn read_str(&mut self) -> String {
        self.read_string()
    }

    pub fn read_string(&mut self) -> String {
        let len = self.read_u64() as usize;
        let v: Vec<u8> = self.0.drain(0..len).collect();
        String::from_utf8(v).unwrap()
    }

    pub fn read_bool(&mut self) -> bool {
        let num = self.read_u8();
        !matches!(num, 0)
    }
}
