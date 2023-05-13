/*
 * Copyright (c) 2020 Great Scott Gadgets <info@greatscottgadgets.com>
 * SPDX-License-Identifier: BSD-3-Clause
 */

#define MIE_MEIE        0x800

.section .init, "ax"

.global __stack_top
.global _start
_start:
    // flush icache
    /*.word(0x100f)
    nop
    nop
    nop
    nop
    nop

    // flush dcache
    .word(0x500f)*/

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

    /* Finally, start our main routine. */
    jal zero, main

loop:
    j loop
