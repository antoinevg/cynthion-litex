/**
 * This file is part of LUNA.
 *
 * Copyright (c) 2020 Great Scott Gadgets <info@greatscottgadgets.com>
 * SPDX-License-Identifier: BSD-3-Clause
 */

#include <stddef.h>
#include <stdint.h>

// - peripheral registers -----------------------------------------------------

#define LITEX 1
//#define LUNASOC 1

#ifdef LITEX
#define IO_BASE 0xf0000000U
#define IO_LEDS (IO_BASE + 0x1000U)
#define IO_UART_TX_DAT (IO_BASE + 0x2000U + 0x00U)
#define IO_UART_TX_RDY (IO_BASE + 0x2000U + 0x18L)
#endif


#ifdef LUNASOC
#define IO_BASE 0xf0000000U
#define IO_LEDS (IO_BASE + 0x1000U)
#define IO_UART_TX_DAT (IO_BASE + 0x2000U + 0x10U)
#define IO_UART_TX_RDY (IO_BASE + 0x2000U + 0x14L)
#endif

#define MMPTR(a) (*((volatile uint32_t *)(a)))

static inline void csr_write(unsigned long a, unsigned long v)
{
    MMPTR(a) = v;
}

static inline unsigned long csr_read(unsigned long a)
{
    return MMPTR(a);
}


// - helpers ------------------------------------------------------------------

/*static inline void delay(uint32_t cycles)
{
    uint32_t real_cyc = 1 + cycles / 2;
    asm("1:\n\t"
        "addi %0, %0, 1;\n\t"
        "bne %0, zero, 1b;"
        :: "r" (real_cyc));
}*/

/*void uart_tx(char *str)
{
    for (char *c = str; *c; ++c) {
        while (csr_read(IO_UART_TX_RDY) == 0) {}
        csr_write(IO_UART_TX_DAT, *c);
    }
    }*/

// - main ---------------------------------------------------------------------

int main(void)
{
    static char* MSG = "Entering C main loop.\n";

    //uart_tx(MSG);

    uint32_t counter = 0;
    while (1) {
        for (int n = 0; n < 1000000; n++);
        //delay(1000000);

        if ((csr_read(IO_UART_TX_RDY) & 0b1) == 1) {
            csr_write(IO_UART_TX_DAT, (char)((counter & 0b1111) + 97));
        } else {
            csr_write(IO_LEDS, counter);
        }

        //csr_write(IO_LEDS, counter);

        counter = counter + 1;
    }
}
