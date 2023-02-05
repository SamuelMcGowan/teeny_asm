mod cpu;

use cpu::{ControlFlow, Cpu};
use tracing::Level;

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .compact()
        .init();

    let mem = Box::new([
        // addi r0 0x0a r0
        0b0000_0000_0000_1010_0000_0000_1000_0000,
        // halt
        0b0000_0000_0000_0000_0000_0000_0010_0000,
    ]);

    let mut cpu = Cpu::new(mem);
    while let ControlFlow::Continue = cpu.tick() {}
}
