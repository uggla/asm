section .text
    global _start

_start:
    ; Allocate 390 bytes on the stack for the utsname structure
    ; 390 bytes is the size of the utsname struct (65 bytes * 6 fields)
    sub rsp, 390            ; Allocate space on the stack

    ; syscall number for uname is 63
    mov rax, 63             ; syscall number for uname
    mov rdi, rsp            ; pass pointer to allocated stack space
    syscall                 ; invoke the kernel

    ; Check for error (if rax is negative, an error occurred)
    cmp rax, 0
    jl error

    ; If successful, write part of the sysname (OS name) to stdout
    ; We will use the sys_write system call (number 1)
    mov rax, 1              ; syscall number for sys_write
    mov rdi, 1              ; file descriptor (stdout)
    mov rsi, rsp            ; pointer to sysname field (it's at the top of the stack)
    mov rdx, 5              ; length of sysname (assuming "Linux")
    syscall                 ; write sysname to stdout

    ; Clean up the stack by adding 390 bytes back to rsp
    add rsp, 390

    ; Exit the program
    mov rax, 60             ; syscall number for sys_exit
    xor rdi, rdi            ; exit status 0
    syscall

error:
    ; If an error occurs, restore the stack and exit with status 1
    add rsp, 390            ; clean up the stack
    mov rax, 60             ; syscall number for sys_exit
    mov rdi, 1              ; exit status 1
    syscall
