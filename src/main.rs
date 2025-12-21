mod bus;
mod ic6502;
mod test;

use bus::*;
use ic6502::IC6502;
use rayon::prelude::*;
use test::TestCase;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn load_tests_rayon_json() -> Result<Vec<Vec<TestCase<IC6502>>>> {
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

    let suites: Vec<_> = dir
        .into_par_iter()
        .filter_map(|f| std::fs::read_to_string(f.path()).ok())
        .filter_map(|json| serde_json::from_str::<Vec<TestCase<IC6502>>>(&json).ok())
        .collect();

    Ok(suites)
}

fn main() -> Result<()> {
    println!("Loading Test Cases...");

    let start = std::time::Instant::now();
    let mut suites = load_tests_rayon_json()?;
    let end = std::time::Instant::now();

    println!(
        "{} Tests loaded in {:.2} seconds",
        suites.len() * suites[0].len(),
        end.duration_since(start).as_secs_f64()
    );

    let total_tests: f64 = (suites.len() * suites[0].len()) as f64;
    let mut total_successful: f64 = 0.;

    println!();

    for suite in &mut suites {
        let mut successful = 0;
        let name = suite[0].name[0..2].to_owned();
        let cases = suite.len();

        let start = std::time::Instant::now();
        let mut start_instruction = 0;
        for case in suite {
            let (pass, time) = run_test(case);
            if pass {
                successful += 1;
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

        total_successful += successful as f64;
    }

    let total = std::time::Instant::now();
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

fn run_test(case: &mut TestCase<IC6502>) -> (bool, u128) {
    let cpu = &mut case.initial.cpu;
    let ram = &mut case.initial.ram;

    let start = std::time::Instant::now();
    cpu.cycle(ram);
    let end = std::time::Instant::now();

    ram.sort_by(|(addr1, _), (addr2, _)| addr1.cmp(addr2));

    (
        *ram == case.target.ram && *cpu == case.target.cpu,
        end.duration_since(start).as_micros(),
    )
}
