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

    #[test]
    fn test_bic() {
        assert_asm!(0x4397, "bics r7, r7, r2");
    }

    #[test]
    fn test_bkpt() {
        assert_asm!(0xbe42, "bkpt #0x42");
    }

    #[test]
    fn test_bl() {
        assert_asm!(0xf099, 0xf866, "bl #0x990d0");
        assert_asm!(0xf799, 0xf866, "bl #0xfff990d0");
    }

    #[test]
    fn test_blx() {
        assert_asm!(0xf099, 0xe866, "blx #0x990d0");
        assert_asm!(0xf799, 0xe866, "blx #0xfff990d0");
        assert_asm!(0xf000, 0xe800, "blx #0x4");
        assert_asm!(0xf000, 0xe801, "blx #0x4");
        assert_asm!(0x47d0, "blx r10");
    }

    #[test]
    fn test_bx() {
        assert_asm!(0x4750, "bx r10");
    }

    #[test]
    fn test_cmn() {
        assert_asm!(0x42d7, "cmn r7, r2");
    }

    #[test]
    fn test_cmp() {
        assert_asm!(0x2942, "cmp r1, #0x42");
        assert_asm!(0x4297, "cmp r7, r2");
        assert_asm!(0x45de, "cmp lr, r11");
    }

    #[test]
    fn test_cps() {
        assert_asm!(0xb677, "cpsid aif");
        assert_asm!(0xb667, "cpsie aif");
    }

    #[test]
    fn test_eor() {
        assert_asm!(0x4057, "eors r7, r7, r2");
    }

    #[test]
    fn test_ldm() {
        assert_asm!(0xc955, "ldm r1!, {r0, r2, r4, r6}");
        assert_asm!(0xc9aa, "ldm r1, {r1, r3, r5, r7}");
    }

    #[test]
    fn test_ldr() {
        assert_asm!(0x6c22, "ldr r2, [r4, #0x40]");
        assert_asm!(0x5822, "ldr r2, [r4, r0]");
        assert_asm!(0x4f42, "ldr r7, [pc, #0x108]");
        assert_asm!(0x9f42, "ldr r7, [sp, #0x108]");
    }

    #[test]
    fn test_ldrb() {
        assert_asm!(0x7c22, "ldrb r2, [r4, #0x10]");
        assert_asm!(0x5c22, "ldrb r2, [r4, r0]");
    }
}
