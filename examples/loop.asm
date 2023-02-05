#include "../asm/lib.asm"

movi 48, r0
movi 58, r1

movi 1, io_addr

loop:
    mov r0, io_data
    io_out

    movi 10, io_data
    io_out

    incr 1, r0
    bri_ltu r0 r1, loop

halt
