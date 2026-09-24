[BITS 32]

global _start
extern kernel_main


; ============================================================
; Multiboot2 header
; ============================================================

section .multiboot
align 8

multiboot_header:
    dd 0xE85250D6                         ; Multiboot2 magic
    dd 0                                    ; architecture: i386
    dd multiboot_header_end - multiboot_header
    dd -(0xE85250D6 + 0 + (multiboot_header_end - multiboot_header))

    ; End tag
    dw 0
    dw 0
    dd 8

multiboot_header_end:


; ============================================================
; 32-bit entry point
; ============================================================

section .text.prologue

_start:
    cli

    ; Debug marker: reached 32-bit entry
    mov word [0xB8000], 0x0B41            ; A

    ; GRUB gives us:
    ; EAX = Multiboot2 magic
    ; EBX = Multiboot2 information structure
    mov [boot_magic], eax
    mov [boot_info], ebx

    ; Temporary 32-bit stack
    mov esp, stack_top

    ; --------------------------------------------------------
    ; Load our 64-bit GDT
    ; --------------------------------------------------------

    lgdt [gdt64.pointer]

    ; --------------------------------------------------------
    ; Load page tables
    ; --------------------------------------------------------

    mov eax, pml4
    mov cr3, eax

    ; --------------------------------------------------------
    ; Enable PAE
    ; --------------------------------------------------------

    mov eax, cr4
    or eax, (1 << 5)
    mov cr4, eax

    ; --------------------------------------------------------
    ; Enable Long Mode in EFER
    ; --------------------------------------------------------

    mov ecx, 0xC0000080                    ; IA32_EFER
    rdmsr

    or eax, (1 << 8)                       ; LME

    wrmsr

    ; --------------------------------------------------------
    ; Enable paging
    ; --------------------------------------------------------

    ; mov eax, cr0
    ;  or eax, (1 << 31)                      ; PG

    ; mov cr0, eax

    ; --------------------------------------------------------
    ; Enter 64-bit code segment
    ; --------------------------------------------------------

    jmp 0x08:long_mode_entry


; ============================================================
; 64-bit entry point
; ============================================================

[BITS 64]

long_mode_entry:

    ; Debug marker: reached 64-bit mode
    mov word [0xB8002], 0x0B42            ; B

    ; 64-bit stack
    mov rsp, stack_top

    cld

    ; Pass Multiboot2 information to Rust.
    ;
    ; boot_info and boot_magic are 32-bit values stored
    ; in low memory, so zero-extend them into 64-bit registers.
    mov edi, [rel boot_info]
    mov esi, [rel boot_magic]

    call kernel_main


.hang:
    cli
    hlt
    jmp .hang


; ============================================================
; GDT
; ============================================================

align 8

gdt64:

    ; Null descriptor
    dq 0x0000000000000000

    ; 64-bit code descriptor
    dq 0x00AF9A000000FFFF

    ; Data descriptor
    dq 0x00AF92000000FFFF

.pointer:
    dw .pointer - gdt64 - 1
    dq gdt64

.code equ 0x08


; ============================================================
; Identity-mapped page tables
; Maps the first 2 MiB using one 2 MiB page
; ============================================================

align 4096

pml4:
    times 512 dq 0

pdpt:
    times 512 dq 0

pd:
    times 512 dq 0


; ============================================================
; BSS
; ============================================================

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