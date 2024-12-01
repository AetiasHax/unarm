#[cfg(feature = "v6k")]
mod tests {
    use unarm::{arm::Ins, ArmVersion, ParseFlags};

    macro_rules! assert_asm {
        ($code:literal, $disasm:literal) => {{
            let flags = ParseFlags {
                version: ArmVersion::V6K,
                ..Default::default()
            };
            let ins = Ins::new($code, &flags);
            let parsed = ins.parse(&flags);
            assert_eq!(parsed.display(Default::default()).to_string(), $disasm)
        }};
    }

    #[test]
    fn test_adc() {
        assert_asm!(0xe0a12003, "adc r2, r1, r3");
        assert_asm!(0xe2a45e23, "adc r5, r4, #0x230");
        assert_asm!(0x10ab960a, "adcne r9, r11, r10, lsl #0xc");
        assert_asm!(0x40a5f238, "adcmi pc, r5, r8, lsr r2");
        assert_asm!(0x70a2046e, "adcvc r0, r2, lr, ror #0x8");
        assert_asm!(0xb0a87060, "adclt r7, r8, r0, rrx");
        assert_asm!(0xe0b52153, "adcs r2, r5, r3, asr r1");
    }

    #[test]
    fn test_add() {
        assert_asm!(0xe0812003, "add r2, r1, r3");
        assert_asm!(0xe2845e23, "add r5, r4, #0x230");
        assert_asm!(0x108b960a, "addne r9, r11, r10, lsl #0xc");
        assert_asm!(0x4085f238, "addmi pc, r5, r8, lsr r2");
        assert_asm!(0x7082046e, "addvc r0, r2, lr, ror #0x8");
        assert_asm!(0xb0887060, "addlt r7, r8, r0, rrx");
        assert_asm!(0xe0952153, "adds r2, r5, r3, asr r1");
        assert_asm!(0xe28f41a5, "add r4, pc, #0x40000029");
    }

    #[test]
    fn test_and() {
        assert_asm!(0xe0012003, "and r2, r1, r3");
        assert_asm!(0xe2045e23, "and r5, r4, #0x230");
        assert_asm!(0x100b960a, "andne r9, r11, r10, lsl #0xc");
        assert_asm!(0x4005f238, "andmi pc, r5, r8, lsr r2");
        assert_asm!(0x7002046e, "andvc r0, r2, lr, ror #0x8");
        assert_asm!(0xb0087060, "andlt r7, r8, r0, rrx");
        assert_asm!(0xe0152153, "ands r2, r5, r3, asr r1");
    }

    #[test]
    fn test_asr() {
        assert_asm!(0xe1b02153, "asrs r2, r3, r1");
    }

    #[test]
    fn test_b() {
        assert_asm!(0xea000000, "b #0x8");
        assert_asm!(0x0a012345, "beq #0x48d1c");
        assert_asm!(0x1affffff, "bne #0x4");
        assert_asm!(0x2afffffe, "bhs #0x0");
        assert_asm!(0x3afffffd, "blo #-0x4");
    }

    #[test]
    fn test_bl() {
        assert_asm!(0xeb000000, "bl #0x8");
        assert_asm!(0x0b012345, "bleq #0x48d1c");
        assert_asm!(0x1bffffff, "blne #0x4");
        assert_asm!(0x2bfffffe, "blhs #0x0");
        assert_asm!(0x3bfffffd, "bllo #-0x4");
    }

    #[test]
    fn test_bic() {
        assert_asm!(0xe1c12003, "bic r2, r1, r3");
        assert_asm!(0xe3c45e23, "bic r5, r4, #0x230");
        assert_asm!(0x11cb960a, "bicne r9, r11, r10, lsl #0xc");
        assert_asm!(0x41c5f238, "bicmi pc, r5, r8, lsr r2");
        assert_asm!(0x71c2046e, "bicvc r0, r2, lr, ror #0x8");
        assert_asm!(0xb1c87060, "biclt r7, r8, r0, rrx");
        assert_asm!(0xe1d52153, "bics r2, r5, r3, asr r1");
    }

    #[test]
    fn test_bkpt() {
        assert_asm!(0xe1200070, "bkpt #0x0");
        assert_asm!(0xe1243271, "bkpt #0x4321");
    }

    #[test]
    fn test_blx() {
        assert_asm!(0xfa000000, "blx #0x8");
        assert_asm!(0xfa012345, "blx #0x48d1c");
        assert_asm!(0xfaffffff, "blx #0x4");
        assert_asm!(0xfafffffe, "blx #0x0");
        assert_asm!(0xfafffffd, "blx #-0x4");
        assert_asm!(0xe12fff30, "blx r0");
        assert_asm!(0x512fff35, "blxpl r5");
        assert_asm!(0xfb000000, "blx #0xa");
    }

    #[test]
    fn test_bx() {
        assert_asm!(0xe12fff10, "bx r0");
        assert_asm!(0x512fff15, "bxpl r5");
    }

    #[test]
    fn test_bxj() {
        assert_asm!(0xe12fff20, "bxj r0");
        assert_asm!(0x512fff25, "bxjpl r5");
    }

    #[test]
    fn test_cdp() {
        assert_asm!(0xee1234a6, "cdp p4, #1, c3, c2, c6, #5");
        assert_asm!(0xae654341, "cdpge p3, #6, c4, c5, c1, #2");
    }

    #[test]
    fn test_cdp2() {
        assert_asm!(0xfe1234a6, "cdp2 p4, #1, c3, c2, c6, #5");
        assert_asm!(0xfe654341, "cdp2 p3, #6, c4, c5, c1, #2");
    }

    #[test]
    fn test_clrex() {
        assert_asm!(0xf57ff01f, "clrex");
    }

    #[test]
    fn test_clz() {
        assert_asm!(0xe16f5f1f, "clz r5, pc");
        assert_asm!(0xd16fef15, "clzle lr, r5");
    }

