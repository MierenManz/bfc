#[derive(Debug, Clone, Copy)]
pub enum Token {
    PtrIncr,
    PtrDecr,
    ValIncr,
    ValDecr,
    Stdout,
    Stdin,
    LoopStart,
    LoopEnd,
}

impl Token {
    pub fn new_unchecked(c: char) -> Self {
        match c {
            '>' => Self::PtrIncr,
            '<' => Self::PtrDecr,
            '+' => Self::ValIncr,
            '-' => Self::ValDecr,
            '.' => Self::Stdout,
            ',' => Self::Stdin,
            '[' => Self::LoopStart,
            ']' => Self::LoopEnd,
            _ => panic!("Unknown Token"),
        }
    }
}
