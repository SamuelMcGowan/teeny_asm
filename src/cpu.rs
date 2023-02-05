use tracing::trace;

const REG_IO_ADDR: u32 = 13;
const REG_IO_DATA: u32 = 14;
const REG_PC: u32 = 15;

#[must_use]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlFlow {
    Continue,
    Halt,
}

pub struct Cpu {
    registers: [u32; 16],
    mem: Box<[u32]>,

    io_out: bool,
    io_in: bool,
}

impl Cpu {
    pub fn new(mem: Box<[u32]>) -> Self {
        Self {
            registers: [0; 16],
            mem,
            io_out: false,
            io_in: false,
        }
    }

    pub fn tick(&mut self) -> ControlFlow {
        self.io_out = false;
        self.io_in = false;

        macro_rules! reg {
            ($reg:expr) => {
                self.registers[(($reg) & 0xf) as usize]
            };
        }

        let instr_pc = reg!(REG_PC);
        let instr = self.load_u32(instr_pc).to_le_bytes();
        reg!(REG_PC) = instr_pc.wrapping_add(1);

        let opcode = instr[0] & 0b0111_1111;

        trace!("{instr_pc:02x?}  {instr:02x?}");

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
                    // this is pretty common so should
                    // be the first thing to check for
                    0x0f => b,

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
            0x11 => {
                reg!(operand_c!()) = instr_pc;
                reg!(REG_PC) = operand_b_or_imm!();
            }

            // conditional branching
            0x12..=0x1f => {
                let a = operand_a!();
                let b = operand_c!();

                let a_s = a as i32;
                let b_s = b as i32;

                let dest = operand_b_or_imm!();

                let should_branch = match opcode {
                    0x12 => a == 0,
                    0x13 => a != 0,
                    0x14 => a == b,
                    0x15 => a != b,

                    // signed
                    0x16 => a_s < b_s,
                    0x17 => a_s > b_s,
                    0x18 => a_s <= b_s,
                    0x19 => a_s >= b_s,
                    0x1a => a_s.is_negative(),
                    0x1b => a_s.is_positive(),

                    // unsigned
                    0x1c => a < b,
                    0x1d => a > b,
                    0x1e => a <= b,
                    0x1f => a >= b,

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

            // nop
            0x21 => {}

            0x22 => {
                self.io_out = true;
            }
            0x23 => {
                self.io_in = false;
            }

            _ => {}
        }

        trace!("    {:02x?}", self.registers);

        ControlFlow::Continue
    }

    fn load_u32(&self, address: u32) -> u32 {
        self.mem.get(address as usize).copied().unwrap_or_default()
    }
}
