
use std::{cell::RefCell, collections::HashMap};

use crate::{chunk::{self, Chunk, init_chunk}, compile::{Compiler}, debug::disassemble_instruction, object::{Obj, ObjString, ObjType}, value::{self, InternalNil, Value, ValueType, print_value}};

#[derive(Clone)]
pub struct VM
{
    chunk: Chunk,
    instructions: Vec<u8>,
    stack: Vec<Value>,
    StackTop: u32,
    ip: u16,
    pub strings: RefCell<HashMap<ObjString, Value>>,
    globals: RefCell<HashMap<ObjString, Value>>,
}

impl VM
{ 

    pub fn interpret(&mut self, source: String) -> InterpretResult
    {
        let mut chunk = init_chunk();
    
        if !Compiler::new_compiler(&mut chunk, self).compile(source)
        {
            return InterpretResult::InterpretCompileError;
        }
    
        self.chunk = chunk;
        self.instructions = self.chunk.code.clone();
    
        let result = self.run();
        return result;
    }

    fn peek(&self, distance: u32) -> Value
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
            print!("          ");
            for slot in &self.stack
            {
                print!("[ ");
                print_value(slot.clone());
                print!(" ]");
            }
            println!("");
            let line = self.instructions[i];
            disassemble_instruction(&self.chunk, i as u8);
            match line
            {
                x if x == chunk::OpCode::OpReturn as u8 =>
                {
                    //print_value(self.pop());
                    //println!("");
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
                    let negated_number = -self.pop().GetNumber();
                    self.push(crate::value::NumberAsValue(negated_number));
                }
                x if x == chunk::OpCode::OpAdd as u8 =>
                {
                    if self.peek(0).IsString() && self.peek(1).IsString()
                    {
                        self.Concatenate();
                    }
                    else if self.peek(0).IsNumber() && self.peek(1).IsNumber()
                    {
                        let b = self.pop().GetNumber();
                        let a = self.pop().GetNumber();
                        self.push(crate::value::NumberAsValue(a + b));
    
                    }
                    else
                    {
                        self.RuntimeError("Operands must be numbers or strings.".to_string());
                        return InterpretResult::InterpretRuntimeError;
                    }
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
                x if x == chunk::OpCode::OpNot as u8 => 
                {
                    let boolean_not = self.pop().IsFalsey();
                    self.push(crate::value::BoolAsValue(boolean_not));
                }
                x if x == chunk::OpCode::OpEqual as u8 => 
                {
                    let is_equal = self.pop().Equals(self.pop());
                    self.push(crate::value::BoolAsValue(is_equal))
                }
                x if x == chunk::OpCode::OpPrint as u8 => 
                {
                    let print = self.pop();
                    print_value(print);
                    println!("");
                }
                x if x == chunk::OpCode::OpPop as u8 => {self.pop();}
                x if x == chunk::OpCode::OpDefineGlobal as u8 => 
                {
                    let name = self.read_string(); //READ_STRING
                    match name
                    {
                        Some(string) =>
                        {
                            self.TableSet(&self.globals.clone(), &string, self.peek(0).clone());
                            self.pop();
                        },
                        _ => return InterpretResult::InterpretRuntimeError,
                    }
                }
                x if x == chunk::OpCode::OpGetGlobal as u8 =>
                {
                    let name = self.read_string();
                    match name
                    {
                        Some(string) =>
                        {
                            let value = self.TableGet(&self.globals.clone(), &string);
                            if !value.0
                            {
                                self.RuntimeError("Undefined variable '".to_owned()+&string.str.to_owned()+"'");
                                return InterpretResult::InterpretRuntimeError
                            } else {
                                self.push(value.1)
                            }
                        }
                        None => return InterpretResult::InterpretRuntimeError,
                    }
                }
                x if x == chunk::OpCode::OpSetGlobal as u8 =>
                {
                    let name = self.read_string();
                    match name
                    {
                        Some(string) =>
                        {
                            let value = self.TableSet(&self.globals.clone(), &string, self.peek(0).clone());
                            if !value
                            {
                                self.TableDelete(&self.globals.clone(), &string);
                                self.RuntimeError("Undefined variable '".to_owned()+&string.str.to_owned()+"'");
                                return InterpretResult::InterpretRuntimeError
                            }
                        }
                        None => return InterpretResult::InterpretRuntimeError,
                    }
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

    fn Concatenate(&mut self)
    {
        let value_of_b = self.pop().GetString();
        let value_of_a = self.pop().GetString();
        let length = value_of_a.length + value_of_b.length;

        let both = ObjString {str: value_of_a.str + &value_of_b.str, length: length };
        self.push(value::ObjAsValue(Obj {typeOfObject: ObjType::ObjString(Box::new(both)) }));
    }

    fn RuntimeError(&self, error: String)
    {
        let instruction = self.ip - self.chunk.code[self.ip as usize -1] as u16;
        let line_number = self.chunk.lines[instruction as usize];
        println!("[line {}] in script", line_number);
        println!("{}", error);
        self.ResetStack();
    }

    fn ResetStack(&self)
    {

    }

    fn read_string(&mut self) -> Option<ObjString>
    {
        let val = self.chunk.constants.values[self.ip as usize].clone();
        match val.ValueType
        {
            ValueType::ValObj(obj) =>
            {
                match obj.typeOfObject
                {
                    ObjType::ObjString(obj_str) => 
                    {
                        return Some(*obj_str);
                    }
                    _ => self.RuntimeError("Tried to grab a string from a non-string object!".to_owned())
                }
            }
            _ => self.RuntimeError("Tried to grab a string from a non-obj value!".to_owned())
        }
        return None
    }

    pub fn TableSet(&mut self, table: &RefCell<HashMap<ObjString, Value>>, key: &ObjString, value: Value) -> bool
    {
        let new_entry = self.FindEntry(table.clone(), &key);
        table.borrow_mut().insert(key.clone(), value);

        match new_entry
        {
            _Value => true,
            _ => false,
        }
    }

    fn TableDelete(&mut self, table: &RefCell<HashMap<ObjString, Value>>, key: &ObjString) -> bool
    {
        let entry_exists = self.FindEntry(table.clone(), &key);
        table.borrow_mut().remove(key);
        match entry_exists.ValueType
        {
            ValueType::ValInternalNil => return false,
            _ => return true,
        }
    }

    fn FindEntry(&self, table: RefCell<HashMap<ObjString, Value>>, key: &ObjString) -> Value
    {
        match table.borrow().get(&key)
        {
            Some(val) => val.to_owned(),
            None => InternalNil(),
        }
    }

    fn TableGet(&self, table: &RefCell<HashMap<ObjString, Value>>, key: &ObjString) -> (bool, Value)
    {
        match table.borrow().get(&key)
        {
            Some(val) => (true, val.to_owned()),
            None => (false, InternalNil()),
        }
    }
}

pub fn init_vm() -> VM
{
    VM
    {
        chunk: chunk::init_chunk(),
        instructions: Vec::with_capacity(0),
        stack: Vec::with_capacity(0),
        StackTop: 0,
        ip: 0,
        strings: RefCell::new(HashMap::new()),
        globals: RefCell::new(HashMap::new()),
    }
}

pub enum InterpretResult
{
    InterpretOk = 1,
    InterpretCompileError = 2,
    InterpretRuntimeError = 3,
}

