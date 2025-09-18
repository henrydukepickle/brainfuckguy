#[derive(Clone, Debug, PartialEq)]
pub struct FunctionCall {
    pub name: String,
    pub input: Vec<Command>,
    pub output: Vec<Command>,
}
#[derive(Clone, Debug, PartialEq)]
pub enum Command {
    MoveLeft,
    MoveRight,
    Plus,
    Minus,
    LoopStart,
    LoopEnd,
    Out,
    In,
    Debug,
    Interact,
    Call(FunctionCall),
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
