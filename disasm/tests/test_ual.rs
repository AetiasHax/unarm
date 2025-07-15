use unarm::{arm, thumb, ParseFlags};

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

#[test]
fn test_data() {
    let unified = ParseFlags {
        ual: true,
        ..Default::default()
    };
    let divided = ParseFlags {
        ual: false,
        ..Default::default()
    };

    assert_arm!(0x00912003, &unified, "addseq r2, r1, r3");
    assert_arm!(0x00912003, &divided, "addeqs r2, r1, r3");

    assert_thumb!(0x1cca, &unified, "adds r2, r1, #0x3");
    assert_thumb!(0x1cca, &divided, "add r2, r1, #0x3");

    assert_thumb!(0xa1ff, &unified, "adr r1, #0x3fc");
    assert_thumb!(0xa1ff, &divided, "add r1, pc, #0x3fc");

    assert_thumb!(0x400a, &unified, "ands r2, r2, r1");
    assert_thumb!(0x400a, &divided, "and r2, r1");

    assert_thumb!(0x1c0a, &unified, "adds r2, r1, #0x0");
    assert_thumb!(0x1c0a, &divided, "mov r2, r1");

    assert_thumb!(0x000a, &unified, "movs r2, r1");
    assert_thumb!(0x000a, &divided, "lsl r2, r1, #0x0");

    assert_thumb!(0x424a, &unified, "rsbs r2, r1, #0x0");
    assert_thumb!(0x424a, &divided, "neg r2, r1");
}

#[test]
fn test_shift() {
    let unified = ParseFlags {
        ual: true,
        ..Default::default()
    };
    let divided = ParseFlags {
        ual: false,
        ..Default::default()
    };

    assert_arm!(0xe1a011c2, &unified, "asr r1, r2, #0x3");
    assert_arm!(0xe1a011c2, &divided, "mov r1, r2, asr #0x3");
    assert_arm!(0xe1a01352, &unified, "asr r1, r2, r3");
    assert_arm!(0xe1a01352, &divided, "mov r1, r2, asr r3");

    assert_arm!(0xe1a01182, &unified, "lsl r1, r2, #0x3");
    assert_arm!(0xe1a01182, &divided, "mov r1, r2, lsl #0x3");
    assert_arm!(0xe1a01312, &unified, "lsl r1, r2, r3");
    assert_arm!(0xe1a01312, &divided, "mov r1, r2, lsl r3");

    assert_arm!(0xe1a011a2, &unified, "lsr r1, r2, #0x3");
    assert_arm!(0xe1a011a2, &divided, "mov r1, r2, lsr #0x3");
    assert_arm!(0xe1a01332, &unified, "lsr r1, r2, r3");
    assert_arm!(0xe1a01332, &divided, "mov r1, r2, lsr r3");

    assert_arm!(0xe1a011e2, &unified, "ror r1, r2, #0x3");
    assert_arm!(0xe1a011e2, &divided, "mov r1, r2, ror #0x3");
    assert_arm!(0xe1a01372, &unified, "ror r1, r2, r3");
    assert_arm!(0xe1a01372, &divided, "mov r1, r2, ror r3");

    assert_arm!(0xe1a01062, &unified, "rrx r1, r2");
    assert_arm!(0xe1a01062, &divided, "mov r1, r2, rrx");
}

#[test]
fn test_ldm_stm() {
    let unified = ParseFlags {
        ual: true,
        ..Default::default()
    };
    let divided = ParseFlags {
        ual: false,
        ..Default::default()
    };

    assert_arm!(0xe8900011, &unified, "ldm r0, {r0, r4}");
    assert_arm!(0xe8900011, &divided, "ldmia r0, {r0, r4}");
    assert_arm!(0x09100011, &unified, "ldmdbeq r0, {r0, r4}");
    assert_arm!(0x09100011, &divided, "ldmeqdb r0, {r0, r4}");

    assert_arm!(0xe8800011, &unified, "stm r0, {r0, r4}");
    assert_arm!(0xe8800011, &divided, "stmia r0, {r0, r4}");
    assert_arm!(0x09000011, &unified, "stmdbeq r0, {r0, r4}");
    assert_arm!(0x09000011, &divided, "stmeqdb r0, {r0, r4}");

    assert_thumb!(0xc811, &unified, "ldm r0, {r0, r4}");
    assert_thumb!(0xc811, &divided, "ldmia r0!, {r0, r4}");

    assert_thumb!(0xc822, &unified, "ldm r0!, {r1, r5}");
    assert_thumb!(0xc822, &divided, "ldmia r0!, {r1, r5}");

    assert_thumb!(0xc011, &unified, "stm r0!, {r0, r4}");
    assert_thumb!(0xc011, &divided, "stmia r0!, {r0, r4}");
}

#[test]
fn test_ldr_str() {
    let unified = ParseFlags {
        ual: true,
        ..Default::default()
    };
    let divided = ParseFlags {
        ual: false,
        ..Default::default()
    };

    assert_arm!(0x01d120b4, &unified, "ldrheq r2, [r1, #0x4]");
    assert_arm!(0x01d120b4, &divided, "ldreqh r2, [r1, #0x4]");
    assert_arm!(0x01d120d4, &unified, "ldrsbeq r2, [r1, #0x4]");
    assert_arm!(0x01d120d4, &divided, "ldreqsb r2, [r1, #0x4]");
    assert_arm!(0x01c120d4, &unified, "ldrdeq r2, r3, [r1, #0x4]");
    assert_arm!(0x01c120d4, &divided, "ldreqd r2, [r1, #0x4]");

    assert_arm!(0x01c120b4, &unified, "strheq r2, [r1, #0x4]");
    assert_arm!(0x01c120b4, &divided, "streqh r2, [r1, #0x4]");
    assert_arm!(0x04e12004, &unified, "strbteq r2, [r1], #0x4");
    assert_arm!(0x04e12004, &divided, "streqbt r2, [r1], #0x4");
    assert_arm!(0x01c120f4, &unified, "strdeq r2, r3, [r1, #0x4]");
    assert_arm!(0x01c120f4, &divided, "streqd r2, [r1, #0x4]");
}

#[test]
fn test_push_pop() {
    let unified = ParseFlags {
        ual: true,
        ..Default::default()
    };
    let divided = ParseFlags {
        ual: false,
        ..Default::default()
    };

    assert_arm!(0xe92d0011, &unified, "push {r0, r4}");
    assert_arm!(0xe92d0011, &divided, "stmdb sp!, {r0, r4}");
    assert_arm!(0xe52d3004, &unified, "push {r3}");
    assert_arm!(0xe52d3004, &divided, "str r3, [sp, #-0x4]!");

    assert_arm!(0xe8bd0011, &unified, "pop {r0, r4}");
    assert_arm!(0xe8bd0011, &divided, "ldmia sp!, {r0, r4}");
    assert_arm!(0xe49d3004, &unified, "pop {r3}");
    assert_arm!(0xe49d3004, &divided, "ldr r3, [sp], #0x4");
}

#[test]
fn test_svc_swi() {
    let unified = ParseFlags {
        ual: true,
        ..Default::default()
    };
    let divided = ParseFlags {
        ual: false,
        ..Default::default()
    };

    assert_arm!(0xef000123, &unified, "svc #0x123");
    assert_arm!(0xef000123, &divided, "swi #0x123");
}
