use crate::instructions::parse;

mod cpu;
mod register;
mod instructions;

fn main() {
    let (instructor, instruction) = parse(0x003100b3);
    dbg!(instructor, instruction);
    println!("Hello, world!");
}
