#[cfg(test)]
mod tests {
    use unarm::{Options, Version, parse_arm, parse_thumb};

    macro_rules! assert_ins {
        ($ins:ident, $disasm:literal, $options:ident) => {{
            let s = $ins.display(&$options).to_string();
            assert_eq!(s, $disasm)
        }};
    }

    macro_rules! options {
        ($version:expr) => {{
            Options {
                version: $version,
                extensions: unarm::Extensions::all(),
                av: false,
                r9_use: unarm::R9Use::R9,
                sl: false,
                fp: false,
                ip: false,
                ual: true,
            }
        }};
    }

    macro_rules! assert_arm {
        ($code:literal, $version:expr, $disasm:literal) => {{
            let options = options!($version);
            let ins = parse_arm($code, 0, &options);
            assert_ins!(ins, $disasm, options)
        }};
    }

    macro_rules! assert_thumb {
        ($code:literal, $version:expr, $disasm:literal) => {{
            let options = options!($version);
            let ins = parse_thumb($code, 0, &options);
            assert_ins!(ins, $disasm, options)
        }};
        ($code:literal, $next:literal, $version:expr, $disasm:literal) => {{
            let options = options!($version);
            let ins = parse_thumb($code | ($next << 16), 0, &options);
            let s = ins.display(&options).to_string();
            assert_eq!(s, $disasm)
        }};
    }

    #[test]
    fn test_v4t() {
        assert_arm!(0xe12fff10, Version::V4T, "bx r0");
        assert_arm!(0xe12fff10, Version::V4, "<illegal>");

        assert_arm!(0xe7f000f0, Version::V4T, "udf #0x0");
        assert_arm!(0xe7f000f0, Version::V4, "<illegal>");
    }

    #[test]
    fn test_v5t() {
        assert_arm!(0xe1200070, Version::V5T, "bkpt #0x0");
        assert_arm!(0xe1200070, Version::V4T, "<illegal>");
        assert_thumb!(0xbe42, Version::V5T, "bkpt #0x42");
        assert_thumb!(0xbe42, Version::V4T, "<illegal>");

        assert_arm!(0xfa000000, Version::V5T, "blx #0x8");
        assert_arm!(0xfa000000, Version::V4T, "<illegal>");
        assert_arm!(0xe12fff30, Version::V5T, "blx r0");
        assert_arm!(0xe12fff30, Version::V4T, "<illegal>");
        assert_thumb!(0xf000, 0xe800, Version::V5T, "blx #0x4");
        assert_thumb!(0xf000, 0xe800, Version::V4T, "<illegal>");
        assert_thumb!(0x47d0, Version::V5T, "blx r10");
        assert_thumb!(0x47d0, Version::V4T, "<illegal>");

        assert_arm!(0xfe1234a6, Version::V5T, "cdp2 p4, #0x1, c3, c2, c6, #0x5");
        assert_arm!(0xfe1234a6, Version::V4T, "<illegal>");

        assert_arm!(0xe16f5f1f, Version::V5T, "clz r5, pc");
        assert_arm!(0xe16f5f1f, Version::V4T, "<illegal>");

        assert_arm!(0xfd132169, Version::V5T, "ldc2 p1, c2, [r3, #-0x1a4]");
        assert_arm!(0xfd132169, Version::V4T, "<illegal>");

        assert_arm!(0xfe2234b6, Version::V5T, "mcr2 p4, #0x1, r3, c2, c6, #0x5");
        assert_arm!(0xfe2234b6, Version::V4T, "<illegal>");

        assert_arm!(0xfe3234b6, Version::V5T, "mrc2 p4, #0x1, r3, c2, c6, #0x5");
        assert_arm!(0xfe3234b6, Version::V4T, "<illegal>");

        assert_arm!(0xfd032169, Version::V5T, "stc2 p1, c2, [r3, #-0x1a4]");
        assert_arm!(0xfd032169, Version::V4T, "<illegal>");
    }

