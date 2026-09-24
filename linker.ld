ENTRY(_start)

SECTIONS {
    . = 1M;

    .text : {
        KEEP(*(.multiboot_header)) 
        *(.text.prologue)
        *(.text .text.*)
    }

    .rodata : { *(.rodata .rodata.*) }
    .data   : { *(.data .data.*) }
    .bss    : { *(.bss .bss.*) }
}
