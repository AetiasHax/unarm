use std::{hint::black_box, ops::RangeInclusive};

use unarm::{v5te::thumb, ParsedIns};

pub fn fuzz(num_threads: usize, iterations: usize) {
    let fuzzers: Vec<_> = (0..num_threads)
        .map(|i| {
            let start = ((0x10000 * i) / num_threads).try_into().unwrap();
            let end = ((0x10000 * (i + 1)) / num_threads - 1).try_into().unwrap();
            Fuzzer::new(start..=end, iterations)
        })
        .collect();

    let handles: Vec<_> = fuzzers.iter().map(|f| f.run()).collect();
    for handle in handles {
        handle.join().unwrap();
    }
}

struct Fuzzer {
    range: RangeInclusive<u32>,
    iterations: usize,
}

impl Fuzzer {
    fn new(range: RangeInclusive<u32>, iterations: usize) -> Self {
        Self { range, iterations }
    }

    fn run(&self) -> std::thread::JoinHandle<()> {
        let range = self.range.clone();
        let iterations = self.iterations;
        std::thread::spawn(move || {
            let mut parsed = ParsedIns::default();
            for _ in 0..iterations {
                for code in range.clone() {
                    #[allow(clippy::unit_arg)]
                    black_box(thumb::parse(&mut parsed, thumb::Ins::new(code)));
                }
            }
        })
    }
}
