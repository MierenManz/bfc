mod memory;

pub use memory::Memory;

use crate::ast::Ir;
use std::io::Read;

pub struct Runtime {
    pc: usize,
    memory: Memory,
}

impl Runtime {
    pub fn new() -> Self {
        Self {
            pc: 0,
            memory: Memory::new(),
        }
    }

    pub fn execute(mut self, ast: &[Ir]) {
        while self.pc != ast.len() {
            let opcode = ast[self.pc];
            match opcode {
                Ir::PtrIncr(v) => self.memory.move_ptr_right(v),
                Ir::PtrDecr(v) => self.memory.move_ptr_left(v),
                Ir::ValIncr(v) => self.memory.add(v),
                Ir::ValDecr(v) => self.memory.sub(v),
                Ir::Stdout => {
                    print!("{}", self.memory.read_value() as char);
                }

                Ir::Stdin => {
                    let mut buf = [0; 1];
                    let mut lock = std::io::stdin().lock();
                    lock.read_exact(&mut buf).unwrap();
                    self.memory.set_value(buf[0]);
                }

                Ir::LoopStart(skip) => {
                    let value = self.memory.read_value();

                    if value == 0 {
                        // At parsing got the +1 correction
                        self.pc = skip;
                    }
                }

                Ir::LoopEnd(jmp_back) => {
                    self.pc = jmp_back;
                }
                _ => todo!(),
            }
            self.pc += 1;
        }
    }
}
