use crate::ast::Ir;
use crate::ast::Token;

use crate::error::BfcError;
use crate::optimizations::Optimizations;
use crate::tokenizer::Tokenizer;
use std::io::Read;

pub struct Parser {
    optimizations: Optimizations,
}

impl Parser {
    pub fn new() -> Self {
        Self::new_with(Optimizations::none())
    }

    pub fn new_with(optimizations: Optimizations) -> Self {
        Self { optimizations }
    }

    fn parse_ptr_incr(&self, next_tokens: &[Token], ast: &mut Vec<Ir>) -> usize {
        if !self.optimizations.instr_compression() {
            ast.push(Ir::PtrIncr(1));
            return 0;
        }

        let mut offset: u16 = 1;
        let mut i = 0;
        for t in next_tokens {
            match t {
                Token::PtrIncr => offset = offset.wrapping_add(1),
                Token::PtrDecr => offset = offset.wrapping_sub(1),
                _ => break,
            }
            i += 1;
        }

        if offset != 0 {
            ast.push(Ir::PtrIncr(offset));
        }

        i
    }

    fn parse_ptr_decr(&self, next_tokens: &[Token], ast: &mut Vec<Ir>) -> usize {
        if !self.optimizations.instr_compression() {
            ast.push(Ir::PtrDecr(1));
            return 0;
        }

        let mut offset: u16 = 1;
        let mut i = 0;
        for t in next_tokens {
            match t {
                Token::PtrIncr => offset = offset.wrapping_sub(1),
                Token::PtrDecr => offset = offset.wrapping_add(1),
                _ => break,
            }
            i += 1;
        }

        if offset != 0 {
            ast.push(Ir::PtrDecr(offset));
        }

        i
    }

    fn parse_val_incr(&self, next_tokens: &[Token], ast: &mut Vec<Ir>) -> usize {
        if !self.optimizations.instr_compression() {
            ast.push(Ir::ValIncr(1));
            return 0;
        }

        let mut offset: u8 = 1;
        let mut i = 0;
        for t in next_tokens {
            match t {
                Token::ValIncr => offset = offset.wrapping_add(1),
                Token::ValDecr => offset = offset.wrapping_sub(1),
                _ => break,
            }
            i += 1;
        }

        if offset != 0 {
            ast.push(Ir::ValIncr(offset));
        }

        i
    }

    fn parse_val_decr(&self, next_tokens: &[Token], ast: &mut Vec<Ir>) -> usize {
        if !self.optimizations.instr_compression() {
            ast.push(Ir::ValDecr(1));
            return 0;
        }

        let mut offset: u8 = 1;
        let mut i = 0;
        for t in next_tokens {
            match t {
                Token::ValIncr => offset = offset.wrapping_sub(1),
                Token::ValDecr => offset = offset.wrapping_add(1),
                _ => break,
            }
            i += 1;
        }

        if offset != 0 {
            ast.push(Ir::ValDecr(offset));
        }

        i
    }

    pub fn parse<R: Read>(&self, src: R) -> Result<Vec<Ir>, BfcError> {
        let tokens = Vec::from_iter(Tokenizer::new(src));
        let mut loops = Vec::with_capacity(20);
        let mut ast = Vec::with_capacity(tokens.len());

        let mut i = 0;
        while i != tokens.len() {
            let token = tokens[i];
            i += 1;

            let offset = match token {
                Token::PtrIncr => self.parse_ptr_incr(&tokens[i..], &mut ast),
                Token::PtrDecr => self.parse_ptr_decr(&tokens[i..], &mut ast),
                Token::ValIncr => self.parse_val_incr(&tokens[i..], &mut ast),
                Token::ValDecr => self.parse_val_decr(&tokens[i..], &mut ast),
                Token::Stdout => {
                    ast.push(Ir::Stdout);
                    0
                }
                Token::Stdin => {
                    ast.push(Ir::Stdin);
                    0
                }
                Token::LoopStart => {
                    loops.push(ast.len());
                    ast.push(Ir::LoopStart(0));
                    0
                }
                Token::LoopEnd => {
                    let result = loops.pop();
                    if result.is_none() {
                        return Err(BfcError::MissingLoopClose);
                    }

                    let ptr = result.unwrap();

                    // If empty loop. Don't include it
                    if ptr == ast.len() {
                        ast.pop();
                        continue;
                    }

                    ast[ptr] = Ir::LoopStart(ast.len());
                    ast.push(Ir::LoopEnd(ptr - 1));
                    0
                }
            };

            i += offset;
        }

        Ok(ast)
    }
}
