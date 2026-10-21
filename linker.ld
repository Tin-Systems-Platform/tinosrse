ENTRY(_start)

KERNEL_OFFSET = 0xffffffff80000000;

SECTIONS
{
    . = KERNEL_OFFSET + 1M;

    .limine_requests ALIGN(8) : AT(ADDR(.limine_requests) - KERNEL_OFFSET)
    {
        KEEP(*(.limine_requests_start_marker))
        KEEP(*(.limine_requests))
        KEEP(*(.limine_requests_end_marker))
    }

    .got ALIGN(8) : AT(ADDR(.got) - KERNEL_OFFSET)
    {
        *(.got*)
    }

    .text ALIGN(4K) : AT(ADDR(.text) - KERNEL_OFFSET)
    {
        *(.text*)
    }

    .rodata ALIGN(4K) : AT(ADDR(.rodata) - KERNEL_OFFSET)
    {
        *(.rodata*)
    }

    .data ALIGN(4K) : AT(ADDR(.data) - KERNEL_OFFSET)
    {
        *(.data*)
    }

    .bss ALIGN(4K) : AT(ADDR(.bss) - KERNEL_OFFSET)
    {
        *(COMMON)
        *(.bss*)
    }
}