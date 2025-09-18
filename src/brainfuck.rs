use std::collections::HashMap;

use crate::{
    commands::{Command, FunctionCall},
    io::read_file_to_string,
    parse::brainfuck_from_string,
};

pub struct InstructionPointer {
    position: usize,
    loop_starts: Vec<usize>,          //end is the most recent loop entered
    loop_ends: HashMap<usize, usize>, //loop_ends[n] is the endpoint of the loop with start point n
}

pub struct Interpreter {
    data_pointer: u16,
    cells: [u8; 65536],
    curr_arg: usize,
    ret: Vec<u8>,
}

impl Interpreter {
    fn interpret(
        &mut self,
        command: &Command,
        args: &Vec<u8>,
        inst: &mut InstructionPointer,
    ) -> bool {
        //returns true if the command was an interact
        // returns the interact if passed
        match command {
            Command::MoveRight => self.data_pointer += 1,
            Command::MoveLeft => self.data_pointer -= 1,
            Command::Plus => self.cells[self.data_pointer as usize] += 1,
            Command::Minus => self.cells[self.data_pointer as usize] -= 1,
            Command::LoopStart => {
                if self.cells[self.data_pointer as usize] == 0 {
                    inst.position = inst.loop_ends[&inst.position]
                } else {
                    inst.loop_starts.push(inst.position);
                }
            }
            Command::LoopEnd => {
                if self.cells[self.data_pointer as usize] == 0 {
                    inst.loop_starts.pop();
                } else {
                    inst.position = *inst.loop_starts.last().unwrap();
                }
            }
            Command::In => {
                if self.curr_arg < args.len() {
                    self.cells[self.data_pointer as usize] = args[self.curr_arg];
                    self.curr_arg += 1
                }
            }
            Command::Out => self.ret.push(self.cells[self.data_pointer as usize]),
            Command::Debug => println!(
                "{}: {}",
                self.data_pointer, self.cells[self.data_pointer as usize]
            ),
            Command::Call(f) => self.call_function(&f, args),
            Command::Interact => return true,
        }
        false
    }
    fn run(&mut self, commands: &Vec<Command>, args: &Vec<u8>) -> Vec<u8> {
        let mut inst = new_pointer(commands);
        while inst.position < commands.len() {
            self.interpret(&commands[inst.position], args, &mut inst);
            inst.position += 1;
        }
        self.ret.clone()
    }
    fn call_function(&mut self, call: &FunctionCall, args: &Vec<u8>) {
        let comm =
            brainfuck_from_string(&read_file_to_string(&format!("Scripts/{}.bf", call.name)))
                .unwrap();
        let mut func = new_interpreter();
        let mut inst_in = new_pointer(&call.input);
        let mut input_args = Vec::new();
        loop {
            if inst_in.position >= call.input.len() {
                break;
            }
            if self.interpret(&call.input[inst_in.position], args, &mut inst_in) {
                input_args.push(self.cells[self.data_pointer as usize]);
                self.cells[self.data_pointer as usize] = 0;
            }
            inst_in.position += 1;
        }
        let func_out = func.run(&comm, &input_args);
        let mut out_ind = 0;
        let mut inst_out = new_pointer(&call.output);
        loop {
            if inst_out.position >= call.output.len() {
                break;
            }
            if self.interpret(&call.output[inst_out.position], args, &mut inst_out) {
                if out_ind < func_out.len() {
                    self.cells[self.data_pointer as usize] = func_out[out_ind];
                    out_ind += 1;
                }
            }
            inst_out.position += 1;
        }
    }
}

fn populate_loop_indices(inst: &mut InstructionPointer, commands: &Vec<Command>) {
    let mut running_starts = Vec::new();
    for i in 0..commands.len() {
        if commands[i] == Command::LoopStart {
            running_starts.push(i);
        } else if commands[i] == Command::LoopEnd {
            inst.loop_ends
                .insert(running_starts.pop().expect("Imbalanced Loops!"), i);
        }
    }
}

pub fn new_interpreter() -> Interpreter {
    Interpreter {
        data_pointer: 0,
        cells: [0; 65536],
        curr_arg: 0,
        ret: Vec::new(),
    }
}
pub fn new_pointer(commands: &Vec<Command>) -> InstructionPointer {
    let mut inst = InstructionPointer {
        position: 0,
        loop_starts: Vec::new(),
        loop_ends: HashMap::new(),
    };
    populate_loop_indices(&mut inst, commands);
    inst
}

pub fn start(args: &Vec<u8>) -> Vec<u8> {
    let main = brainfuck_from_string(&read_file_to_string(&format!("Scripts/main.bf"))).unwrap();
    let mut int = new_interpreter();
    int.run(&main, args)
}
