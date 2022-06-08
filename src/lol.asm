;
; assemble and link with:
; nasm -f elf printf-test.asm && gcc -m32 -o printf-test printf-test.o
;
section .text
global main
extern printf

main:

  mov eax, 0xDEADBEEF
  push 420
  push message
  call printf
  add esp, 8
  ret

message db "Register = %d", 10, 0