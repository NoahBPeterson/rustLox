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
        x if x == OpCode::OpReturn as u8 => return simple_instruction("OpReturn\n".to_string(), offset),
        x if x == OpCode::OpConstant as u8 => return constant_instruction("OpConstant\n".to_string(), chunk, offset),
        x if x == OpCode::OpNegate as u8 => return simple_instruction("OpNegate\n".to_string(), offset),
        x if x == OpCode::OpAdd as u8 => return simple_instruction("OpAdd\n".to_string(), offset),
        x if x == OpCode::OpSubtract as u8 => return simple_instruction("OpSubtract\n".to_string(), offset),
        x if x == OpCode::OpMultiply as u8 => return simple_instruction("OpMultiply\n".to_string(), offset),
        x if x == OpCode::OpDivide as u8 => return simple_instruction("OpDivide\n".to_string(), offset),
        x if x == OpCode::OpNil as u8 =>  return simple_instruction("OpNil\n".to_string(), offset),
        x if x == OpCode::OpTrue as u8 =>  return simple_instruction("OpTrue\n".to_string(), offset),
        x if x == OpCode::OpFalse as u8 =>  return simple_instruction("OpFalse\n".to_string(), offset),
        x if x == OpCode::OpNot as u8 =>  return simple_instruction("OpNot\n".to_string(), offset),
        x if x == OpCode::OpEqual as u8 =>  return simple_instruction("OpEqual\n".to_string(), offset),
        x if x == OpCode::OpGreater as u8 =>  return simple_instruction("OpGreater\n".to_string(), offset),
        x if x == OpCode::OpLess as u8 =>  return simple_instruction("OpLess\n".to_string(), offset),
        x if x == OpCode::OpPrint as u8 =>  return simple_instruction("OpPrint\n".to_string(), offset),
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
    value::print_value(chunk.constants.values[constant as usize].clone());
    print!("'\n");
    return offset + 2;
}