#[cfg(feature = "v6k")]
mod tests {
    use unarm::{arm, thumb, ArmVersion, ParseFlags};

    macro_rules! assert_arm {
        ($code:literal, $disasm:literal) => {{
            let flags = ParseFlags {
                version: ArmVersion::V6K,
                ..Default::default()
            };
            let ins = arm::Ins::new($code, &flags);
            let parsed = ins.parse(&flags);
            assert_eq!(parsed.display(Default::default()).to_string(), $disasm)
        }};
    }

    macro_rules! assert_thumb {
        ($code:literal, $disasm:literal) => {{
            let flags = ParseFlags {
                version: ArmVersion::V6K,
                ..Default::default()
            };
            let ins = thumb::Ins::new($code, &flags);
            let parsed = ins.parse(&flags);
            assert_eq!(parsed.display(Default::default()).to_string(), $disasm)
        }};
    }

    #[test]
    fn test_blx() {
        assert_arm!(0x512fff35, "blxpl r5");
        assert_arm!(0x512fef35, "<illegal>");
        assert_thumb!(0x47d0, "blx r10");
        assert_thumb!(0x47d1, "<illegal>");
    }

    #[test]
    fn test_bx() {
        assert_arm!(0xe12fff10, "bx r0");
        assert_arm!(0xe12fef10, "<illegal>");
        assert_thumb!(0x4750, "bx r10");
        assert_thumb!(0x4751, "<illegal>");
    }

    #[test]
    fn test_bxj() {
        assert_arm!(0xe12fff20, "bxj r0");
        assert_arm!(0xe12fef20, "<illegal>");
    }

    #[test]
    fn test_clrex() {
        assert_arm!(0xf57ff01f, "clrex");
        assert_arm!(0xf57ff11f, "<illegal>");
    }

    #[test]
    fn test_clz() {
        assert_arm!(0xe16f5f1f, "clz r5, pc");
        assert_arm!(0xe16f5e1f, "<illegal>");
    }

    #[test]
    fn test_cmn() {
        assert_arm!(0xe1710003, "cmn r1, r3");
        assert_arm!(0xe1711003, "<illegal>");
    }

    #[test]
    fn test_cmp() {
        assert_arm!(0xe1510003, "cmp r1, r3");
        assert_arm!(0xe1511003, "<illegal>");
    }

    #[test]
    fn test_cps() {
        assert_arm!(0xf102001a, "cps #0x1a");
        assert_arm!(0xf102021a, "<illegal>");
    }

    #[test]
    fn test_csdb() {
        assert_arm!(0xe320f014, "csdb");
        assert_arm!(0xe320f114, "<illegal>");
    }

    #[test]
    fn test_dbg() {
        assert_arm!(0xe320f0f5, "dbg #0x5");
        assert_arm!(0xe320f1f5, "<illegal>");
    }

    #[test]
    fn test_ldrex() {
        assert_arm!(0xe1912f9f, "ldrex r2, [r1]");
        assert_arm!(0xe1912e9f, "<illegal>");
    }

    #[test]
    fn test_mov() {
        assert_arm!(0xe1a02003, "mov r2, r3");
        assert_arm!(0xe1a12003, "<illegal>");
    }

    #[test]
    fn test_mrs() {
        assert_arm!(0xe10f7000, "mrs r7, cpsr");
        assert_arm!(0xe10f7010, "<illegal>");
    }

    #[test]
    fn test_msr() {
        assert_arm!(0xe36cf042, "msr spsr_fs, #0x42");
        assert_arm!(0xe36ce042, "<illegal>");
        assert_arm!(0xe165f001, "msr spsr_sc, r1");
        assert_arm!(0xe165f101, "<illegal>");
    }

    #[test]
    fn test_mul() {
        assert_arm!(0xe0010293, "mul r1, r3, r2");
        assert_arm!(0xe0011293, "<illegal>");
    }

    #[test]
    fn test_mvn() {
        assert_arm!(0xe1e02003, "mvn r2, r3");
        assert_arm!(0xe1e12003, "<illegal>");
    }

    #[test]
    fn test_nop() {
        assert_arm!(0xe320f000, "nop");
        assert_arm!(0xe320f100, "<illegal>");
    }

    #[test]
    fn test_qadd() {
        assert_arm!(0xe1012053, "qadd r2, r3, r1");
        assert_arm!(0xe1012153, "<illegal>");
    }

