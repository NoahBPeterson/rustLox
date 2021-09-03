use std::convert::TryInto;

use crate::chunk::{Chunk, OpCode, add_constant, init_chunk, write_chunk};
use crate::scanner::{self, Init_Scanner, Make_Token, Scan_Token, Scanner, Token, TokenType};

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

#[derive(Copy, Clone)]
pub enum Precedence
{
    PrecNone,
    PrecAssignment,
    PrecOr,
    PrecAnd,
    PrecEquality,
    PrecComparison,
    PrecTerm,
    PrecUnary,
    PrecCall,
    PrecPrimary
}

#[derive(Copy, Clone)]
pub struct ParseRule
{
    prefix: Option<ParserFn>,
    infix: Option<ParserFn>,
    precedence: Precedence
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

pub struct Compiler<'a>
{
    scanner: Scanner,
    parser: Parser,
    current_chunk: &'a mut Chunk
}

impl Compiler<'_>
{
    pub fn new_compiler(chunk: &mut Chunk) -> Compiler
    {
        Compiler
        {
            scanner: scanner::Init_Scanner(&"".to_string()),
            parser: Parser::new(),
            current_chunk: chunk
        }
    }
    
    pub fn compile(&mut self, source: &String) -> bool
    {
        self.scanner = scanner::Init_Scanner(source);
        self.parser = init_parser(&mut self.scanner, self.current_chunk);
        let mut compiling_chunk: Chunk = init_chunk();
        self.advance();
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

    fn grouping(&mut self)
    {
        self.expression();
        self.consume(TokenType::TokenRightParen, "Expect ')' after expression.".to_string());
    }

    fn binary(&mut self)
    {
        let operator_type = self.parser.previous.token_type;
        let rule = self.get_rule(operator_type);
        self.parse_precedence(rule.precedence + 1);

        match operator_type
        {
            x if x == TokenType::TokenPlus =>
            {
                self.emit_byte(OpCode::OpAdd as u8);
            }
            x if x == TokenType::TokenMinus =>
            {
                self.emit_byte(OpCode::OpSubtract as u8);
            }
            x if x == TokenType::TokenStar =>
            {
                self.emit_byte(OpCode::OpMultiply as u8);
            }
            x if x == TokenType::TokenSlash =>
            {
                self.emit_byte(OpCode::OpDivide as u8);
            }
            _ => return,
        }
    }

    fn expression(&mut self)
    {
        self.parse_precedence(Precedence::PrecAssignment);
    }

    fn number(&mut self, number: String)
    {
        let value: f64 = number.parse().unwrap();
        self.emit_constant(value);
    }

    fn unary(&mut self)
    {
        let operator_type: TokenType = self.parser.previous.token_type;

        self.parse_precedence(Precedence::PrecUnary);

        match operator_type
        {
            x if x == TokenType::TokenMinus =>
            {
                self.emit_byte(OpCode::OpNegate as u8);
            }
            _ => 
            {
                return;
            }
        }
    }


    fn parse_precedence(&mut self, precedence: Precedence)
    {
        self.advance();
    }

    fn make_constant(&mut self, value: f64) -> u8
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
        self.emit_byte(TokenType::TokenReturn as u8)
    }



    fn emit_constant(&mut self, value: f64)
    {
        self.emit_bytes(OpCode::OpConstant as u8, self.make_constant(value));
    }

    fn end_compiler(&mut self)
    {
        self.emit_return();
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



/*fn get_rule(token_type: TokenType) -> ParseRule
{
    let rules: Vec<ParseRule> = Vec::with_capacity(0);
    let array: [fn(); fn(); Precedence] = 
    rules.push(ParseRule {prefix: grouping, precedence: Precedence::PrecNone, infix: () });
    let ParseRule rules[] = {
        [TokenType::TokenLeftParen]    = {grouping, NULL,   PREC_NONE},
        [TOKEN_RIGHT_PAREN]   = {NULL,     NULL,   PREC_NONE},
        [TOKEN_LEFT_BRACE]    = {NULL,     NULL,   PREC_NONE}, 
        [TOKEN_RIGHT_BRACE]   = {NULL,     NULL,   PREC_NONE},
        [TOKEN_COMMA]         = {NULL,     NULL,   PREC_NONE},
        [TOKEN_DOT]           = {NULL,     NULL,   PREC_NONE},
        [TOKEN_MINUS]         = {unary,    binary, PREC_TERM},
        [TOKEN_PLUS]          = {NULL,     binary, PREC_TERM},
        [TOKEN_SEMICOLON]     = {NULL,     NULL,   PREC_NONE},
        [TOKEN_SLASH]         = {NULL,     binary, PREC_FACTOR},
        [TOKEN_STAR]          = {NULL,     binary, PREC_FACTOR},
        [TOKEN_BANG]          = {NULL,     NULL,   PREC_NONE},
        [TOKEN_BANG_EQUAL]    = {NULL,     NULL,   PREC_NONE},
        [TOKEN_EQUAL]         = {NULL,     NULL,   PREC_NONE},
        [TOKEN_EQUAL_EQUAL]   = {NULL,     NULL,   PREC_NONE},
        [TOKEN_GREATER]       = {NULL,     NULL,   PREC_NONE},
        [TOKEN_GREATER_EQUAL] = {NULL,     NULL,   PREC_NONE},
        [TOKEN_LESS]          = {NULL,     NULL,   PREC_NONE},
        [TOKEN_LESS_EQUAL]    = {NULL,     NULL,   PREC_NONE},
        [TOKEN_IDENTIFIER]    = {NULL,     NULL,   PREC_NONE},
        [TOKEN_STRING]        = {NULL,     NULL,   PREC_NONE},
        [TOKEN_NUMBER]        = {number,   NULL,   PREC_NONE},
        [TOKEN_AND]           = {NULL,     NULL,   PREC_NONE},
        [TOKEN_CLASS]         = {NULL,     NULL,   PREC_NONE},
        [TOKEN_ELSE]          = {NULL,     NULL,   PREC_NONE},
        [TOKEN_FALSE]         = {NULL,     NULL,   PREC_NONE},
        [TOKEN_FOR]           = {NULL,     NULL,   PREC_NONE},
        [TOKEN_FUN]           = {NULL,     NULL,   PREC_NONE},
        [TOKEN_IF]            = {NULL,     NULL,   PREC_NONE},
        [TOKEN_NIL]           = {NULL,     NULL,   PREC_NONE},
        [TOKEN_OR]            = {NULL,     NULL,   PREC_NONE},
        [TOKEN_PRINT]         = {NULL,     NULL,   PREC_NONE},
        [TOKEN_RETURN]        = {NULL,     NULL,   PREC_NONE},
        [TOKEN_SUPER]         = {NULL,     NULL,   PREC_NONE},
        [TOKEN_THIS]          = {NULL,     NULL,   PREC_NONE},
        [TOKEN_TRUE]          = {NULL,     NULL,   PREC_NONE},
        [TOKEN_VAR]           = {NULL,     NULL,   PREC_NONE},
        [TOKEN_WHILE]         = {NULL,     NULL,   PREC_NONE},
        [TOKEN_ERROR]         = {NULL,     NULL,   PREC_NONE},
        [TOKEN_EOF]           = {NULL,     NULL,   PREC_NONE},
      };
    return 
}*/


