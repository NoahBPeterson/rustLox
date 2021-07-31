use std::usize;

use crate::chunk::{Chunk, OpCode};
use crate::value::{self};


pub fn disassemble_chunk(chunk: &Chunk, string: String)
{
    println!("{}", string);

    let mut offset: u8 = 0;
    while (offset as usize) < chunk.code.len()
    {
        offset = disassemble_instruction(&chunk, offset);
    }
}

pub fn disassemble_instruction(chunk: &Chunk, offset: u8) -> u8
{
    print!("{:04} ", offset);

    if offset > 0 && chunk.lines[offset as usize] == chunk.lines[(offset as usize) - 1]
    {
        print!("   | ");
    } else {
        print!("{:4} ", chunk.lines[offset as usize]);
    }

    let instruction: u8 = chunk.code[offset as usize];
    match instruction
    {
        x if x == OpCode::OpReturn as u8 =>
        {
            return simple_instruction("OP_RETURN\n".to_string(), offset);
        }
        x if x == OpCode::OpConstant as u8 =>
        {
            return constant_instruction("OP_CONSTANT\n".to_string(), chunk, offset);
        }
        _ => 
        {
            print!("Unknown opcode {}\n", instruction);
            return offset + 1;
        }
    }
}

pub fn simple_instruction(name: String, offset: u8) -> u8
{
    print!("{}", name);
    return offset + 1;
}

pub fn constant_instruction(name: String, chunk: &Chunk, offset: u8) -> u8
{
    let constant: u8 = chunk.code[offset as usize + 1];
    print!("{:16}\t{:4} '", name, constant);
    value::print_value(chunk.constants.values[constant as usize]);
    print!("'\n");
    return offset + 2;
}