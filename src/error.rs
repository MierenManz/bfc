use std::error::Error;

#[derive(Debug)]
pub enum BfcError {
    MissingLoopClose,
}

impl std::fmt::Display for BfcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl Error for BfcError {}