    #[test]
    fn test_cmn() {
        assert_asm!(0xe1710003, "cmn r1, r3");
        assert_asm!(0xe3740e23, "cmn r4, #0x230");
        assert_asm!(0x117b060a, "cmnne r11, r10, lsl #0xc");
        assert_asm!(0x41750238, "cmnmi r5, r8, lsr r2");
        assert_asm!(0x7172046e, "cmnvc r2, lr, ror #0x8");
        assert_asm!(0xb1780060, "cmnlt r8, r0, rrx");
        assert_asm!(0xe1750153, "cmn r5, r3, asr r1");
    }

    #[test]
    fn test_cmp() {
        assert_asm!(0xe1510003, "cmp r1, r3");
        assert_asm!(0xe3540e23, "cmp r4, #0x230");
        assert_asm!(0x115b060a, "cmpne r11, r10, lsl #0xc");
        assert_asm!(0x41550238, "cmpmi r5, r8, lsr r2");
        assert_asm!(0x7152046e, "cmpvc r2, lr, ror #0x8");
        assert_asm!(0xb1580060, "cmplt r8, r0, rrx");
        assert_asm!(0xe1550153, "cmp r5, r3, asr r1");
    }

    #[test]
    fn test_cps() {
        assert_asm!(0xf102001a, "cps #0x1a");
        assert_asm!(0xf10a01df, "cpsie aif, #0x1f");
        assert_asm!(0xf10c0000, "cpsid none");
    }

    #[test]
    fn test_csdb() {
        assert_asm!(0xe320f014, "csdb");
        assert_asm!(0xa320f014, "csdbge");
    }

    #[test]
    fn test_dbg() {
        assert_asm!(0xe320f0f5, "dbg #0x5");
        assert_asm!(0xa320f0f0, "dbgge #0x0");
    }

    #[test]
    fn test_eor() {
        assert_asm!(0xe0212003, "eor r2, r1, r3");
        assert_asm!(0xe2245e23, "eor r5, r4, #0x230");
        assert_asm!(0x102b960a, "eorne r9, r11, r10, lsl #0xc");
        assert_asm!(0x4025f238, "eormi pc, r5, r8, lsr r2");
        assert_asm!(0x7022046e, "eorvc r0, r2, lr, ror #0x8");
        assert_asm!(0xb0287060, "eorlt r7, r8, r0, rrx");
        assert_asm!(0xe0352153, "eors r2, r5, r3, asr r1");
    }

    #[test]
    fn test_ldc() {
        assert_asm!(0xed132169, "ldc p1, c2, [r3, #-0x1a4]");
        assert_asm!(0x4d332169, "ldcmi p1, c2, [r3, #-0x1a4]!");
        assert_asm!(0x6c332169, "ldcvs p1, c2, [r3], #-0x1a4");
        assert_asm!(0x3db32169, "ldclo p1, c2, [r3, #0x1a4]!");
        assert_asm!(0x5c932169, "ldcpl p1, c2, [r3], {0x69}");
    }

    #[test]
    fn test_ldc2() {
        assert_asm!(0xfd132169, "ldc2 p1, c2, [r3, #-0x1a4]");
        assert_asm!(0xfd332169, "ldc2 p1, c2, [r3, #-0x1a4]!");
        assert_asm!(0xfc332169, "ldc2 p1, c2, [r3], #-0x1a4");
        assert_asm!(0xfdb32169, "ldc2 p1, c2, [r3, #0x1a4]!");
        assert_asm!(0xfc932169, "ldc2 p1, c2, [r3], {0x69}");
    }

    #[test]
    fn test_ldm() {
        assert_asm!(0xe831aaaa, "ldmda r1!, {r1, r3, r5, r7, r9, r11, sp, pc}");
        assert_asm!(0xb8b25555, "ldmlt r2!, {r0, r2, r4, r6, r8, r10, r12, lr}");
        assert_asm!(0xd913cccc, "ldmdble r3, {r2, r3, r6, r7, r10, r11, lr, pc}");
        assert_asm!(0xc9943333, "ldmibgt r4, {r0, r1, r4, r5, r8, r9, r12, sp}");
        assert_asm!(0xe8550003, "ldmda r5, {r0, r1}^");
        assert_asm!(0xe8568003, "ldmda r6, {r0, r1, pc}^");
        assert_asm!(0xe8778003, "ldmda r7!, {r0, r1, pc}^");
        assert_asm!(
            0xe9f17fff,
            "ldmib r1!, {r0, r1, r2, r3, r4, r5, r6, r7, r8, r9, r10, r11, r12, sp, lr}^"
        );
    }

    #[test]
    fn test_ldr() {
        assert_asm!(0xe5912fff, "ldr r2, [r1, #0xfff]");
        assert_asm!(0xe7112003, "ldr r2, [r1, -r3]");
        assert_asm!(0xe79120e3, "ldr r2, [r1, r3, ror #0x1]");
        assert_asm!(0xe5312fff, "ldr r2, [r1, #-0xfff]!");
        assert_asm!(0xe7b12003, "ldr r2, [r1, r3]!");
        assert_asm!(0xe7312063, "ldr r2, [r1, -r3, rrx]!");
        assert_asm!(0xe4912fff, "ldr r2, [r1], #0xfff");
        assert_asm!(0xe6112003, "ldr r2, [r1], -r3");
        assert_asm!(0xe6912023, "ldr r2, [r1], r3, lsr #0x20");
    }

    #[test]
    fn test_ldrb() {
        assert_asm!(0xe5d12fff, "ldrb r2, [r1, #0xfff]");
        assert_asm!(0xe7512003, "ldrb r2, [r1, -r3]");
        assert_asm!(0xe7d120e3, "ldrb r2, [r1, r3, ror #0x1]");
        assert_asm!(0xe5712fff, "ldrb r2, [r1, #-0xfff]!");
        assert_asm!(0xe7f12003, "ldrb r2, [r1, r3]!");
        assert_asm!(0xe7712063, "ldrb r2, [r1, -r3, rrx]!");
        assert_asm!(0xe4d12fff, "ldrb r2, [r1], #0xfff");
        assert_asm!(0xe6512003, "ldrb r2, [r1], -r3");
        assert_asm!(0xe6d12023, "ldrb r2, [r1], r3, lsr #0x20");
    }

