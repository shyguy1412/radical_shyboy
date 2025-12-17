#![allow(unused)]

mod test;
mod ic6502;

use ic6502::IC6502;


trait Bus {
    fn read(&self, addr: u16) -> u8;
    fn write(&mut self, addr: u16, byte: u8);
}

impl Bus for [u8; u16::MAX as usize] {
    fn read(&self, addr: u16) -> u8 {
        self[addr as usize]
    }

    fn write(&mut self, addr: u16, byte: u8) {
        self[addr as usize] = byte
    }
}

const ACCURACY_COIN: &[u8] = include_bytes!("../AccuracyCoin/AccuracyCoin.nes");

fn main() {
    let ram = Box::new([0; u16::MAX as usize]);
    let cpu = IC6502::default();
    println!("Hello, world!");
}
