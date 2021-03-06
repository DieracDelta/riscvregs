//Approved for public release. Distribution is unlimited.

//This material is based upon work supported by the Under Secretary of Defense for Research and Engineering under Air Force Contract No. FA8702-15-D-0001. Any opinions, findings, conclusions or recommendations expressed in this material are those of the author(s) and do not necessarily reflect the views of the Under Secretary of Defense for Research and Engineering.

//© 2019 Massachusetts Institute of Technology.

//The software/firmware is provided to you on an As-Is basis

//Delivered to the U.S. Government with Unlimited Rights, as defined in DFARS Part 252.227-7013 or 7014 (Feb 2014). Notwithstanding any copyright notice, U.S. Government rights in this work are defined by DFARS 252.227-7013 or DFARS 252.227-7014 as detailed above. Use of this work other than as specifically authorized by the U.S. Government may violate any copyrights that exist in this work.

//! pmpaddr3 register
//!
//!
pub struct PmpAddr3 {
    bits: usize,
}

impl PmpAddr3 {
    pub fn bits(&self) -> usize {
        self.bits
    }
}

read_csr_as!(PmpAddr3, 0x3B3, __read_pmpaddr3);
write_csr!(0x3B3, __write_pmpaddr3);

#[inline]
pub unsafe fn write(addr: usize) {
    _write(addr);
}
