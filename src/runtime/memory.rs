const BYTE: usize = 1024;
const PAGE: usize = BYTE * 64;

pub struct Memory {
    ptr: u16,
    op_count: usize,
    slice: [u8; PAGE],
}

impl Memory {
    pub fn new() -> Self {
        Self {
            ptr: 0,
            op_count: 0,
            slice: [0; PAGE],
        }
    }

    pub fn add(&mut self, value: u8) {
        let lhs = &mut self.slice[self.ptr as usize];
        *lhs = lhs.wrapping_add(value);
        self.op_count += 1;
    }

    pub fn sub(&mut self, value: u8) {
        let lhs = &mut self.slice[self.ptr as usize];
        *lhs = lhs.wrapping_sub(value);
        self.op_count += 1;
    }

    pub fn move_ptr_right(&mut self, ptr_offset: u16) {
        let ptr = self.ptr;
        self.ptr = ptr.wrapping_add(ptr_offset);
        self.op_count += 1;
    }

    pub fn move_ptr_left(&mut self, ptr_offset: u16) {
        let ptr = self.ptr;
        self.ptr = ptr.wrapping_sub(ptr_offset);
        self.op_count += 1;
    }

    pub fn set_value(&mut self, value: u8) {
        self.slice[self.ptr as usize] = value;
        self.op_count += 1;
    }

    pub fn read_value(&mut self) -> u8 {
        self.op_count += 1;
        self.slice[self.ptr as usize]
    }
}