    #[test]
    fn test_qadd16() {
        assert_arm!(0xe6212f13, "qadd16 r2, r1, r3");
        assert_arm!(0xe6212e13, "<illegal>");
    }

    #[test]
    fn test_qadd8() {
        assert_arm!(0xe6212f93, "qadd8 r2, r1, r3");
        assert_arm!(0xe6212e93, "<illegal>");
    }

    #[test]
    fn test_qasx() {
        assert_arm!(0xe6212f33, "qasx r2, r1, r3");
        assert_arm!(0xe6212e33, "<illegal>");
    }

    #[test]
    fn test_qdadd() {
        assert_arm!(0xe1412053, "qdadd r2, r3, r1");
        assert_arm!(0xe1412153, "<illegal>");
    }

    #[test]
    fn test_qdsub() {
        assert_arm!(0xe1612053, "qdsub r2, r3, r1");
        assert_arm!(0xe1612153, "<illegal>");
    }

    #[test]
    fn test_qsub() {
        assert_arm!(0xe1212053, "qsub r2, r3, r1");
        assert_arm!(0xe1212153, "<illegal>");
    }

    #[test]
    fn test_qsub16() {
        assert_arm!(0xe6212f73, "qsub16 r2, r1, r3");
        assert_arm!(0xe6212e73, "<illegal>");
    }

    #[test]
    fn test_qsub8() {
        assert_arm!(0xe6212ff3, "qsub8 r2, r1, r3");
        assert_arm!(0xe6212ef3, "<illegal>");
    }

    #[test]
    fn test_qsax() {
        assert_arm!(0xe6212f53, "qsax r2, r1, r3");
        assert_arm!(0xe6212e53, "<illegal>");
    }

    #[test]
    fn test_rev() {
        assert_arm!(0xe6bf1f32, "rev r1, r2");
        assert_arm!(0xe6bf1e32, "<illegal>");
    }

    #[test]
    fn test_rev16() {
        assert_arm!(0xe6bf1fb2, "rev16 r1, r2");
        assert_arm!(0xe6bf1eb2, "<illegal>");
    }

    #[test]
    fn test_revsh() {
        assert_arm!(0xe6ff1fb2, "revsh r1, r2");
        assert_arm!(0xe6ff1eb2, "<illegal>");
    }

    #[test]
    fn test_rfe() {
        assert_arm!(0xf8170a00, "rfeda r7");
        assert_arm!(0xf8170a01, "<illegal>");
    }

    #[test]
    fn test_sadd16() {
        assert_arm!(0xe6112f13, "sadd16 r2, r1, r3");
        assert_arm!(0xe6112e13, "<illegal>");
    }

    #[test]
    fn test_sadd8() {
        assert_arm!(0xe6112f93, "sadd8 r2, r1, r3");
        assert_arm!(0xe6112e93, "<illegal>");
    }

    #[test]
    fn test_sasx() {
        assert_arm!(0xe6112f33, "sasx r2, r1, r3");
        assert_arm!(0xe6112e33, "<illegal>");
    }

    #[test]
    fn test_sel() {
        assert_arm!(0xe6812fb3, "sel r2, r1, r3");
        assert_arm!(0xe6812eb3, "<illegal>");
    }

    #[test]
    fn test_setend() {
        assert_arm!(0xf1010000, "setend le");
        assert_arm!(0xf1010100, "<illegal>");
        assert_thumb!(0xb650, "setend le");
        assert_thumb!(0xb651, "<illegal>");
    }

    #[test]
    fn test_sev() {
        assert_arm!(0xe320f004, "sev");
        assert_arm!(0xe320f104, "<illegal>");
    }

    #[test]
    fn test_shadd16() {
        assert_arm!(0xe6312f13, "shadd16 r2, r1, r3");
        assert_arm!(0xe6312e13, "<illegal>");
    }

    #[test]
    fn test_shadd8() {
        assert_arm!(0xe6312f93, "shadd8 r2, r1, r3");
        assert_arm!(0xe6312e93, "<illegal>");
    }

    #[test]
    fn test_shasx() {
        assert_arm!(0xe6312f33, "shasx r2, r1, r3");
        assert_arm!(0xe6312e33, "<illegal>");
    }

    #[test]
    fn test_shsub16() {
        assert_arm!(0xe6312f73, "shsub16 r2, r1, r3");
        assert_arm!(0xe6312e73, "<illegal>");
    }

    #[test]
    fn test_shsub8() {
        assert_arm!(0xe6312ff3, "shsub8 r2, r1, r3");
        assert_arm!(0xe6312ef3, "<illegal>");
    }