    #[test]
    fn test_ldrbt() {
        assert_asm!(0xe4f12fff, "ldrbt r2, [r1], #0xfff");
        assert_asm!(0xe6712003, "ldrbt r2, [r1], -r3");
        assert_asm!(0xe6f12023, "ldrbt r2, [r1], r3, lsr #0x20");
    }

    #[test]
    fn test_ldrex() {
        assert_asm!(0xe1912f9f, "ldrex r2, [r1]");
        assert_asm!(0x21912f9f, "ldrexhs r2, [r1]");
    }

    #[test]
    fn test_ldrd() {
        assert_asm!(0xe1c12fdf, "ldrd r2, r3, [r1, #0xff]");
        assert_asm!(0xe10120d3, "ldrd r2, r3, [r1, -r3]");
        assert_asm!(0xe1612fdf, "ldrd r2, r3, [r1, #-0xff]!");
        assert_asm!(0xe1a120d3, "ldrd r2, r3, [r1, r3]!");
        assert_asm!(0xe0c12fdf, "ldrd r2, r3, [r1], #0xff");
        assert_asm!(0xe00120d3, "ldrd r2, r3, [r1], -r3");
    }

    #[test]
    fn test_ldrh() {
        assert_asm!(0xe1d12fbf, "ldrh r2, [r1, #0xff]");
        assert_asm!(0xe11120b3, "ldrh r2, [r1, -r3]");
        assert_asm!(0xe1712fbf, "ldrh r2, [r1, #-0xff]!");
        assert_asm!(0xe1b120b3, "ldrh r2, [r1, r3]!");
        assert_asm!(0xe0d12fbf, "ldrh r2, [r1], #0xff");
        assert_asm!(0xe01120b3, "ldrh r2, [r1], -r3");
        assert_asm!(0xe15101b4, "ldrh r0, [r1, #-0x14]");
        assert_asm!(0xe1f030b1, "ldrh r3, [r0, #0x1]!");
    }

    #[test]
    fn test_ldrsb() {
        assert_asm!(0xe1d12fdf, "ldrsb r2, [r1, #0xff]");
        assert_asm!(0xe11120d3, "ldrsb r2, [r1, -r3]");
        assert_asm!(0xe1712fdf, "ldrsb r2, [r1, #-0xff]!");
        assert_asm!(0xe1b120d3, "ldrsb r2, [r1, r3]!");
        assert_asm!(0xe0d12fdf, "ldrsb r2, [r1], #0xff");
        assert_asm!(0xe01120d3, "ldrsb r2, [r1], -r3");
        assert_asm!(0xe15101d4, "ldrsb r0, [r1, #-0x14]");
        assert_asm!(0xe1f030d1, "ldrsb r3, [r0, #0x1]!");
    }

    #[test]
    fn test_ldrsh() {
        assert_asm!(0xe1d12fff, "ldrsh r2, [r1, #0xff]");
        assert_asm!(0xe11120f3, "ldrsh r2, [r1, -r3]");
        assert_asm!(0xe1712fff, "ldrsh r2, [r1, #-0xff]!");
        assert_asm!(0xe1b120f3, "ldrsh r2, [r1, r3]!");
        assert_asm!(0xe0d12fff, "ldrsh r2, [r1], #0xff");
        assert_asm!(0xe01120f3, "ldrsh r2, [r1], -r3");
        assert_asm!(0xe15101f4, "ldrsh r0, [r1, #-0x14]");
        assert_asm!(0xe1f030f1, "ldrsh r3, [r0, #0x1]!");
    }

    #[test]
    fn test_ldrt() {
        assert_asm!(0xe4b12fff, "ldrt r2, [r1], #0xfff");
        assert_asm!(0xe6312003, "ldrt r2, [r1], -r3");
        assert_asm!(0xe6b12023, "ldrt r2, [r1], r3, lsr #0x20");
    }

    #[test]
    fn test_lsl() {
        assert_asm!(0x11a0960a, "lslne r9, r10, #0xc");
    }

    #[test]
    fn test_lsr() {
        assert_asm!(0x41a0f238, "lsrmi pc, r8, r2");
    }

    #[test]
    fn test_mcr() {
        assert_asm!(0xee2234b6, "mcr p4, #1, r3, c2, c6, #5");
        assert_asm!(0x3ec54351, "mcrlo p3, #6, r4, c5, c1, #2");
    }

    #[test]
    fn test_mcr2() {
        assert_asm!(0xfe2234b6, "mcr2 p4, #1, r3, c2, c6, #5");
        assert_asm!(0xfec54351, "mcr2 p3, #6, r4, c5, c1, #2");
    }

    #[test]
    fn test_mcrr() {
        assert_asm!(0xec412345, "mcrr p3, #4, r2, r1, c5");
    }

    #[test]
    fn test_mcrr2() {
        assert_asm!(0xfc412345, "mcrr2 p3, #4, r2, r1, c5");
    }

    #[test]
    fn test_mla() {
        assert_asm!(0xe0212394, "mla r1, r4, r3, r2");
        assert_asm!(0xa0312394, "mlasge r1, r4, r3, r2");
    }

    #[test]
    fn test_mov() {
        assert_asm!(0xe1a02003, "mov r2, r3");
        assert_asm!(0xe3a05e23, "mov r5, #0x230");
    }

    #[test]
    fn test_mrc() {
        assert_asm!(0xee3234b6, "mrc p4, #1, r3, c2, c6, #5");
        assert_asm!(0x3ed54351, "mrclo p3, #6, r4, c5, c1, #2");
    }

    #[test]
    fn test_mrc2() {
        assert_asm!(0xfe3234b6, "mrc2 p4, #1, r3, c2, c6, #5");
        assert_asm!(0xfed54351, "mrc2 p3, #6, r4, c5, c1, #2");
    }

