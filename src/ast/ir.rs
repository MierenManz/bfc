#[derive(Clone, Copy, Debug)]
pub enum Ir {
    PtrIncr(u16),
    PtrDecr(u16),
    ValIncr(u8),
    ValDecr(u8),
    LoopStart(usize),
    LoopEnd(usize),
    Stdout,
    Stdin,

    // Optimized Instructions
    /// Mov + add value of `ptr` into `ptr + u16`
    MovAdd(u16),
    /// Add value of `ptr` into `ptr + u16`
    Add(u16),
    /// Mov + sub value of `ptr` from `ptr + u16`
    MovSub(u16),
    /// sub value of `ptr` from `ptr + u16`
    Sub(u16),
    /// Set current memory value to zero
    Szero,
    /// Search empty (cell) Forwards
    SemptyF,
    /// Search empty (cell) Backwards
    SemptyB,
}
