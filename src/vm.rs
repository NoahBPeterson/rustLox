use core::f64;
use std::{string, sync::Arc};

use crate::{chunk::{self, Chunk, init_chunk}, compile::{self, Compiler}, debug::disassemble_instruction, value::{BoolAsValue, NumberAsValue, NilAsValue, Value, print_value}};

pub struct VM<'a>
{
    chunk: &'a mut Chunk,
    instructions: Vec<u8>,
    stack: Vec<Value>,
    StackTop: u32,
    ip: u16,
}

impl VM<'_>
{ 

    pub fn interpret(&mut self, source: String) -> InterpretResult
    {
        let mut chunk = init_chunk();
    
        if !Compiler::new_compiler(&mut chunk).compile(source)
        {
            return InterpretResult::InterpretCompileError;
        }
    
        self.chunk = &mut chunk;
        self.instructions = self.chunk.code.clone();
    
        let result = self.run();
        return result;
    }

    fn peek(self, distance: u32) -> Value
    {
        self.stack[((self.StackTop -1) - distance) as usize].clone()
    }

    pub fn push(&mut self, value: Value)
    {
        self.stack.push(value);
        self.StackTop = self.StackTop + 1;
    }

    pub fn pop(&mut self) -> Value
    {
        if self.StackTop == 0
        {
            panic!("Attempted to pop from an empty stack!");
        }

        let stack_pop = self.stack[(self.StackTop - 1) as usize].clone();
        self.StackTop = self.StackTop - 1;
        self.stack.remove(self.StackTop as usize);

        return stack_pop;
    }

    pub fn run(&mut self) -> InterpretResult
    {
        let mut slot = 0;
        while slot < self.StackTop
        {
            print_value(self.stack[slot as usize].clone());
            slot = slot + 1;
        }
        let mut i = 0;
        while i < self.instructions.len()
        {
            /*print!("          ");
            for slot in &vm.stack
            {
                print!("[ ");
                print_value(*slot);
                print!(" ]");
            }
            println!("");*/
            let line = self.instructions[i];
            disassemble_instruction(&self.chunk, i as u8);
            match line
            {
                x if x == chunk::OpCode::OpReturn as u8 =>
                {
                    print_value(self.pop());
                    println!("");
                    return InterpretResult::InterpretOk;
                }
                x if x == chunk::OpCode::OpConstant as u8 =>
                {
                    let constant = self.chunk.constants.values[self.ip as usize].clone();
                    self.ip = self.ip + 1;
                    i = i + 1;
                    print_value(constant.clone());
                    println!("");
                    self.push(constant);
                }
                x if x == chunk::OpCode::OpNegate as u8 =>
                {
                    if !self.peek(0).IsNumber()
                    {
                        self.RuntimeError( "Operand must be a number.".to_string());
                        return InterpretResult::InterpretRuntimeError;
                    }
                    self.push(crate::value::NumberAsValue(-self.pop().GetNumber()));
                }
                x if x == chunk::OpCode::OpAdd as u8 =>
                {
                    if !self.peek(0).IsNumber() || !self.peek(1).IsNumber()
                    {
                        self.RuntimeError("Operands must be numbers.".to_string());
                        return InterpretResult::InterpretRuntimeError;
                    }
                    let b = self.pop().GetNumber();
                    let a = self.pop().GetNumber();
                    self.push(crate::value::NumberAsValue(a + b));
                }
                x if x == chunk::OpCode::OpSubtract as u8 =>
                {
                    if !self.peek(0).IsNumber() || !self.peek(1).IsNumber()
                    {
                        self.RuntimeError("Operands must be numbers.".to_string());
                        return InterpretResult::InterpretRuntimeError;
                    }
                    let b = self.pop().GetNumber();
                    let a = self.pop().GetNumber();
                    print_value(crate::value::NumberAsValue(a - b));
                    self.push(crate::value::NumberAsValue(a - b));
                }
                x if x == chunk::OpCode::OpMultiply as u8 =>
                {
                    if !self.peek(0).IsNumber() || !self.peek(1).IsNumber()
                    {
                        self.RuntimeError("Operands must be numbers.".to_string());
                        return InterpretResult::InterpretRuntimeError;
                    }
                    let b = self.pop().GetNumber();
                    let a = self.pop().GetNumber();
                    print_value(crate::value::NumberAsValue(a * b));
                    self.push(crate::value::NumberAsValue(a * b));
                }
                x if x == chunk::OpCode::OpDivide as u8 =>
                {
                    if !self.peek(0).IsNumber() || !self.peek(1).IsNumber()
                    {
                        self.RuntimeError("Operands must be numbers.".to_string());
                        return InterpretResult::InterpretRuntimeError;
                    }
                    let b = self.pop().GetNumber();
                    let a = self.pop().GetNumber();
                    print_value(crate::value::NumberAsValue(a / b));
                    self.push(crate::value::NumberAsValue(a / b));
                }
                x if x == chunk::OpCode::OpGreater as u8 =>
                {
                    if !self.peek(0).IsNumber() || !self.peek(1).IsNumber()
                    {
                        self.RuntimeError("Operands must be numbers.".to_string());
                        return InterpretResult::InterpretRuntimeError;
                    }
                    let b = self.pop().GetNumber();
                    let a = self.pop().GetNumber();
                    print_value(crate::value::BoolAsValue(a > b));
                    self.push(crate::value::BoolAsValue(a > b));
                }
                x if x == chunk::OpCode::OpLess as u8 =>
                {
                    if !self.peek(0).IsNumber() || !self.peek(1).IsNumber()
                    {
                        self.RuntimeError("Operands must be numbers.".to_string());
                        return InterpretResult::InterpretRuntimeError;
                    }
                    let b = self.pop().GetNumber();
                    let a = self.pop().GetNumber();
                    print_value(crate::value::BoolAsValue(a < b));
                    self.push(crate::value::BoolAsValue(a < b));
                }
                x if x == chunk::OpCode::OpNil as u8 => self.push(crate::value::NilAsValue()),
                x if x == chunk::OpCode::OpTrue as u8 => self.push(crate::value::BoolAsValue(true)),
                x if x == chunk::OpCode::OpFalse as u8 => self.push(crate::value::BoolAsValue(false)),
                x if x == chunk::OpCode::OpNot as u8 => self.push(crate::value::BoolAsValue(self.pop().IsFalsey())),
                x if x == chunk::OpCode::OpEqual as u8 => self.push(crate::value::BoolAsValue(self.pop().Equals(self.pop()))),
                _ =>
                {
                    return InterpretResult::InterpretRuntimeError;
                }
            }
            i = i + 1;
        }
        return InterpretResult::InterpretOk;
    }

    fn RuntimeError(self, error: String)
    {
        let instruction = self.ip - self.chunk.code[self.ip as usize -1] as u16;
        let lineNumber = self.chunk.lines[instruction as usize];
        println!("[line {}] in script", lineNumber);
        println!("{}", error);
        self.ResetStack();
    }

    fn ResetStack(self)
    {

    }
}

pub fn init_vm() -> VM
{
    VM
    {
        chunk: &mut chunk::init_chunk(),
        instructions: Vec::with_capacity(0),
        stack: Vec::with_capacity(0),
        StackTop: 0,
        ip: 0
    }
}

pub enum InterpretResult
{
    InterpretOk = 1,
    InterpretCompileError = 2,
    InterpretRuntimeError = 3,
}

