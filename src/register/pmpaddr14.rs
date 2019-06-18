//! pmpaddr14 register
//!
//!
pub struct PmpAddr14 {
    bits: usize,
}

impl PmpAddr14 {
    pub fn bits(&self) -> usize {
        self.bits
    }
}

read_csr_as!(PmpAddr14, 0x3BE, __read_pmpaddr14);
write_csr!(0x3BE, __write_pmpaddr14);

#[inline]
pub unsafe fn write(addr: usize) {
    _write(addr);
}