    #[test]
    fn test_mrrc() {
        assert_asm!(0xec512345, "mrrc p3, #4, r2, r1, c5");
    }

    #[test]
    fn test_mrrc2() {
        assert_asm!(0xfc512345, "mrrc2 p3, #4, r2, r1, c5");
    }

    #[test]
    fn test_mrs() {
        assert_asm!(0xe10f7000, "mrs r7, cpsr");
        assert_asm!(0xe14f7000, "mrs r7, spsr");
    }

    #[test]
    fn test_msr() {
        assert_asm!(0xe36cf042, "msr spsr_fs, #0x42");
        assert_asm!(0xe323f042, "msr cpsr_xc, #0x42");
        assert_asm!(0xe165f001, "msr spsr_sc, r1");
        assert_asm!(0xe12af001, "msr cpsr_fx, r1");
    }

    #[test]
    fn test_mul() {
        assert_asm!(0xe0010293, "mul r1, r3, r2");
        assert_asm!(0x10110293, "mulsne r1, r3, r2");
    }

    #[test]
    fn test_mvn() {
        assert_asm!(0xe1e02003, "mvn r2, r3");
        assert_asm!(0xe3e05e23, "mvn r5, #0x230");
        assert_asm!(0x11e0960a, "mvnne r9, r10, lsl #0xc");
        assert_asm!(0x41e0f238, "mvnmi pc, r8, lsr r2");
        assert_asm!(0x71e0046e, "mvnvc r0, lr, ror #0x8");
        assert_asm!(0xb1e07060, "mvnlt r7, r0, rrx");
        assert_asm!(0xe1f02153, "mvns r2, r3, asr r1");
    }

    #[test]
    fn test_nop() {
        assert_asm!(0xe320f000, "nop");
        assert_asm!(0x0320f000, "nopeq");
    }

    #[test]
    fn test_orr() {
        assert_asm!(0xe1812003, "orr r2, r1, r3");
        assert_asm!(0xe3845e23, "orr r5, r4, #0x230");
        assert_asm!(0x118b960a, "orrne r9, r11, r10, lsl #0xc");
        assert_asm!(0x4185f238, "orrmi pc, r5, r8, lsr r2");
        assert_asm!(0x7182046e, "orrvc r0, r2, lr, ror #0x8");
        assert_asm!(0xb1887060, "orrlt r7, r8, r0, rrx");
        assert_asm!(0xe1952153, "orrs r2, r5, r3, asr r1");
    }

    #[test]
    fn test_pkhbt() {
        assert_asm!(0xe6812893, "pkhbt r2, r1, r3, lsl #0x11");
        assert_asm!(0x06812013, "pkhbteq r2, r1, r3");
    }

    #[test]
    fn test_pkhtb() {
        assert_asm!(0xe68128d3, "pkhtb r2, r1, r3, asr #0x11");
        assert_asm!(0x06812053, "pkhtbeq r2, r1, r3, asr #0x20");
    }

    #[test]
    fn test_pld() {
        assert_asm!(0xf5d1ffff, "pld [r1, #0xfff]");
        assert_asm!(0xf751f003, "pld [r1, -r3]");
        assert_asm!(0xf7d1f0e3, "pld [r1, r3, ror #0x1]");
    }

    #[test]
    fn test_pop() {
        assert_asm!(0xe8bd0505, "pop {r0, r2, r8, r10}");
        assert_asm!(0xa49d5004, "popge {r5}");
    }

    #[test]
    fn test_push() {
        assert_asm!(0xe92d0505, "push {r0, r2, r8, r10}");
        assert_asm!(0xa52d5004, "pushge {r5}");
    }

    #[test]
    fn test_qadd() {
        assert_asm!(0xe1012053, "qadd r2, r3, r1");
    }

    #[test]
    fn test_qadd16() {
        assert_asm!(0xe6212f13, "qadd16 r2, r1, r3");
    }

    #[test]
    fn test_qadd8() {
        assert_asm!(0xe6212f93, "qadd8 r2, r1, r3");
    }

    #[test]
    fn test_qasx() {
        assert_asm!(0xe6212f33, "qasx r2, r1, r3");
    }

    #[test]
    fn test_qdadd() {
        assert_asm!(0xe1412053, "qdadd r2, r3, r1");
    }

    #[test]
    fn test_qdsub() {
        assert_asm!(0xe1612053, "qdsub r2, r3, r1");
    }

    #[test]
    fn test_qsub() {
        assert_asm!(0xe1212053, "qsub r2, r3, r1");
    }

    #[test]
    fn test_qsub16() {
        assert_asm!(0xe6212f73, "qsub16 r2, r1, r3");
    }

    #[test]
    fn test_qsub8() {
        assert_asm!(0xe6212ff3, "qsub8 r2, r1, r3");
    }

    #[test]
    fn test_qsax() {
        assert_asm!(0xe6212f53, "qsax r2, r1, r3");
    }

    #[test]
    fn test_rev() {
        assert_asm!(0xe6bf1f32, "rev r1, r2");
        assert_asm!(0xa6bfff3e, "revge pc, lr");
    }

    #[test]
    fn test_rev16() {
        assert_asm!(0xe6bf1fb2, "rev16 r1, r2");
        assert_asm!(0xa6bfffbe, "rev16ge pc, lr");
    }

    #[test]
    fn test_revsh() {
        assert_asm!(0xe6ff1fb2, "revsh r1, r2");
        assert_asm!(0xa6ffffbe, "revshge pc, lr");
    }

    #[test]
    fn test_rfe() {
        assert_asm!(0xf8170a00, "rfeda r7");
        assert_asm!(0xf9370a00, "rfedb r7!");
        assert_asm!(0xf8970a00, "rfeia r7");
        assert_asm!(0xf9b70a00, "rfeib r7!");
    }

    #[test]
    fn test_ror() {
        assert_asm!(0x71a0046e, "rorvc r0, lr, #0x8");
    }

