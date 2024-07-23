use unarm::{arm, thumb, ArmVersion, ParseFlags};

macro_rules! assert_arm {
    ($code:literal, $flags:expr, $disasm:literal) => {{
        let ins = arm::Ins::new($code, $flags);
        let parsed = ins.parse($flags);
        assert_eq!(parsed.display(Default::default()).to_string(), $disasm)
    }};
}

macro_rules! assert_thumb {
    ($code:literal, $flags:expr, $disasm:literal) => {{
        let ins = thumb::Ins::new($code, $flags);
        let parsed = ins.parse($flags);
        assert_eq!(parsed.display(Default::default()).to_string(), $disasm)
    }};
}

const UNIFIED_FLAGS: ParseFlags = ParseFlags {
    ual: true,
    version: ArmVersion::V6K,
};

const DIVIDED_FLAGS: ParseFlags = ParseFlags {
    ual: false,
    version: ArmVersion::V6K,
};

#[test]
fn test_data() {
    assert_arm!(0x00912003, &UNIFIED_FLAGS, "addseq r2, r1, r3");
    assert_arm!(0x00912003, &DIVIDED_FLAGS, "addeqs r2, r1, r3");

    assert_thumb!(0x1cca, &UNIFIED_FLAGS, "adds r2, r1, #0x3");
    assert_thumb!(0x1cca, &DIVIDED_FLAGS, "add r2, r1, #0x3");

    assert_thumb!(0xa1ff, &UNIFIED_FLAGS, "adr r1, #0x3fc");
    assert_thumb!(0xa1ff, &DIVIDED_FLAGS, "add r1, pc, #0x3fc");

    assert_thumb!(0x400a, &UNIFIED_FLAGS, "ands r2, r2, r1");
    assert_thumb!(0x400a, &DIVIDED_FLAGS, "and r2, r1");

    assert_thumb!(0x1c0a, &UNIFIED_FLAGS, "adds r2, r1, #0x0");
    assert_thumb!(0x1c0a, &DIVIDED_FLAGS, "mov r2, r1");

    assert_thumb!(0x000a, &UNIFIED_FLAGS, "movs r2, r1");
    assert_thumb!(0x000a, &DIVIDED_FLAGS, "lsl r2, r1, #0x0");

    assert_thumb!(0x424a, &UNIFIED_FLAGS, "rsbs r2, r1, #0x0");
    assert_thumb!(0x424a, &DIVIDED_FLAGS, "neg r2, r1");
}

#[test]
fn test_shift() {
    assert_arm!(0xe1a011c2, &UNIFIED_FLAGS, "asr r1, r2, #0x3");
    assert_arm!(0xe1a011c2, &DIVIDED_FLAGS, "mov r1, r2, asr #0x3");
    assert_arm!(0xe1a01182, &UNIFIED_FLAGS, "lsl r1, r2, #0x3");
    assert_arm!(0xe1a01182, &DIVIDED_FLAGS, "mov r1, r2, lsl #0x3");
    assert_arm!(0xe1a011a2, &UNIFIED_FLAGS, "lsr r1, r2, #0x3");
    assert_arm!(0xe1a011a2, &DIVIDED_FLAGS, "mov r1, r2, lsr #0x3");
    assert_arm!(0xe1a011e2, &UNIFIED_FLAGS, "ror r1, r2, #0x3");
    assert_arm!(0xe1a011e2, &DIVIDED_FLAGS, "mov r1, r2, ror #0x3");
    assert_arm!(0xe1a01062, &UNIFIED_FLAGS, "rrx r1, r2");
    assert_arm!(0xe1a01062, &DIVIDED_FLAGS, "mov r1, r2, rrx");
}

