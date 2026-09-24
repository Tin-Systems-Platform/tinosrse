[BITS 32]

global _start
extern kernel_main


section .multiboot_header
align 8
multiboot_header_start:
    dd 0xE85250D6               
    dd 0                         
    dd multiboot_header_end - multiboot_header_start 
    dd 0x100000000 - (0xE85250D6 + 0 + (multiboot_header_end - multiboot_header_start))

    dw 0                         
    dw 0                        
    dd 8                         
multiboot_header_end:


section .text.prologue

_start:
    cli

    mov [boot_magic], eax
    mov [boot_info], ebx

    mov esp, stack_top


[BITS 64]

long_mode_entry:
 
    mov rsp, stack_top
    cld


    mov edi, [rel boot_info]   
    mov esi, [rel boot_magic]  
    
    call kernel_main

.hang:
    hlt
    jmp .hang

align 8

gdt64:
    dq 0x0000000000000000     ; Null descriptor

    ; 64-bit code descriptor
    dq 0x00AF9A000000FFFF

    ; Data descriptor
    dq 0x00AF92000000FFFF

.pointer:
    dw .pointer - gdt64 - 1
    dq gdt64

.code equ 0x08


align 4096

pml4:
    times 512 dq 0

pdpt:
    times 512 dq 0

pd:
    times 512 dq 0

section .bss

align 16

stack_bottom:
    resb 8192

stack_top:

align 8
boot_magic:
    resd 1
boot_info:
    resd 1