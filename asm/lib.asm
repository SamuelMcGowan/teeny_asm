#once
#include "def.asm"

#ruledef
{
    zero {dest:reg} => asm { movi 0, {dest} }

    incr {n:u8}, {dest:reg} => asm { addi {dest} n`8, {dest} }
    decr {n:u8}, {dest:reg} => asm { subi {dest} n`8, {dest} }
}

#ruledef stack
{
    push {data:reg} => asm {
        store {data} sp
        incr 1, sp
    }

    pop {dest:reg} => asm {
        decr 1, sp
        load sp, {dest}
    }
}

#ruledef calls
{
    call {addr:reg} => asm {
        bl {addr}, link
    }

    calli {addr} => asm {
        bli addr, link
    }

    ret => asm {
        br link
    }
}

lib_start:
    calli start
    halt

; takes addr in r0, ret addr in ret
; also overwrites r2, io_addr, io_data
print_cstr:
    ; set device to printer
    movi 1, io_addr

    .looptop:
        ; load the word from the position in r0
        load r0, r2

        ; print bytes one at a time,
        ; or exit if at the end

        andi r2 0xff, io_data
        shri r2 8, r2
        bri_z io_data, .end
        io_out

        andi r2 0xff, io_data
        shri r2 8, r2
        bri_z io_data, .end
        io_out

        andi r2 0xff, io_data
        shri r2 8, r2
        bri_z io_data, .end
        io_out

        andi r2 0xff, io_data
        bri_z io_data, .end
        io_out

        incr 1, r0
        bri .looptop

    .end:
        ret