#[test]
fn test_ldm_stm() {
    assert_arm!(0xe8900011, &UNIFIED_FLAGS, "ldm r0, {r0, r4}");
    assert_arm!(0xe8900011, &DIVIDED_FLAGS, "ldmia r0, {r0, r4}");
    assert_arm!(0x09100011, &UNIFIED_FLAGS, "ldmdbeq r0, {r0, r4}");
    assert_arm!(0x09100011, &DIVIDED_FLAGS, "ldmeqdb r0, {r0, r4}");

    assert_arm!(0xe8800011, &UNIFIED_FLAGS, "stm r0, {r0, r4}");
    assert_arm!(0xe8800011, &DIVIDED_FLAGS, "stmia r0, {r0, r4}");
    assert_arm!(0x09000011, &UNIFIED_FLAGS, "stmdbeq r0, {r0, r4}");
    assert_arm!(0x09000011, &DIVIDED_FLAGS, "stmeqdb r0, {r0, r4}");

    assert_thumb!(0xc811, &UNIFIED_FLAGS, "ldm r0, {r0, r4}");
    assert_thumb!(0xc811, &DIVIDED_FLAGS, "ldmia r0!, {r0, r4}");

    assert_thumb!(0xc822, &UNIFIED_FLAGS, "ldm r0!, {r1, r5}");
    assert_thumb!(0xc822, &DIVIDED_FLAGS, "ldmia r0!, {r1, r5}");

    assert_thumb!(0xc011, &UNIFIED_FLAGS, "stm r0!, {r0, r4}");
    assert_thumb!(0xc011, &DIVIDED_FLAGS, "stmia r0!, {r0, r4}");
}

#[test]
fn test_ldr_str() {
    assert_arm!(0x01d120b4, &UNIFIED_FLAGS, "ldrheq r2, [r1, #0x4]");
    assert_arm!(0x01d120b4, &DIVIDED_FLAGS, "ldreqh r2, [r1, #0x4]");
    assert_arm!(0x01d120d4, &UNIFIED_FLAGS, "ldrsbeq r2, [r1, #0x4]");
    assert_arm!(0x01d120d4, &DIVIDED_FLAGS, "ldreqsb r2, [r1, #0x4]");
    assert_arm!(0x01c120d4, &UNIFIED_FLAGS, "ldrdeq r2, r3, [r1, #0x4]");
    assert_arm!(0x01c120d4, &DIVIDED_FLAGS, "ldreqd r2, [r1, #0x4]");

    assert_arm!(0x01c120b4, &UNIFIED_FLAGS, "strheq r2, [r1, #0x4]");
    assert_arm!(0x01c120b4, &DIVIDED_FLAGS, "streqh r2, [r1, #0x4]");
    assert_arm!(0x04e12004, &UNIFIED_FLAGS, "strbteq r2, [r1], #0x4");
    assert_arm!(0x04e12004, &DIVIDED_FLAGS, "streqbt r2, [r1], #0x4");
    assert_arm!(0x01c120f4, &UNIFIED_FLAGS, "strdeq r2, r3, [r1, #0x4]");
    assert_arm!(0x01c120f4, &DIVIDED_FLAGS, "streqd r2, [r1, #0x4]");
}

#[test]
fn test_push_pop() {
    assert_arm!(0xe92d0011, &UNIFIED_FLAGS, "push {r0, r4}");
    assert_arm!(0xe92d0011, &DIVIDED_FLAGS, "stmdb sp!, {r0, r4}");
    assert_arm!(0xe52d3004, &UNIFIED_FLAGS, "push {r3}");
    assert_arm!(0xe52d3004, &DIVIDED_FLAGS, "str r3, [sp, #-0x4]!");

    assert_arm!(0xe8bd0011, &UNIFIED_FLAGS, "pop {r0, r4}");
    assert_arm!(0xe8bd0011, &DIVIDED_FLAGS, "ldmia sp!, {r0, r4}");
    assert_arm!(0xe49d3004, &UNIFIED_FLAGS, "pop {r3}");
    assert_arm!(0xe49d3004, &DIVIDED_FLAGS, "ldr r3, [sp], #0x4");
}

#[test]
fn test_svc_swi() {
    assert_arm!(0xef000123, &UNIFIED_FLAGS, "svc #0x123");
    assert_arm!(0xef000123, &DIVIDED_FLAGS, "swi #0x123");
}