    #[test]
    fn test_rrx() {
        assert_asm!(0xb1a07060, "rrxlt r7, r0");
    }

    #[test]
    fn test_rsb() {
        assert_asm!(0xe0612003, "rsb r2, r1, r3");
        assert_asm!(0xe2645e23, "rsb r5, r4, #0x230");
        assert_asm!(0x106b960a, "rsbne r9, r11, r10, lsl #0xc");
        assert_asm!(0x4065f238, "rsbmi pc, r5, r8, lsr r2");
        assert_asm!(0x7062046e, "rsbvc r0, r2, lr, ror #0x8");
        assert_asm!(0xb0687060, "rsblt r7, r8, r0, rrx");
        assert_asm!(0xe0752153, "rsbs r2, r5, r3, asr r1");
    }

    #[test]
    fn test_rsc() {
        assert_asm!(0xe0e12003, "rsc r2, r1, r3");
        assert_asm!(0xe2e45e23, "rsc r5, r4, #0x230");
        assert_asm!(0x10eb960a, "rscne r9, r11, r10, lsl #0xc");
        assert_asm!(0x40e5f238, "rscmi pc, r5, r8, lsr r2");
        assert_asm!(0x70e2046e, "rscvc r0, r2, lr, ror #0x8");
        assert_asm!(0xb0e87060, "rsclt r7, r8, r0, rrx");
        assert_asm!(0xe0f52153, "rscs r2, r5, r3, asr r1");
    }

    #[test]
    fn test_sadd16() {
        assert_asm!(0xe6112f13, "sadd16 r2, r1, r3");
    }

    #[test]
    fn test_sadd8() {
        assert_asm!(0xe6112f93, "sadd8 r2, r1, r3");
    }

    #[test]
    fn test_sasx() {
        assert_asm!(0xe6112f33, "sasx r2, r1, r3");
    }

    #[test]
    fn test_sbc() {
        assert_asm!(0xe0c12003, "sbc r2, r1, r3");
        assert_asm!(0xe2c45e23, "sbc r5, r4, #0x230");
        assert_asm!(0x10cb960a, "sbcne r9, r11, r10, lsl #0xc");
        assert_asm!(0x40c5f238, "sbcmi pc, r5, r8, lsr r2");
        assert_asm!(0x70c2046e, "sbcvc r0, r2, lr, ror #0x8");
        assert_asm!(0xb0c87060, "sbclt r7, r8, r0, rrx");
        assert_asm!(0xe0d52153, "sbcs r2, r5, r3, asr r1");
    }

    #[test]
    fn test_sel() {
        assert_asm!(0xe6812fb3, "sel r2, r1, r3");
    }

    #[test]
    fn test_setend() {
        assert_asm!(0xf1010000, "setend le");
        assert_asm!(0xf1010200, "setend be");
    }

    #[test]
    fn test_sev() {
        assert_asm!(0xe320f004, "sev");
        assert_asm!(0x3320f004, "sevlo");
    }

    #[test]
    fn test_shadd16() {
        assert_asm!(0xe6312f13, "shadd16 r2, r1, r3");
    }

    #[test]
    fn test_shadd8() {
        assert_asm!(0xe6312f93, "shadd8 r2, r1, r3");
    }

    #[test]
    fn test_shasx() {
        assert_asm!(0xe6312f33, "shasx r2, r1, r3");
    }

    #[test]
    fn test_shsub16() {
        assert_asm!(0xe6312f73, "shsub16 r2, r1, r3");
    }

    #[test]
    fn test_shsub8() {
        assert_asm!(0xe6312ff3, "shsub8 r2, r1, r3");
    }

    #[test]
    fn test_shsax() {
        assert_asm!(0xe6312f53, "shsax r2, r1, r3");
    }

    #[test]
    fn test_smla() {
        assert_asm!(0xe1012384, "smlabb r1, r4, r3, r2");
        assert_asm!(0xe10123a4, "smlatb r1, r4, r3, r2");
        assert_asm!(0xe10123c4, "smlabt r1, r4, r3, r2");
        assert_asm!(0xe10123e4, "smlatt r1, r4, r3, r2");
    }

    #[test]
    fn test_smlad() {
        assert_asm!(0xe7012314, "smlad r1, r4, r3, r2");
        assert_asm!(0x07012334, "smladxeq r1, r4, r3, r2");
    }

    #[test]
    fn test_smlal() {
        assert_asm!(0xe0e12394, "smlal r2, r1, r4, r3");
        assert_asm!(0xa0f12394, "smlalsge r2, r1, r4, r3");
        assert_asm!(0xe1412384, "smlalbb r2, r1, r4, r3");
        assert_asm!(0xe14123a4, "smlaltb r2, r1, r4, r3");
        assert_asm!(0xe14123c4, "smlalbt r2, r1, r4, r3");
        assert_asm!(0xe14123e4, "smlaltt r2, r1, r4, r3");
    }

    #[test]
    fn test_smlald() {
        assert_asm!(0xe7412314, "smlald r2, r1, r4, r3");
        assert_asm!(0x07412334, "smlaldxeq r2, r1, r4, r3");
    }

    #[test]
    fn test_smlaw() {
        assert_asm!(0xe1212384, "smlawb r1, r4, r3, r2");
        assert_asm!(0xe12123c4, "smlawt r1, r4, r3, r2");
    }

    #[test]
    fn test_smlsd() {
        assert_asm!(0xe7012354, "smlsd r1, r4, r3, r2");
        assert_asm!(0x07012374, "smlsdxeq r1, r4, r3, r2");
    }

    #[test]
    fn test_smlsld() {
        assert_asm!(0xe7412354, "smlsld r2, r1, r4, r3");
        assert_asm!(0x07412374, "smlsldxeq r2, r1, r4, r3");
    }

    #[test]
    fn test_smmla() {
        assert_asm!(0xe7512314, "smmla r1, r4, r3, r2");
        assert_asm!(0x07512334, "smmlareq r1, r4, r3, r2");
    }

