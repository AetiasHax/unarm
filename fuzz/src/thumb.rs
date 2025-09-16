use std::{hint::black_box, ops::RangeInclusive};

use unarm::{parse_thumb, Options};

use crate::Test;

pub fn fuzz(num_threads: usize, iterations: usize, options: Options, test: Test) {
    let fuzzers: Vec<_> = (0..num_threads)
        .map(|i| {
            let start = ((0x100000000 * i) / num_threads).try_into().unwrap();
            let end = ((0x100000000 * (i + 1)) / num_threads - 1).try_into().unwrap();
            Fuzzer::new(start..=end, iterations, options)
        })
        .collect();

    let handles: Vec<_> = match test {
        Test::Parse => fuzzers.iter().map(|f| f.parse()).collect(),
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
        std::thread::spawn(move || {
            for _ in 0..iterations {
                for code in range.clone() {
                    black_box(parse_thumb(code, 0));
                }
            }
        })
    }

    fn parse_and_write(&self) -> std::thread::JoinHandle<()> {
        let range = self.range.clone();
        let iterations = self.iterations;
        let options = self.options;
        std::thread::spawn(move || {
            for _ in 0..iterations {
                for code in range.clone() {
                    let Some(ins) = parse_thumb(code, 0) else { continue };
                    black_box(ins.display(&options).to_string());
                }
            }
        })
    }
}
