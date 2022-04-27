use std::convert::TryInto;

use crate::chunk::{Chunk, OpCode, add_constant, init_chunk, write_chunk};
use crate::debug::disassemble_chunk;
use crate::object::Obj;
use crate::scanner::{self, Make_Token, Scan_Token, Scanner, Token, TokenType};
use crate::value::{self, NumberAsValue, ObjAsValue, Value};
use crate::vm::{self, VM};

const debug_print_code: bool = true;
const debug_trace_execution: bool = true;

#[derive(Clone)]
pub struct Parser
{
    current: Token,
    previous: Token,
    had_error: bool,
    panic_mode: bool
}

impl Parser
{
    pub fn new() -> Parser
    {
        Parser
        {
            current: Token
            {
                token_type: TokenType::TokenWhile,
                line: 79680,
                length: 0,
                start: "".to_string(),
            },
            previous: Token
            {
                token_type: TokenType::TokenWhile,
                line: 79680,
                length: 0,
                start: "".to_string(),
            },
            had_error: false,
            panic_mode: false
        }
    }
}

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum Precedence
{
    PrecNone = 1,
    PrecAssignment = 2,
    PrecOr = 3,
    PrecAnd = 4,
    PrecEquality = 5,
    PrecComparison = 6,
    PrecTerm = 7,
    PrecFactor = 8,
    PrecUnary = 9,
    PrecCall = 10,
    PrecPrimary = 11
}

impl Precedence
{
    const fn get_precedence(precedence: u8) -> Option<Precedence>
    {
        match precedence
        {
            1 => Some(Precedence::PrecNone),
            2 => Some(Precedence::PrecAssignment),
            3 => Some(Precedence::PrecOr),
            4 => Some(Precedence::PrecAnd),
            5 => Some(Precedence::PrecEquality),
            6 => Some(Precedence::PrecComparison),
            7 => Some(Precedence::PrecTerm),
            8 => Some(Precedence::PrecFactor),
            9 => Some(Precedence::PrecUnary),
            10 => Some(Precedence::PrecCall),
            11 => Some(Precedence::PrecPrimary),
            _ => None
        }
    }
}

fn init_parser(scanner: &mut Scanner, chunk: &Chunk) -> Parser
{
    return Parser
    {
        current: Scan_Token(scanner),
        previous: Make_Token(TokenType::TokenSuper, &scanner),
        had_error: false,
        panic_mode: false,
    };

}

pub struct Compiler<'a, 'b>
{
    scanner: Scanner,
    parser: Parser,
    current_chunk: &'a mut Chunk,
    vm: &'b mut VM,
}

impl Compiler<'_, '_>
{
    pub fn new_compiler<'a, 'b>(chunk: &'a mut Chunk, vm: &'b mut VM) -> Compiler<'a, 'b>
    {
        Compiler
        {
            scanner: scanner::Init_Scanner("".to_string()),
            parser: Parser::new(),
            current_chunk: chunk,
            vm: vm
        }
    }
    
    pub fn compile(&mut self, source: String) -> bool
    {
        self.scanner = scanner::Init_Scanner(source);
        self.parser = init_parser(&mut self.scanner, self.current_chunk);
        let mut compiling_chunk: Chunk = init_chunk();
        //self.advance();
        //self.expression();

        while !self.match_token(TokenType::TokenEof)
        {
            self.declaration();
        }

        self.end_compiler();
        !self.parser.had_error
    }

    fn advance(&mut self)
    {
        self.parser.previous = self.parser.current.clone();

        loop
        {
            self.parser.current = Scan_Token(&mut self.scanner);
            if self.parser.current.token_type != TokenType::TokenError
            {
                break;
            }
            self.error_at_current(self.parser.current.start.clone());
        }
    }

    fn consume(&mut self, token_type: TokenType, message: String)
    {
        if self.parser.current.token_type == token_type
        {
            self.advance();
            return;
        }
        self.error_at_current(message);
    }

    fn match_token(&mut self, token_type: TokenType) -> bool
    {
        if self.parser.current.token_type != token_type
        {
            return false;
        }
            
        self.advance();
        return true;
    }

    fn grouping(&mut self)
    {
        self.expression();
        self.consume(TokenType::TokenRightParen, "Expect ')' after expression.".to_string());
    }

    fn binary(&mut self)
    {
        let operator_type = self.parser.previous.token_type;
        let rule = get_rule(operator_type);
        self.parse_precedence(Precedence::get_precedence(rule.precedence as u8 + 1).unwrap());

        match operator_type
        {
            x if x == TokenType::TokenPlus => self.emit_byte(OpCode::OpAdd as u8),
            x if x == TokenType::TokenMinus => self.emit_byte(OpCode::OpSubtract as u8),
            x if x == TokenType::TokenStar => self.emit_byte(OpCode::OpMultiply as u8),
            x if x == TokenType::TokenSlash => self.emit_byte(OpCode::OpDivide as u8),
            x if x == TokenType::TokenBangEqual => self.emit_bytes(OpCode::OpEqual as u8, OpCode::OpNot as u8),
            x if x == TokenType::TokenEqualEqual => self.emit_byte(OpCode::OpEqual as u8),
            x if x == TokenType::TokenGreater => self.emit_byte(OpCode::OpGreater as u8),
            x if x == TokenType::TokenGreaterEqual => self.emit_bytes(OpCode::OpLess as u8, OpCode::OpNot as u8),
            x if x == TokenType::TokenLess => self.emit_byte(OpCode::OpLess as u8),
            x if x == TokenType::TokenLessEqual => self.emit_bytes(OpCode::OpGreater as u8, OpCode::OpNot as u8),
            _ => return,
        }
    }