    #[test]
    fn test_smmls() {
        assert_asm!(0xe75123d4, "smmls r1, r4, r3, r2");
        assert_asm!(0x075123f4, "smmlsreq r1, r4, r3, r2");
    }

    #[test]
    fn test_smmul() {
        assert_asm!(0xe751f314, "smmul r1, r4, r3");
        assert_asm!(0x0751f334, "smmulreq r1, r4, r3");
    }

    #[test]
    fn test_smuad() {
        assert_asm!(0xe701f314, "smuad r1, r4, r3");
        assert_asm!(0x0701f334, "smuadxeq r1, r4, r3");
    }

    #[test]
    fn test_smul() {
        assert_asm!(0xe1610384, "smulbb r1, r4, r3");
        assert_asm!(0xe16103a4, "smultb r1, r4, r3");
        assert_asm!(0xe16103c4, "smulbt r1, r4, r3");
        assert_asm!(0xe16103e4, "smultt r1, r4, r3");
    }

    #[test]
    fn test_smull() {
        assert_asm!(0xe0c12394, "smull r2, r1, r4, r3");
        assert_asm!(0xa0d12394, "smullsge r2, r1, r4, r3");
    }

    #[test]
    fn test_smulw() {
        assert_asm!(0xe12103a4, "smulwb r1, r4, r3");
        assert_asm!(0xe12103e4, "smulwt r1, r4, r3");
    }

    #[test]
    fn test_smusd() {
        assert_asm!(0xe701f354, "smusd r1, r4, r3");
        assert_asm!(0x0701f374, "smusdxeq r1, r4, r3");
    }

    #[test]
    fn test_srs() {
        assert_asm!(0xf84d051f, "srsda sp, #0x1f");
        assert_asm!(0xf96d051f, "srsdb sp!, #0x1f");
        assert_asm!(0xf8cd051f, "srsia sp, #0x1f");
        assert_asm!(0xf9ed051f, "srsib sp!, #0x1f");
    }

    #[test]
    fn test_ssat() {
        assert_asm!(0xe6af1512, "ssat r1, #0x10, r2, lsl #0xa");
        assert_asm!(0x06b94a53, "ssateq r4, #0x1a, r3, asr #0x14");
    }

    #[test]
    fn test_ssat16() {
        assert_asm!(0xe6af1f32, "ssat16 r1, #0x10, r2");
        assert_asm!(0x06a94f33, "ssat16eq r4, #0xa, r3");
    }

    #[test]
    fn test_ssub16() {
        assert_asm!(0xe6112f73, "ssub16 r2, r1, r3");
    }

    #[test]
    fn test_ssub8() {
        assert_asm!(0xe6112ff3, "ssub8 r2, r1, r3");
    }

    #[test]
    fn test_ssax() {
        assert_asm!(0xe6112f53, "ssax r2, r1, r3");
    }

    #[test]
    fn test_stc() {
        assert_asm!(0xed032169, "stc p1, c2, [r3, #-0x1a4]");
        assert_asm!(0x4d232169, "stcmi p1, c2, [r3, #-0x1a4]!");
        assert_asm!(0x6c232169, "stcvs p1, c2, [r3], #-0x1a4");
        assert_asm!(0x3da32169, "stclo p1, c2, [r3, #0x1a4]!");
        assert_asm!(0x5c832169, "stcpl p1, c2, [r3], {0x69}");
    }
    #[test]
    fn test_stc2() {
        assert_asm!(0xfd032169, "stc2 p1, c2, [r3, #-0x1a4]");
        assert_asm!(0xfd232169, "stc2 p1, c2, [r3, #-0x1a4]!");
        assert_asm!(0xfc232169, "stc2 p1, c2, [r3], #-0x1a4");
        assert_asm!(0xfda32169, "stc2 p1, c2, [r3, #0x1a4]!");
        assert_asm!(0xfc832169, "stc2 p1, c2, [r3], {0x69}");
    }

    #[test]
    fn test_stm() {
        assert_asm!(0xe821aaaa, "stmda r1!, {r1, r3, r5, r7, r9, r11, sp, pc}");
        assert_asm!(0xb8a25555, "stmlt r2!, {r0, r2, r4, r6, r8, r10, r12, lr}");
        assert_asm!(0xd903cccc, "stmdble r3, {r2, r3, r6, r7, r10, r11, lr, pc}");
        assert_asm!(0xc9843333, "stmibgt r4, {r0, r1, r4, r5, r8, r9, r12, sp}");
        assert_asm!(0xe8450003, "stmda r5, {r0, r1}^");
        assert_asm!(0xe8468003, "stmda r6, {r0, r1, pc}^");
        assert_asm!(
            0xe9e07ffc,
            "stmib r0!, {r2, r3, r4, r5, r6, r7, r8, r9, r10, r11, r12, sp, lr}^"
        );
    }

    #[test]
    fn test_str() {
        assert_asm!(0xe5812fff, "str r2, [r1, #0xfff]");
        assert_asm!(0xe7012003, "str r2, [r1, -r3]");
        assert_asm!(0xe78120e3, "str r2, [r1, r3, ror #0x1]");
        assert_asm!(0xe5212fff, "str r2, [r1, #-0xfff]!");
        assert_asm!(0xe7a12003, "str r2, [r1, r3]!");
        assert_asm!(0xe7212063, "str r2, [r1, -r3, rrx]!");
        assert_asm!(0xe4812fff, "str r2, [r1], #0xfff");
        assert_asm!(0xe6012003, "str r2, [r1], -r3");
        assert_asm!(0xe6812023, "str r2, [r1], r3, lsr #0x20");
    }

