#once
#include "def.asm"

#ruledef
{
    zero {dest:reg} => asm { movi 0, {dest} }

    incr {n:u8}, {dest:reg} => asm { addi {dest} n, {dest} }
    decr {n:u8}, {dest:reg} => asm { subi {dest} n, {dest} }
}
