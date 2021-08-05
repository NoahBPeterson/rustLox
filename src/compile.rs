use crate::scanner;

pub fn compile(source: &String)
{
    let mut Scanner = scanner::Init_Scanner(source);
    let mut line: u32 = 2147000000;
    loop
    {
        let token: scanner::Token = scanner::Scan_Token(&mut Scanner);
        if token.line != line
        {
            print!("{:04}", token.line);
            line = token.line;
        }
        else
        {
            print!("   | ");
        }
        print!("{:02} ", token.token_type as u32);
        println!("{}", token.start);

        if token.token_type as u32 == scanner::TokenType::TokenEof as u32
        {
            break;}
    }
}