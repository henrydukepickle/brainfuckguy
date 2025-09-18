use crate::{brainfuck::brainfuck_from_string, io::read_file_to_string};

mod brainfuck;
mod commands;
mod io;
fn main() {
    let bf = read_file_to_string(&String::from("Scripts/test.bf"));
    let f = |args| brainfuck_from_string(bf, args);
    f(&vec![10, 5])
}
