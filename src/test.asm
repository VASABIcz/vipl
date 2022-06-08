section .text
global main
extern printf

message db "debug = %d", 10, 0


debug:
    push message
    call printf
    add esp, 8
    leave

main:
    push esp
    push ebp

    jmp debug

    ret