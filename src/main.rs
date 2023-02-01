fn main() {
    let mem = Box::new([
        // addi r0 0x0a r0
        0b0000_0000_0000_1010_0000_0000_1000_0000,

        // halt
        0b0000_0000_0000_0000_0000_0000_0010_0000,
    ]);

    let mut cpu = Cpu::new(mem);
    while let ControlFlow::Continue = cpu.tick() {}

    println!("Result: {:02x}", cpu.registers[0]);
}

const REG_IO_ADDR: u32 = 13;
const REG_IO_DATA: u32 = 14;
const REG_PC: u32 = 15;

#[must_use]
enum ControlFlow {
    Continue,
    Halt,
}

struct Cpu {
    registers: [u32; 16],
    mem: Box<[u32]>,
}

impl Cpu {
    fn new(mem: Box<[u32]>) -> Self {
        let registers = [0; 16];
        Self { registers, mem }
    }

    fn tick(&mut self) -> ControlFlow {
        macro_rules! reg {
            ($reg:expr) => {
                self.registers[(($reg) & 0xf) as usize]
            };
        }

        let instr_pc = reg!(REG_PC);
        let instr = self.load_u32(instr_pc).to_le_bytes();
        reg!(REG_PC) = instr_pc.wrapping_add(1);

        let opcode = instr[0] & 0b0111_1111;

        println!("Instr: {instr:02x?}");

        macro_rules! operand_a {
            () => {
                reg!(instr[1])
            };
        }

        macro_rules! operand_b_or_imm {
            () => {{
                let imm = (instr[0] >> 7) == 1;
                if imm {
                    instr[2] as u32
                } else {
                    reg!(instr[2])
                }
            }};
        }

        macro_rules! operand_c {
            () => {
                reg!(instr[3])
            };
        }

        // TODO: handle errors instead of just returning

        match opcode {
            // arithmetic and logic
            0x00..=0x0a => {
                let a = operand_a!();
                let b = operand_b_or_imm!();

                let c = match opcode {
                    0x00 => a.wrapping_add(b),
                    0x01 => a.wrapping_sub(b),
                    0x02 => a.wrapping_mul(b),
                    0x03 => {
                        if let Some(c) = a.checked_div(b) {
                            c
                        } else {
                            return ControlFlow::Continue;
                        }
                    }
                    0x04 => a & b,
                    0x05 => a | b,
                    0x06 => !(a & b),
                    0x07 => !(a | b),
                    0x08 => a ^ b,
                    0x09 => a << b,
                    0x0a => a >> b,
                    _ => return ControlFlow::Continue,
                };

                operand_c!() = c;
            }

            // branch
            0x10 => {
                reg!(REG_PC) = operand_b_or_imm!();
            }

            // call
            0x1f => {
                reg!(operand_a!()) = instr_pc;
                reg!(REG_PC) = operand_b_or_imm!();
            }

            // conditional branching
            0x11..=0x1e => {
                let a = operand_a!();
                let b = operand_c!();

                let a_s = a as i32;
                let b_s = b as i32;

                let dest = operand_b_or_imm!();

                let should_branch = match opcode {
                    0x11 => a == 0,
                    0x12 => a != 0,
                    0x13 => a == b,
                    0x14 => a != b,

                    // signed
                    0x15 => a_s < b_s,
                    0x16 => a_s > b_s,
                    0x17 => a_s <= b_s,
                    0x18 => a_s >= b_s,
                    0x19 => a_s.is_negative(),
                    0x1a => a_s.is_positive(),

                    // unsigned
                    0x1b => a < b,
                    0x1c => a > b,
                    0x1d => a <= b,
                    0x1e => a >= b,

                    _ => unreachable!(),
                };

                if should_branch {
                    reg!(REG_PC) = dest;
                }
            }

            // halt
            0x20 => {
                return ControlFlow::Halt;
            }

            _ => {}
        }

        ControlFlow::Continue
    }

    fn load_u32(&self, address: u32) -> u32 {
        self.mem.get(address as usize).copied().unwrap_or_default()
    }
}
