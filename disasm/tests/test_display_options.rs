use unarm::{arm::Ins, DisplayOptions, R9Use, RegNames};

macro_rules! assert_asm {
    ($code:literal, $options:expr, $disasm:literal) => {{
        let flags = Default::default();
        let ins = Ins::new($code, &flags);
        let parsed = ins.parse(&flags);
        assert_eq!(parsed.display($options).to_string(), $disasm)
    }};
}

#[test]
pub fn test_av() {
    let options = DisplayOptions {
        reg_names: RegNames {
            av_registers: true,
            ..Default::default()
        },
        ..Default::default()
    };
    assert_asm!(0xe0812007, options, "add a3, a2, v4");
    assert_asm!(0xe1d52153, options, "bics a3, v2, a4, asr a2");
    assert_asm!(0x4d332169, options, "ldcmi p1, c2, [a4, #-0x1a4]!");
    assert_asm!(0xe8b25555, options, "ldm a3!, {a1, a3, v1, v3, v5, v7, r12, lr}");
    assert_asm!(0xe7312063, options, "ldr a3, [a2, -a4, rrx]!");
    assert_asm!(0xe00120d3, options, "ldrd a3, a4, [a2], -a4");
}

#[test]
pub fn test_r9() {
    let pid = DisplayOptions {
        reg_names: RegNames {
            r9_use: R9Use::Pid,
            ..Default::default()
        },
        ..Default::default()
    };
    let tls = DisplayOptions {
        reg_names: RegNames {
            r9_use: R9Use::Tls,
            ..Default::default()
        },
        ..Default::default()
    };
    let v6 = DisplayOptions {
        reg_names: RegNames {
            av_registers: true,
            ..Default::default()
        },
        ..Default::default()
    };
    let r9 = Default::default();

    assert_asm!(0x102b960a, pid, "eorne sb, r11, r10, lsl #0xc");
    assert_asm!(0x102b960a, tls, "eorne tr, r11, r10, lsl #0xc");
    assert_asm!(0x102b960a, v6, "eorne v6, v8, v7, lsl #0xc");
    assert_asm!(0x102b960a, r9, "eorne r9, r11, r10, lsl #0xc");

    assert_asm!(0xe831aaaa, pid, "ldmda r1!, {r1, r3, r5, r7, sb, r11, sp, pc}");
    assert_asm!(0xe831aaaa, tls, "ldmda r1!, {r1, r3, r5, r7, tr, r11, sp, pc}");
    assert_asm!(0xe831aaaa, v6, "ldmda a2!, {a2, a4, v2, v4, v6, v8, sp, pc}");
    assert_asm!(0xe831aaaa, r9, "ldmda r1!, {r1, r3, r5, r7, r9, r11, sp, pc}");
}

#[test]
pub fn test_r10() {
    let sl = DisplayOptions {
        reg_names: RegNames {
            explicit_stack_limit: true,
            ..Default::default()
        },
        ..Default::default()
    };
    let v7 = DisplayOptions {
        reg_names: RegNames {
            av_registers: true,
            ..Default::default()
        },
        ..Default::default()
    };
    let r10 = Default::default();

    assert_asm!(0x10ab960a, sl, "adcne r9, r11, sl, lsl #0xc");
    assert_asm!(0x10ab960a, v7, "adcne v6, v8, v7, lsl #0xc");
    assert_asm!(0x10ab960a, r10, "adcne r9, r11, r10, lsl #0xc");

    assert_asm!(0xb8b25555, sl, "ldmlt r2!, {r0, r2, r4, r6, r8, sl, r12, lr}");
    assert_asm!(0xb8b25555, v7, "ldmlt a3!, {a1, a3, v1, v3, v5, v7, r12, lr}");
    assert_asm!(0xb8b25555, r10, "ldmlt r2!, {r0, r2, r4, r6, r8, r10, r12, lr}");
}

#[test]
pub fn test_r11() {
    let fp = DisplayOptions {
        reg_names: RegNames {
            frame_pointer: true,
            ..Default::default()
        },
        ..Default::default()
    };
    let v8 = DisplayOptions {
        reg_names: RegNames {
            av_registers: true,
            ..Default::default()
        },
        ..Default::default()
    };
    let r11 = Default::default();

    assert_asm!(0x106b960a, fp, "rsbne r9, fp, r10, lsl #0xc");
    assert_asm!(0x106b960a, v8, "rsbne v6, v8, v7, lsl #0xc");
    assert_asm!(0x106b960a, r11, "rsbne r9, r11, r10, lsl #0xc");

    assert_asm!(0xd903cccc, fp, "stmdble r3, {r2, r3, r6, r7, r10, fp, lr, pc}");
    assert_asm!(0xd903cccc, v8, "stmdble a4, {a3, a4, v3, v4, v7, v8, lr, pc}");
    assert_asm!(0xd903cccc, r11, "stmdble r3, {r2, r3, r6, r7, r10, r11, lr, pc}");
}

#[test]
pub fn test_r12() {
    let ip = DisplayOptions {
        reg_names: RegNames {
            ip: true,
            ..Default::default()
        },
        ..Default::default()
    };
    let r12 = Default::default();

    assert_asm!(0x708c046e, ip, "addvc r0, ip, lr, ror #0x8");
    assert_asm!(0x708c046e, r12, "addvc r0, r12, lr, ror #0x8");

    assert_asm!(0xb8a25555, ip, "stmlt r2!, {r0, r2, r4, r6, r8, r10, ip, lr}");
    assert_asm!(0xb8a25555, r12, "stmlt r2!, {r0, r2, r4, r6, r8, r10, r12, lr}");
}