    #[test]
    fn test_v5te() {
        assert_arm!(0xe1c12fdf, Version::V5Te, "ldrd r2, r3, [r1, #0xff]");
        assert_arm!(0xe1c12fdf, Version::V5T, "<illegal>");

        assert_arm!(0xec412345, Version::V5Te, "mcrr p3, #0x4, r2, r1, c5");
        assert_arm!(0xec412345, Version::V5T, "<illegal>");

        assert_arm!(0xec512345, Version::V5Te, "mrrc p3, #0x4, r2, r1, c5");
        assert_arm!(0xec512345, Version::V5T, "<illegal>");

        assert_arm!(0xf5d1ffff, Version::V5Te, "pld [r1, #0xfff]");
        assert_arm!(0xf5d1ffff, Version::V5T, "<illegal>");

        assert_arm!(0xe1012053, Version::V5Te, "qadd r2, r3, r1");
        assert_arm!(0xe1012053, Version::V5T, "<illegal>");

        assert_arm!(0xe1412053, Version::V5Te, "qdadd r2, r3, r1");
        assert_arm!(0xe1412053, Version::V5T, "<illegal>");

        assert_arm!(0xe1612053, Version::V5Te, "qdsub r2, r3, r1");
        assert_arm!(0xe1612053, Version::V5T, "<illegal>");

        assert_arm!(0xe1212053, Version::V5Te, "qsub r2, r3, r1");
        assert_arm!(0xe1212053, Version::V5T, "<illegal>");

        assert_arm!(0xe1012384, Version::V5Te, "smlabb r1, r4, r3, r2");
        assert_arm!(0xe1012384, Version::V5T, "<illegal>");

        assert_arm!(0xe1412384, Version::V5Te, "smlalbb r2, r1, r4, r3");
        assert_arm!(0xe1412384, Version::V5T, "<illegal>");

        assert_arm!(0xe1212384, Version::V5Te, "smlawb r1, r4, r3, r2");
        assert_arm!(0xe1212384, Version::V5T, "<illegal>");

        assert_arm!(0xe1610384, Version::V5Te, "smulbb r1, r4, r3");
        assert_arm!(0xe1610384, Version::V5T, "<illegal>");

        assert_arm!(0xe12103a4, Version::V5Te, "smulwb r1, r4, r3");
        assert_arm!(0xe12103a4, Version::V5T, "<illegal>");

        assert_arm!(0xe1c12fff, Version::V5Te, "strd r2, r3, [r1, #0xff]");
        assert_arm!(0xe1c12fff, Version::V5T, "<illegal>");
    }

    #[test]
    fn test_v5tej() {
        assert_arm!(0xe12fff20, Version::V5Tej, "bxj r0");
        assert_arm!(0xe12fff20, Version::V5Te, "<illegal>");
    }

