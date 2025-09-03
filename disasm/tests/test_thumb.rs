#[cfg(test)]
mod tests {
    use unarm::{Options, parse_thumb};

    macro_rules! options {
        () => {{
            Options {
                version: unarm::Version::V6K,
                av: false,
                r9_use: unarm::R9Use::R9,
                sl: false,
                fp: false,
                ip: false,
                ual: true,
            }
        }};
    }

    macro_rules! assert_asm {
        ($code:literal, $disasm:literal) => {{
            let ins = parse_thumb($code, None);
            let options = options!();
            let s = ins.display(&options).to_string();
            assert_eq!(s, $disasm)
        }};
        ($code:literal, $next:literal, $disasm:literal) => {{
            let ins = parse_thumb($code, Some($next));
            let options = options!();
            let s = ins.display(&options).to_string();
            assert_eq!(s, $disasm)
        }};
    }

    #[test]
    fn test_adc() {
        assert_asm!(0x4157, "adcs r7, r7, r2");
    }
}
