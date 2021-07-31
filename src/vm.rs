use core::f64;

use crate::{chunk::{self, Chunk}, debug::disassemble_instruction, value::print_value};

pub struct VM
{
    chunk: Chunk,
    instructions: Vec<u8>,
    stack: Vec<f64>,
    stackTop: u32,
    ip: u16,
}

pub fn init_vm() -> VM
{
    return VM
    {
        chunk: chunk::init_chunk(),
        instructions: Vec::with_capacity(0),
        stack: Vec::with_capacity(0),
        stackTop: 0,
        ip: 0
    };
}

pub fn interpret(chunk: Chunk, vm: &mut VM) -> InterpretResult
{
    vm.chunk = chunk;
    vm.instructions = vm.chunk.code.clone();
    return run(vm);
}

pub fn run(vm: &mut VM) -> InterpretResult
{
    let mut slot = 0;
    while slot < vm.stackTop
    {
        print_value(vm.stack[slot as usize]);
        slot = slot + 1;
    }
    let mut i = 0;
    while i < vm.instructions.len()
    {
        

        let line = vm.instructions[i];
        disassemble_instruction(&vm.chunk, line - vm.chunk.code[0]);
        match line
        {
            x if x == chunk::OpCode::OpReturn as u8 =>
            {
                return InterpretResult::InterpretOk;
            }
            x if x == chunk::OpCode::OpConstant as u8 =>
            {
                let constant: f64 = vm.chunk.constants.values[vm.instructions[i+1] as usize];
                i = i + 1;
                print_value(constant);
                push(constant, vm);
            }
            _ =>
            {
                return InterpretResult::InterpretRuntimeError;
            }
        }
    }
    return InterpretResult::InterpretOk;
}

pub fn push(value: f64, vm: &mut VM)
{
    vm.stack.push(value);
    vm.stackTop = vm.stackTop + 1;
}

pub fn pop(vm: &mut VM) -> f64
{
    if vm.stackTop == 0
    {
        panic!("Attempted to pop from an empty stack!");
    }

    let stack_pop = vm.stack[(vm.stackTop - 1) as usize];
    vm.stackTop = vm.stackTop - 1;

    return stack_pop;
}

pub enum InterpretResult
{
    InterpretOk = 1,
    InterpretCompileErrror = 2,
    InterpretRuntimeError = 3,
}