#![allow(unused)]

mod bus;
mod ic6502;
mod test;

use bus::*;
use ic6502::IC6502;
use test::TestCase;

const CASES_RAW: &str = include_str!("../65x02/rockwell65c02/v1/00.json");

fn main() {
    let ram = Box::new([0; u16::MAX as usize]);
    let cpu = IC6502::default();

    let test_cases: Vec<TestCase<IC6502>> = match serde_json::from_str(CASES_RAW) {
        Ok(val) => val,
        Err(err) => panic!("{}", err),
    };

    for case in test_cases.iter() {
        let mut cpu = case.initial.cpu.clone();
        let mut ram = case.initial.ram.clone();
        let cycles = case.cycles.len();
        for _ in 0..cycles {
            cpu.cycle(&mut ram);
        }
        break;
    }
    println!("Hello, world!");
}
