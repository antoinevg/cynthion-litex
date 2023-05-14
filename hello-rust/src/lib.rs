#![no_std]

#[macro_use]
pub mod macros;

/// Machine IRQ Mask
pub mod mim {
    crate::macros::read_csr_as_usize!(0x330);
    crate::macros::write_csr_as_usize!(0x330);
}

/// Machine IRQ Pending
pub mod mip {
    crate::macros::read_csr_as_usize!(0x360);
}
