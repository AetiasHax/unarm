#[cfg(test)]
mod tests {
    use unarm::{
        CoReg, DefUseArgument, Dreg, DregIndex, DregList, Fpscr, Options, Reg, RegList, Sreg,
        SregList, StatusFields, StatusReg, parse_arm, parse_thumb,
    };

    macro_rules! options {
        () => {{
            Options {
                version: unarm::Version::V6K,
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
        ($code:literal, $disasm:literal, $defs:expr, $uses:expr) => {{
            let options = options!();
            let ins = parse_arm($code, 0, &options);
            let defs = ins.defs().into_iter().collect::<Vec<_>>();
            assert_eq!(defs, $defs, "mismatched defs");
            let uses = ins.uses().into_iter().collect::<Vec<_>>();
            assert_eq!(uses, $uses, "mismatched uses")
        }};
    }

    macro_rules! assert_thumb {
        ($code:literal, $disasm:literal, $defs:expr, $uses:expr) => {{
            let options = options!();
            let (ins, _size) = parse_thumb($code, 0, &options);
            let defs = ins.defs().into_iter().collect::<Vec<_>>();
            assert_eq!(defs, $defs, "mismatched defs");
            let uses = ins.uses().into_iter().collect::<Vec<_>>();
            assert_eq!(uses, $uses, "mismatched uses")
        }};
        ($code:literal, $next:literal, $disasm:literal, $defs:expr, $uses:expr) => {{
            let options = options!();
            let (ins, _size) = parse_thumb($code | ($next << 16), 0, &options);
            let defs = ins.defs().into_iter().collect::<Vec<_>>();
            assert_eq!(defs, $defs, "mismatched defs");
            let uses = ins.uses().into_iter().collect::<Vec<_>>();
            assert_eq!(uses, $uses, "mismatched uses")
        }};
    }

    macro_rules! defs {
        ( $( $arg:expr ),* ) => {
            vec![$(DefUseArgument::from($arg)),*]
        }
    }
    macro_rules! uses {
        ( $( $arg:expr ),* ) => {
            vec![$(DefUseArgument::from($arg)),*]
        }
    }

    #[test]
    fn test_arm() {
        assert_arm!(0xe0a12003, "adc r2, r1, r3", defs!(Reg::R2), uses!(Reg::R1, Reg::R3));
        assert_arm!(0xe2a45e23, "adc r5, r4, #0x230", defs!(Reg::R5), uses!(Reg::R4));
        assert_arm!(
            0x40a5e238,
            "adcmi lr, r5, r8, lsr r2",
            defs!(Reg::Lr),
            uses!(Reg::R5, Reg::R8, Reg::R2)
        );

        assert_arm!(0xe1b02153, "asrs r2, r3, r1", defs!(Reg::R2), uses!(Reg::R3, Reg::R1));

        assert_arm!(0xea000000, "b #0x8", defs!(Reg::Pc), uses!());

        assert_arm!(0xe1200070, "bkpt #0x0", defs!(), uses!());

        assert_arm!(0xfa000000, "blx #0x8", defs!(Reg::Pc), uses!());
        assert_arm!(0xe12fff30, "blx r0", defs!(Reg::Pc), uses!(Reg::R0));

        assert_arm!(0xe12fff10, "bx r0", defs!(Reg::Pc), uses!(Reg::R0));

        assert_arm!(
            0xee1234a6,
            "cdp p4, #0x1, c3, c2, c6, #0x5",
            defs!(CoReg::C3),
            uses!(CoReg::C2, CoReg::C6)
        );

        assert_arm!(0xe16f5f1f, "clz r5, pc", defs!(Reg::R5), uses!(Reg::Pc));

        assert_arm!(0xe1510003, "cmp r1, r3", defs!(), uses!(Reg::R1, Reg::R3));
        assert_arm!(0xe3540e23, "cmp r4, #0x230", defs!(), uses!(Reg::R4));
        assert_arm!(0x41550238, "cmpmi r5, r8, lsr r2", defs!(), uses!(Reg::R5, Reg::R8, Reg::R2));

        assert_arm!(0xf102001a, "cps #0x1a", defs!(), uses!());

        assert_arm!(0xed132169, "ldc p1, c2, [r3, #-0x1a4]", defs!(CoReg::C2), uses!(Reg::R3));
        assert_arm!(
            0x4d332169,
            "ldcmi p1, c2, [r3, #-0x1a4]!",
            defs!(CoReg::C2, Reg::R3),
            uses!(Reg::R3)
        );

        assert_arm!(
            0xe831aaaa,
            "ldmda r1!, {r1, r3, r5, r7, r9, r11, sp, pc}",
            defs!(
                RegList::of(&[
                    Reg::R1,
                    Reg::R3,
                    Reg::R5,
                    Reg::R7,
                    Reg::R9,
                    Reg::R11,
                    Reg::Sp,
                    Reg::Pc
                ]),
                Reg::R1
            ),
            uses!(Reg::R1)
        );
        assert_arm!(
            0xe8550003,
            "ldmda r5, {r0, r1}^",
            defs!(RegList::of(&[Reg::R0, Reg::R1])),
            uses!(Reg::R5)
        );

        assert_arm!(0xe5912fff, "ldr r2, [r1, #0xfff]", defs!(Reg::R2), uses!(Reg::R1));
        assert_arm!(
            0xe79120e3,
            "ldr r2, [r1, r3, ror #0x1]",
            defs!(Reg::R2),
            uses!(Reg::R1, Reg::R3)
        );
        assert_arm!(0xe5312fff, "ldr r2, [r1, #-0xfff]!", defs!(Reg::R2, Reg::R1), uses!(Reg::R1));
        assert_arm!(
            0xe7312063,
            "ldr r2, [r1, -r3, rrx]!",
            defs!(Reg::R2, Reg::R1),
            uses!(Reg::R1, Reg::R3)
        );

        assert_arm!(0xe4f12fff, "ldrbt r2, [r1], #0xfff", defs!(Reg::R2), uses!(Reg::R1));
        assert_arm!(0xe6712003, "ldrbt r2, [r1], -r3", defs!(Reg::R2), uses!(Reg::R1, Reg::R3));

        assert_arm!(
            0xe1c12fdf,
            "ldrd r2, r3, [r1, #0xff]",
            defs!(Reg::R2, Reg::R3),
            uses!(Reg::R1)
        );
        assert_arm!(
            0xe10120d3,
            "ldrd r2, r3, [r1, -r3]",
            defs!(Reg::R2, Reg::R3),
            uses!(Reg::R1, Reg::R3)
        );
        assert_arm!(
            0xe1612fdf,
            "ldrd r2, r3, [r1, #-0xff]!",
            defs!(Reg::R2, Reg::R3, Reg::R1),
            uses!(Reg::R1)
        );
        assert_arm!(
            0xe1a120d3,
            "ldrd r2, r3, [r1, r3]!",
            defs!(Reg::R2, Reg::R3, Reg::R1),
            uses!(Reg::R1, Reg::R3)
        );

        assert_arm!(0xe1912f9f, "ldrex r2, [r1]", defs!(Reg::R2), uses!(Reg::R1));

        assert_arm!(0xe1b12f9f, "ldrexd r2, r3, [r1]", defs!(Reg::R2, Reg::R3), uses!(Reg::R1));

        assert_arm!(
            0xee2234b6,
            "mcr p4, #0x1, r3, c2, c6, #0x5",
            defs!(CoReg::C2, CoReg::C6),
            uses!(Reg::R3, CoReg::C6)
        );

        assert_arm!(
            0xec412345,
            "mcrr p3, #0x4, r2, r1, c5",
            defs!(CoReg::C5),
            uses!(Reg::R2, Reg::R1)
        );

        assert_arm!(
            0xe0212394,
            "mla r1, r4, r3, r2",
            defs!(Reg::R1),
            uses!(Reg::R4, Reg::R3, Reg::R2)
        );

        assert_arm!(0xe1a02003, "mov r2, r3", defs!(Reg::R2), uses!(Reg::R3));
        assert_arm!(0xe3a05e23, "mov r5, #0x230", defs!(Reg::R5), uses!());

        assert_arm!(
            0xee3234b6,
            "mrc p4, #0x1, r3, c2, c6, #0x5",
            defs!(Reg::R3, CoReg::C6),
            uses!(CoReg::C2, CoReg::C6)
        );

        assert_arm!(
            0xec512345,
            "mrrc p3, #0x4, r2, r1, c5",
            defs!(Reg::R2, Reg::R1),
            uses!(CoReg::C5)
        );

        assert_arm!(0xe10f7000, "mrs r7, cpsr", defs!(Reg::R7), uses!(StatusReg::Cpsr));

        assert_arm!(
            0xe36cf042,
            "msr spsr_fs, #0x42",
            defs!(StatusFields { reg: StatusReg::Spsr, c: false, x: false, s: true, f: true }),
            uses!()
        );
        assert_arm!(
            0xe12af001,
            "msr cpsr_fx, r1",
            defs!(StatusFields { reg: StatusReg::Cpsr, c: false, x: true, s: false, f: true }),
            uses!(Reg::R1)
        );

        assert_arm!(0xe0010293, "mul r1, r3, r2", defs!(Reg::R1), uses!(Reg::R3, Reg::R2));

        assert_arm!(0xe320f000, "nop", defs!(), uses!());

        assert_arm!(
            0xe68128d3,
            "pkhtb r2, r1, r3, asr #0x11",
            defs!(Reg::R2),
            uses!(Reg::R1, Reg::R3)
        );

        assert_arm!(0xf5d1ffff, "pld [r1, #0xfff]", defs!(), uses!(Reg::R1));
        assert_arm!(0xf751f003, "pld [r1, -r3]", defs!(), uses!(Reg::R1, Reg::R3));

        assert_arm!(
            0xe8bd0505,
            "pop {r0, r2, r8, r10}",
            defs!(Reg::Sp, RegList::of(&[Reg::R0, Reg::R2, Reg::R8, Reg::R10])),
            uses!(Reg::Sp)
        );

        assert_arm!(
            0xe92d0505,
            "push {r0, r2, r8, r10}",
            defs!(Reg::Sp),
            uses!(Reg::Sp, RegList::of(&[Reg::R0, Reg::R2, Reg::R8, Reg::R10]))
        );

        assert_arm!(0xe1012053, "qadd r2, r3, r1", defs!(Reg::R2), uses!(Reg::R3, Reg::R1));

        assert_arm!(0xe6212f13, "qadd16 r2, r1, r3", defs!(Reg::R2), uses!(Reg::R1, Reg::R3));

        assert_arm!(0xe1412053, "qdadd r2, r3, r1", defs!(Reg::R2), uses!(Reg::R3, Reg::R1));

        assert_arm!(0xe6bf1f32, "rev r1, r2", defs!(Reg::R1), uses!(Reg::R2));

        assert_arm!(0xf8170a00, "rfeda r7", defs!(), uses!(Reg::R7));
        assert_arm!(0xf9370a00, "rfedb r7!", defs!(Reg::R7), uses!(Reg::R7));

        assert_arm!(0xe6812fb3, "sel r2, r1, r3", defs!(Reg::R2), uses!(Reg::R1, Reg::R3));

        assert_arm!(
            0xe1012384,
            "smlabb r1, r4, r3, r2",
            defs!(Reg::R1),
            uses!(Reg::R4, Reg::R3, Reg::R2)
        );

        assert_arm!(
            0xe1212384,
            "smlawb r1, r4, r3, r2",
            defs!(Reg::R1),
            uses!(Reg::R4, Reg::R3, Reg::R2)
        );

        assert_arm!(
            0xe7412354,
            "smlsld r2, r1, r4, r3",
            defs!(Reg::R2, Reg::R1),
            uses!(Reg::R4, Reg::R3)
        );

        assert_arm!(0xe751f314, "smmul r1, r4, r3", defs!(Reg::R1), uses!(Reg::R4, Reg::R3));

        assert_arm!(0xf84d051f, "srsda sp, #0x1f", defs!(), uses!(Reg::Sp));
        assert_arm!(0xf96d051f, "srsdb sp!, #0x1f", defs!(Reg::Sp), uses!(Reg::Sp));

        assert_arm!(0xe6af1512, "ssat r1, #0x10, r2, lsl #0xa", defs!(Reg::R1), uses!(Reg::R2));

        assert_arm!(0xed032169, "stc p1, c2, [r3, #-0x1a4]", defs!(), uses!(CoReg::C2, Reg::R3));
        assert_arm!(
            0x4d232169,
            "stcmi p1, c2, [r3, #-0x1a4]!",
            defs!(Reg::R3),
            uses!(CoReg::C2, Reg::R3)
        );

        assert_arm!(
            0xe821aaaa,
            "stmda r1!, {r1, r3, r5, r7, r9, r11, sp, pc}",
            defs!(Reg::R1),
            uses!(
                RegList::of(&[
                    Reg::R1,
                    Reg::R3,
                    Reg::R5,
                    Reg::R7,
                    Reg::R9,
                    Reg::R11,
                    Reg::Sp,
                    Reg::Pc
                ]),
                Reg::R1
            )
        );
        assert_arm!(
            0xe8450003,
            "stmda r5, {r0, r1}^",
            defs!(),
            uses!(RegList::of(&[Reg::R0, Reg::R1]), Reg::R5)
        );

        assert_arm!(0xe5812fff, "str r2, [r1, #0xfff]", defs!(), uses!(Reg::R2, Reg::R1));
        assert_arm!(
            0xe78120e3,
            "str r2, [r1, r3, ror #0x1]",
            defs!(),
            uses!(Reg::R2, Reg::R1, Reg::R3)
        );
        assert_arm!(0xe5212fff, "str r2, [r1, #-0xfff]!", defs!(Reg::R1), uses!(Reg::R2, Reg::R1));
        assert_arm!(
            0xe7212063,
            "str r2, [r1, -r3, rrx]!",
            defs!(Reg::R1),
            uses!(Reg::R2, Reg::R1, Reg::R3)
        );

        assert_arm!(0xe1012093, "swp r2, r3, [r1]", defs!(Reg::R2), uses!(Reg::R3, Reg::R1));

        assert_arm!(0xe6a12073, "sxtab r2, r1, r3", defs!(Reg::R2), uses!(Reg::R1, Reg::R3));

        assert_arm!(0xe7f000f0, "udf #0x0", defs!(), uses!());

        assert_arm!(0xeef0fae9, "vabs.f32 s31, s19", defs!(Sreg::S31), uses!(Sreg::S19));
        assert_arm!(0x0ef0fbe9, "vabseq.f64 d31, d25", defs!(Dreg::D31), uses!(Dreg::D25));

        assert_arm!(
            0xee73fa29,
            "vadd.f32 s31, s6, s19",
            defs!(Sreg::S31),
            uses!(Sreg::S6, Sreg::S19)
        );
        assert_arm!(
            0x0e73fb29,
            "vaddeq.f64 d31, d3, d25",
            defs!(Dreg::D31),
            uses!(Dreg::D3, Dreg::D25)
        );

        assert_arm!(0xeef4fa69, "vcmp.f32 s31, s19", defs!(), uses!(Sreg::S31, Sreg::S19));
        assert_arm!(0xeef5fa40, "vcmp.f32 s31, #0.0", defs!(), uses!(Sreg::S31));
        assert_arm!(0x0ef4fb69, "vcmpeq.f64 d31, d25", defs!(), uses!(Dreg::D31, Dreg::D25));
        assert_arm!(0x0ef5fb40, "vcmpeq.f64 d31, #0.0", defs!(), uses!(Dreg::D31));

        assert_arm!(0x0ef7fbe9, "vcvteq.f32.f64 s31, d25", defs!(Sreg::S31), uses!(Dreg::D25));
        assert_arm!(0x1ef8fae9, "vcvtne.f32.s32 s31, s19", defs!(Sreg::S31), uses!(Sreg::S19));
        assert_arm!(0x3ef7fae9, "vcvtlo.f64.f32 d31, s19", defs!(Dreg::D31), uses!(Sreg::S19));

        assert_arm!(
            0xecb79a08,
            "vldmia r7!, {s18, s19, s20, s21, s22, s23, s24, s25}",
            defs!(SregList::range(Sreg::S18, Sreg::S25), Reg::R7),
            uses!(Reg::R7)
        );
        assert_arm!(
            0x0c979b08,
            "vldmiaeq r7, {d9, d10, d11, d12}",
            defs!(DregList::range(Dreg::D9, Dreg::D12)),
            uses!(Reg::R7)
        );

        assert_arm!(0xedd79a29, "vldr s19, [r7, #0xa4]", defs!(Sreg::S19), uses!(Reg::R7));
        assert_arm!(0x0d579b29, "vldreq d25, [r7, #-0xa4]", defs!(Dreg::D25), uses!(Reg::R7));

        assert_arm!(0xee097a90, "vmov s19, r7", defs!(Sreg::S19), uses!(Reg::R7));
        assert_arm!(
            0x0e097b90,
            "vmoveq.32 d25[0x0], r7",
            defs!(DregIndex { dreg: Dreg::D25, index: 0 }),
            uses!(Reg::R7)
        );
        assert_arm!(0xeef0fa69, "vmov.f32 s31, s19", defs!(Sreg::S31), uses!(Sreg::S19));
        assert_arm!(0x0ef0fb69, "vmoveq.f64 d31, d25", defs!(Dreg::D31), uses!(Dreg::D25));
        assert_arm!(0xee1f7a90, "vmov r7, s31", defs!(Reg::R7), uses!(Sreg::S31));
        assert_arm!(
            0x0e1f7b90,
            "vmoveq.32 r7, d31[0x0]",
            defs!(Reg::R7),
            uses!(DregIndex { dreg: Dreg::D31, index: 0 })
        );
        assert_arm!(
            0xec5b7a39,
            "vmov r7, r11, s19, s20",
            defs!(Reg::R7, Reg::R11),
            uses!(Sreg::S19, Sreg::S20)
        );
        assert_arm!(
            0xec4b7a39,
            "vmov s19, s20, r7, r11",
            defs!(Sreg::S19, Sreg::S20),
            uses!(Reg::R7, Reg::R11)
        );

        assert_arm!(0xeef17a10, "vmrs r7, fpscr", defs!(Reg::R7), uses!(Fpscr {}));

        assert_arm!(0xeee17a10, "vmsr fpscr, r7", defs!(Fpscr {}), uses!(Reg::R7));

        assert_arm!(
            0xecbd9a08,
            "vpop {s18, s19, s20, s21, s22, s23, s24, s25}",
            defs!(Reg::Sp, SregList::range(Sreg::S18, Sreg::S25)),
            uses!(Reg::Sp)
        );
        assert_arm!(
            0x0cbd9b08,
            "vpopeq {d9, d10, d11, d12}",
            defs!(Reg::Sp, DregList::range(Dreg::D9, Dreg::D12)),
            uses!(Reg::Sp)
        );

        assert_arm!(
            0xed2d9a08,
            "vpush {s18, s19, s20, s21, s22, s23, s24, s25}",
            defs!(Reg::Sp),
            uses!(Reg::Sp, SregList::range(Sreg::S18, Sreg::S25))
        );
        assert_arm!(
            0x0d2d9b08,
            "vpusheq {d9, d10, d11, d12}",
            defs!(Reg::Sp),
            uses!(Reg::Sp, DregList::range(Dreg::D9, Dreg::D12))
        );

        assert_arm!(
            0xece79a08,
            "vstmia r7!, {s19, s20, s21, s22, s23, s24, s25, s26}",
            defs!(Reg::R7),
            uses!(SregList::range(Sreg::S19, Sreg::S26), Reg::R7)
        );
        assert_arm!(
            0x0cc79b08,
            "vstmiaeq r7, {d25, d26, d27, d28}",
            defs!(),
            uses!(DregList::range(Dreg::D25, Dreg::D28), Reg::R7)
        );

        assert_arm!(0xedc79a29, "vstr s19, [r7, #0xa4]", defs!(), uses!(Sreg::S19, Reg::R7));
        assert_arm!(0x0d479b29, "vstreq d25, [r7, #-0xa4]", defs!(), uses!(Dreg::D25, Reg::R7));
    }

    #[test]
    fn test_thumb() {
        assert_thumb!(0x4157, "adcs r7, r7, r2", defs!(Reg::R7), uses!(Reg::R7, Reg::R2));

        assert_thumb!(0x1cca, "adds r2, r1, #0x3", defs!(Reg::R2), uses!(Reg::R1));
        assert_thumb!(0x1853, "adds r3, r2, r1", defs!(Reg::R3), uses!(Reg::R2, Reg::R1));
        assert_thumb!(0x44de, "add lr, lr, r11", defs!(Reg::Lr), uses!(Reg::Lr, Reg::R11));
        assert_thumb!(0xacff, "add r4, sp, #0x3fc", defs!(Reg::R4), uses!(Reg::Sp));
        assert_thumb!(0xb03a, "add sp, sp, #0xe8", defs!(Reg::Sp), uses!(Reg::Sp));
        assert_thumb!(0xa413, "add r4, pc, #0x4c", defs!(Reg::R4), uses!(Reg::Pc));

        assert_thumb!(0x1023, "asrs r3, r4, #0x20", defs!(Reg::R3), uses!(Reg::R4));
        assert_thumb!(0x4117, "asrs r7, r7, r2", defs!(Reg::R7), uses!(Reg::R7, Reg::R2));

        assert_thumb!(0xd042, "beq #0x88", defs!(Reg::Pc), uses!());

        assert_thumb!(0xf000, 0xe801, "blx #0x4", defs!(Reg::Pc), uses!());
        assert_thumb!(0x47d0, "blx r10", defs!(Reg::Pc), uses!(Reg::R10));

        assert_thumb!(0x4750, "bx r10", defs!(Reg::Pc), uses!(Reg::R10));

        assert_thumb!(0x2942, "cmp r1, #0x42", defs!(), uses!(Reg::R1));
        assert_thumb!(0x4297, "cmp r7, r2", defs!(), uses!(Reg::R7, Reg::R2));
        assert_thumb!(0x45de, "cmp lr, r11", defs!(), uses!(Reg::Lr, Reg::R11));

        assert_thumb!(
            0xc955,
            "ldm r1!, {r0, r2, r4, r6}",
            defs!(RegList::of(&[Reg::R0, Reg::R2, Reg::R4, Reg::R6]), Reg::R1),
            uses!(Reg::R1)
        );
        assert_thumb!(
            0xc9aa,
            "ldm r1, {r1, r3, r5, r7}",
            defs!(RegList::of(&[Reg::R1, Reg::R3, Reg::R5, Reg::R7])),
            uses!(Reg::R1)
        );

        assert_thumb!(0x6c22, "ldr r2, [r4, #0x40]", defs!(Reg::R2), uses!(Reg::R4));
        assert_thumb!(0x5822, "ldr r2, [r4, r0]", defs!(Reg::R2), uses!(Reg::R4, Reg::R0));

        assert_thumb!(0x2163, "movs r1, #0x63", defs!(Reg::R1), uses!());
        assert_thumb!(0x0017, "movs r7, r2", defs!(Reg::R7), uses!(Reg::R2));

        assert_thumb!(
            0xbdff,
            "pop {r0, r1, r2, r3, r4, r5, r6, r7, pc}",
            defs!(
                Reg::Sp,
                RegList::of(&[
                    Reg::R0,
                    Reg::R1,
                    Reg::R2,
                    Reg::R3,
                    Reg::R4,
                    Reg::R5,
                    Reg::R6,
                    Reg::R7,
                    Reg::Pc
                ])
            ),
            uses!(Reg::Sp)
        );

        assert_thumb!(
            0xb5ff,
            "push {r0, r1, r2, r3, r4, r5, r6, r7, lr}",
            defs!(Reg::Sp),
            uses!(
                Reg::Sp,
                RegList::of(&[
                    Reg::R0,
                    Reg::R1,
                    Reg::R2,
                    Reg::R3,
                    Reg::R4,
                    Reg::R5,
                    Reg::R6,
                    Reg::R7,
                    Reg::Lr
                ])
            )
        );

        assert_thumb!(
            0xc155,
            "stm r1!, {r0, r2, r4, r6}",
            defs!(Reg::R1),
            uses!(RegList::of(&[Reg::R0, Reg::R2, Reg::R4, Reg::R6]), Reg::R1)
        );

        assert_thumb!(0x6422, "str r2, [r4, #0x40]", defs!(), uses!(Reg::R2, Reg::R4));
        assert_thumb!(0x5022, "str r2, [r4, r0]", defs!(), uses!(Reg::R2, Reg::R4, Reg::R0));
    }
}
