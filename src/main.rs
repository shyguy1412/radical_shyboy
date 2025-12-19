mod bus;
mod ic6502;
mod test;

use bus::*;
use ic6502::IC6502;
use test::TestCase;

const CASES_RAW: &str = include_str!("../65x02/rockwell65c02/v1/00.json");

fn main() {
    let test_cases: Vec<TestCase<IC6502>> = match serde_json::from_str(CASES_RAW) {
        Ok(val) => val,
        Err(err) => panic!("{}", err),
    };

    for case in test_cases.iter() {
        let mut cpu = case.initial.cpu.clone();
        let mut ram = case.initial.ram.clone();
        let cycles = case.cycles.len() as u8;

        let mut i = 0;
        while i <= cycles {
            let used @ 1..=u8::MAX = cpu.cycle(&mut ram) else {
                println!("Encountered an invalid instruction");
                break;
            };
            i += used;
        }

        if ram != case.target.ram {
            println!("YOU FUCKED UP!")
        }
        break;
    }
    println!("Hello, world!");
}
