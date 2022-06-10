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

    ; V-I-P-L : v0.0000001
    ; Vasova
    ; Insane
    ; Programing
    ; Language
    ; TODO programing language :D

    ; variable angus
    push 0
    push 5
    ; 5 + 7
    mov eax, [ebp-4]
    add eax, 7
    mov dword [ebp-4], eax
    pop eax
    mov dword [ebp-0], eax

    jmp debug
    ret