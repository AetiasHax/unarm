use std::{hint::black_box, ops::RangeInclusive};

use unarm::{thumb, ParseFlags, ParsedIns};

pub fn fuzz(num_threads: usize, iterations: usize, flags: ParseFlags) {
    let fuzzers: Vec<_> = (0..num_threads)
        .map(|i| {
            let start = ((0x10000 * i) / num_threads).try_into().unwrap();
            let end = ((0x10000 * (i + 1)) / num_threads - 1).try_into().unwrap();
            Fuzzer::new(start..=end, iterations, flags)
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
    flags: ParseFlags,
}

impl Fuzzer {
    fn new(range: RangeInclusive<u32>, iterations: usize, flags: ParseFlags) -> Self {
        Self {
            range,
            iterations,
            flags,
        }
    }

    fn run(&self) -> std::thread::JoinHandle<()> {
        let range = self.range.clone();
        let iterations = self.iterations;
        let flags = self.flags;
        std::thread::spawn(move || {
            let mut parsed = ParsedIns::default();
            for _ in 0..iterations {
                for code in range.clone() {
                    #[allow(clippy::unit_arg)]
                    black_box(thumb::parse(&mut parsed, thumb::Ins::new(code, &flags), &flags));
                }
            }
        })
    }
}
