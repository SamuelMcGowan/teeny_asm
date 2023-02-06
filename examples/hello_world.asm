#include "../asm/lib.asm"

start:
    movi msg, r0
    calli print_cstr, r1
    halt

msg:
    #d "hello, world!\r\n\0"
