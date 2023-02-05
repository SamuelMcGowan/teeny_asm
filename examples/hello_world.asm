#include "../asm/lib.asm"

movi msg, r0
movi 1, io_addr

loop:
    load r0, r1

    andi r1 0xff, io_data
    shri r1 8, r1
    bri_z io_data, loop_end
    io_out

    andi r1 0xff, io_data
    shri r1 8, r1
    bri_z io_data, loop_end
    io_out

    andi r1 0xff, io_data
    shri r1 8, r1
    bri_z io_data, loop_end
    io_out

    andi r1 0xff, io_data
    bri_z io_data, loop_end
    io_out

    incr 1, r0
    bri loop

loop_end:
    halt

msg:
    #d "hello, world!\r\n\0"
