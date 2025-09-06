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
            let ins = parse_thumb($code, None, 0);
            let options = options!();
            let s = ins.display(&options).to_string();
            assert_eq!(s, $disasm)
        }};
        ($code:literal, $next:literal, $disasm:literal) => {{
            let ins = parse_thumb($code, Some($next), 0);
            let options = options!();
            let s = ins.display(&options).to_string();
            assert_eq!(s, $disasm)
        }};
    }

    #[test]
    fn test_adc() {
        assert_asm!(0x4157, "adcs r7, r7, r2");
    }

    #[test]
    fn test_add() {
        assert_asm!(0x1cca, "adds r2, r1, #0x3");
        assert_asm!(0x3642, "adds r6, r6, #0x42");
        assert_asm!(0x1853, "adds r3, r2, r1");
        assert_asm!(0x44de, "add lr, lr, r11");
        assert_asm!(0xacff, "add r4, sp, #0x3fc");
        assert_asm!(0xb03a, "add sp, sp, #0xe8");
        assert_asm!(0xa413, "add r4, pc, #0x4c");
    }

    #[test]
    fn test_and() {
        assert_asm!(0x4017, "ands r7, r7, r2");
    }

    #[test]
    fn test_b() {
        assert_asm!(0xd042, "beq #0x88");
        assert_asm!(0xd942, "bls #0x88");
        assert_asm!(0xdc42, "bgt #0x88");
        assert_asm!(0xdbf3, "blt #0xffffffea");
        assert_asm!(0xe5ee, "b #0xfffffbe0");
    }
}
