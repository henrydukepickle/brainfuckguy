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
