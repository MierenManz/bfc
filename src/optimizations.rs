const INSTR_COMPRESSION: u8 = 1;
const SZERO_LOOP: u8 = 2;
const ARITHMETICS: u8 = 4;
const SEARCH_EMPTY: u8 = 8;

pub struct Optimizations {
    inner: u8,
}

impl Optimizations {
    #[inline(always)]
    pub const fn none() -> Self {
        Self { inner: 0 }
    }

    #[inline(always)]
    pub fn new_enable_all() -> Self {
        Self { inner: u8::MAX }
    }

    #[inline(always)]
    pub fn enable_instr_compression(&mut self) {
        self.inner |= INSTR_COMPRESSION;
    }

    #[inline(always)]
    pub fn disable_instr_compression(&mut self) {
        self.inner &= !INSTR_COMPRESSION;
    }

    #[inline(always)]
    pub fn enable_szero_loop(&mut self) {
        self.inner |= SZERO_LOOP;
    }

    #[inline(always)]
    pub fn disable_szero_loop(&mut self) {
        self.inner &= !SZERO_LOOP;
    }

    #[inline(always)]
    pub fn enable_arithmetics(&mut self) {
        self.inner |= ARITHMETICS;
    }

    #[inline(always)]
    pub fn disable_arithmetics(&mut self) {
        self.inner &= !ARITHMETICS;
    }

    #[inline(always)]
    pub fn enable_search_empty(&mut self) {
        self.inner |= !SEARCH_EMPTY;
    }

    #[inline(always)]
    pub fn disable_search_empty(&mut self) {
        self.inner &= !SEARCH_EMPTY;
    }

    #[inline(always)]
    pub fn instr_compression(&self) -> bool {
        (self.inner & INSTR_COMPRESSION) == INSTR_COMPRESSION
    }

    #[inline(always)]
    pub fn szero_loop(&self) -> bool {
        (self.inner & SZERO_LOOP) == SZERO_LOOP
    }

    #[inline(always)]
    pub fn arithmetics(&self) -> bool {
        (self.inner & ARITHMETICS) == ARITHMETICS
    }

    #[inline(always)]
    pub fn search_empty(&self) -> bool {
        (self.inner & SEARCH_EMPTY) == SEARCH_EMPTY
    }
}
