#![allow(dead_code, unused_imports, unused_mut, unused_variables)]
#![no_std]
#![no_main]

// - start of day -------------------------------------------------------------

// _start
core::arch::global_asm!(
    r#"
.global _start
_start:
    j reset_vector
    .align 2

reset_vector:
    la sp, _fstack
    la t0, trap_vector
    csrw mtvec, t0

    // initialize .data
    la t0, _fdata
    la t1, _edata
    la t2, _fdata_bootrom
1:	beq t0, t1, 2f
    lw t3, 0(t2)
    sw t3, 0(t0)
    addi t0, t0, 4
    addi t2, t2, 4
    j 1b
2:

    // initialize .bss
    la t0, _fbss
    la t1, _ebss
1:	beq t0, t1, 2f
    sw zero, 0(t0)
    addi t0, t0, 4
    j 1b
2:
    // enable external interrupts
    li t0, 0x800 // MIE_MEIE
    csrs mie, t0

    call main
1:	j 1b

trap_vector:
    addi sp, sp, -18*4

    sw ra,  0*4(sp)
    sw t0,  1*4(sp)
    sw t1,  2*4(sp)
    sw t2,  3*4(sp)
    sw a0,  4*4(sp)
    sw a1,  5*4(sp)
    sw a2,  6*4(sp)
    sw a3,  7*4(sp)
    sw a4,  8*4(sp)
    sw a5,  9*4(sp)
    sw a6, 10*4(sp)
    sw a7, 11*4(sp)
    sw t3, 12*4(sp)
    sw t4, 13*4(sp)
    sw t5, 14*4(sp)
    sw t6, 15*4(sp)

    csrr t0, mepc
    sw t0, 16*4(sp)

    csrr t0, mcause
    sw t0, 17*4(sp)

    mv a0, sp
    call trap_handler

    lw ra,  0*4(sp)
    lw t1,  2*4(sp)
    lw t2,  3*4(sp)
    lw a0,  4*4(sp)
    lw a1,  5*4(sp)
    lw a2,  6*4(sp)
    lw a3,  7*4(sp)
    lw a4,  8*4(sp)
    lw a5,  9*4(sp)
    lw a6, 10*4(sp)
    lw a7, 11*4(sp)
    lw t3, 12*4(sp)
    lw t4, 13*4(sp)
    lw t5, 14*4(sp)
    lw t6, 15*4(sp)

    lw t0, 16*4(sp)
    csrw mepc, t0

    lw t0, 17*4(sp)
    csrw mcause, t0

    lw t0, 1*4(sp)

    addi sp, sp, 18*4
    mret
"#
);

// boot_helper
core::arch::global_asm!(
    r#"
.section .text, "ax", @progbits
.global boot_helper
boot_helper:
    // disable external interrupts
    li t0, 0x800 // MIE_MEIE
    csrc mie, t0

    // flush instruction cache
    fence.i

    // jump to payload
    jr a3
"#
);



// - main ---------------------------------------------------------------------

//#[link_section = ".text"]
#[no_mangle]
pub unsafe extern "C" fn main() -> ! {
    const MSG: &'static str = "Entering main loop.\n";

    /*while (core::ptr::read_volatile(reg::IO_UART_RX_RDY as *mut u32) & 0b1) == 1 {
        let b = core::ptr::read_volatile(reg::IO_UART_RX_DAT as *mut u32);
        core::ptr::write_volatile(reg::IO_LEDS as *mut u32, b & 0b11_1111);
    }*/

    /*let mut writer = Writer;
    writeln!(writer, "0x{:08x} IO_LEDS", reg::IO_LEDS).unwrap();
    writeln!(writer, "0x{:08x} IO_UART_TX_DAT", reg::IO_UART_TX_DAT).unwrap();
    writeln!(writer, "0x{:08x} IO_UART_TX_RDY", reg::IO_UART_TX_RDY).unwrap();
    writeln!(writer, "{}", MSG).unwrap();*/

    //for c in MSG.chars() {}
    //uart_tx(MSG);

    let mut counter = 0;
    loop {
        asm::delay(1_000_000);
        //for _ in 0..10000 {}

        if (core::ptr::read_volatile(reg::IO_UART_TX_RDY as *mut u32) & 0b1) == 1 {
            core::ptr::write_volatile(reg::IO_UART_TX_DAT as *mut u32, (counter & 0b00_1111) + 97);
        } else {
            core::ptr::write_volatile(reg::IO_LEDS as *mut u32, counter & 0b11_1111);
        }

        //core::ptr::write_volatile(reg::IO_LEDS as *mut u32, counter & 0b11_1111);
        /*if counter % 100 == 0 {
            writeln!(writer, "Uptime: {}", counter).unwrap();
        }*/

        counter += 1;
    }
}

