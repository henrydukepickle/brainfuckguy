use crate::{brainfuck::start, io::read_file_to_string};

mod brainfuck;
mod commands;
mod io;
mod parse;
fn main() {
    dbg!(start(&vec![0, 123]));
}
