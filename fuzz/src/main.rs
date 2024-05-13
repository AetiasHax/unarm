mod arm;
mod thumb;

use std::time::Instant;

fn main() {
    let (threads, iterations, arm, thumb) = {
        let mut threads = num_cpus::get();
        let mut iterations = 1;
        let mut arm = false;
        let mut thumb = false;
        let mut args = std::env::args();
        args.next(); // skip program name
        while let Some(arg) = args.next() {
            match arg.as_str() {
                "-t" => threads = args.next().and_then(|a| a.parse().ok()).expect("Expected number after -t"),
                "-n" => iterations = args.next().and_then(|a| a.parse().ok()).expect("Expected number after -n"),
                "arm" => arm = true,
                "thumb" => thumb = true,
                _ => panic!("Unknown argument '{}'", arg),
            }
        }
        (threads, iterations, arm, thumb)
    };
    if threads == 0 {
        panic!("Number of threads must be positive");
    }
    if iterations == 0 {
        panic!("Number of iterations must be positive");
    }
    if arm == thumb {
        panic!("Expected one of 'arm' and 'thumb'");
    }

    println!("Starting {} threads running {} iterations", threads, iterations);
    let start = Instant::now();
    if arm {
        arm::fuzz(threads, iterations);
    }
    if thumb {
        thumb::fuzz(threads, iterations);
    }
    println!("Finished in {:.2}s", start.elapsed().as_secs_f32());
}
