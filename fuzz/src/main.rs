mod arm;
mod thumb;

use std::time::Instant;

use unarm::{Extensions, Options, R9Use, Version};

#[derive(Clone, Copy)]
pub enum Test {
    Parse,
    ParseRandom,
    ParseAndWrite,
}

fn main() {
    let (threads, iterations, arm, thumb, version, ual, test) = {
        let mut threads = num_cpus::get();
        let mut iterations = 1;
        let mut arm = false;
        let mut thumb = false;
        let mut version = None;
        let mut ual = false;
        let mut test = None;
        let mut args = std::env::args();
        args.next(); // skip program name
        while let Some(arg) = args.next() {
            match arg.as_str() {
                "-t" => {
                    threads =
                        args.next().and_then(|a| a.parse().ok()).expect("Expected number after -t")
                }
                "-n" => {
                    iterations =
                        args.next().and_then(|a| a.parse().ok()).expect("Expected number after -n")
                }
                "arm" => arm = true,
                "thumb" => thumb = true,
                "v4" => version = Some(Version::V4),
                "v4t" => version = Some(Version::V4T),
                "v5t" => version = Some(Version::V5T),
                "v5te" => version = Some(Version::V5Te),
                "v5tej" => version = Some(Version::V5Tej),
                "v6" => version = Some(Version::V6),
                "v6k" => version = Some(Version::V6K),
                "ual" => ual = true,
                "parse" => test = Some(Test::Parse),
                "parse-random" => test = Some(Test::ParseRandom),
                "parse-and-write" => test = Some(Test::ParseAndWrite),
                _ => panic!("Unknown argument '{}'", arg),
            }
        }
        (threads, iterations, arm, thumb, version, ual, test)
    };
    if threads == 0 {
        panic!("Number of threads must be positive");
    }
    if iterations == 0 {
        panic!("Number of iterations must be positive");
    }
    if arm == thumb {
        panic!("Expected one of: arm, thumb");
    }
    let Some(version) = version else {
        panic!("Expected one of: v4, v4t, v5t, v5te, v5tej, v6, v6k");
    };
    let Some(test) = test else {
        panic!("Expected one of: parse, parse-random, parse-and-write");
    };
    let options = Options {
        version,
        extensions: Extensions::all(),
        av: false,
        r9_use: R9Use::R9,
        sl: false,
        fp: false,
        ip: false,
        ual,
    };

    println!("Starting {} threads running {} iterations", threads, iterations);
    let start = Instant::now();
    if arm {
        arm::fuzz(threads, iterations, options.clone(), test);
    }
    if thumb {
        thumb::fuzz(threads, iterations, options.clone(), test);
    }
    println!("Finished in {:.2}s", start.elapsed().as_secs_f32());
}
