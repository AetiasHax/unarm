#[cfg(feature = "v5te")]
mod tests {
    use unarm::{thumb::Ins, ArmVersion, ParseFlags};

    macro_rules! assert_asm {
        ($code:literal, $disasm:literal) => {{
            let flags = ParseFlags {
                version: ArmVersion::V5Te,
                ..Default::default()
            };
            let ins = Ins::new($code, &flags);
            let parsed = ins.parse(&flags);
            assert_eq!(parsed.display(Default::default()).to_string(), $disasm)
        }};
    }

    macro_rules! assert_bl {
        ($code:literal, $disasm:literal) => {{
            let flags = Default::default();
            let first = Ins::new($code >> 16, &flags);
            assert!(first.is_half_bl());
            let second = Ins::new($code & 0xffff, &flags);
            let first = first.parse(&flags);
            let second = second.parse(&flags);
            let ins = first.combine_thumb_bl(&second);
            assert_eq!(ins.display(Default::default()).to_string(), $disasm);
        }};
    }

    #[test]
    fn test_adc() {
        assert_asm!(0x4157, "adcs r7, r7, r2");
    }

    #[test]
    fn test_add() {
        assert_asm!(0x1cca, "adds r2, r1, #0x3");
        assert_asm!(0x3642, "adds r6, #0x42");
        assert_asm!(0x1853, "adds r3, r2, r1");
        assert_asm!(0x44de, "add lr, lr, r11");
        assert_asm!(0xacff, "add r4, sp, #0x3fc");
        assert_asm!(0xb03a, "add sp, sp, #0xe8");
        assert_asm!(0x1c17, "adds r7, r2, #0x0");
    }

    #[test]
    fn test_adr() {
        assert_asm!(0xa413, "adr r4, #0x4c");
    }

    #[test]
    fn test_and() {
        assert_asm!(0x4017, "ands r7, r7, r2");
    }

    #[test]
    fn test_asr() {
        assert_asm!(0x1023, "asrs r3, r4, #0x20");
        assert_asm!(0x1163, "asrs r3, r4, #0x5");
        assert_asm!(0x4117, "asrs r7, r7, r2");
    }

    #[test]
    fn test_b() {
        assert_asm!(0xd042, "beq #0x88");
        assert_asm!(0xd942, "bls #0x88");
        assert_asm!(0xdc42, "bgt #0x88");
        assert_asm!(0xdbf3, "blt #-0x16");
        assert_asm!(0xe5ee, "b #-0x420");
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
        assert_bl!(0xf099f866, "bl #0x990d0");
        assert_bl!(0xf799f866, "bl #-0x66f30");
    }

    #[test]
    fn test_blx() {
        assert_bl!(0xf099e866, "blx #0x990d0");
        assert_bl!(0xf799e866, "blx #-0x66f30");
        assert_bl!(0xf000e800, "blx #0x4");
        assert_bl!(0xf000e801, "blx #0x4");
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
    fn test_eor() {
        assert_asm!(0x4057, "eors r7, r7, r2");
    }

    #[test]
    fn test_ldm() {
        assert_asm!(0xc955, "ldm r1!, {r0, r2, r4, r6}");
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

    #[test]
    fn test_ldrh() {
        assert_asm!(0x8c22, "ldrh r2, [r4, #0x20]");
        assert_asm!(0x5a22, "ldrh r2, [r4, r0]");
    }

    #[test]
    fn test_ldrsb() {
        assert_asm!(0x5622, "ldrsb r2, [r4, r0]");
    }

    #[test]
    fn test_ldrsh() {
        assert_asm!(0x5e22, "ldrsh r2, [r4, r0]");
    }

    #[test]
    fn test_lsl() {
        assert_asm!(0x0163, "lsls r3, r4, #0x5");
        assert_asm!(0x4097, "lsls r7, r7, r2");
    }

    #[test]
    fn test_lsr() {
        assert_asm!(0x0823, "lsrs r3, r4, #0x20");
        assert_asm!(0x0963, "lsrs r3, r4, #0x5");
        assert_asm!(0x40d7, "lsrs r7, r7, r2");
    }

    #[test]
    fn test_mov() {
        assert_asm!(0x2163, "movs r1, #0x63");
        assert_asm!(0x0017, "movs r7, r2");
    }

    #[test]
    fn test_mul() {
        assert_asm!(0x4357, "muls r7, r7, r2");
    }

    #[test]
    fn test_mvn() {
        assert_asm!(0x43d7, "mvns r7, r2");
    }

    #[test]
    fn test_orr() {
        assert_asm!(0x4317, "orrs r7, r7, r2");
    }

    #[test]
    fn test_pop() {
        assert_asm!(0xbdff, "pop {r0, r1, r2, r3, r4, r5, r6, r7, pc}");
    }

    #[test]
    fn test_push() {
        assert_asm!(0xb5ff, "push {r0, r1, r2, r3, r4, r5, r6, r7, lr}");
    }

    #[test]
    fn test_ror() {
        assert_asm!(0x41d7, "rors r7, r7, r2");
    }

    #[test]
    fn test_rsb() {
        assert_asm!(0x4257, "rsbs r7, r2, #0x0");
    }

    #[test]
    fn test_sbc() {
        assert_asm!(0x4197, "sbcs r7, r7, r2");
    }

    #[test]
    fn test_stm() {
        assert_asm!(0xc155, "stm r1!, {r0, r2, r4, r6}");
    }

    #[test]
    fn test_str() {
        assert_asm!(0x6422, "str r2, [r4, #0x40]");
        assert_asm!(0x5022, "str r2, [r4, r0]");
        assert_asm!(0x9742, "str r7, [sp, #0x108]");
    }

    #[test]
    fn test_strb() {
        assert_asm!(0x7422, "strb r2, [r4, #0x10]");
        assert_asm!(0x5422, "strb r2, [r4, r0]");
    }

    #[test]
    fn test_strh() {
        assert_asm!(0x8422, "strh r2, [r4, #0x20]");
        assert_asm!(0x5222, "strh r2, [r4, r0]");
    }

    #[test]
    fn test_sub() {
        assert_asm!(0x1eca, "subs r2, r1, #0x3");
        assert_asm!(0x3e42, "subs r6, #0x42");
        assert_asm!(0x1a53, "subs r3, r2, r1");
        assert_asm!(0xb0ff, "sub sp, sp, #0x1fc");
    }

    #[test]
    fn test_svc() {
        assert_asm!(0xdf42, "svc #0x42");
    }

    #[test]
    fn test_tst() {
        assert_asm!(0x4217, "tst r7, r2");
    }

    #[test]
    fn test_udf() {
        assert_asm!(0xde42, "udf #0x42");
    }
}
