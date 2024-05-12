use std::{hint::black_box, ops::RangeInclusive, time::Instant};

use armv5te::arm;

fn main() {
    let threads = num_cpus::get();
    println!("Starting {} threads", threads);
    let start = Instant::now();
    fuzz(threads);
    println!("Finished in {:.2}s", start.elapsed().as_secs_f32());
}

fn fuzz(num_threads: usize) {
    let fuzzers: Vec<_> = (0..num_threads)
        .map(|i| {
            let start = ((0x100000000 * i) / num_threads).try_into().unwrap();
            let end = ((0x100000000 * (i + 1)) / num_threads - 1).try_into().unwrap();
            Fuzzer::new(start..=end)
        })
        .collect();

    let handles: Vec<_> = fuzzers.iter().map(|f| f.run()).collect();
    for handle in handles {
        handle.join().unwrap();
    }
}

struct Fuzzer {
    range: RangeInclusive<u32>,
}

impl Fuzzer {
    fn new(range: RangeInclusive<u32>) -> Self {
        Self { range }
    }

    fn run(&self) -> std::thread::JoinHandle<()> {
        let range = self.range.clone();
        std::thread::spawn(move || {
            let mut parsed = arm::ParsedIns::default();
            for code in range {
                #[allow(clippy::unit_arg)]
                black_box(arm::parse(&mut parsed, arm::Ins::new(code)));
            }
        })
    }
}
