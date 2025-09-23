#[cfg(test)]
mod tests {
    use unarm::{Options, parse_arm, parse_thumb};

    macro_rules! assert_ins {
        ($ins:ident, $disasm:literal, $options:ident) => {{
            let s = $ins.display(&$options).to_string();
            assert_eq!(s, $disasm)
        }};
    }

    macro_rules! assert_arm {
        ($code:literal, $options:ident, $disasm:literal) => {{
            let ins = parse_arm($code, 0, &$options).expect("Illegal instruction");
            assert_ins!(ins, $disasm, $options)
        }};
    }

    macro_rules! assert_thumb {
        ($code:literal, $options:ident, $disasm:literal) => {{
            let ins = parse_thumb($code, 0, &$options).expect("Illegal instruction");
            assert_ins!(ins, $disasm, $options)
        }};
    }

    const UNIFIED: Options = Options {
        version: unarm::Version::V6K,
        extensions: unarm::Extensions::all(),
        av: false,
        r9_use: unarm::R9Use::R9,
        sl: false,
        fp: false,
        ip: false,
        ual: true,
    };

    const DIVIDED: Options = Options {
        version: unarm::Version::V6K,
        extensions: unarm::Extensions::all(),
        av: false,
        r9_use: unarm::R9Use::R9,
        sl: false,
        fp: false,
        ip: false,
        ual: false,
    };

    #[test]
    fn test_data() {
        assert_arm!(0x00912003, UNIFIED, "addseq r2, r1, r3");
        assert_arm!(0x00912003, DIVIDED, "addeqs r2, r1, r3");

        assert_thumb!(0x1cca, UNIFIED, "adds r2, r1, #0x3");
        assert_thumb!(0x1cca, DIVIDED, "add r2, r1, #0x3");

        assert_thumb!(0x400a, UNIFIED, "ands r2, r2, r1");
        assert_thumb!(0x400a, DIVIDED, "and r2, r1");

        assert_thumb!(0x1c0a, UNIFIED, "adds r2, r1, #0x0");
        assert_thumb!(0x1c0a, DIVIDED, "mov r2, r1");

        assert_thumb!(0x000a, UNIFIED, "movs r2, r1");
        assert_thumb!(0x000a, DIVIDED, "lsl r2, r1, #0x0");

        assert_thumb!(0x424a, UNIFIED, "rsbs r2, r1, #0x0");
        assert_thumb!(0x424a, DIVIDED, "neg r2, r1");
    }

    #[test]
    fn test_shift() {
        assert_arm!(0xe1a011c2, UNIFIED, "asr r1, r2, #0x3");
        assert_arm!(0xe1a011c2, DIVIDED, "mov r1, r2, asr #0x3");
        assert_arm!(0xe1a01352, UNIFIED, "asr r1, r2, r3");
        assert_arm!(0xe1a01352, DIVIDED, "mov r1, r2, asr r3");

        assert_arm!(0xe1a01182, UNIFIED, "lsl r1, r2, #0x3");
        assert_arm!(0xe1a01182, DIVIDED, "mov r1, r2, lsl #0x3");
        assert_arm!(0xe1a01312, UNIFIED, "lsl r1, r2, r3");
        assert_arm!(0xe1a01312, DIVIDED, "mov r1, r2, lsl r3");

        assert_arm!(0xe1a011a2, UNIFIED, "lsr r1, r2, #0x3");
        assert_arm!(0xe1a011a2, DIVIDED, "mov r1, r2, lsr #0x3");
        assert_arm!(0xe1a01332, UNIFIED, "lsr r1, r2, r3");
        assert_arm!(0xe1a01332, DIVIDED, "mov r1, r2, lsr r3");

        assert_arm!(0xe1a011e2, UNIFIED, "ror r1, r2, #0x3");
        assert_arm!(0xe1a011e2, DIVIDED, "mov r1, r2, ror #0x3");
        assert_arm!(0xe1a01372, UNIFIED, "ror r1, r2, r3");
        assert_arm!(0xe1a01372, DIVIDED, "mov r1, r2, ror r3");

        assert_arm!(0xe1a01062, UNIFIED, "rrx r1, r2");
        assert_arm!(0xe1a01062, DIVIDED, "mov r1, r2, rrx");
    }

