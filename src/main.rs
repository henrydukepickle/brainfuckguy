use crate::brainfuck::start;

mod brainfuck;
mod commands;
mod io;
mod parse;
fn main() {
    dbg!(start(&vec![
        1, 6, 2, 3, 4, 5, 7, 8, 2, 6, 8, 12, 123, 6, 34, 225, 21, 12, 56
    ]));
}
