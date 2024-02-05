mod memory;

pub use memory::Memory;

use crate::ast::Ir;
use std::io::Read;
use std::io::Write;

pub struct Runtime<W: Write> {
    pc: usize,
    memory: Memory,
    out: W,
}

impl<W: Write> Runtime<W> {
    pub fn new(writer: W) -> Self {
        Self {
            pc: 0,
            memory: Memory::new(),
            out: writer,
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
                    self.out.write(&[self.memory.read_value()]).unwrap();
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
                Ir::Szero => self.memory.set_value(0),
                _ => todo!(),
            }
            self.pc += 1;
        }
    }
}