    fn literal(&mut self)
    {
        let operator_type = self.parser.previous.token_type;
        match operator_type
        {
            x if x == TokenType::TokenFalse => self.emit_byte(OpCode::OpFalse as u8),
            x if x == TokenType::TokenTrue => self.emit_byte(OpCode::OpTrue as u8),
            x if x == TokenType::TokenNil => self.emit_byte(OpCode::OpNil as u8),
            _ => return,
        }
    }

    fn expression(&mut self)
    {
        self.parse_precedence(Precedence::PrecAssignment);
    }

    fn print_statement(&mut self)
    {
        self.expression();
        self.consume(TokenType::TokenSemicolon, "Expect ';' after value.".to_owned());
        self.emit_byte(OpCode::OpPrint as u8)
    }

    fn declaration(&mut self)
    {
        self.statement();
    }

    fn statement(&mut self)
    {
        if self.match_token(TokenType::TokenPrint)
        {
            self.print_statement();
        }
    }

    
    fn number(&mut self)
    {
        let value: f64 = self.parser.previous.start.replace(" ", "").parse().unwrap();
        self.emit_constant(crate::value::NumberAsValue(value));
    }

    fn string(&mut self)
    {
        let val = value::ObjAsValue(
            Obj::CopyString(
                &mut self.vm,
                self.parser.previous.start.clone()[(1 as usize)..((self.parser.previous.length-1) as usize)].to_string(),
                self.parser.previous.length-2)
        );
        self.emit_constant(val);
    }

    fn unary(&mut self)
    {
        let operator_type: TokenType = self.parser.previous.token_type;

        self.parse_precedence(Precedence::PrecUnary);

        match operator_type
        {
            x if x == TokenType::TokenMinus => self.emit_byte(OpCode::OpNegate as u8),
            x if x == TokenType::TokenBang => self.emit_byte(OpCode::OpNot as u8),
            _ => 
            {
                return;
            }
        }
    }


    fn parse_precedence(&mut self, precedence: Precedence)
    {
        self.advance();
        let prefixRule = get_rule(self.parser.previous.token_type).prefix;

        match prefixRule
        {
            Some(prefix) => prefix(self),
            None => self.error("Expect expression".to_owned())
        }

        while precedence as u8 <= get_rule(self.parser.current.token_type).precedence as u8
        {
            self.advance();
            let infixRule = get_rule(self.parser.previous.token_type).infix;
            match infixRule
            {
                Some(infix) => infix(self),
                None => (),
            }
        }
    }

    fn make_constant(&mut self, value: Value) -> u8
    {
        let constant = add_constant(&mut self.current_chunk, value);
        if constant > 255
        {
            self.error("Too many constants in one chunk.".to_string());
            return 0;
        }
        let constant_byte: u8 = constant.try_into().unwrap();
        return constant_byte;
    }

    fn emit_byte(&mut self, byte: u8)
    {
        write_chunk(&mut self.current_chunk, byte, self.parser.previous.line)
    }

    fn emit_bytes(&mut self, byte1: u8, byte2: u8)
    {
        self.emit_byte(byte1);
        self.emit_byte(byte2);
    }

    fn emit_return(&mut self)
    {
        self.emit_byte(OpCode::OpReturn as u8)
    }



    fn emit_constant(&mut self, value: Value)
    {
        let byte_constant = self.make_constant(value);
        self.emit_bytes(OpCode::OpConstant as u8, byte_constant);
    }

    fn end_compiler(&mut self)
    {
        self.emit_return();
        if debug_print_code
        {
            if self.parser.had_error
            {
                disassemble_chunk(self.current_chunk, "code".to_owned())
            }
        }
    }

    fn error_at_current(&mut self, message: String)
    {
        self.error_at(&self.parser.current.clone(), message);
    }

    fn error(&mut self, message: String)
    {
        self.error_at(&self.parser.previous.clone(), message);
    }

    fn error_at(&mut self, token: &Token, message: String)
    {
        if self.parser.panic_mode
        {
            return;
        }
        self.parser.panic_mode = true;
        print!("[line {}] Error", token.line);
        if token.token_type == TokenType::TokenEof
        {
            print!(" at end");
        }
        else if token.token_type == TokenType::TokenError
        {
            
        }
        else
        {
            print!(" at {}", token.start)
        }
        print!(": {}\n", message);
        self.parser.had_error = true;
    }
}

