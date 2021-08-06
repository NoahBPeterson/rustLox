#[derive(Clone)]
pub struct Scanner
{
    source: String,
    start: u32,
    current: u32,
    line: u32,
}

pub struct Token
{
    pub token_type: TokenType,
    pub start: String,
    pub length: u32,
    pub line: u32
}

pub fn Init_Scanner(source: &String) -> Scanner
{
    return Scanner {source: source.clone(), start: 0, current: 0, line: 1};
}

pub fn Scan_Token(scanner: &mut Scanner) -> Token
{
    scanner.start = scanner.current;
    Skip_Whitespace(scanner);
    if scanner.source.chars().nth(scanner.current as usize).unwrap().eq(&'\0')
    {
        return Make_Token(TokenType::TokenEof, &scanner);
    }

    let character: String = advance(scanner);

    match character
    {
        x if x == "(" => return Make_Token(TokenType::TokenLeftParen, scanner),
        x if x == ")" => return Make_Token(TokenType::TokenRightParen, scanner),
        x if x == "{" => return Make_Token(TokenType::TokenLeftBrace, scanner),
        x if x == "}" => return Make_Token(TokenType::TokenRightBrace, scanner),
        x if x == ";" => return Make_Token(TokenType::TokenSemicolon, scanner),
        x if x == "," => return Make_Token(TokenType::TokenComma, scanner),
        x if x == "." => return Make_Token(TokenType::TokenDot, scanner),
        x if x == "-" => return Make_Token(TokenType::TokenMinus, scanner),
        x if x == "+" => return Make_Token(TokenType::TokenPlus, scanner),
        x if x == "/" => return Make_Token(TokenType::TokenSlash, scanner),
        x if x == "*" => return Make_Token(TokenType::TokenStar, scanner),
        x if x == "!" => 
        {
            if matchCharacter("=".to_string(), scanner)
            {
                return Make_Token(TokenType::TokenBangEqual, scanner);
            }
            return Make_Token(TokenType::TokenBang, scanner);
        }
        x if x == "=" => 
        {
            if matchCharacter("=".to_string(), scanner)
            {
                return Make_Token(TokenType::TokenEqualEqual, scanner);
            }
            return Make_Token(TokenType::TokenEqual, scanner);
        }
        x if x == "<" => 
        {
            if matchCharacter("=".to_string(), scanner)
            {
                return Make_Token(TokenType::TokenLessEqual, scanner);
            }
            return Make_Token(TokenType::TokenLess, scanner);
        }
        x if x == ">" => 
        {
            if matchCharacter("=".to_string(), scanner)
            {
                return Make_Token(TokenType::TokenGreaterEqual, scanner);
            }
            return Make_Token(TokenType::TokenGreater, scanner);
        }
        _ => return Error_Token(&"Unexpected character".to_string(), scanner),
    }
}

fn Skip_Whitespace(scanner: &mut Scanner)
{
    loop
    {
        let character: String = peek(scanner);
        match character
        {
            x if x == " " => advance(scanner),
            x if x == '\r'.to_string() => advance(scanner),
            x if x == '\t'.to_string() => advance(scanner),
            x if x == '\n'.to_string() => 
            {
                scanner.line = scanner.line + 1;
                advance(scanner);
                break;
            }
            _ => break
        };
    }
}

fn peek(scanner: &mut Scanner) -> String
{
    return " ".to_string();
}

fn matchCharacter(expectedString: String, scanner: &mut Scanner) -> bool
{
    if scanner.source.chars().nth(scanner.current as usize).unwrap().eq(&'\0')
    {
        return false;
    }
    if scanner.source.chars().nth(scanner.current as usize).unwrap().eq(&expectedString.chars().next().unwrap())
    {
        return false;
    }
    scanner.current = scanner.current + 1;
    return true;
}

fn advance(scanner: &mut Scanner) -> String
{
    scanner.current = scanner.current + 1;
    return scanner.source.clone()[(scanner.current - 1) as usize..scanner.current as usize].to_string();
}

pub fn Make_Token(tokenType: TokenType, scanner: &Scanner) -> Token
{
    return Token
    {
        token_type: tokenType,
        start: scanner.source.clone()[scanner.start as usize..scanner.current as usize].to_string(),
        length: (scanner.current - scanner.start),
        line: scanner.line
    };
}

pub fn Error_Token(error_message: &String, scanner: &Scanner) -> Token
{
    return Token
    {
        token_type: TokenType::TokenError,
        start: error_message.clone(),
        length: error_message.clone().len() as u32,
        line: scanner.line
    }
}

#[derive(Copy, Clone)]
pub enum TokenType
{
    // Single-character tokens.
    TokenLeftParen, TokenRightParen,
    TokenLeftBrace, TokenRightBrace,
    TokenComma, TokenDot, TokenMinus, TokenPlus,
    TokenSemicolon, TokenSlash, TokenStar,
    // One or two character tokens.
    TokenBang, TokenBangEqual,
    TokenEqual, TokenEqualEqual,
    TokenGreater, TokenGreaterEqual,
    TokenLess, TokenLessEqual,
    // Literals.
    TokenIdentifier, TokenString, TokenNumber,
    // Keywords.
    TokenAnd, TokenClass, TokenElse, TokenFalse,
    TokenFor, TokenFun, TokenIf, TokenNil, TokenOr,
    TokenPrint, TokenReturn, TokenSuper, TokenThis,
    TokenTrue, TokenVar, TokenWhile,

    TokenError, TokenEof
}