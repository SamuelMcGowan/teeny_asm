mod cpu;
mod device;

use std::fs::File;
use std::io;

use byteorder::{NativeEndian, ReadBytesExt};
use cpu::{ControlFlow, Cpu};
use device::{Devices, Printer, Reader};
use tracing::{warn, Level};

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .compact()
        .init();

    let Some(path) = std::env::args().nth(1) else {
        eprintln!("USAGE: teeny_asm [path]");
        return;
    };

    let mut devices = Devices::default();
    devices.insert(1, Printer(std::io::stdout()));
    devices.insert(2, Reader(std::io::stdin()));

    run_bin(&path, 0x1000, devices).unwrap();
}

fn run_bin(path: &str, mem_size: usize, devices: Devices) -> io::Result<()> {
    let mut mem = read_bin(path)?;
    if mem.len() < mem_size {
        mem.resize(mem_size, 0);
    } else {
        warn!("memory is larger than expected");
    }

    let mem = mem.into_boxed_slice();

    let mut cpu = Cpu::new(mem, devices);
    while let ControlFlow::Continue = cpu.tick() {}

    Ok(())
}

fn read_bin(path: &str) -> io::Result<Vec<u32>> {
    let mut f = File::open(path)?;

    // I think it's ok to assume that no platform will return
    // a file size larger than their word size.
    let len = f.metadata()?.len() as usize;
    let len_words = len / 4;

    let mut words = Vec::with_capacity(len_words);

    for _ in 0..len_words {
        let word = f.read_u32::<NativeEndian>()?;
        words.push(word.to_le());
    }

    Ok(words)
}