    #[test]
    fn test_strb() {
        assert_asm!(0xe5c12fff, "strb r2, [r1, #0xfff]");
        assert_asm!(0xe7412003, "strb r2, [r1, -r3]");
        assert_asm!(0xe7c120e3, "strb r2, [r1, r3, ror #0x1]");
        assert_asm!(0xe5612fff, "strb r2, [r1, #-0xfff]!");
        assert_asm!(0xe7e12003, "strb r2, [r1, r3]!");
        assert_asm!(0xe7612063, "strb r2, [r1, -r3, rrx]!");
        assert_asm!(0xe4c12fff, "strb r2, [r1], #0xfff");
        assert_asm!(0xe6412003, "strb r2, [r1], -r3");
        assert_asm!(0xe6c12023, "strb r2, [r1], r3, lsr #0x20");
    }

    #[test]
    fn test_strbt() {
        assert_asm!(0xe4e12fff, "strbt r2, [r1], #0xfff");
        assert_asm!(0xe6612003, "strbt r2, [r1], -r3");
        assert_asm!(0xe6e12023, "strbt r2, [r1], r3, lsr #0x20");
    }

    #[test]
    fn test_strd() {
        assert_asm!(0xe1c12fff, "strd r2, r3, [r1, #0xff]");
        assert_asm!(0xe10120f3, "strd r2, r3, [r1, -r3]");
        assert_asm!(0xe1612fff, "strd r2, r3, [r1, #-0xff]!");
        assert_asm!(0xe1a120f3, "strd r2, r3, [r1, r3]!");
        assert_asm!(0xe0c12fff, "strd r2, r3, [r1], #0xff");
        assert_asm!(0xe00120f3, "strd r2, r3, [r1], -r3");
    }

    #[test]
    fn test_strex() {
        assert_asm!(0xe1812f93, "strex r2, r3, [r1]");
        assert_asm!(0x21812f93, "strexhs r2, r3, [r1]");
    }

    #[test]
    fn test_strh() {
        assert_asm!(0xe1c12fbf, "strh r2, [r1, #0xff]");
        assert_asm!(0xe10120b3, "strh r2, [r1, -r3]");
        assert_asm!(0xe1612fbf, "strh r2, [r1, #-0xff]!");
        assert_asm!(0xe1a120b3, "strh r2, [r1, r3]!");
        assert_asm!(0xe0c12fbf, "strh r2, [r1], #0xff");
        assert_asm!(0xe00120b3, "strh r2, [r1], -r3");
    }

    #[test]
    fn test_strt() {
        assert_asm!(0xe4a12fff, "strt r2, [r1], #0xfff");
        assert_asm!(0xe6212003, "strt r2, [r1], -r3");
        assert_asm!(0xe6a12023, "strt r2, [r1], r3, lsr #0x20");
    }

    #[test]
    fn test_sub() {
        assert_asm!(0xe0412003, "sub r2, r1, r3");
        assert_asm!(0xe2445e23, "sub r5, r4, #0x230");
        assert_asm!(0x104b960a, "subne r9, r11, r10, lsl #0xc");
        assert_asm!(0x4045f238, "submi pc, r5, r8, lsr r2");
        assert_asm!(0x7042046e, "subvc r0, r2, lr, ror #0x8");
        assert_asm!(0xb0487060, "sublt r7, r8, r0, rrx");
        assert_asm!(0xe0552153, "subs r2, r5, r3, asr r1");
        assert_asm!(0xe24f41a5, "sub r4, pc, #0x40000029");
    }

    #[test]
    fn test_svc() {
        assert_asm!(0xef123456, "svc #0x123456");
        assert_asm!(0x0fabcdef, "svceq #0xabcdef");
    }

    #[test]
    fn test_swp() {
        assert_asm!(0xe1012093, "swp r2, r3, [r1]");
    }

    #[test]
    fn test_swpb() {
        assert_asm!(0xe1412093, "swpb r2, r3, [r1]");
    }

    #[test]
    fn test_sxtab() {
        assert_asm!(0xe6a12073, "sxtab r2, r1, r3");
        assert_asm!(0x06a12c73, "sxtabeq r2, r1, r3, ror #0x18");
    }

    #[test]
    fn test_sxtab16() {
        assert_asm!(0xe6812073, "sxtab16 r2, r1, r3");
        assert_asm!(0x06812c73, "sxtab16eq r2, r1, r3, ror #0x18");
    }

    #[test]
    fn test_sxtah() {
        assert_asm!(0xe6b12073, "sxtah r2, r1, r3");
        assert_asm!(0x06b12c73, "sxtaheq r2, r1, r3, ror #0x18");
    }

    #[test]
    fn test_sxtb() {
        assert_asm!(0xe6af2073, "sxtb r2, r3");
        assert_asm!(0x06af2c73, "sxtbeq r2, r3, ror #0x18");
    }

    #[test]
    fn test_sxtb16() {
        assert_asm!(0xe68f2073, "sxtb16 r2, r3");
        assert_asm!(0x068f2c73, "sxtb16eq r2, r3, ror #0x18");
    }

    #[test]
    fn test_sxth() {
        assert_asm!(0xe6bf2073, "sxth r2, r3");
        assert_asm!(0x06bf2c73, "sxtheq r2, r3, ror #0x18");
    }

    #[test]
    fn test_teq() {
        assert_asm!(0xe1310003, "teq r1, r3");
        assert_asm!(0xe3340e23, "teq r4, #0x230");
        assert_asm!(0x113b060a, "teqne r11, r10, lsl #0xc");
        assert_asm!(0x41350238, "teqmi r5, r8, lsr r2");
        assert_asm!(0x7132046e, "teqvc r2, lr, ror #0x8");
        assert_asm!(0xb1380060, "teqlt r8, r0, rrx");
        assert_asm!(0xe1350153, "teq r5, r3, asr r1");
    }

    #[test]
    fn test_tst() {
        assert_asm!(0xe1110003, "tst r1, r3");
        assert_asm!(0xe3140e23, "tst r4, #0x230");
        assert_asm!(0x111b060a, "tstne r11, r10, lsl #0xc");
        assert_asm!(0x41150238, "tstmi r5, r8, lsr r2");
        assert_asm!(0x7112046e, "tstvc r2, lr, ror #0x8");
        assert_asm!(0xb1180060, "tstlt r8, r0, rrx");
        assert_asm!(0xe1150153, "tst r5, r3, asr r1");
    }

