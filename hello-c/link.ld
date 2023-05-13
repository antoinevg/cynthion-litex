OUTPUT_FORMAT("elf32-littleriscv")
OUTPUT_ARCH("riscv")

MEMORY
{
    bootrom         : ORIGIN = 0x00000000, LENGTH = 0x00004000
    scratchpad      : ORIGIN = 0x10000000, LENGTH = 0x00001000
    internal_sram   : ORIGIN = 0x40000000, LENGTH = 0x00008000
    csr             : ORIGIN = 0xf0000000, LENGTH = 0x00010000
}

ENTRY(_start)

SECTIONS
{
    . = ORIGIN(internal_sram);

    /* .init */
    .init : ALIGN(4)
    {
        *(.init) *(.init.*)
    } > internal_sram /* internal_srom */

    /* .text */
    .text : ALIGN(4)
    {
        *(.text) *(.text.*)
    } > internal_sram /* internal_srom */

    /* .rodata */
    .rodata : ALIGN(4)
    {
        *(.rodata) *(.rodata.*)
    } > internal_sram /* internal_rodata */

    /* .sdata */
    .sdata : ALIGN(4)
    {
        PROVIDE(__global_pointer$ = .);
        *(.sdata) *(.sdata.*)
    } > internal_sram

    /* .data */
    .data : ALIGN(4)
    {
        *(.data) *(.data.*)
    } > internal_sram

    /* .bss */
    .bss : ALIGN(4)
    {
        *(.bss) *(.bss.*)
    } > internal_sram

}

/* stack */
PROVIDE(__stack_top = ORIGIN(internal_sram) + LENGTH(internal_sram));
