use unarm::{v5te::arm::Ins, ParseFlags};

macro_rules! assert_asm {
    ($code:literal, $flags:expr, $disasm:literal) => {{
        let ins = Ins::new($code, $flags);
        let parsed = ins.parse($flags);
        assert_eq!(parsed.display(Default::default()).to_string(), $disasm)
    }};
}

#[test]
fn test_data() {
    let unified = ParseFlags { ual: true };
    let divided = ParseFlags { ual: false };

    assert_asm!(0x00912003, &unified, "addseq r2, r1, r3");
    assert_asm!(0x00912003, &divided, "addeqs r2, r1, r3");
}

#[test]
fn test_shift() {
    let unified = ParseFlags { ual: true };
    let divided = ParseFlags { ual: false };

    assert_asm!(0xe1a011c2, &unified, "asr r1, r2, #0x3");
    assert_asm!(0xe1a011c2, &divided, "mov r1, r2, asr #0x3");
    assert_asm!(0xe1a01182, &unified, "lsl r1, r2, #0x3");
    assert_asm!(0xe1a01182, &divided, "mov r1, r2, lsl #0x3");
    assert_asm!(0xe1a011a2, &unified, "lsr r1, r2, #0x3");
    assert_asm!(0xe1a011a2, &divided, "mov r1, r2, lsr #0x3");
    assert_asm!(0xe1a011e2, &unified, "ror r1, r2, #0x3");
    assert_asm!(0xe1a011e2, &divided, "mov r1, r2, ror #0x3");
    assert_asm!(0xe1a01062, &unified, "rrx r1, r2");
    assert_asm!(0xe1a01062, &divided, "mov r1, r2, rrx");
}

#[test]
fn test_ldm_stm() {
    let unified = ParseFlags { ual: true };
    let divided = ParseFlags { ual: false };

    assert_asm!(0xe8900011, &unified, "ldm r0, {r0, r4}");
    assert_asm!(0xe8900011, &divided, "ldmia r0, {r0, r4}");
    assert_asm!(0x09100011, &unified, "ldmdbeq r0, {r0, r4}");
    assert_asm!(0x09100011, &divided, "ldmeqdb r0, {r0, r4}");

    assert_asm!(0xe8800011, &unified, "stm r0, {r0, r4}");
    assert_asm!(0xe8800011, &divided, "stmia r0, {r0, r4}");
    assert_asm!(0x09000011, &unified, "stmdbeq r0, {r0, r4}");
    assert_asm!(0x09000011, &divided, "stmeqdb r0, {r0, r4}");
}

#[test]
fn test_ldr_str() {
    let unified = ParseFlags { ual: true };
    let divided = ParseFlags { ual: false };

    assert_asm!(0x01d120b4, &unified, "ldrheq r2, [r1, #0x4]");
    assert_asm!(0x01d120b4, &divided, "ldreqh r2, [r1, #0x4]");
    assert_asm!(0x01d120d4, &unified, "ldrsbeq r2, [r1, #0x4]");
    assert_asm!(0x01d120d4, &divided, "ldreqsb r2, [r1, #0x4]");

    assert_asm!(0x01c120b4, &unified, "strheq r2, [r1, #0x4]");
    assert_asm!(0x01c120b4, &divided, "streqh r2, [r1, #0x4]");
    assert_asm!(0x04e12004, &unified, "strbteq r2, [r1], #0x4");
    assert_asm!(0x04e12004, &divided, "streqbt r2, [r1], #0x4");
}

#[test]
fn test_push_pop() {
    let unified = ParseFlags { ual: true };
    let divided = ParseFlags { ual: false };

    assert_asm!(0xe92d0011, &unified, "push {r0, r4}");
    assert_asm!(0xe92d0011, &divided, "stmdb sp!, {r0, r4}");
    assert_asm!(0xe52d3004, &unified, "push {r3}");
    assert_asm!(0xe52d3004, &divided, "str r3, [sp, #-0x4]!");

    assert_asm!(0xe8bd0011, &unified, "pop {r0, r4}");
    assert_asm!(0xe8bd0011, &divided, "ldmia sp!, {r0, r4}");
    assert_asm!(0xe49d3004, &unified, "pop {r3}");
    assert_asm!(0xe49d3004, &divided, "ldr r3, [sp], #0x4");
}

#[test]
fn test_svc_swi() {
    let unified = ParseFlags { ual: true };
    let divided = ParseFlags { ual: false };

    assert_asm!(0xef000123, &unified, "svc #0x123");
    assert_asm!(0xef000123, &divided, "swi #0x123");
}
