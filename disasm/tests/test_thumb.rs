use armv5te::thumb::{Ins, ParsedIns};

macro_rules! assert_asm {
    ($code:literal, $disasm:literal) => {{
        let ins = Ins::new($code);
        let parsed = ParsedIns::parse(ins);
        assert_eq!(parsed.to_string(), $disasm)
    }};
}

macro_rules! assert_bl {
    ($code:literal, $disasm:literal) => {{
        let first = Ins::new($code >> 16);
        assert!(first.is_half_bl());
        let second = Ins::new($code & 0xffff);
        let first = ParsedIns::parse(first);
        let second = ParsedIns::parse(second);
        let ins = first.combine_bl(&second);
        assert_eq!(ins.to_string(), $disasm);
    }};
}

#[test]
fn test_adc() {
    assert_asm!(0x4157, "adc r7, r2");
}

#[test]
fn test_add() {
    assert_asm!(0x1cca, "add r2, r1, #0x3");
    assert_asm!(0x3642, "add r6, #0x42");
    assert_asm!(0x1853, "add r3, r2, r1");
    assert_asm!(0x44de, "add lr, fp");
    assert_asm!(0xa413, "add r4, pc, #0x4c");
    assert_asm!(0xacff, "add r4, sp, #0x3fc");
    assert_asm!(0xb03a, "add sp, #0xe8");
}

#[test]
fn test_and() {
    assert_asm!(0x4017, "and r7, r2");
}

#[test]
fn test_asr() {
    assert_asm!(0x1023, "asr r3, r4, #0x20");
    assert_asm!(0x1163, "asr r3, r4, #0x5");
    assert_asm!(0x4117, "asr r7, r2");
}

#[test]
fn test_b() {
    assert_asm!(0xd042, "beq #0x88");
    assert_asm!(0xd942, "bls #0x88");
    assert_asm!(0xdc42, "bgt #0x88");
}

#[test]
fn test_bic() {
    assert_asm!(0x4397, "bic r7, r2");
}

#[test]
fn test_bkpt() {
    assert_asm!(0xde42, "bkpt #0x42");
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
    assert_asm!(0x45de, "cmp lr, fp");
}

#[test]
fn test_eor() {
    assert_asm!(0x4057, "eor r7, r2");
}

#[test]
fn test_ldmia() {
    assert_asm!(0xc955, "ldmia r1!, {r0, r2, r4, r6}");
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
    assert_asm!(0x7c22, "ldrb r2, [r4, #0x40]");
    assert_asm!(0x5c22, "ldrb r2, [r4, r0]");
}

#[test]
fn test_ldrh() {
    assert_asm!(0x8c22, "ldrh r2, [r4, #0x40]");
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
    assert_asm!(0x0163, "lsl r3, r4, #0x5");
    assert_asm!(0x4097, "lsl r7, r2");
}

#[test]
fn test_lsr() {
    assert_asm!(0x0823, "lsr r3, r4, #0x20");
    assert_asm!(0x0963, "lsr r3, r4, #0x5");
    assert_asm!(0x40d7, "lsr r7, r2");
}

#[test]
fn test_mov() {
    assert_asm!(0x2163, "mov r1, #0x63");
    assert_asm!(0x1c17, "mov r7, r2");
}

#[test]
fn test_mul() {
    assert_asm!(0x4357, "mul r7, r2");
}

#[test]
fn test_mvn() {
    assert_asm!(0x43d7, "mvn r7, r2");
}

#[test]
fn test_neg() {
    assert_asm!(0x4257, "neg r7, r2");
}

#[test]
fn test_orr() {
    assert_asm!(0x4317, "orr r7, r2");
}

#[test]
fn test_pop() {
    assert_asm!(0xbdff, "pop {r0, r1, r2, r3, r4, r5, r6, r7, pc}");
}

#[test]
fn test_push() {
    assert_asm!(0xb5ff, "push {r0, r1, r2, r3, r4, r5, r6, r7, pc}");
}

#[test]
fn test_ror() {
    assert_asm!(0x41d7, "ror r7, r2");
}

#[test]
fn test_sbc() {
    assert_asm!(0x4197, "sbc r7, r2");
}

#[test]
fn test_stmia() {
    assert_asm!(0xc155, "stmia r1!, {r0, r2, r4, r6}");
}

#[test]
fn test_str() {
    assert_asm!(0x6422, "str r2, [r4, #0x40]");
    assert_asm!(0x5022, "str r2, [r4, r0]");
    assert_asm!(0x9742, "str r7, [sp, #0x108]");
}

#[test]
fn test_strb() {
    assert_asm!(0x7422, "strb r2, [r4, #0x40]");
    assert_asm!(0x5422, "strb r2, [r4, r0]");
}

#[test]
fn test_strh() {
    assert_asm!(0x8422, "strh r2, [r4, #0x40]");
    assert_asm!(0x5222, "strh r2, [r4, r0]");
}

#[test]
fn test_sub() {
    assert_asm!(0x1eca, "sub r2, r1, #0x3");
    assert_asm!(0x3e42, "sub r6, #0x42");
    assert_asm!(0x1a53, "sub r3, r2, r1");
    assert_asm!(0xb0ff, "sub sp, #0x1fc");
}

#[test]
fn test_swi() {
    assert_asm!(0xdf42, "swi #0x42");
}

#[test]
fn test_tst() {
    assert_asm!(0x4217, "tst r7, r2");
}
