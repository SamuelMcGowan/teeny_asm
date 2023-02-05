# README

## Overview

- 32-bit, little-endian
- RISC (of course)
- No virtual addressing
- No privileges

## Instruction Format

All instructions are one word.

    - 1-bit immediate flag
    - 7-bit opcode
    - Either:
      - Type A: 8-bit source reg, 8-bit source reg / 8-bit immediate, 8-bit dest
      - Type B: 8-bit source reg / 24-bit intermediate

Instructions that use an operand that can be an immediate have a separate immediate mnemonic. For example, `addi r0 12 r1` would denote adding `12` to the value in `r0` and placing the result in `r1`.

## Registers

There are 16 32-bit registers.

    - r0-r7
    - r8-rc
    - io_addr
    - io_data
    - pc

`r8`-`rc` could end up not being general purpose, so they should be treated as unused for now.

## Instruction Set

### Arithmetic & Logic

    Type A
    ======
    Operand 1: operand a
    Operand 2: operand b

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

### Unconditional Branching

    Type A
    ======
    Operand 1: link register (for call)
    Operand 2: dest addr

    10      br
    1f      call

### Conditional Branching

    Type A
    ======
    Operand 1: operand a
    Operand 2: dest addr
    Operand 3: operand b

    The immediate operand (operand B) is used for the destination address, rather than the value being compared.

    11      br_z
    12      br_nz
    13      br_eq
    14      br_neq

    15      br_lt
    16      br_gt
    17      br_le
    18      br_ge
    19      br_neg
    1a      br_pos

    1b      br_ltu
    1c      br_gtu
    1d      br_leu
    1e      br_geu

### Control

    20      halt
