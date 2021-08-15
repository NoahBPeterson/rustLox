use std::convert::TryInto;
use std::iter::Scan;

use crate::chunk::{Chunk, OpCode, add_constant, init_chunk, write_chunk};
use crate::scanner::{self, Make_Token, Scan_Token, Scanner, Token, TokenType};

#[derive(Clone)]
pub struct Parser
{
    current: Token,
    previous: Token,
    had_error: bool,
    panic_mode: bool,
    compiling_chunk: Chunk
}

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

pub struct ParseRule
{
    prefix: ParseFn,
    infix: ParseFn,
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
        compiling_chunk: chunk.clone()
    };

}

pub fn compile(source: &String, chunk: &Chunk) -> bool
{
    let mut scanner = scanner::Init_Scanner(source);
    let mut parser = init_parser(&mut scanner, chunk);
    let mut compiling_chunk: Chunk = init_chunk();
    advance(&mut parser, &mut scanner);
    end_compiler(&mut parser);
    return !parser.had_error;
}

fn advance(parser: &mut Parser, scanner: &mut Scanner)
{
    parser.previous = parser.current.clone();

    loop
    {
        parser.current = Scan_Token(scanner);
        if parser.current.token_type != TokenType::TokenError
        {
            break;
        }
        error_at_current(parser.current.start.clone(), parser);
    }
}

fn consume(parser: &mut Parser, scanner: &mut Scanner, token_type: TokenType, message: String)
{
    if parser.current.token_type == token_type
    {
        advance(parser, scanner);
        return;
    }
    error_at_current(message, parser);
}

fn emit_byte(byte: u8, parser: &mut Parser)
{
    write_chunk(&mut parser.compiling_chunk, byte, parser.previous.line)
}

fn emit_bytes(byte1: u8, byte2: u8, parser: &mut Parser)
{
    emit_byte(byte1, parser);
    emit_byte(byte2, parser);
}

fn end_compiler(parser: &mut Parser)
{
    emit_return(parser);
}

fn binary(parser: &mut Parser)
{
    let operator_type = parser.previous.token_type;
    let rule = get_rule(operator_type);
    parse_precedence(rule.precedence + 1);

    match operator_type
    {
        x if x == TokenType::TokenPlus =>
        {
            emit_byte(OpCode::OpAdd as u8, parser);
        }
        x if x == TokenType::TokenMinus =>
        {
            emit_byte(OpCode::OpSubtract as u8, parser);
        }
        x if x == TokenType::TokenStar =>
        {
            emit_byte(OpCode::OpMultiply as u8, parser);
        }
        x if x == TokenType::TokenSlash =>
        {
            emit_byte(OpCode::OpDivide as u8, parser);
        }
        _ => return,
    }
}

fn grouping(parser: &mut Parser, scanner: &mut Scanner)
{
    expression();
    consume(parser, scanner, TokenType::TokenRightParen, "Expect ')' after expression.".to_string());
}

fn expression()
{
    parse_precedence(Precedence::PrecAssignment);
}

fn number(number: String, parser: &mut Parser)
{
    let value: f64 = number.parse().unwrap();
    emit_constant(value, parser);
}

fn unary(parser: &mut Parser)
{
    let operator_type: TokenType = parser.previous.token_type;

    parse_precedence(Precedence::PrecUnary);

    match operator_type
    {
        x if x == TokenType::TokenMinus =>
        {
            emit_byte(OpCode::OpNegate as u8, parser);
        }
        _ => 
        {
            return;
        }
    }
}

fn parse_precedence(precedence: Precedence)
{

}

fn get_rule(token_type: TokenType) -> ParseRule
{
    let rules: Vec<ParseRule> = Vec::with_capacity(0);
    rules.append(other)
    ParseRule rules[] = {
        [TOKEN_LEFT_PAREN]    = {grouping, NULL,   PREC_NONE},
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
}

fn emit_return(parser: &mut Parser)
{
    emit_byte(TokenType::TokenReturn as u8, parser)
}

fn make_constant(value: f64, parser: &mut Parser) -> u8
{
    let constant = add_constant(&mut parser.compiling_chunk, value);
    if constant > 255
    {
        error("Too many constants in one chunk.".to_string(), parser);
        return 0;
    }
    let constant_byte: u8 = constant.try_into().unwrap();
    return constant_byte;
}

fn emit_constant(value: f64, parser: &mut Parser)
{
    emit_bytes(OpCode::OpConstant as u8, make_constant(value, parser), parser);
}

fn error_at_current(message: String, parser: &mut Parser)
{
    error_at(&parser.current.clone(), message, parser);
}

fn error(message: String, parser: &mut Parser)
{
    error_at(&parser.previous.clone(), message, parser);
}

fn error_at(token: &Token, message: String, parser: &mut Parser)
{
    if parser.panic_mode
    {
        return;
    }
    parser.panic_mode = true;
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
    parser.had_error = true;
}
