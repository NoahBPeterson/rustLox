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
    if get_character_from_scanner_source(scanner.current, scanner).eq(&'\0')
    {
        return Make_Token(TokenType::TokenEof, &scanner);
    }

    let character = advance(scanner);

    match character
    {
        x if x == '(' => return Make_Token(TokenType::TokenLeftParen, scanner),
        x if x == ')' => return Make_Token(TokenType::TokenRightParen, scanner),
        x if x == '{' => return Make_Token(TokenType::TokenLeftBrace, scanner),
        x if x == '}' => return Make_Token(TokenType::TokenRightBrace, scanner),
        x if x == ';' => return Make_Token(TokenType::TokenSemicolon, scanner),
        x if x == ',' => return Make_Token(TokenType::TokenComma, scanner),
        x if x == '.' => return Make_Token(TokenType::TokenDot, scanner),
        x if x == '-' => return Make_Token(TokenType::TokenMinus, scanner),
        x if x == '+' => return Make_Token(TokenType::TokenPlus, scanner),
        x if x == '/' => return Make_Token(TokenType::TokenSlash, scanner),
        x if x == '*' => return Make_Token(TokenType::TokenStar, scanner),
        x if x == '!' => 
        {
            if matchCharacter('=', scanner)
            {
                return Make_Token(TokenType::TokenBangEqual, scanner);
            }
            return Make_Token(TokenType::TokenBang, scanner);
        }
        x if x == '=' => 
        {
            if matchCharacter('=', scanner)
            {
                return Make_Token(TokenType::TokenEqualEqual, scanner);
            }
            return Make_Token(TokenType::TokenEqual, scanner);
        }
        x if x == '<' => 
        {
            if matchCharacter('=', scanner)
            {
                return Make_Token(TokenType::TokenLessEqual, scanner);
            }
            return Make_Token(TokenType::TokenLess, scanner);
        }
        x if x == '>' => 
        {
            if matchCharacter('=', scanner)
            {
                return Make_Token(TokenType::TokenGreaterEqual, scanner);
            }
            return Make_Token(TokenType::TokenGreater, scanner);
        }
        x if x == '"' => return string_token(scanner),
        x if x.is_ascii_digit() => return digit(scanner),
        x if x.is_alphanumeric() => return identifier(scanner),
        _ => return Error_Token(&"Unexpected character".to_string(), scanner),
    }
}

fn identifier(scanner: &mut Scanner) -> Token
{
    loop
    {
        if peek(scanner).is_alphanumeric() || peek(scanner).is_ascii_digit()
        {
            advance(scanner);
        }
        else
        {
            return Make_Token(identifier_type(scanner), scanner);
        }
    }
}

fn identifier_type(scanner: &mut Scanner) -> TokenType
{
    match get_character_from_scanner_source(scanner.current, scanner)
    {
        x if x.eq(&'a') => return check_keyword(1, 2, "nd".to_string(), TokenType::TokenAnd, scanner),
        x if x.eq(&'c') => return check_keyword(1, 4, "lass".to_string(), TokenType::TokenClass, scanner),
        x if x.eq(&'e') => return check_keyword(1, 3, "lse".to_string(), TokenType::TokenElse, scanner),
        x if x.eq(&'i') => return check_keyword(1, 1, "f".to_string(), TokenType::TokenIf, scanner),
        x if x.eq(&'n') => return check_keyword(1, 2, "il".to_string(), TokenType::TokenNil, scanner),
        x if x.eq(&'o') => return check_keyword(1, 1, "r".to_string(), TokenType::TokenOr, scanner),
        x if x.eq(&'p') => return check_keyword(1, 4, "rint".to_string(), TokenType::TokenPrint, scanner),
        x if x.eq(&'r') => return check_keyword(1, 5, "eturn".to_string(), TokenType::TokenReturn, scanner),
        x if x.eq(&'s') => return check_keyword(1, 4, "uper".to_string(), TokenType::TokenSuper, scanner),
        x if x.eq(&'v') => return check_keyword(1, 2, "ar".to_string(), TokenType::TokenVar, scanner),
        x if x.eq(&'w') => return check_keyword(1, 4, "hile".to_string(), TokenType::TokenWhile, scanner),
        x if x.eq(&'f') =>
        {
            if (scanner.current - scanner.start) > 1
            {
                match get_character_from_scanner_source(scanner.start + 1, scanner)
                {
                    x if x.eq(&'a') => return check_keyword(1, 3, "lse".to_string(), TokenType::TokenFalse, scanner),
                    x if x.eq(&'o') => return check_keyword(1, 1, "r".to_string(), TokenType::TokenFor, scanner),
                    x if x.eq(&'u') => return check_keyword(1, 1, "n".to_string(), TokenType::TokenFun, scanner),
                    _ => return TokenType::TokenIdentifier
                }
            }
        }
        x  if x.eq(&'t') =>
        {
            if (scanner.current - scanner.start) > 1
            {
                match get_character_from_scanner_source(scanner.start + 1, scanner)
                {
                    x if x.eq(&'h') => return check_keyword(1, 2, "is".to_string(), TokenType::TokenThis, scanner),
                    x if x.eq(&'r') => return check_keyword(1, 3, "ue".to_string(), TokenType::TokenFalse, scanner),
                    _ => return TokenType::TokenIdentifier
                }
            }
        }
        _ => return TokenType::TokenIdentifier
    }
    return TokenType::TokenIdentifier;
}

