# README

## Overview

- 32-bit, little-endian
- RISC (of course)
- No virtual addressing
- No privileges
- Assembly via [customasm](https://github.com/hlorenzi/customasm/).

## Instruction Format

All instructions are one word.

    - 1-bit immediate flag
    - 7-bit opcode
    - Either:
      - Type A: 8-bit source reg, 8-bit source reg / 8-bit immediate, 8-bit dest
      - Type B: 8-bit source reg / 24-bit intermediate

Instructions that use an operand that can be an immediate have a separate immediate mnemonic in the assembly language. For example, `addi r0 12 r1` would denote adding `12` to the value in `r0` and placing the result in `r1`.

## Registers

There are 16 32-bit registers.

    - r0-r7
    - r8-ra
    - rb / link
    - io_addr
    - io_data
    - sp
    - pc

`r8`-`ra` could end up not being general purpose, so they should be treated as unused for now.

## Instruction Set

### Arithmetic & Logic

    Type A
    ======
    Operand 1: operand a
    Operand 2: operand b
    Operand 3: dest

    00      add
    01      sub
    02      mul
    03      div
    04      and
    05      or
    06      nand
    07      nor
    08      xor
    09      shl
    0a      shr

    0f      mov (moves operand b to dest)

### Unconditional Branching

    Type A
    ======
    Operand 2: dest addr
    Operand 3: link register (for bl)

    10      br
    11      bl

### Conditional Branching

    Type A
    ======
    Operand 1: operand a
    Operand 2: dest addr
    Operand 3: operand b (if needed)

    The immediate operand (operand B) is used for the destination address, rather than the value being compared.

    12      br_z
    13      br_nz
    14      br_eq
    15      br_neq

    16      br_lt
    17      br_gt
    18      br_le
    19      br_ge
    1a      br_neg
    1b      br_pos

    1c      br_ltu
    1d      br_gtu
    1e      br_leu
    1f      br_geu

### Control

    20      halt
    21      nop
    22      io_out
    23      io_in

### Memory

    Type A
    ======
    Operand 1: data (store)
    Operand 2: addr
    Operand 3: dest reg (load)

    30      store
    31      load
