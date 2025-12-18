#![allow(unused)]

mod bus;
mod ic6502;
mod test;

use bus::*;
use ic6502::IC6502;
use test::TestCase;

const ACCURACY_COIN: &[u8] = include_bytes!("../AccuracyCoin/AccuracyCoin.nes");

fn main() {
    let ram = Box::new([0; u16::MAX as usize]);
    let cpu = IC6502::default();
    println!("Hello, world!");
}
