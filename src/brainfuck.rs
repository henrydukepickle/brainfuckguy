use std::collections::HashMap;

use crate::{
    commands::{Command, FunctionCall, command_from_char},
    io::read_file_to_string,
};

struct InstructionPointer {
    position: usize,
    loop_starts: Vec<usize>,          //end is the most recent loop entered
    loop_ends: HashMap<usize, usize>, //loop_ends[n] is the endpoint of the loop with start point n
}

pub struct Interpreter {
    instruction_pointer: InstructionPointer,
    data_pointer: u16,
    cells: [u8; 65536],
    curr_arg: usize,
    ret: Vec<u8>,
}

impl Interpreter {
    fn run(&mut self, commands: &Vec<Command>, args: &Vec<u8>) {
        while self.instruction_pointer.position < commands.len() {
            let x = self.instruction_pointer.position;
            interpret_bf(
                &mut self.data_pointer,
                &mut self.cells,
                &mut self.instruction_pointer,
                &commands[x],
                &mut self.curr_arg,
                args,
                &mut self.ret,
            );
            self.instruction_pointer.position += 1;
        }
    }
    fn call_function(&mut self, call: &FunctionCall) {
        let mut func =
            brainfuck_from_string(read_file_to_string(&format!("Scripts/{}.bf", call.name)));
    }
    fn populate_loop_indices(&mut self, commands: &Vec<Command>) {
        let mut running_starts = Vec::new();
        for i in 0..commands.len() {
            if commands[i] == Command::LoopStart {
                running_starts.push(i);
            } else if commands[i] == Command::LoopEnd {
                self.instruction_pointer
                    .loop_ends
                    .insert(running_starts.pop().expect("Imbalanced Loops!"), i);
            }
        }
    }
}

pub fn interpret_bf(
    data_pointer: &mut u16,
    cells: &mut [u8; 65536],
    inst: &mut InstructionPointer,
    command: &Command,
    curr_arg: &mut usize,
    args: &Vec<u8>,
    ret: &mut Vec<u8>,
) -> Option<FunctionCall> {
    // returns the interact if passed
    match command {
        Command::MoveRight => *data_pointer += 1,
        Command::MoveLeft => *data_pointer -= 1,
        Command::Plus => cells[*data_pointer as usize] += 1,
        Command::Minus => cells[*data_pointer as usize] -= 1,
        Command::LoopStart => {
            if cells[*data_pointer as usize] == 0 {
                inst.position = inst.loop_ends[&inst.position]
            } else {
                inst.loop_starts.push(inst.position);
            }
        }
        Command::LoopEnd => {
            if cells[*data_pointer as usize] == 0 {
                inst.loop_starts.pop();
            } else {
                inst.position = *inst.loop_starts.last().unwrap();
            }
        }
        Command::In => {
            if *curr_arg < args.len() {
                cells[*data_pointer as usize] = args[*curr_arg];
                *curr_arg += 1
            }
        }
        Command::Out => ret.push(cells[*data_pointer as usize]),
        Command::Debug => println!("{}: {}", *data_pointer, cells[*data_pointer as usize]),
        Command::Call(f) => return Some(f.clone()),
        Command::Interact => {}
    }
    None
}

pub fn brainfuck_from_string(string: String) -> Interpreter {
    let mut int = Interpreter {
        instruction_pointer: InstructionPointer {
            position: 0,
            loop_starts: Vec::new(),
            loop_ends: HashMap::new(),
        },
        data_pointer: 0,
        cells: [0; 65536],
        curr_arg: 0,
        ret: Vec::new(),
    };
    let mut commands = Vec::new();
    for char in string.chars() {
        if let Some(x) = command_from_char(char) {
            commands.push(x)
        }
    }
    int.populate_loop_indices(&commands);
    int
}
