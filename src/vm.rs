use core::f64;
use std::string;

use crate::{chunk::{self, Chunk, init_chunk}, compile::{self, Compiler}, debug::disassemble_instruction, value::print_value};

pub struct VM
{
    chunk: Chunk,
    instructions: Vec<u8>,
    stack: Vec<f64>,
    StackTop: u32,
    ip: u16,
}

pub fn init_vm() -> VM
{
    return VM
    {
        chunk: chunk::init_chunk(),
        instructions: Vec::with_capacity(0),
        stack: Vec::with_capacity(0),
        StackTop: 0,
        ip: 0
    };
}

pub fn interpret(source: String, vm: &mut VM) -> InterpretResult
{
    let mut chunk = init_chunk();

    if !Compiler::new_compiler(&mut chunk).compile(source)
    {
        return InterpretResult::InterpretCompileError;
    }

    vm.chunk = chunk;
    vm.instructions = vm.chunk.code.clone();

    let result = run(vm);
    return result;
}

pub fn run(vm: &mut VM) -> InterpretResult
{
    let mut slot = 0;
    while slot < vm.StackTop
    {
        print_value(vm.stack[slot as usize]);
        slot = slot + 1;
    }
    let mut i = 0;
    while i < vm.instructions.len()
    {
        /*print!("          ");
        for slot in &vm.stack
        {
            print!("[ ");
            print_value(*slot);
            print!(" ]");
        }
        println!("");*/
        let line = vm.instructions[i];
        disassemble_instruction(&vm.chunk, i as u8);
        match line
        {
            x if x == chunk::OpCode::OpReturn as u8 =>
            {
                print_value(pop(vm));
                println!("");
                return InterpretResult::InterpretOk;
            }
            x if x == chunk::OpCode::OpConstant as u8 =>
            {
                let constant: f64 = vm.chunk.constants.values[vm.ip as usize];
                vm.ip = vm.ip + 1;
                i = i + 1;
                print_value(constant);
                println!("");
                push(constant, vm);
            }
            x if x == chunk::OpCode::OpNegate as u8 =>
            {
                push(-pop(vm), vm);
            }
            x if x == chunk::OpCode::OpAdd as u8 =>
            {
                let b = pop(vm);
                let a = pop(vm);
                push(a + b, vm);
            }
            x if x == chunk::OpCode::OpSubtract as u8 =>
            {
                let b = pop(vm);
                let a = pop(vm);
                print_value(a - b);
                push(a - b, vm);
            }
            x if x == chunk::OpCode::OpMultiply as u8 =>
            {
                let b = pop(vm);
                let a = pop(vm);
                print_value(a * b);
                push(a * b, vm);
            }
            x if x == chunk::OpCode::OpDivide as u8 =>
            {
                let b = pop(vm);
                let a = pop(vm);
                print_value(a / b);
                push(a / b, vm);
            }
            _ =>
            {
                return InterpretResult::InterpretRuntimeError;
            }
        }
        i = i + 1;
    }
    return InterpretResult::InterpretOk;
}

pub fn push(value: f64, vm: &mut VM)
{
    vm.stack.push(value);
    vm.StackTop = vm.StackTop + 1;
}

pub fn pop(vm: &mut VM) -> f64
{
    if vm.StackTop == 0
    {
        panic!("Attempted to pop from an empty stack!");
    }

    let stack_pop = vm.stack[(vm.StackTop - 1) as usize];
    vm.StackTop = vm.StackTop - 1;
    vm.stack.remove(vm.StackTop as usize);

    return stack_pop;
}

pub enum InterpretResult
{
    InterpretOk = 1,
    InterpretCompileError = 2,
    InterpretRuntimeError = 3,
}