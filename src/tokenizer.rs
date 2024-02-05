use crate::ast::Token;
use std::io::Read;

pub struct Tokenizer<R: Read> {
    inner: R,
}

impl<R: Read> Tokenizer<R> {
    pub fn new(reader: R) -> Self {
        Self { inner: reader }
    }

    pub fn read_char(&mut self) -> Option<char> {
        let mut buf = [0; 1];

        if self.inner.read(&mut buf).unwrap() == 0 {
            return None;
        };

        Some(buf[0].into())
    }

    pub fn read_while<F: Fn(char) -> bool>(&mut self, predicate: F) -> Option<char> {
        loop {
            let character = self.read_char()?;

            if !predicate(character) {
                return Some(character);
            }
        }
    }
}

const POSSIBLE_CHARS: [char; 8] = ['+', ',', '-', '.', '<', '>', '[', ']'];

impl<R: Read> Iterator for Tokenizer<R> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.read_while(|x| !POSSIBLE_CHARS.contains(&x))?;

        Some(Token::new_unchecked(result))
    }
}
