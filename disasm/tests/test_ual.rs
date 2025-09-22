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
}
