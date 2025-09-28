use std::{hint::black_box, ops::RangeInclusive};

use rand::RngCore;
use unarm::{parse_arm, Options};

use crate::Test;

pub fn fuzz(num_threads: usize, iterations: usize, options: Options, test: Test) {
    let fuzzers: Vec<_> = (0..num_threads)
        .map(|i| {
            let start = ((0x100000000 * i) / num_threads).try_into().unwrap();
            let end = ((0x100000000 * (i + 1)) / num_threads - 1).try_into().unwrap();
            Fuzzer::new(start..=end, iterations, options.clone())
        })
        .collect();

    let handles: Vec<_> = match test {
        Test::Parse => fuzzers.iter().map(|f| f.parse()).collect(),
        Test::ParseRandom => fuzzers.iter().map(|f| f.parse_random()).collect(),
        Test::ParseAndWrite => fuzzers.iter().map(|f| f.parse_and_write()).collect(),
    };
    for handle in handles {
        handle.join().unwrap();
    }
}

struct Fuzzer {
    range: RangeInclusive<u32>,
    iterations: usize,
    options: Options,
}

impl Fuzzer {
    fn new(range: RangeInclusive<u32>, iterations: usize, options: Options) -> Self {
        Self { range, iterations, options }
    }

    fn parse(&self) -> std::thread::JoinHandle<()> {
        let range = self.range.clone();
        let iterations = self.iterations;
        let options = self.options.clone();
        std::thread::spawn(move || {
            for _ in 0..iterations {
                for code in range.clone() {
                    #[allow(clippy::unit_arg)]
                    black_box(parse_arm(code, 0, &options));
                }
            }
        })
    }

    fn parse_random(&self) -> std::thread::JoinHandle<()> {
        let range = self.range.clone();
        let iterations = self.iterations;
        let options = self.options.clone();
        std::thread::spawn(move || {
            let mut rng = rand::rng();
            for _ in 0..iterations {
                for _ in range.clone() {
                    let code = rng.next_u32();
                    #[allow(clippy::unit_arg)]
                    black_box(parse_arm(code, 0, &options));
                }
            }
        })
    }

    fn parse_and_write(&self) -> std::thread::JoinHandle<()> {
        let range = self.range.clone();
        let iterations = self.iterations;
        let options = self.options.clone();
        std::thread::spawn(move || {
            for _ in 0..iterations {
                for code in range.clone() {
                    let ins = parse_arm(code, 0, &options);
                    black_box(ins.display(&options).to_string());
                }
            }
        })
    }
}
