[BITS 64]

global _start
extern kernel_main

section .text.prologue

_start:
    cli
    cld
    call kernel_main

.hang:
    hlt
    jmp .hang
