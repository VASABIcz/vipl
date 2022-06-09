section .text
global main
extern printf

message db "debug = %d", 10, 0


debug:
    push message
    call printf
    add esp, 8
    ret

main:
    mov ebp, esp
    sub ebp, 4


    jmp debug
    ret