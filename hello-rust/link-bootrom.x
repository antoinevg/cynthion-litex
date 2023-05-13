OUTPUT_FORMAT("elf32-littleriscv")
OUTPUT_ARCH("riscv")

/* _start */
ENTRY(_start)

SECTIONS
{
    . = ORIGIN(bootrom);

    /* .init */
    .init : ALIGN(4)
    {
        *(.init) *(.init.*)
    } > bootrom

    /* .text */
    .text : ALIGN(4)
    {
        *(.text) *(.text.*)
    } > bootrom

    /* .rodata */
    .rodata : ALIGN(4)
    {
        *(.rodata) *(.rodata.*)
    } > bootrom

    /* .sdata */
    .sdata : ALIGN(4)
    {
        PROVIDE(__global_pointer$ = .);
        *(.sdata) *(.sdata.*)
    } > scratchpad

    /* .data */
    .data : ALIGN(4)
    {
        *(.data) *(.data.*)
    } > scratchpad

    /* .bss */
    .bss : ALIGN(4)
    {
        *(.bss) *(.bss.*)
    } > scratchpad

}

/* stack */
PROVIDE(__stack_top = ORIGIN(scratchpad) + LENGTH(scratchpad));
