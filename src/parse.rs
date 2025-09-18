use crate::commands::Command;
use crate::commands::FunctionCall;

enum State {
    Outside,
    Name,
    Input,
    Output,
}

pub fn brainfuck_from_string(string: &String) -> Option<Vec<Command>> {
    let mut commands = Vec::new();
    let mut state = State::Outside;
    let mut running_string = String::new();
    let mut running_function = FunctionCall {
        name: String::new(),
        input: Vec::new(),
        output: Vec::new(),
    };
    let mut count = 0;
    for char in string.chars() {
        match state {
            State::Outside => {
                if char == '{' {
                    state = State::Name;
                    running_function = FunctionCall {
                        name: String::new(),
                        input: Vec::new(),
                        output: Vec::new(),
                    };
                    continue;
                } else if let Some(x) = command_from_char(char) {
                    commands.push(x)
                }
            }
            State::Name => {
                if char == '}' {
                    running_function.name = running_string.clone();
                    running_string = String::new();
                    state = State::Input;
                    continue;
                } else {
                    running_string.push(char);
                }
            }
            State::Input => {
                if char == '(' {
                    if count > 0 {
                        running_string.push(char);
                    }
                    count += 1;
                    continue;
                } else if char == ')' {
                    count -= 1;
                    if count == 0 {
                        running_function.input = brainfuck_from_string(&running_string)?;
                        running_string = String::new();
                        state = State::Output;
                        continue;
                    } else {
                        running_string.push(char)
                    }
                } else {
                    running_string.push(char);
                }
            }
            State::Output => {
                if char == '(' {
                    if count > 0 {
                        running_string.push(char);
                    }
                    count += 1;
                    continue;
                } else if char == ')' {
                    count -= 1;
                    if count == 0 {
                        running_function.output = brainfuck_from_string(&running_string)?;
                        running_string = String::new();
                        state = State::Outside;
                        commands.push(Command::Call(running_function.clone()));
                        continue;
                    } else {
                        running_string.push(char);
                    }
                } else {
                    running_string.push(char);
                }
            }
        }
    }
    Some(commands)
}

pub fn command_from_char(ch: char) -> Option<Command> {
    Some(match ch {
        '<' => Command::MoveLeft,
        '>' => Command::MoveRight,
        '+' => Command::Plus,
        '-' => Command::Minus,
        '[' => Command::LoopStart,
        ']' => Command::LoopEnd,
        '.' => Command::Out,
        ',' => Command::In,
        '#' => Command::Debug,
        '|' => Command::Interact,
        _ => return None,
    })
}
