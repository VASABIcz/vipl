section .text
global main
extern printf

message db "debug = %d", 10, 0


debug:
    push message
    call printf
    add esp, 8
    ret

; nasm -f elf test.asm && gcc -m32 -o test test.o
main:
    mov ebp, esp
    sub ebp, 4

    ; variable a
    push 0 ; ebp-0
    push 79 ; ebp-4
    push 38 ; ebp-8
    ; 38 * 91
    mov eax, [ebp-8]
    imul eax, 91
    mov dword [ebp-8], eax
    mov eax, [ebp-4]
    add eax, [ebp-8]
    mov dword [ebp-4], eax
    pop edx
    pop eax
    mov dword [ebp-0], eax

    jmp debug
    ret