type ParserFn = fn(&mut Compiler) -> ();

#[derive(Copy, Clone)]
pub struct ParseRule
{
    prefix: Option<ParserFn>,
    infix: Option<ParserFn>,
    precedence: Precedence
}

fn get_rule(token_type: TokenType) -> ParseRule
{
    parse_rules[token_type as usize]
}

static parse_rules : [ParseRule; 40] = [
    ParseRule {prefix: Some(|compiler| compiler.grouping()), infix: None, precedence: Precedence::PrecNone}, //'('
    ParseRule {prefix: None, infix: None, precedence: Precedence::PrecNone}, // ')'
    ParseRule {prefix: None, infix: None, precedence: Precedence::PrecNone}, // '{'
    ParseRule {prefix: None, infix: None, precedence: Precedence::PrecNone}, // '}'
    ParseRule {prefix: None, infix: None, precedence: Precedence::PrecNone}, // ','
    ParseRule {prefix: None, infix: None, precedence: Precedence::PrecNone}, // '.'
    ParseRule {prefix: Some(|compiler | compiler.unary()), infix: Some(|compiler| compiler.binary()), precedence: Precedence::PrecTerm}, // '-'
    ParseRule {prefix: None, infix: Some(|compiler| compiler.binary()), precedence: Precedence::PrecTerm}, // '+'
    ParseRule {prefix: None, infix: None, precedence: Precedence::PrecNone}, // ';'
    ParseRule {prefix: None, infix: Some(|compiler| compiler.binary()), precedence: Precedence::PrecFactor}, // '/'
    ParseRule {prefix: None, infix: Some(|compiler| compiler.binary()), precedence: Precedence::PrecFactor}, // '*'
    ParseRule {prefix: Some(|compiler | compiler.unary()), infix: None, precedence: Precedence::PrecNone}, // '!'
    ParseRule {prefix: None, infix: Some(|compiler| compiler.binary()), precedence: Precedence::PrecEquality}, // '!='
    ParseRule {prefix: None, infix: None, precedence: Precedence::PrecNone}, // '='
    ParseRule {prefix: None, infix: Some(|compiler| compiler.binary()), precedence: Precedence::PrecComparison}, // '=='
    ParseRule {prefix: None, infix: Some(|compiler| compiler.binary()), precedence: Precedence::PrecComparison}, // '>'
    ParseRule {prefix: None, infix: Some(|compiler| compiler.binary()), precedence: Precedence::PrecComparison}, // '>='
    ParseRule {prefix: None, infix: Some(|compiler| compiler.binary()), precedence: Precedence::PrecComparison}, // '<'
    ParseRule {prefix: None, infix: Some(|compiler| compiler.binary()), precedence: Precedence::PrecComparison}, // '<='
    ParseRule {prefix: None, infix: None, precedence: Precedence::PrecNone}, // 'identifier'
    ParseRule {prefix: Some(|compiler| compiler.string()), infix: None, precedence: Precedence::PrecNone}, // 'string'
    ParseRule {prefix: Some(|compiler| compiler.number()), infix: None, precedence: Precedence::PrecNone}, // 'number'
    ParseRule {prefix: None, infix: None, precedence: Precedence::PrecNone}, // 'and'
    ParseRule {prefix: None, infix: None, precedence: Precedence::PrecNone}, // 'class'
    ParseRule {prefix: None, infix: None, precedence: Precedence::PrecNone}, // 'else'
    ParseRule {prefix: Some(|compiler|compiler.literal()), infix: None, precedence: Precedence::PrecNone}, // 'false'
    ParseRule {prefix: None, infix: None, precedence: Precedence::PrecNone}, // 'for'
    ParseRule {prefix: None, infix: None, precedence: Precedence::PrecNone}, // 'fun'
    ParseRule {prefix: None, infix: None, precedence: Precedence::PrecNone}, // 'if'
    ParseRule {prefix: Some(|compiler|compiler.literal()), infix: None, precedence: Precedence::PrecNone}, // 'nil'
    ParseRule {prefix: None, infix: None, precedence: Precedence::PrecNone}, // 'or'
    ParseRule {prefix: Some(|compiler|compiler.expression()), infix: None, precedence: Precedence::PrecNone}, // 'print'
    ParseRule {prefix: None, infix: None, precedence: Precedence::PrecNone}, // 'return'
    ParseRule {prefix: None, infix: None, precedence: Precedence::PrecNone}, // 'super'
    ParseRule {prefix: None, infix: None, precedence: Precedence::PrecNone}, // 'this'
    ParseRule {prefix: Some(|compiler|compiler.literal()), infix: None, precedence: Precedence::PrecNone}, // 'true'
    ParseRule {prefix: None, infix: None, precedence: Precedence::PrecNone}, // 'var'
    ParseRule {prefix: None, infix: None, precedence: Precedence::PrecNone}, // 'while'
    ParseRule {prefix: None, infix: None, precedence: Precedence::PrecNone}, // 'error'
    ParseRule {prefix: None, infix: None, precedence: Precedence::PrecNone}, // 'eof'
];


