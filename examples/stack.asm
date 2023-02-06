#include "../asm/lib.asm"

movi stack, sp

movi 1, io_addr

movi 48, r0
push r0

movi 49, r0
push r0

movi 0, r0

pop io_data
io_out

pop io_data
io_out

movi 10, io_data
io_out

halt

; define stack
#align 32
stack:
    #res 0x1000
; force stack to exist
#d 0x00