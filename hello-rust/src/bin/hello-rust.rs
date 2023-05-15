#![allow(dead_code, unused_imports, unused_mut, unused_variables)]

#![no_std]
#![no_main]

// - start of day -------------------------------------------------------------

core::arch::global_asm!(
r#"
.section .init
.global __stack_top
_start:
    // flush icache
    .word(0x100f)
    nop
    nop
    nop
    nop
    nop

    // flush dcache
    .word(0x500f)

    // global pointer
    .option push
    .option norelax
    la gp, __global_pointer$
    .option pop

    // stack pointer
    la sp, __stack_top
    add s0, sp, zero

    // jump to main
    jal zero, main

loop:
    j loop
"#
);

// - main ---------------------------------------------------------------------

#[link_section = ".text"]
#[no_mangle]
pub unsafe extern "C" fn main() -> ! {
    const MSG: &'static str = "Entering main loop.\n";

    uart_tx(MSG);

    let mut counter = 0;
    loop {
        while (core::ptr::read_volatile(reg::IO_UART_TX_RDY as *mut u32) & 0b1) == 0 { }
        core::ptr::write_volatile(reg::IO_UART_TX_DAT as *mut u32, (counter & 0b00_1111) + 97);

        counter += 1;

        #[cfg(not(feature = "litex_sim"))]
        {
            core::ptr::write_volatile(reg::IO_LEDS as *mut u32, counter & 0b11_1111);
            asm::delay(1_000_000);
        }

        #[cfg(feature = "litex_sim")]
        if counter > 32 {
            loop {}
        }
    }
}

// - helpers ------------------------------------------------------------------

fn uart_tx(s: &str) {
    for b in s.bytes() {
        while unsafe { core::ptr::read_volatile(reg::IO_UART_TX_RDY as *mut u32) & 0b1 } == 0 {}
        unsafe {
            core::ptr::write_volatile(reg::IO_UART_TX_DAT as *mut u32, b as u32 & 0b1111_1111)
        };
    }
}

mod asm {
    #[inline(always)]
    pub unsafe fn delay(cycles: u32) {
        let real_cyc = 1 + cycles / 2;
        core::arch::asm!(
            "1:",
            "addi {0}, {0}, -1",
            "bne {0}, zero, 1b",
            inout(reg) real_cyc => _,
            options(nomem, nostack),
        )
    }

    #[inline(always)]
    pub unsafe fn flush_icache() {
        core::arch::asm!(".word(0x100f)", "nop", "nop", "nop", "nop", "nop",);
    }
    #[inline(always)]
    pub unsafe fn flush_dcache() {
        core::arch::asm!(".word(0x500f)");
    }
}

// - trait: core::fmt::Write --------------------------------------------------

use core::fmt::Write;

struct Writer;

impl core::fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        uart_tx(s);
        Ok(())
    }
}

// - peripheral registers -----------------------------------------------------

#[cfg(feature = "litex")]
mod reg {
    pub const IO_BASE: usize = 0xf000_0000;
    pub const IO_LEDS: usize = IO_BASE + 0x1000;
    pub const IO_UART_TX_DAT: usize = IO_BASE + 0x2000 + 0x00; // RXTX
    pub const IO_UART_TX_RDY: usize = IO_BASE + 0x2000 + 0x18; // TXEMPTY
}

#[cfg(feature = "litex_sim")]
mod reg {
    pub const IO_BASE: usize = 0xf000_0000;
    pub const IO_LEDS: usize = IO_BASE + 0x1000;
    pub const IO_UART_TX_DAT: usize = IO_BASE + 0x1800 + 0x00; // RXTX
    pub const IO_UART_TX_RDY: usize = IO_BASE + 0x1800 + 0x18; // TXEMPTY
}

#[cfg(feature = "lunasoc")]
mod reg {
    pub const IO_BASE: usize = 0xf000_0000;
    pub const IO_LEDS: usize = IO_BASE + 0x1000;
    pub const IO_UART_RX_DAT: usize = IO_BASE + 0x2000 + 0x04; // rx_dat
    pub const IO_UART_RX_RDY: usize = IO_BASE + 0x2000 + 0x08; // rx_rdy
    pub const IO_UART_TX_DAT: usize = IO_BASE + 0x2000 + 0x10; // tx_data
    pub const IO_UART_TX_RDY: usize = IO_BASE + 0x2000 + 0x14; // tx_rdy
}

// - panic_handler ------------------------------------------------------------

#[no_mangle]
#[panic_handler]
fn panic(_panic_info: &core::panic::PanicInfo) -> ! {
    unsafe { core::ptr::write_volatile(reg::IO_LEDS as *mut u32, 0b11_1100) };
    loop {}
}