    #[test]
    fn test_ldm_stm() {
        assert_arm!(0xe8900011, UNIFIED, "ldm r0, {r0, r4}");
        assert_arm!(0xe8900011, DIVIDED, "ldmia r0, {r0, r4}");
        assert_arm!(0x09100011, UNIFIED, "ldmdbeq r0, {r0, r4}");
        assert_arm!(0x09100011, DIVIDED, "ldmeqdb r0, {r0, r4}");

        assert_arm!(0xe8800011, UNIFIED, "stm r0, {r0, r4}");
        assert_arm!(0xe8800011, DIVIDED, "stmia r0, {r0, r4}");
        assert_arm!(0x09000011, UNIFIED, "stmdbeq r0, {r0, r4}");
        assert_arm!(0x09000011, DIVIDED, "stmeqdb r0, {r0, r4}");

        assert_thumb!(0xc811, UNIFIED, "ldm r0, {r0, r4}");
        assert_thumb!(0xc811, DIVIDED, "ldmia r0, {r0, r4}");

        assert_thumb!(0xc822, UNIFIED, "ldm r0!, {r1, r5}");
        assert_thumb!(0xc822, DIVIDED, "ldmia r0!, {r1, r5}");

        assert_thumb!(0xc011, UNIFIED, "stm r0!, {r0, r4}");
        assert_thumb!(0xc011, DIVIDED, "stmia r0!, {r0, r4}");
    }

    #[test]
    fn test_ldr_str() {
        assert_arm!(0x01d120b4, UNIFIED, "ldrheq r2, [r1, #0x4]");
        assert_arm!(0x01d120b4, DIVIDED, "ldreqh r2, [r1, #0x4]");
        assert_arm!(0x01d120d4, UNIFIED, "ldrsbeq r2, [r1, #0x4]");
        assert_arm!(0x01d120d4, DIVIDED, "ldreqsb r2, [r1, #0x4]");
        assert_arm!(0x01c120d4, UNIFIED, "ldrdeq r2, r3, [r1, #0x4]");
        assert_arm!(0x01c120d4, DIVIDED, "ldreqd r2, [r1, #0x4]");

        assert_arm!(0x01c120b4, UNIFIED, "strheq r2, [r1, #0x4]");
        assert_arm!(0x01c120b4, DIVIDED, "streqh r2, [r1, #0x4]");
        assert_arm!(0x04e12004, UNIFIED, "strbteq r2, [r1], #0x4");
        assert_arm!(0x04e12004, DIVIDED, "streqbt r2, [r1], #0x4");
        assert_arm!(0x01c120f4, UNIFIED, "strdeq r2, r3, [r1, #0x4]");
        assert_arm!(0x01c120f4, DIVIDED, "streqd r2, [r1, #0x4]");
    }

    #[test]
    fn test_push_pop() {
        assert_arm!(0xe92d0011, UNIFIED, "push {r0, r4}");
        assert_arm!(0xe92d0011, DIVIDED, "stmdb sp!, {r0, r4}");
        assert_arm!(0xe52d3004, UNIFIED, "push {r3}");
        assert_arm!(0xe52d3004, DIVIDED, "str r3, [sp, #-0x4]!");

        assert_arm!(0xe8bd0011, UNIFIED, "pop {r0, r4}");
        assert_arm!(0xe8bd0011, DIVIDED, "ldmia sp!, {r0, r4}");
        assert_arm!(0xe49d3004, UNIFIED, "pop {r3}");
        assert_arm!(0xe49d3004, DIVIDED, "ldr r3, [sp], #0x4");
    }

    #[test]
    fn test_svc_swi() {
        assert_arm!(0xef000123, UNIFIED, "svc #0x123");
        assert_arm!(0xef000123, DIVIDED, "swi #0x123");
    }

