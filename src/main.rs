mod bus;
mod ic6502;
mod test;

use bus::*;
use ic6502::IC6502;
use rayon::prelude::*;
use test::TestCase;

use crate::ic6502::{Instruction, Operation};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn load_tests_rayon_json() -> Result<impl ParallelIterator<Item = Vec<TestCase<IC6502>>>> {
    let mut dir: Vec<_> = std::fs::read_dir("./65x02/nes6502/v1")?
        .filter_map(|f| f.ok())
        .collect();

    dir.sort_by(|a, b| {
        let a_name = &a.file_name().into_string().unwrap()[0..2];
        let b_name = &b.file_name().into_string().unwrap()[0..2];
        let a_val = u16::from_str_radix(a_name, 16).unwrap();
        let b_val = u16::from_str_radix(b_name, 16).unwrap();
        a_val.cmp(&b_val)
    });

    let suites = dir
        .into_par_iter()
        .filter(|f| {
            f.file_name()
                .into_string()
                .ok()
                .and_then(|f| u8::from_str_radix(&f[0..2], 16).ok())
                .map(|code| match code.into() {
                    Instruction::Invalid => false,
                    // Instruction::Valid {
                    //     operation: Operation::BranchOnCarrySet,
                    //     ..
                    // } => true,
                    Instruction::Valid { .. } => true,
                })
                .unwrap_or(false)
        })
        .filter_map(|f| std::fs::read_to_string(f.path()).ok())
        .filter_map(|json| serde_json::from_str::<Vec<TestCase<IC6502>>>(&json).ok());

    Ok(suites)
}

fn main() -> Result<()> {
    println!("Loading Test Cases...");

    let start = std::time::Instant::now();
    let suites = load_tests_rayon_json()?;

    let start_load = std::time::Instant::now();
    let suites = suites.collect::<Vec<Vec<TestCase<IC6502>>>>();
    let end_load = std::time::Instant::now();
    println!(
        "{} Tests loaded in {:.2} seconds",
        suites.len() * suites[0].len(),
        end_load.duration_since(start_load).as_secs_f64()
    );
    let suites = suites.into_iter();

    let mut total_tests: f64 = 0.;
    let mut total_successful: f64 = 0.;

    println!();

    let (sender, receiver) = std::sync::mpsc::channel();

    suites.for_each(|mut suite| {
        let result = run_suite(&mut suite);
        let _ = sender.clone().send(result);
    });

    let total = std::time::Instant::now();

    drop(sender);

    for (successful, cases) in receiver {
        total_tests += cases;
        total_successful += successful;
    }

    let total_failed: f64 = total_tests - total_successful;
    let total_sucessful_percent: f64 = (total_successful / total_tests) * 100.;
    let total_failed_percent: f64 = (total_failed / total_tests) * 100.;

    println!();

    println!(
        "{:7}/{} ({:6.2}%) passed;",
        total_successful, total_tests, total_sucessful_percent
    );

    println!(
        "{:7}/{} ({:6.2}%) failed;",
        total_failed, total_tests, total_failed_percent
    );

    println!(
        "Ran all tests in {:.2}s;",
        total.duration_since(start).as_secs_f64(),
    );
    Ok(())
}

fn run_suite(suite: &mut Vec<TestCase<IC6502>>) -> (f64, f64) {
    let mut successful: f64 = 0.;
    let name = suite[0].name[0..2].to_owned();
    let cases = suite.len() as f64;

    let start = std::time::Instant::now();
    let mut start_instruction = 0;
    for case in suite {
        // if case.name != "88 45 70" {
        //     continue;
        // }
        let (pass, time) = run_test(case);
        if pass {
            successful += 1.;
        }
        start_instruction += time;
    }
    let end = std::time::Instant::now();

    println!(
        "./65x02/nes6502/v1/{}.json: {:5}/{}; {:6.2}%;{:3}ms/{:3}µs;",
        name,
        successful,
        cases,
        (successful as f64 / cases as f64) * 100.,
        end.duration_since(start).as_millis(),
        start_instruction
    );

    (successful, cases)
}

fn run_test(case: &mut TestCase<IC6502>) -> (bool, u128) {
    let name = case.name.clone();
    let cpu = &mut case.initial.cpu;
    let ram = &mut case.initial.ram;

    let start = std::time::Instant::now();
    cpu.cycle(ram).or_else(|| {
        // drop(name);
        // println!("CPU CRASH AT {}", name);
        Some(8)
    });
    let end = std::time::Instant::now();

    ram.sort_by(|(addr1, _), (addr2, _)| addr1.cmp(addr2));

    case.target
        .ram
        .sort_by(|(addr1, _), (addr2, _)| addr1.cmp(addr2));

    let ram_pass = *ram == case.target.ram;
    let cpu_pass = *cpu == case.target.cpu;

    if !ram_pass || !cpu_pass {
        drop("".to_string())
    }

    (ram_pass && cpu_pass, end.duration_since(start).as_micros())
}
