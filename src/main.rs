mod cpu;

use std::fs::File;
use std::io::{self, Write};

use byteorder::{NativeEndian, ReadBytesExt};
use cpu::{ControlFlow, Cpu};
use tracing::Level;

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .compact()
        .init();

    let Some(path) = std::env::args().nth(1) else {
        eprintln!("USAGE: teeny_asm [path]");
        return;
    };

    run_bin(&path).unwrap();
}

fn run_bin(path: &str) -> io::Result<()> {
    let mem = read_bin(path)?.into_boxed_slice();

    let mut cpu = Cpu::new(mem);
    loop {
        let ctrl = cpu.tick();

        if let Some((1, data)) = cpu.output() {
            std::io::stdout().write_all(&[data as u8])?;
        }

        if let ControlFlow::Halt = ctrl {
            break;
        }
    }

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