    #[test]
    fn test_vfp() {
        assert_arm!(0xeef0fae9, UNIFIED, "vabs.f32 s31, s19");
        assert_arm!(0xeef0fae9, DIVIDED, "fabss s31, s19");
        assert_arm!(0x0ef0fbe9, UNIFIED, "vabseq.f64 d31, d25");
        assert_arm!(0x0ef0fbe9, DIVIDED, "fabsdeq d31, d25");

        assert_arm!(0xee73fa29, UNIFIED, "vadd.f32 s31, s6, s19");
        assert_arm!(0xee73fa29, DIVIDED, "fadds s31, s6, s19");
        assert_arm!(0x0e73fb29, UNIFIED, "vaddeq.f64 d31, d3, d25");
        assert_arm!(0x0e73fb29, DIVIDED, "fadddeq d31, d3, d25");

        assert_arm!(0xeef4fae9, UNIFIED, "vcmpe.f32 s31, s19");
        assert_arm!(0xeef4fae9, DIVIDED, "fcmpes s31, s19");
        assert_arm!(0xeef4fa69, UNIFIED, "vcmp.f32 s31, s19");
        assert_arm!(0xeef4fa69, DIVIDED, "fcmps s31, s19");
        assert_arm!(0xeef5fa40, UNIFIED, "vcmp.f32 s31, #0.0");
        assert_arm!(0xeef5fa40, DIVIDED, "fcmpzs s31");
        assert_arm!(0xeef5fac0, UNIFIED, "vcmpe.f32 s31, #0.0");
        assert_arm!(0xeef5fac0, DIVIDED, "fcmpezs s31");
        assert_arm!(0x0ef4fbe9, UNIFIED, "vcmpeeq.f64 d31, d25");
        assert_arm!(0x0ef4fbe9, DIVIDED, "fcmpedeq d31, d25");
        assert_arm!(0x0ef4fb69, UNIFIED, "vcmpeq.f64 d31, d25");
        assert_arm!(0x0ef4fb69, DIVIDED, "fcmpdeq d31, d25");
        assert_arm!(0x0ef5fb40, UNIFIED, "vcmpeq.f64 d31, #0.0");
        assert_arm!(0x0ef5fb40, DIVIDED, "fcmpzdeq d31");
        assert_arm!(0x0ef5fbc0, UNIFIED, "vcmpeeq.f64 d31, #0.0");
        assert_arm!(0x0ef5fbc0, DIVIDED, "fcmpezdeq d31");

        assert_arm!(0x0ef7fbe9, UNIFIED, "vcvteq.f32.f64 s31, d25");
        assert_arm!(0x0ef7fbe9, DIVIDED, "fcvtsdeq s31, d25");
        assert_arm!(0x1ef8fae9, UNIFIED, "vcvtne.f32.s32 s31, s19");
        assert_arm!(0x1ef8fae9, DIVIDED, "fsitosne s31, s19");
        assert_arm!(0x2ef8fa69, UNIFIED, "vcvths.f32.u32 s31, s19");
        assert_arm!(0x2ef8fa69, DIVIDED, "fuitoshs s31, s19");
        assert_arm!(0x3ef7fae9, UNIFIED, "vcvtlo.f64.f32 d31, s19");
        assert_arm!(0x3ef7fae9, DIVIDED, "fcvtdslo d31, s19");
        assert_arm!(0x4ef8fbe9, UNIFIED, "vcvtmi.f64.s32 d31, s19");
        assert_arm!(0x4ef8fbe9, DIVIDED, "fsitodmi d31, s19");
        assert_arm!(0x5ef8fb69, UNIFIED, "vcvtpl.f64.u32 d31, s19");
        assert_arm!(0x5ef8fb69, DIVIDED, "fuitodpl d31, s19");
        assert_arm!(0x6efdfae9, UNIFIED, "vcvtvs.s32.f32 s31, s19");
        assert_arm!(0x6efdfae9, DIVIDED, "ftosisvs s31, s19");
        assert_arm!(0x7efdfb69, UNIFIED, "vcvtrvc.s32.f64 s31, d25");
        assert_arm!(0x7efdfb69, DIVIDED, "ftosizdvc s31, d25");
        assert_arm!(0x8efcfae9, UNIFIED, "vcvthi.u32.f32 s31, s19");
        assert_arm!(0x8efcfae9, DIVIDED, "ftouishi s31, s19");
        assert_arm!(0x9efcfb69, UNIFIED, "vcvtrls.u32.f64 s31, d25");
        assert_arm!(0x9efcfb69, DIVIDED, "ftouizdls s31, d25");

        assert_arm!(0xeec3fa29, UNIFIED, "vdiv.f32 s31, s6, s19");
        assert_arm!(0xeec3fa29, DIVIDED, "fdivs s31, s6, s19");
        assert_arm!(0x0ec3fb29, UNIFIED, "vdiveq.f64 d31, d3, d25");
        assert_arm!(0x0ec3fb29, DIVIDED, "fdivdeq d31, d3, d25");

        assert_arm!(0xecb79a08, UNIFIED, "vldmia r7!, {s18, s19, s20, s21, s22, s23, s24, s25}");
        assert_arm!(0xecb79a08, DIVIDED, "fldmias r7!, {s18, s19, s20, s21, s22, s23, s24, s25}");
        assert_arm!(0x0c979b08, UNIFIED, "vldmiaeq r7, {d9, d10, d11, d12}");
        assert_arm!(0x0c979b08, DIVIDED, "fldmiadeq r7, {d9, d10, d11, d12}");

        assert_arm!(0xedd79a29, UNIFIED, "vldr s19, [r7, #0xa4]");
        assert_arm!(0xedd79a29, DIVIDED, "flds s19, [r7, #0xa4]");
        assert_arm!(0x0d579b29, UNIFIED, "vldreq d25, [r7, #-0xa4]");
        assert_arm!(0x0d579b29, DIVIDED, "flddeq d25, [r7, #-0xa4]");

        assert_arm!(0xee43fa29, UNIFIED, "vmla.f32 s31, s6, s19");
        assert_arm!(0xee43fa29, DIVIDED, "fmacs s31, s6, s19");
        assert_arm!(0x0e43fb29, UNIFIED, "vmlaeq.f64 d31, d3, d25");
        assert_arm!(0x0e43fb29, DIVIDED, "fmacdeq d31, d3, d25");

        assert_arm!(0xee43fa69, UNIFIED, "vmls.f32 s31, s6, s19");
        assert_arm!(0xee43fa69, DIVIDED, "fmscs s31, s6, s19");
        assert_arm!(0x0e43fb69, UNIFIED, "vmlseq.f64 d31, d3, d25");
        assert_arm!(0x0e43fb69, DIVIDED, "fmscdeq d31, d3, d25");

        assert_arm!(0xee097a90, UNIFIED, "vmov s19, r7");
        assert_arm!(0xee097a90, DIVIDED, "fmsr s19, r7");
        assert_arm!(0x0e097b90, UNIFIED, "vmoveq.32 d25[0x0], r7");
        assert_arm!(0x0e097b90, DIVIDED, "fmdlreq d25, r7");
        assert_arm!(0xeef0fa69, UNIFIED, "vmov.f32 s31, s19");
        assert_arm!(0xeef0fa69, DIVIDED, "fcpys s31, s19");
        assert_arm!(0x0ef0fb69, UNIFIED, "vmoveq.f64 d31, d25");
        assert_arm!(0x0ef0fb69, DIVIDED, "fcpydeq d31, d25");
        assert_arm!(0xee0f7a90, UNIFIED, "vmov s31, r7");
        assert_arm!(0xee0f7a90, DIVIDED, "fmsr s31, r7");
        assert_arm!(0x0e2f7b90, UNIFIED, "vmoveq.32 d31[0x1], r7");
        assert_arm!(0x0e2f7b90, DIVIDED, "fmdhreq d31, r7");
        assert_arm!(0xee1f7a90, UNIFIED, "vmov r7, s31");
        assert_arm!(0xee1f7a90, DIVIDED, "fmrs r7, s31");
        assert_arm!(0x0e1f7b90, UNIFIED, "vmoveq.32 r7, d31[0x0]");
        assert_arm!(0x0e1f7b90, DIVIDED, "fmrdleq r7, d31");
        assert_arm!(0x0e3f7b90, UNIFIED, "vmoveq.32 r7, d31[0x1]");
        assert_arm!(0x0e3f7b90, DIVIDED, "fmrdheq r7, d31");
        assert_arm!(0xec5b7a39, UNIFIED, "vmov r7, r11, s19, s20");
        assert_arm!(0xec5b7a39, DIVIDED, "fmrrs r7, r11, s19, s20");
        assert_arm!(0xec4b7a39, UNIFIED, "vmov s19, s20, r7, r11");
        assert_arm!(0xec4b7a39, DIVIDED, "fmsrr s19, s20, r7, r11");

        assert_arm!(0xeef17a10, UNIFIED, "vmrs r7, fpscr");
        assert_arm!(0xeef17a10, DIVIDED, "fmrx r7, fpscr");

        assert_arm!(0xeee17a10, UNIFIED, "vmsr fpscr, r7");
        assert_arm!(0xeee17a10, DIVIDED, "fmxr fpscr, r7");

        assert_arm!(0xee63fa29, UNIFIED, "vmul.f32 s31, s6, s19");
        assert_arm!(0xee63fa29, DIVIDED, "fmuls s31, s6, s19");
        assert_arm!(0x0e63fb29, UNIFIED, "vmuleq.f64 d31, d3, d25");
        assert_arm!(0x0e63fb29, DIVIDED, "fmuldeq d31, d3, d25");

        assert_arm!(0xeef1fa69, UNIFIED, "vneg.f32 s31, s19");
        assert_arm!(0xeef1fa69, DIVIDED, "fnegs s31, s19");
        assert_arm!(0x0ef1fb69, UNIFIED, "vnegeq.f64 d31, d25");
        assert_arm!(0x0ef1fb69, DIVIDED, "fnegdeq d31, d25");

        assert_arm!(0xee53fa69, UNIFIED, "vnmla.f32 s31, s6, s19");
        assert_arm!(0xee53fa69, DIVIDED, "fnmacs s31, s6, s19");
        assert_arm!(0x0e53fb69, UNIFIED, "vnmlaeq.f64 d31, d3, d25");
        assert_arm!(0x0e53fb69, DIVIDED, "fnmacdeq d31, d3, d25");

        assert_arm!(0xee53fa29, UNIFIED, "vnmls.f32 s31, s6, s19");
        assert_arm!(0xee53fa29, DIVIDED, "fnmscs s31, s6, s19");
        assert_arm!(0x0e53fb29, UNIFIED, "vnmlseq.f64 d31, d3, d25");
        assert_arm!(0x0e53fb29, DIVIDED, "fnmscdeq d31, d3, d25");

        assert_arm!(0xee63fa69, UNIFIED, "vnmul.f32 s31, s6, s19");
        assert_arm!(0xee63fa69, DIVIDED, "fnmuls s31, s6, s19");
        assert_arm!(0x0e63fb69, UNIFIED, "vnmuleq.f64 d31, d3, d25");
        assert_arm!(0x0e63fb69, DIVIDED, "fnmuldeq d31, d3, d25");

        assert_arm!(0xecbd9a08, UNIFIED, "vpop {s18, s19, s20, s21, s22, s23, s24, s25}");
        assert_arm!(0xecbd9a08, DIVIDED, "fldmias sp!, {s18, s19, s20, s21, s22, s23, s24, s25}");
        assert_arm!(0x0cbd9b08, UNIFIED, "vpopeq {d9, d10, d11, d12}");
        assert_arm!(0x0cbd9b08, DIVIDED, "fldmiadeq sp!, {d9, d10, d11, d12}");

        assert_arm!(0xed2d9a08, UNIFIED, "vpush {s18, s19, s20, s21, s22, s23, s24, s25}");
        assert_arm!(0xed2d9a08, DIVIDED, "fstmdbs sp!, {s18, s19, s20, s21, s22, s23, s24, s25}");
        assert_arm!(0x0d2d9b08, UNIFIED, "vpusheq {d9, d10, d11, d12}");
        assert_arm!(0x0d2d9b08, DIVIDED, "fstmdbdeq sp!, {d9, d10, d11, d12}");

        assert_arm!(0xeef1fae9, UNIFIED, "vsqrt.f32 s31, s19");
        assert_arm!(0xeef1fae9, DIVIDED, "fsqrts s31, s19");
        assert_arm!(0x0ef1fbe9, UNIFIED, "vsqrteq.f64 d31, d25");
        assert_arm!(0x0ef1fbe9, DIVIDED, "fsqrtdeq d31, d25");

        assert_arm!(0xece79a08, UNIFIED, "vstmia r7!, {s19, s20, s21, s22, s23, s24, s25, s26}");
        assert_arm!(0xece79a08, DIVIDED, "fstmias r7!, {s19, s20, s21, s22, s23, s24, s25, s26}");
        assert_arm!(0x0cc79b08, UNIFIED, "vstmiaeq r7, {d25, d26, d27, d28}");
        assert_arm!(0x0cc79b08, DIVIDED, "fstmiadeq r7, {d25, d26, d27, d28}");

        assert_arm!(0xedc79a29, UNIFIED, "vstr s19, [r7, #0xa4]");
        assert_arm!(0xedc79a29, DIVIDED, "fsts s19, [r7, #0xa4]");
        assert_arm!(0x0d479b29, UNIFIED, "vstreq d25, [r7, #-0xa4]");
        assert_arm!(0x0d479b29, DIVIDED, "fstdeq d25, [r7, #-0xa4]");

        assert_arm!(0xee73fa69, UNIFIED, "vsub.f32 s31, s6, s19");
        assert_arm!(0xee73fa69, DIVIDED, "fsubs s31, s6, s19");
        assert_arm!(0x0e73fb69, UNIFIED, "vsubeq.f64 d31, d3, d25");
        assert_arm!(0x0e73fb69, DIVIDED, "fsubdeq d31, d3, d25");
    }
}
