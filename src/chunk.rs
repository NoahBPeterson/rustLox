use std::u32;

use crate::value::{self, ValueArray};

#[repr(u8)]
pub enum OpCode
{
    OpConstant = 0,
    OpReturn = 1,
    OpNegate = 2,
    OpAdd = 3,
    OpSubtract = 4,
    OpMultiply = 5,
    OpDivide = 6,
}

pub struct Chunk
{
    pub code: Vec<u8>, // Array of bytes.
    pub constants: ValueArray, // Vec<f64>
    pub lines: Vec<u32>, // Array of lines
}

pub fn init_chunk() -> Chunk
{
    let chunk_init = Chunk
    {
        code: Vec::with_capacity(0),
        constants: value::init_value_array(),
        lines: Vec::with_capacity(0),
        };
    return chunk_init;
}

pub fn write_chunk(chunk: &mut Chunk, byte: u8, line: u32)
{
    chunk.code.push(byte);
    chunk.lines.push(line);
}

pub fn add_constant(chunk: &mut Chunk, value: f64) -> u32
{
    value::write_value_array(&mut chunk.constants, value);
    return (chunk.constants.values.len() - 1) as u32;
}