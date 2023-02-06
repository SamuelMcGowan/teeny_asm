#once

#bits 32

#subruledef reg
{
    r0 => 0x00
    r1 => 0x01
    r2 => 0x02
    r3 => 0x03
    r4 => 0x04
    r5 => 0x05
    r6 => 0x06
    r7 => 0x07
    r8 => 0x08
    r9 => 0x09
    ra => 0x0a

    rb => 0x0b
    link => 0x0b

    io_addr => 0x0c
    io_data => 0x0d
    sp => 0x0e
    pc => 0x0f
}

#ruledef move
{
    mov  {src:reg}, {dest:reg} =>       0x0f   @ 0x00 @ src @ dest
    movi {imm:u8},  {dest:reg} => 0b1 @ 0x0f`7 @ 0x00 @ imm @ dest
}

#ruledef arithmetic
{
    add {a:reg} {b:reg}, {c:reg} =>       0x00   @ a @ b @ c
    sub {a:reg} {b:reg}, {c:reg} =>       0x01   @ a @ b @ c
    mul {a:reg} {b:reg}, {c:reg} =>       0x02   @ a @ b @ c
    div {a:reg} {b:reg}, {c:reg} =>       0x03   @ a @ b @ c

    addi {a:reg} {b:u8}, {c:reg} => 0b1 @ 0x00`7 @ a @ b @ c
    subi {a:reg} {b:u8}, {c:reg} => 0b1 @ 0x01`7 @ a @ b @ c
    muli {a:reg} {b:u8}, {c:reg} => 0b1 @ 0x02`7 @ a @ b @ c
    divi {a:reg} {b:u8}, {c:reg} => 0b1 @ 0x03`7 @ a @ b @ c
}

#ruledef logic
{
    and  {a:reg} {b:reg}, {c:reg} =>       0x04   @ a @ b @ c
    or   {a:reg} {b:reg}, {c:reg} =>       0x05   @ a @ b @ c
    nand {a:reg} {b:reg}, {c:reg} =>       0x06   @ a @ b @ c
    nor  {a:reg} {b:reg}, {c:reg} =>       0x07   @ a @ b @ c
    xor  {a:reg} {b:reg}, {c:reg} =>       0x08   @ a @ b @ c
    shl  {a:reg} {b:reg}, {c:reg} =>       0x09   @ a @ b @ c
    shr  {a:reg} {b:reg}, {c:reg} =>       0x0a   @ a @ b @ c

    andi  {a:reg} {b:u8}, {c:reg} => 0b1 @ 0x04`7 @ a @ b @ c
    ori   {a:reg} {b:u8}, {c:reg} => 0b1 @ 0x05`7 @ a @ b @ c
    nandi {a:reg} {b:u8}, {c:reg} => 0b1 @ 0x06`7 @ a @ b @ c
    nori  {a:reg} {b:u8}, {c:reg} => 0b1 @ 0x07`7 @ a @ b @ c
    xori  {a:reg} {b:u8}, {c:reg} => 0b1 @ 0x08`7 @ a @ b @ c
    shli  {a:reg} {b:u8}, {c:reg} => 0b1 @ 0x09`7 @ a @ b @ c
    shri  {a:reg} {b:u8}, {c:reg} => 0b1 @ 0x0a`7 @ a @ b @ c
}

#ruledef branching
{
    br  {dest:reg}             =>       0x10   @ 0x00 @ dest @ 0x00
    bri {dest:u8}              => 0b1 @ 0x10`7 @ 0x00 @ dest @ 0x00

    bl  {dest:reg}, {link:reg} =>       0x11   @ 0x00 @ dest @ link
    bli {dest:u8},  {link:reg} => 0b1 @ 0x11`7 @ 0x00 @ dest @ link
}

