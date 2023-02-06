use std::collections::HashMap;
use std::io::{Read, Write};

pub trait Device {
    fn get_input(&mut self) -> u32 {
        0
    }

    fn set_output(&mut self, _data: u32) {}
}

pub struct Printer<W: Write>(pub W);

impl<W: Write> Device for Printer<W> {
    fn set_output(&mut self, data: u32) {
        let byte = u32::from_le(data) as u8;
        self.0.write_all(&[byte]).expect("failed to write byte");
    }
}

pub struct Reader<R: Read>(pub R);

impl<R: Read> Device for Reader<R> {
    fn get_input(&mut self) -> u32 {
        let mut byte = 0;
        self.0
            .read_exact(std::slice::from_mut(&mut byte))
            .expect("failed to read byte");
        byte as u32
    }
}

#[derive(Default)]
pub struct Devices {
    devices: HashMap<u32, Box<dyn Device>>,
}

impl Devices {
    pub fn insert(&mut self, addr: u32, device: impl Device + 'static) -> Option<Box<dyn Device>> {
        self.devices.insert(addr, Box::new(device))
    }

    pub fn remove(&mut self, addr: u32) -> Option<Box<dyn Device>> {
        self.devices.remove(&addr)
    }

    pub fn get_input(&mut self, addr: u32) -> u32 {
        if let Some(device) = self.devices.get_mut(&addr) {
            device.get_input()
        } else {
            0
        }
    }

    pub fn set_output(&mut self, addr: u32, data: u32) {
        if let Some(device) = self.devices.get_mut(&addr) {
            device.set_output(data);
        }
    }
}
