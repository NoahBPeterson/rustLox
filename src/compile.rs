use crate::chunk::Chunk;
use crate::scanner;

pub fn compile(source: &String, chunk: &Chunk) -> bool
{
    let mut Scanner = scanner::Init_Scanner(source);
    scanner::advance(&mut Scanner);
    //expression();
    //consume(scanner::TokenType::TokenEof, "Expect end of expression");
    return true;
}