    #[test]
    fn test_shsax() {
        assert_arm!(0xe6312f53, "shsax r2, r1, r3");
        assert_arm!(0xe6312e53, "<illegal>");
    }

    #[test]
    fn test_smul() {
        assert_arm!(0xe1610384, "smulbb r1, r4, r3");
        assert_arm!(0xe1611384, "<illegal>");
    }

    #[test]
    fn test_smulw() {
        assert_arm!(0xe12103a4, "smulwb r1, r4, r3");
        assert_arm!(0xe12113a4, "<illegal>");
    }

    #[test]
    fn test_srs() {
        assert_arm!(0xf84d051f, "srsda sp, #0x1f");
        assert_arm!(0xf84d053f, "<illegal>");
    }

    #[test]
    fn test_ssat16() {
        assert_arm!(0xe6af1f32, "ssat16 r1, #0x10, r2");
        assert_arm!(0xe6af1e32, "<illegal>");
    }

    #[test]
    fn test_ssub16() {
        assert_arm!(0xe6112f73, "ssub16 r2, r1, r3");
        assert_arm!(0xe6112e73, "<illegal>");
    }

    #[test]
    fn test_ssub8() {
        assert_arm!(0xe6112ff3, "ssub8 r2, r1, r3");
        assert_arm!(0xe6112ef3, "<illegal>");
    }

    #[test]
    fn test_ssax() {
        assert_arm!(0xe6112f53, "ssax r2, r1, r3");
        assert_arm!(0xe6112e53, "<illegal>");
    }

    #[test]
    fn test_strex() {
        assert_arm!(0xe1812f93, "strex r2, r3, [r1]");
        assert_arm!(0xe1812e93, "<illegal>");
    }

    #[test]
    fn test_swp() {
        assert_arm!(0xe1012093, "swp r2, r3, [r1]");
        assert_arm!(0xe1012193, "<illegal>");
    }

    #[test]
    fn test_swpb() {
        assert_arm!(0xe1412093, "swpb r2, r3, [r1]");
        assert_arm!(0xe1412193, "<illegal>");
    }

    #[test]
    fn test_sxtab() {
        assert_arm!(0xe6a12073, "sxtab r2, r1, r3");
        assert_arm!(0xe6a12173, "<illegal>");
    }

    #[test]
    fn test_sxtab16() {
        assert_arm!(0xe6812073, "sxtab16 r2, r1, r3");
        assert_arm!(0xe6812173, "<illegal>");
    }

    #[test]
    fn test_sxtah() {
        assert_arm!(0xe6b12073, "sxtah r2, r1, r3");
        assert_arm!(0xe6b12173, "<illegal>");
    }

    #[test]
    fn test_sxtb() {
        assert_arm!(0xe6af2073, "sxtb r2, r3");
        assert_arm!(0xe6af2173, "<illegal>");
    }

    #[test]
    fn test_sxtb16() {
        assert_arm!(0xe68f2073, "sxtb16 r2, r3");
        assert_arm!(0xe68f2173, "<illegal>");
    }

    #[test]
    fn test_sxth() {
        assert_arm!(0xe6bf2073, "sxth r2, r3");
        assert_arm!(0xe6bf2173, "<illegal>");
    }

    #[test]
    fn test_teq() {
        assert_arm!(0xe1310003, "teq r1, r3");
        assert_arm!(0xe1311003, "<illegal>");
    }

    #[test]
    fn test_tst() {
        assert_arm!(0xe1110003, "tst r1, r3");
        assert_arm!(0xe1111003, "<illegal>");
    }

    #[test]
    fn test_uadd16() {
        assert_arm!(0xe6512f13, "uadd16 r2, r1, r3");
        assert_arm!(0xe6512e13, "<illegal>");
    }

    #[test]
    fn test_uadd8() {
        assert_arm!(0xe6512f93, "uadd8 r2, r1, r3");
        assert_arm!(0xe6512e93, "<illegal>");
    }

    #[test]
    fn test_uasx() {
        assert_arm!(0xe6512f33, "uasx r2, r1, r3");
        assert_arm!(0xe6512e33, "<illegal>");
    }

    #[test]
    fn test_uhadd16() {
        assert_arm!(0xe6712f13, "uhadd16 r2, r1, r3");
        assert_arm!(0xe6712e13, "<illegal>");
    }