fn check_keyword(start: u32, length: u32, the_rest: String, token: TokenType, scanner: &mut Scanner) -> TokenType
{
    if scanner.source.clone()[scanner.start as usize..(scanner.start + length) as usize].to_string().eq(&the_rest)
    {
        return token;
    }

    return TokenType::TokenIdentifier;
}

fn digit(scanner: &mut Scanner) -> Token
{
    consume_digits(scanner);

    if get_character_from_scanner_source(scanner.current, scanner).eq(&'.')
    {
        advance(scanner);
        consume_digits(scanner);
    }

    return Make_Token(TokenType::TokenNumber, scanner);
}

fn consume_digits(scanner: &mut Scanner)
{
    loop
    {
        if get_character_from_scanner_source(scanner.current, scanner).is_ascii_digit()
        {
            advance(scanner);
        }
        else
        {
            return;
        }
    }
}

fn string_token(scanner: &mut Scanner) -> Token
{
    loop
    {
        if peek(scanner).ne(&'"') && !isAtEnd(scanner)
        {
            if peek(scanner).eq(&'\n')
            {
                scanner.line = scanner.line + 1;
            }
            advance(scanner);
        }
        else
        {
            break;
        }
    }
    if isAtEnd(scanner)
    {
        return Error_Token(&"Unterminated string.".to_string(), scanner);
    }

    advance(scanner); // The closing quote.
    return Make_Token(TokenType::TokenString, scanner);
}

fn Skip_Whitespace(scanner: &mut Scanner)
{
    loop
    {
        let character = peek(scanner);
        match character
        {
            x if x == ' ' => advance(scanner),
            x if x == '\r' => advance(scanner),
            x if x == '\t' => advance(scanner),
            x if x == '\n' => 
            {
                scanner.line = scanner.line + 1;
                advance(scanner);
                break;
            }
            x if x == '/' =>
            {
                if peekNext(scanner).eq(&'/') && !isAtEnd(scanner)
                {
                    loop
                    {
                        if peek(scanner).ne(&'\n') && !isAtEnd(scanner)
                        {
                            advance(scanner);
                        }else
                        {
                            break;
                        }
                    }
                }
                break;
            }
            _ => break
        };
    }
}

fn peek(scanner: &mut Scanner) -> char
{
    return get_character_from_scanner_source(scanner.current, &scanner);
}

fn isAtEnd(scanner: &Scanner) -> bool
{
    return get_character_from_scanner_source(scanner.current, scanner).eq(&'\0');
}

fn peekNext(scanner: &Scanner) -> char
{
    if (isAtEnd(scanner))
    {
        return '\0';
    }
    return get_character_from_scanner_source(scanner.current + 1, scanner);
}

fn get_character_from_scanner_source(location: u32, scanner: &Scanner) -> char
{
    return scanner.source.chars().nth(location as usize).unwrap();
}

fn matchCharacter(expected_string: char, scanner: &mut Scanner) -> bool
{
    if get_character_from_scanner_source(scanner.current, scanner).eq(&'\0')
    {
        return false;
    }
    if get_character_from_scanner_source(scanner.current, scanner).eq(&expected_string)
    {
        return false;
    }
    scanner.current = scanner.current + 1;
    return true;
}

fn advance(scanner: &mut Scanner) -> char
{
    scanner.current = scanner.current + 1;
    return scanner.source.clone()[(scanner.current - 1) as usize..scanner.current as usize].chars().next().unwrap();
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