#ruledef cond_branching
{
    ; no immediates
    br_z    {a:reg},         {dest:reg} =>       0x12   @ a @ dest @ 0x00
    br_nz   {a:reg},         {dest:reg} =>       0x13   @ a @ dest @ 0x00

    br_eq   {a:reg} {b:reg}, {dest:reg} =>       0x14   @ a @ dest @ b
    br_neq  {a:reg} {b:reg}, {dest:reg} =>       0x15   @ a @ dest @ b

    br_lt   {a:reg} {b:reg}, {dest:reg} =>       0x16   @ a @ dest @ b
    br_gt   {a:reg} {b:reg}, {dest:reg} =>       0x17   @ a @ dest @ b
    br_le   {a:reg} {b:reg}, {dest:reg} =>       0x18   @ a @ dest @ b
    br_ge   {a:reg} {b:reg}, {dest:reg} =>       0x19   @ a @ dest @ b
    br_neg  {a:reg},         {dest:reg} =>       0x1a   @ a @ dest @ 0x00
    br_pos  {a:reg},         {dest:reg} =>       0x1b   @ a @ dest @ 0x00

    br_ltu  {a:reg} {b:reg}, {dest:reg} =>       0x1c   @ a @ dest @ b
    br_gtu  {a:reg} {b:reg}, {dest:reg} =>       0x1d   @ a @ dest @ b
    br_leu  {a:reg} {b:reg}, {dest:reg} =>       0x1e   @ a @ dest @ b
    br_geu  {a:reg} {b:reg}, {dest:reg} =>       0x1f   @ a @ dest @ b

    ; immediates
    bri_z   {a:reg},         {dest:u8}  => 0b1 @ 0x12`7 @ a @ dest @ 0x00
    bri_nz  {a:reg},         {dest:u8}  => 0b1 @ 0x13`7 @ a @ dest @ 0x00

    bri_eq  {a:reg} {b:reg}, {dest:u8}  => 0b1 @ 0x14`7 @ a @ dest @ b
    bri_neq {a:reg} {b:reg}, {dest:u8}  => 0b1 @ 0x15`7 @ a @ dest @ b

    bri_lt  {a:reg} {b:reg}, {dest:u8}  => 0b1 @ 0x16`7 @ a @ dest @ b
    bri_gt  {a:reg} {b:reg}, {dest:u8}  => 0b1 @ 0x17`7 @ a @ dest @ b
    bri_le  {a:reg} {b:reg}, {dest:u8}  => 0b1 @ 0x18`7 @ a @ dest @ b
    bri_ge  {a:reg} {b:reg}, {dest:u8}  => 0b1 @ 0x19`7 @ a @ dest @ b
    bri_neg {a:reg},         {dest:u8}  => 0b1 @ 0x1a`7 @ a @ dest @ 0x00
    bri_pos {a:reg},         {dest:u8}  => 0b1 @ 0x1b`7 @ a @ dest @ 0x00

    bri_ltu {a:reg} {b:reg}, {dest:u8}  => 0b1 @ 0x1c`7 @ a @ dest @ b
    bri_gtu {a:reg} {b:reg}, {dest:u8}  => 0b1 @ 0x1d`7 @ a @ dest @ b
    bri_leu {a:reg} {b:reg}, {dest:u8}  => 0b1 @ 0x1e`7 @ a @ dest @ b
    bri_geu {a:reg} {b:reg}, {dest:u8}  => 0b1 @ 0x1f`7 @ a @ dest @ b
}

#ruledef control
{
    halt   => 0x20_00_00_00
    nop    => 0x21_00_00_00
    io_out => 0x22_00_00_00
    io_in  => 0x23_00_00_00
}

#ruledef memory
{
    store  {data:reg} {addr:reg}  =>       0x30   @ data @ addr @ 0x00
    load   {addr:reg}, {dest:reg} =>       0x31   @ 0x00 @ addr @ dest

    storei {data:reg} {addr:u8}   => 0b1 @ 0x30`7 @ data @ addr @ 0x00
    loadi  {addr:reg}, {dest:reg} => 0b1 @ 0x31`7 @ 0x00 @ addr @ dest
}