    #[test]
    fn test_uadd16() {
        assert_asm!(0xe6512f13, "uadd16 r2, r1, r3");
    }

    #[test]
    fn test_uadd8() {
        assert_asm!(0xe6512f93, "uadd8 r2, r1, r3");
    }

    #[test]
    fn test_uasx() {
        assert_asm!(0xe6512f33, "uasx r2, r1, r3");
    }

    #[test]
    fn test_udf() {
        assert_asm!(0xe7f000f0, "udf #0x0");
        assert_asm!(0xe7fa45f5, "udf #0xa455");
        assert_asm!(0xe7ffffff, "udf #0xffff");
    }

    #[test]
    fn test_uhadd16() {
        assert_asm!(0xe6712f13, "uhadd16 r2, r1, r3");
    }

    #[test]
    fn test_uhadd8() {
        assert_asm!(0xe6712f93, "uhadd8 r2, r1, r3");
    }

    #[test]
    fn test_uhasx() {
        assert_asm!(0xe6712f33, "uhasx r2, r1, r3");
    }

    #[test]
    fn test_uhsub16() {
        assert_asm!(0xe6712f73, "uhsub16 r2, r1, r3");
    }

    #[test]
    fn test_uhsub8() {
        assert_asm!(0xe6712ff3, "uhsub8 r2, r1, r3");
    }

    #[test]
    fn test_uhsax() {
        assert_asm!(0xe6712f53, "uhsax r2, r1, r3");
    }

    #[test]
    fn test_umaal() {
        assert_asm!(0xe0412394, "umaal r2, r1, r4, r3");
    }

    #[test]
    fn test_umlal() {
        assert_asm!(0xe0a12394, "umlal r2, r1, r4, r3");
        assert_asm!(0xa0b12394, "umlalsge r2, r1, r4, r3");
    }

    #[test]
    fn test_umull() {
        assert_asm!(0xe0812394, "umull r2, r1, r4, r3");
        assert_asm!(0xa0912394, "umullsge r2, r1, r4, r3");
    }

    #[test]
    fn test_uqadd16() {
        assert_asm!(0xe6612f13, "uqadd16 r2, r1, r3");
    }

    #[test]
    fn test_uqadd8() {
        assert_asm!(0xe6612f93, "uqadd8 r2, r1, r3");
    }

    #[test]
    fn test_uqasx() {
        assert_asm!(0xe6612f33, "uqasx r2, r1, r3");
    }

    #[test]
    fn test_uqsub16() {
        assert_asm!(0xe6612f73, "uqsub16 r2, r1, r3");
    }

    #[test]
    fn test_uqsub8() {
        assert_asm!(0xe6612ff3, "uqsub8 r2, r1, r3");
    }

    #[test]
    fn test_uqsax() {
        assert_asm!(0xe6612f53, "uqsax r2, r1, r3");
    }

    #[test]
    fn test_usad8() {
        assert_asm!(0xe781f213, "usad8 r1, r3, r2");
    }

    #[test]
    fn test_usada8() {
        assert_asm!(0xe7814213, "usada8 r1, r3, r2, r4");
    }

    #[test]
    fn test_usat() {
        assert_asm!(0xe6ef1512, "usat r1, #0xf, r2, lsl #0xa");
        assert_asm!(0x06e94a53, "usateq r4, #0x9, r3, asr #0x14");
    }

    #[test]
    fn test_usat16() {
        assert_asm!(0xe6ef1f32, "usat16 r1, #0xf, r2");
        assert_asm!(0x06e94f33, "usat16eq r4, #0x9, r3");
    }

    #[test]
    fn test_usub16() {
        assert_asm!(0xe6512f73, "usub16 r2, r1, r3");
    }

    #[test]
    fn test_usub8() {
        assert_asm!(0xe6512ff3, "usub8 r2, r1, r3");
    }

    #[test]
    fn test_usax() {
        assert_asm!(0xe6512f53, "usax r2, r1, r3");
    }

    #[test]
    fn test_uxtab() {
        assert_asm!(0xe6e12073, "uxtab r2, r1, r3");
        assert_asm!(0x06e12c73, "uxtabeq r2, r1, r3, ror #0x18");
    }

    #[test]
    fn test_uxtab16() {
        assert_asm!(0xe6c12073, "uxtab16 r2, r1, r3");
        assert_asm!(0x06c12c73, "uxtab16eq r2, r1, r3, ror #0x18");
    }

    #[test]
    fn test_uxtah() {
        assert_asm!(0xe6f12073, "uxtah r2, r1, r3");
        assert_asm!(0x06f12c73, "uxtaheq r2, r1, r3, ror #0x18");
    }

    #[test]
    fn test_uxtb() {
        assert_asm!(0xe6ef2073, "uxtb r2, r3");
        assert_asm!(0x06ef2c73, "uxtbeq r2, r3, ror #0x18");
    }

    #[test]
    fn test_uxtb16() {
        assert_asm!(0xe6cf2073, "uxtb16 r2, r3");
        assert_asm!(0x06cf2c73, "uxtb16eq r2, r3, ror #0x18");
    }

    #[test]
    fn test_uxth() {
        assert_asm!(0xe6ff2073, "uxth r2, r3");
        assert_asm!(0x06ff2c73, "uxtheq r2, r3, ror #0x18");
    }

    #[test]
    fn test_wfe() {
        assert_asm!(0xe320f002, "wfe");
        assert_asm!(0xa320f002, "wfege");
    }

    #[test]
    fn test_wfi() {
        assert_asm!(0xe320f003, "wfi");
        assert_asm!(0x4320f003, "wfimi");
    }

    #[test]
    fn test_yield() {
        assert_asm!(0xe320f001, "yield");
        assert_asm!(0xd320f001, "yieldle");
    }
}
