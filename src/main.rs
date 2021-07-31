use crate::chunk::add_constant;
mod chunk;
mod debug;
mod value;
mod vm;

fn main()
{
    let mut vm = vm::init_vm();
    let mut chunk: chunk::Chunk = chunk::init_chunk();

    let constant: u32 = add_constant(&mut chunk, 1.2);
    chunk::write_chunk(&mut chunk, chunk::OpCode::OpConstant as u8, 123);
    chunk::write_chunk(&mut chunk, constant as u8, 123);

    chunk::write_chunk(&mut chunk, chunk::OpCode::OpReturn as u8, 123);

    debug::disassemble_chunk(&chunk, "test chunk".to_string());

    vm::interpret(chunk, &mut vm);
}