    #[test]
    fn test_v6() {
        assert_arm!(0xf102001a, Version::V6, "cps #0x1a");
        assert_arm!(0xf102001a, Version::V5Te, "<illegal>");
        assert_thumb!(0xb677, Version::V6, "cpsid aif");
        assert_thumb!(0xb677, Version::V5Te, "<illegal>");

        assert_arm!(0xe1912f9f, Version::V6, "ldrex r2, [r1]");
        assert_arm!(0xe1912f9f, Version::V5Te, "<illegal>");

        assert_arm!(0xfc412345, Version::V6, "mcrr2 p3, #0x4, r2, r1, c5");
        assert_arm!(0xfc412345, Version::V5Te, "<illegal>");

        assert_arm!(0xfc512345, Version::V6, "mrrc2 p3, #0x4, r2, r1, c5");
        assert_arm!(0xfc512345, Version::V5Te, "<illegal>");

        assert_arm!(0xe6812893, Version::V6, "pkhbt r2, r1, r3, lsl #0x11");
        assert_arm!(0xe6812893, Version::V5Te, "<illegal>");

        assert_arm!(0xe68128d3, Version::V6, "pkhtb r2, r1, r3, asr #0x11");
        assert_arm!(0xe68128d3, Version::V5Te, "<illegal>");

        assert_arm!(0xe6212f13, Version::V6, "qadd16 r2, r1, r3");
        assert_arm!(0xe6212f13, Version::V5Te, "<illegal>");

        assert_arm!(0xe6212f93, Version::V6, "qadd8 r2, r1, r3");
        assert_arm!(0xe6212f93, Version::V5Te, "<illegal>");

        assert_arm!(0xe6212f33, Version::V6, "qasx r2, r1, r3");
        assert_arm!(0xe6212f33, Version::V5Te, "<illegal>");

        assert_arm!(0xe6212f73, Version::V6, "qsub16 r2, r1, r3");
        assert_arm!(0xe6212f73, Version::V5Te, "<illegal>");

        assert_arm!(0xe6212ff3, Version::V6, "qsub8 r2, r1, r3");
        assert_arm!(0xe6212ff3, Version::V5Te, "<illegal>");

        assert_arm!(0xe6212f53, Version::V6, "qsax r2, r1, r3");
        assert_arm!(0xe6212f53, Version::V5Te, "<illegal>");

        assert_arm!(0xe6bf1f32, Version::V6, "rev r1, r2");
        assert_arm!(0xe6bf1f32, Version::V5Te, "<illegal>");
        assert_thumb!(0xba0a, Version::V6, "rev r2, r1");
        assert_thumb!(0xba0a, Version::V5Te, "<illegal>");

        assert_arm!(0xe6bf1fb2, Version::V6, "rev16 r1, r2");
        assert_arm!(0xe6bf1fb2, Version::V5Te, "<illegal>");
        assert_thumb!(0xba4a, Version::V6, "rev16 r2, r1");
        assert_thumb!(0xba4a, Version::V5Te, "<illegal>");

        assert_arm!(0xe6ff1fb2, Version::V6, "revsh r1, r2");
        assert_arm!(0xe6ff1fb2, Version::V5Te, "<illegal>");
        assert_thumb!(0xbaca, Version::V6, "revsh r2, r1");
        assert_thumb!(0xbaca, Version::V5Te, "<illegal>");

        assert_arm!(0xf8170a00, Version::V6, "rfeda r7");
        assert_arm!(0xf8170a00, Version::V5Te, "<illegal>");

        assert_arm!(0xe6112f13, Version::V6, "sadd16 r2, r1, r3");
        assert_arm!(0xe6112f13, Version::V5Te, "<illegal>");

        assert_arm!(0xe6112f93, Version::V6, "sadd8 r2, r1, r3");
        assert_arm!(0xe6112f93, Version::V5Te, "<illegal>");

        assert_arm!(0xe6112f33, Version::V6, "sasx r2, r1, r3");
        assert_arm!(0xe6112f33, Version::V5Te, "<illegal>");

        assert_arm!(0xe6812fb3, Version::V6, "sel r2, r1, r3");
        assert_arm!(0xe6812fb3, Version::V5Te, "<illegal>");

        assert_arm!(0xf1010000, Version::V6, "setend le");
        assert_arm!(0xf1010000, Version::V5Te, "<illegal>");
        assert_thumb!(0xb650, Version::V6, "setend le");
        assert_thumb!(0xb650, Version::V5Te, "<illegal>");

        assert_arm!(0xe6312f13, Version::V6, "shadd16 r2, r1, r3");
        assert_arm!(0xe6312f13, Version::V5Te, "<illegal>");

        assert_arm!(0xe6312f93, Version::V6, "shadd8 r2, r1, r3");
        assert_arm!(0xe6312f93, Version::V5Te, "<illegal>");

        assert_arm!(0xe6312f33, Version::V6, "shasx r2, r1, r3");
        assert_arm!(0xe6312f33, Version::V5Te, "<illegal>");

        assert_arm!(0xe6312f53, Version::V6, "shsax r2, r1, r3");
        assert_arm!(0xe6312f53, Version::V5Te, "<illegal>");

        assert_arm!(0xe6312f73, Version::V6, "shsub16 r2, r1, r3");
        assert_arm!(0xe6312f73, Version::V5Te, "<illegal>");

        assert_arm!(0xe6312ff3, Version::V6, "shsub8 r2, r1, r3");
        assert_arm!(0xe6312ff3, Version::V5Te, "<illegal>");

        assert_arm!(0xe7012314, Version::V6, "smlad r1, r4, r3, r2");
        assert_arm!(0xe7012314, Version::V5Te, "<illegal>");

        assert_arm!(0xe7412314, Version::V6, "smlald r2, r1, r4, r3");
        assert_arm!(0xe7412314, Version::V5Te, "<illegal>");

        assert_arm!(0xe7012354, Version::V6, "smlsd r1, r4, r3, r2");
        assert_arm!(0xe7012354, Version::V5Te, "<illegal>");

        assert_arm!(0xe7412354, Version::V6, "smlsld r2, r1, r4, r3");
        assert_arm!(0xe7412354, Version::V5Te, "<illegal>");

        assert_arm!(0xe7512314, Version::V6, "smmla r1, r4, r3, r2");
        assert_arm!(0xe7512314, Version::V5Te, "<illegal>");

        assert_arm!(0xe75123d4, Version::V6, "smmls r1, r4, r3, r2");
        assert_arm!(0xe75123d4, Version::V5Te, "<illegal>");

        assert_arm!(0xe751f314, Version::V6, "smmul r1, r4, r3");
        assert_arm!(0xe751f314, Version::V5Te, "<illegal>");

        assert_arm!(0xe701f314, Version::V6, "smuad r1, r4, r3");
        assert_arm!(0xe701f314, Version::V5Te, "<illegal>");

        assert_arm!(0xe701f354, Version::V6, "smusd r1, r4, r3");
        assert_arm!(0xe701f354, Version::V5Te, "<illegal>");

        assert_arm!(0xf84d051f, Version::V6, "srsda sp, #0x1f");
        assert_arm!(0xf84d051f, Version::V5Te, "<illegal>");

        assert_arm!(0xe6af1512, Version::V6, "ssat r1, #0x10, r2, lsl #0xa");
        assert_arm!(0xe6af1512, Version::V5Te, "<illegal>");

        assert_arm!(0xe6af1f32, Version::V6, "ssat16 r1, #0x10, r2");
        assert_arm!(0xe6af1f32, Version::V5Te, "<illegal>");

        assert_arm!(0xe6112f73, Version::V6, "ssub16 r2, r1, r3");
        assert_arm!(0xe6112f73, Version::V5Te, "<illegal>");

        assert_arm!(0xe6112ff3, Version::V6, "ssub8 r2, r1, r3");
        assert_arm!(0xe6112ff3, Version::V5Te, "<illegal>");

        assert_arm!(0xe6112f53, Version::V6, "ssax r2, r1, r3");
        assert_arm!(0xe6112f53, Version::V5Te, "<illegal>");

        assert_arm!(0xe1812f93, Version::V6, "strex r2, r3, [r1]");
        assert_arm!(0xe1812f93, Version::V5Te, "<illegal>");

        assert_arm!(0xe6a12073, Version::V6, "sxtab r2, r1, r3");
        assert_arm!(0xe6a12073, Version::V5Te, "<illegal>");

        assert_arm!(0xe6812073, Version::V6, "sxtab16 r2, r1, r3");
        assert_arm!(0xe6812073, Version::V5Te, "<illegal>");

        assert_arm!(0xe6b12073, Version::V6, "sxtah r2, r1, r3");
        assert_arm!(0xe6b12073, Version::V5Te, "<illegal>");

        assert_arm!(0xe6af2073, Version::V6, "sxtb r2, r3");
        assert_arm!(0xe6af2073, Version::V5Te, "<illegal>");
        assert_thumb!(0xb24a, Version::V6, "sxtb r2, r1");
        assert_thumb!(0xb24a, Version::V5Te, "<illegal>");

        assert_arm!(0xe68f2073, Version::V6, "sxtb16 r2, r3");
        assert_arm!(0xe68f2073, Version::V5Te, "<illegal>");

        assert_arm!(0xe6bf2073, Version::V6, "sxth r2, r3");
        assert_arm!(0xe6bf2073, Version::V5Te, "<illegal>");
        assert_thumb!(0xb20a, Version::V6, "sxth r2, r1");
        assert_thumb!(0xb20a, Version::V5Te, "<illegal>");

        assert_arm!(0xe6512f13, Version::V6, "uadd16 r2, r1, r3");
        assert_arm!(0xe6512f13, Version::V5Te, "<illegal>");

        assert_arm!(0xe6512f93, Version::V6, "uadd8 r2, r1, r3");
        assert_arm!(0xe6512f93, Version::V5Te, "<illegal>");

        assert_arm!(0xe6512f33, Version::V6, "uasx r2, r1, r3");
        assert_arm!(0xe6512f33, Version::V5Te, "<illegal>");

        assert_arm!(0xe6712f13, Version::V6, "uhadd16 r2, r1, r3");
        assert_arm!(0xe6712f13, Version::V5Te, "<illegal>");

        assert_arm!(0xe6712f93, Version::V6, "uhadd8 r2, r1, r3");
        assert_arm!(0xe6712f93, Version::V5Te, "<illegal>");

        assert_arm!(0xe6712f33, Version::V6, "uhasx r2, r1, r3");
        assert_arm!(0xe6712f33, Version::V5Te, "<illegal>");

        assert_arm!(0xe6712f73, Version::V6, "uhsub16 r2, r1, r3");
        assert_arm!(0xe6712f73, Version::V5Te, "<illegal>");

        assert_arm!(0xe6712ff3, Version::V6, "uhsub8 r2, r1, r3");
        assert_arm!(0xe6712ff3, Version::V5Te, "<illegal>");

        assert_arm!(0xe6712f53, Version::V6, "uhsax r2, r1, r3");
        assert_arm!(0xe6712f53, Version::V5Te, "<illegal>");

        assert_arm!(0xe0412394, Version::V6, "umaal r2, r1, r4, r3");
        assert_arm!(0xe0412394, Version::V5Te, "<illegal>");

        assert_arm!(0xe6612f13, Version::V6, "uqadd16 r2, r1, r3");
        assert_arm!(0xe6612f13, Version::V5Te, "<illegal>");

        assert_arm!(0xe6612f93, Version::V6, "uqadd8 r2, r1, r3");
        assert_arm!(0xe6612f93, Version::V5Te, "<illegal>");

        assert_arm!(0xe6612f33, Version::V6, "uqasx r2, r1, r3");
        assert_arm!(0xe6612f33, Version::V5Te, "<illegal>");

        assert_arm!(0xe6612f73, Version::V6, "uqsub16 r2, r1, r3");
        assert_arm!(0xe6612f73, Version::V5Te, "<illegal>");

        assert_arm!(0xe6612ff3, Version::V6, "uqsub8 r2, r1, r3");
        assert_arm!(0xe6612ff3, Version::V5Te, "<illegal>");

        assert_arm!(0xe6612f53, Version::V6, "uqsax r2, r1, r3");
        assert_arm!(0xe6612f53, Version::V5Te, "<illegal>");

        assert_arm!(0xe781f213, Version::V6, "usad8 r1, r3, r2");
        assert_arm!(0xe781f213, Version::V5Te, "<illegal>");

        assert_arm!(0xe7814213, Version::V6, "usada8 r1, r3, r2, r4");
        assert_arm!(0xe7814213, Version::V5Te, "<illegal>");

        assert_arm!(0xe6ef1512, Version::V6, "usat r1, #0xf, r2, lsl #0xa");
        assert_arm!(0xe6ef1512, Version::V5Te, "<illegal>");

        assert_arm!(0xe6ef1f32, Version::V6, "usat16 r1, #0xf, r2");
        assert_arm!(0xe6ef1f32, Version::V5Te, "<illegal>");

        assert_arm!(0xe6512f73, Version::V6, "usub16 r2, r1, r3");
        assert_arm!(0xe6512f73, Version::V5Te, "<illegal>");

        assert_arm!(0xe6512ff3, Version::V6, "usub8 r2, r1, r3");
        assert_arm!(0xe6512ff3, Version::V5Te, "<illegal>");

        assert_arm!(0xe6512f53, Version::V6, "usax r2, r1, r3");
        assert_arm!(0xe6512f53, Version::V5Te, "<illegal>");

        assert_arm!(0xe6e12073, Version::V6, "uxtab r2, r1, r3");
        assert_arm!(0xe6e12073, Version::V5Te, "<illegal>");

        assert_arm!(0xe6c12073, Version::V6, "uxtab16 r2, r1, r3");
        assert_arm!(0xe6c12073, Version::V5Te, "<illegal>");

        assert_arm!(0xe6f12073, Version::V6, "uxtah r2, r1, r3");
        assert_arm!(0xe6f12073, Version::V5Te, "<illegal>");

        assert_arm!(0xe6ef2073, Version::V6, "uxtb r2, r3");
        assert_arm!(0xe6ef2073, Version::V5Te, "<illegal>");
        assert_thumb!(0xb2ca, Version::V6, "uxtb r2, r1");
        assert_thumb!(0xb2ca, Version::V5Te, "<illegal>");

        assert_arm!(0xe6cf2073, Version::V6, "uxtb16 r2, r3");
        assert_arm!(0xe6cf2073, Version::V5Te, "<illegal>");

        assert_arm!(0xe6ff2073, Version::V6, "uxth r2, r3");
        assert_arm!(0xe6ff2073, Version::V5Te, "<illegal>");
        assert_thumb!(0xb28a, Version::V6, "uxth r2, r1");
        assert_thumb!(0xb28a, Version::V5Te, "<illegal>");
    }

