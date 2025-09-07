#[cfg(test)]
mod tests {
    use unarm::{Options, parse_arm};

    macro_rules! assert_asm {
        ($code:literal, $disasm:literal) => {{
            let ins = parse_arm($code, 0);
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

    #[test]
    fn test_add() {
        assert_asm!(0xe0812003, "add r2, r1, r3");
        assert_asm!(0xe2845e23, "add r5, r4, #0x230");
        assert_asm!(0x108b960a, "addne r9, r11, r10, lsl #0xc");
        assert_asm!(0x4085f238, "addmi pc, r5, r8, lsr r2");
        assert_asm!(0x7082046e, "addvc r0, r2, lr, ror #0x8");
        assert_asm!(0xb0887060, "addlt r7, r8, r0, rrx");
        assert_asm!(0xe0952153, "adds r2, r5, r3, asr r1");
        assert_asm!(0xe28f41a5, "add r4, pc, #0x40000029");
    }

    #[test]
    fn test_and() {
        assert_asm!(0xe0012003, "and r2, r1, r3");
        assert_asm!(0xe2045e23, "and r5, r4, #0x230");
        assert_asm!(0x100b960a, "andne r9, r11, r10, lsl #0xc");
        assert_asm!(0x4005f238, "andmi pc, r5, r8, lsr r2");
        assert_asm!(0x7002046e, "andvc r0, r2, lr, ror #0x8");
        assert_asm!(0xb0087060, "andlt r7, r8, r0, rrx");
        assert_asm!(0xe0152153, "ands r2, r5, r3, asr r1");
    }

    #[test]
    fn test_b() {
        assert_asm!(0xea000000, "b #0x8");
        assert_asm!(0x0a012345, "beq #0x48d1c");
        assert_asm!(0x1affffff, "bne #0x4");
        assert_asm!(0x2afffffe, "bhs #0x0");
        assert_asm!(0x3afffffd, "blo #0xfffffffc");
    }

    #[test]
    fn test_bic() {
        assert_asm!(0xe1c12003, "bic r2, r1, r3");
        assert_asm!(0xe3c45e23, "bic r5, r4, #0x230");
        assert_asm!(0x11cb960a, "bicne r9, r11, r10, lsl #0xc");
        assert_asm!(0x41c5f238, "bicmi pc, r5, r8, lsr r2");
        assert_asm!(0x71c2046e, "bicvc r0, r2, lr, ror #0x8");
        assert_asm!(0xb1c87060, "biclt r7, r8, r0, rrx");
        assert_asm!(0xe1d52153, "bics r2, r5, r3, asr r1");
    }

    #[test]
    fn test_bkpt() {
        assert_asm!(0xe1200070, "bkpt #0x0");
        assert_asm!(0xe1243271, "bkpt #0x4321");
    }

    #[test]
    fn test_bl() {
        assert_asm!(0xeb000000, "bl #0x8");
        assert_asm!(0x0b012345, "bleq #0x48d1c");
        assert_asm!(0x1bffffff, "blne #0x4");
        assert_asm!(0x2bfffffe, "blhs #0x0");
        assert_asm!(0x3bfffffd, "bllo #0xfffffffc");
    }

    #[test]
    fn test_blx() {
        assert_asm!(0xfa000000, "blx #0x8");
        assert_asm!(0xfa012345, "blx #0x48d1c");
        assert_asm!(0xfaffffff, "blx #0x4");
        assert_asm!(0xfafffffe, "blx #0x0");
        assert_asm!(0xfafffffd, "blx #0xfffffffc");
        assert_asm!(0xe12fff30, "blx r0");
        assert_asm!(0x512fff35, "blxpl r5");
        assert_asm!(0xfb000000, "blx #0xa");
    }

    #[test]
    fn test_bx() {
        assert_asm!(0xe12fff10, "bx r0");
        assert_asm!(0x512fff15, "bxpl r5");
    }

    #[test]
    fn test_bxj() {
        assert_asm!(0xe12fff20, "bxj r0");
        assert_asm!(0x512fff25, "bxjpl r5");
    }

    #[test]
    fn test_cdp() {
        assert_asm!(0xee1234a6, "cdp p4, #0x1, c3, c2, c6, #0x5");
        assert_asm!(0xae654341, "cdpge p3, #0x6, c4, c5, c1, #0x2");
    }

    #[test]
    fn test_cdp2() {
        assert_asm!(0xfe1234a6, "cdp2 p4, #0x1, c3, c2, c6, #0x5");
        assert_asm!(0xfe654341, "cdp2 p3, #0x6, c4, c5, c1, #0x2");
    }

    #[test]
    fn test_clrex() {
        assert_asm!(0xf57ff01f, "clrex");
    }
}
