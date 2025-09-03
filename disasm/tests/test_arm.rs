#[cfg(test)]
mod tests {
    use unarm::{Options, parse_arm};

    macro_rules! assert_asm {
        ($code:literal, $disasm:literal) => {{
            let ins = parse_arm($code);
            let options = Options {
                version: unarm::Version::V6K,
                av: false,
                r9_use: unarm::R9Use::R9,
                sl: false,
                fp: false,
                ip: false,
                ual: true,
            };
            let s = ins.display(&options).to_string();
            assert_eq!(s, $disasm)
        }};
    }

    #[test]
    fn test_adc() {
        assert_asm!(0xe0a12003, "adc r2, r1, r3");
        assert_asm!(0xe2a45e23, "adc r5, r4, #0x230");
        assert_asm!(0x10ab960a, "adcne r9, r11, r10, lsl #0xc");
        assert_asm!(0x40a5f238, "adcmi pc, r5, r8, lsr r2");
        assert_asm!(0x70a2046e, "adcvc r0, r2, lr, ror #0x8");
        assert_asm!(0xb0a87060, "adclt r7, r8, r0, rrx");
        assert_asm!(0xe0b52153, "adcs r2, r5, r3, asr r1");
    }
}