    #[test]
    fn test_v6k() {
        assert_arm!(0xf57ff01f, Version::V6K, "clrex");
        assert_arm!(0xf57ff01f, Version::V6, "<illegal>");

        assert_arm!(0xe320f0f8, Version::V6K, "dbg #0x8");
        assert_arm!(0xe320f0f8, Version::V6, "<illegal>");

        assert_arm!(0xe1d12f9f, Version::V6K, "ldrexb r2, [r1]");
        assert_arm!(0xe1d12f9f, Version::V5Te, "<illegal>");

        assert_arm!(0xe1b12f9f, Version::V6K, "ldrexd r2, r3, [r1]");
        assert_arm!(0xe1b12f9f, Version::V5Te, "<illegal>");

        assert_arm!(0xe1f12f9f, Version::V6K, "ldrexh r2, [r1]");
        assert_arm!(0xe1f12f9f, Version::V5Te, "<illegal>");

        assert_arm!(0xe320f000, Version::V6K, "nop");
        assert_arm!(0xe320f000, Version::V6, "<illegal>");

        assert_arm!(0xe320f004, Version::V6K, "sev");
        assert_arm!(0xe320f004, Version::V6, "<illegal>");

        assert_arm!(0xe1c12f93, Version::V6K, "strexb r2, r3, [r1]");
        assert_arm!(0xe1c12f93, Version::V6, "<illegal>");

        assert_arm!(0xe1a12f94, Version::V6K, "strexd r2, r4, r5, [r1]");
        assert_arm!(0xe1a12f94, Version::V6, "<illegal>");

        assert_arm!(0xe1e12f93, Version::V6K, "strexh r2, r3, [r1]");
        assert_arm!(0xe1e12f93, Version::V6, "<illegal>");

        assert_arm!(0xe320f002, Version::V6K, "wfe");
        assert_arm!(0xe320f002, Version::V6, "<illegal>");

        assert_arm!(0xe320f003, Version::V6K, "wfi");
        assert_arm!(0xe320f003, Version::V6, "<illegal>");

        assert_arm!(0xe320f001, Version::V6K, "yield");
        assert_arm!(0xe320f001, Version::V6, "<illegal>");
    }
}