    #[test]
    fn test_uhadd8() {
        assert_arm!(0xe6712f93, "uhadd8 r2, r1, r3");
        assert_arm!(0xe6712e93, "<illegal>");
    }

    #[test]
    fn test_uhasx() {
        assert_arm!(0xe6712f33, "uhasx r2, r1, r3");
        assert_arm!(0xe6712e33, "<illegal>");
    }

    #[test]
    fn test_uhsub16() {
        assert_arm!(0xe6712f73, "uhsub16 r2, r1, r3");
        assert_arm!(0xe6712e73, "<illegal>");
    }

    #[test]
    fn test_uhsub8() {
        assert_arm!(0xe6712ff3, "uhsub8 r2, r1, r3");
        assert_arm!(0xe6712ef3, "<illegal>");
    }

    #[test]
    fn test_uhsax() {
        assert_arm!(0xe6712f53, "uhsax r2, r1, r3");
        assert_arm!(0xe6712e53, "<illegal>");
    }

    #[test]
    fn test_uqadd16() {
        assert_arm!(0xe6612f13, "uqadd16 r2, r1, r3");
        assert_arm!(0xe6612e13, "<illegal>");
    }

    #[test]
    fn test_uqadd8() {
        assert_arm!(0xe6612f93, "uqadd8 r2, r1, r3");
        assert_arm!(0xe6612e93, "<illegal>");
    }

    #[test]
    fn test_uqasx() {
        assert_arm!(0xe6612f33, "uqasx r2, r1, r3");
        assert_arm!(0xe6612e33, "<illegal>");
    }

    #[test]
    fn test_uqsub16() {
        assert_arm!(0xe6612f73, "uqsub16 r2, r1, r3");
        assert_arm!(0xe6612e73, "<illegal>");
    }

    #[test]
    fn test_uqsub8() {
        assert_arm!(0xe6612ff3, "uqsub8 r2, r1, r3");
        assert_arm!(0xe6612ef3, "<illegal>");
    }

    #[test]
    fn test_uqsax() {
        assert_arm!(0xe6612f53, "uqsax r2, r1, r3");
        assert_arm!(0xe6612e53, "<illegal>");
    }

    #[test]
    fn test_usat16() {
        assert_arm!(0xe6ef1f32, "usat16 r1, #0xf, r2");
        assert_arm!(0xe6ef1e32, "<illegal>");
    }

    #[test]
    fn test_usub16() {
        assert_arm!(0xe6512f73, "usub16 r2, r1, r3");
        assert_arm!(0xe6512e73, "<illegal>");
    }

    #[test]
    fn test_usub8() {
        assert_arm!(0xe6512ff3, "usub8 r2, r1, r3");
        assert_arm!(0xe6512ef3, "<illegal>");
    }

    #[test]
    fn test_usax() {
        assert_arm!(0xe6512f53, "usax r2, r1, r3");
        assert_arm!(0xe6512e53, "<illegal>");
    }

    #[test]
    fn test_uxtab() {
        assert_arm!(0xe6e12073, "uxtab r2, r1, r3");
        assert_arm!(0xe6e12173, "<illegal>");
    }

    #[test]
    fn test_uxtab16() {
        assert_arm!(0xe6c12073, "uxtab16 r2, r1, r3");
        assert_arm!(0xe6c12173, "<illegal>");
    }

    #[test]
    fn test_uxtah() {
        assert_arm!(0xe6f12073, "uxtah r2, r1, r3");
        assert_arm!(0xe6f12173, "<illegal>");
    }

    #[test]
    fn test_uxtb() {
        assert_arm!(0xe6ef2073, "uxtb r2, r3");
        assert_arm!(0xe6ef2173, "<illegal>");
    }

    #[test]
    fn test_uxtb16() {
        assert_arm!(0xe6cf2073, "uxtb16 r2, r3");
        assert_arm!(0xe6cf2173, "<illegal>");
    }

    #[test]
    fn test_uxth() {
        assert_arm!(0xe6ff2073, "uxth r2, r3");
        assert_arm!(0xe6ff2173, "<illegal>");
    }

    #[test]
    fn test_wfe() {
        assert_arm!(0xe320f002, "wfe");
        assert_arm!(0xe320f102, "<illegal>");
    }

    #[test]
    fn test_wfi() {
        assert_arm!(0xe320f003, "wfi");
        assert_arm!(0xe320f103, "<illegal>");
    }

    #[test]
    fn test_yield() {
        assert_arm!(0xe320f001, "yield");
        assert_arm!(0xe320f101, "<illegal>");
    }
}