// - panic_handler ------------------------------------------------------------

#[no_mangle]
#[panic_handler]
fn panic(_panic_info: &core::panic::PanicInfo) -> ! {
    unsafe { core::ptr::write_volatile(reg::IO_LEDS as *mut u32, 0b11_1100) };
    //writeln!(Writer, "PANIC {:?}", _panic_info).unwrap();
    loop {}
}

// - trap_handler -------------------------------------------------------------

#[repr(C)]
#[derive(Debug)]
pub struct TrapFrame {
    pub ra: u32,
    pub t0: u32,
    pub t1: u32,
    pub t2: u32,
    pub a0: u32,
    pub a1: u32,
    pub a2: u32,
    pub a3: u32,
    pub a4: u32,
    pub a5: u32,
    pub a6: u32,
    pub a7: u32,
    pub t3: u32,
    pub t4: u32,
    pub t5: u32,
    pub t6: u32,
    pub mepc: u32,
    pub mcause: u32,
}

//#[link_section = ".trap.rust"]
#[export_name = "trap_handler"]
pub unsafe extern "C" fn trap_handler(trap_frame: *const TrapFrame) {
    if (*trap_frame).mcause & 0x80000000 > 0 {
        let pending = irq_pending() & irq_get_mask();
        if pending & (1 << reg::INTC_UART_IRQ) > 0 {
            // ...
        }
    } else {
        // panic
        /*writeln!(
            Writer,
            "Panic! at mepc=0x{:08x} (mcause=0x{:08x})\n",
            (*trap_frame).mepc,
            (*trap_frame).mcause
        )
        .unwrap();*/
    }
}

// - helpers ------------------------------------------------------------------

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

fn uart_tx(s: &str) {
    for b in s.bytes() {
        while unsafe { core::ptr::read_volatile(reg::IO_UART_TX_RDY as *mut u32) } == 0 {}
        unsafe {
            core::ptr::write_volatile(reg::IO_UART_TX_DAT as *mut u32, b as u32 & 0b1111_1111)
        };
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
    pub const INTC_UART_TIMER: usize = 0;
    pub const INTC_UART_IRQ: usize = 1;
}

#[cfg(feature = "lunasoc")]
mod reg {
    pub const IO_BASE: usize = 0xf000_0000;
    pub const IO_LEDS: usize = IO_BASE + 0x1000;
    pub const IO_UART_RX_DAT: usize = IO_BASE + 0x2000 + 0x04; // rx_dat
    pub const IO_UART_RX_RDY: usize = IO_BASE + 0x2000 + 0x08; // rx_rdy
    pub const IO_UART_TX_DAT: usize = IO_BASE + 0x2000 + 0x10; // tx_data
    pub const IO_UART_TX_RDY: usize = IO_BASE + 0x2000 + 0x14; // tx_rdy
    pub const INTC_UART_TIMER: usize = 0;
    pub const INTC_UART_IRQ: usize = 1;
}

unsafe fn irq_pending() -> usize {
    hello_rust::mip::read()
}

unsafe fn irq_get_mask() -> usize {
    hello_rust::mim::read()
}
