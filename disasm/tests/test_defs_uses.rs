mod tests {
    use unarm::{
        args::{Argument, CoReg, OffsetReg, Reg, RegList, Register, Shift, ShiftReg, StatusMask, StatusReg},
        arm, thumb, ParseFlags,
    };

    macro_rules! assert_arm {
        ($code:literal, $defs:expr, $uses:expr) => {{
            let flags = ParseFlags::default();
            let ins = arm::Ins::new($code, &flags);

            let defs = ins.defs(&flags);
            let mut args = [Argument::None; 6];
            let target = $defs;
            args[0..target.len()].copy_from_slice(&target);
            assert_eq!(defs, args, "wrong defs");

            let uses = ins.uses(&flags);
            let mut args = [Argument::None; 6];
            let target = $uses;
            args[0..target.len()].copy_from_slice(&target);
            assert_eq!(uses, args, "wrong uses")
        }};
    }

    macro_rules! assert_thumb {
        ($code:literal, $defs:expr, $uses:expr) => {{
            let flags = ParseFlags::default();
            let ins = thumb::Ins::new($code, &flags);

            let defs = ins.defs(&flags);
            let mut args = [Argument::None; 6];
            let target = $defs;
            args[0..target.len()].copy_from_slice(&target);
            assert_eq!(defs, args, "wrong defs");

            let uses = ins.uses(&flags);
            let mut args = [Argument::None; 6];
            let target = $uses;
            args[0..target.len()].copy_from_slice(&target);
            assert_eq!(uses, args, "wrong uses")
        }};
    }

    macro_rules! reg {
        ($reg:ident) => {
            Argument::Reg(Reg {
                reg: Register::$reg,
                ..Default::default()
            })
        };
        (deref $reg:ident) => {
            Argument::Reg(Reg {
                reg: Register::$reg,
                deref: true,
                ..Default::default()
            })
        };
        (write $reg:ident) => {
            Argument::Reg(Reg {
                reg: Register::$reg,
                writeback: true,
                ..Default::default()
            })
        };
    }

    macro_rules! reglist {
        ($($reg:ident),+) => {
            Argument::RegList(RegList {
                regs: $((1 << Register::$reg as u32))|+,
                ..Default::default()
            })
        };
    }

    macro_rules! shiftreg {
        ($shift:ident, $reg:ident) => {
            Argument::ShiftReg(ShiftReg {
                reg: Register::$reg,
                op: Shift::$shift,
            })
        };
    }

    macro_rules! offsetreg {
        ($reg:ident) => {
            Argument::OffsetReg(OffsetReg {
                add: true,
                post_indexed: false,
                reg: Register::$reg,
            })
        };
        (sub $reg:ident) => {
            Argument::OffsetReg(OffsetReg {
                add: false,
                post_indexed: false,
                reg: Register::$reg,
            })
        };
    }

    macro_rules! coreg {
        ($reg:ident) => {
            Argument::CoReg(CoReg::$reg)
        };
    }

    #[test]
    pub fn test_arm() {
        // add r2, r1, r3
        assert_arm!(0xe0812003, [reg!(R2)], [reg!(R1), reg!(R3)]);
        // addmi pc, r5, r8, lsr r2
        assert_arm!(0x4085f238, [reg!(Pc)], [reg!(R5), reg!(R8), shiftreg!(Lsr, R2)]);
        // asr r1, r2, #0x3
        assert_arm!(0xe1a011c2, [reg!(R1)], [reg!(R2)]);
        // asr r1, r2, r3
        assert_arm!(0xe1a01352, [reg!(R1)], [reg!(R2), reg!(R3)]);
        // bl #0x8
        assert_arm!(0xeb000000, [], []);
        // blx r0
        assert_arm!(0xe12fff30, [], [reg!(R0)]);
        // bx r0
        assert_arm!(0xe12fff10, [], [reg!(R0)]);
        // cdp p4, #1, c3, c2, c6, #5
        assert_arm!(0xee1234a6, [coreg!(C3)], [coreg!(C2), coreg!(C6)]);
        // cmp r1, r3
        assert_arm!(0xe1510003, [], [reg!(R1), reg!(R3)]);
        // ldc p1, c2, [r3, #-0x1a4]
        assert_arm!(0xed132169, [coreg!(C2)], [reg!(deref R3)]);
        // ldmda r1!, {r1, r3, r5, r7, r9, r11, sp, pc}
        assert_arm!(
            0xe831aaaa,
            [reg!(write R1), reglist!(R1, R3, R5, R7, R9, R11, Sp, Pc)],
            [reg!(write R1)]
        );
        // ldmdble r3, {r2, r3, r6, r7, r10, r11, lr, pc}
        assert_arm!(0xd913cccc, [reglist!(R2, R3, R6, R7, R10, R11, Lr, Pc)], [reg!(R3)]);
        // ldr r2, [r1, #0xfff]
        assert_arm!(0xe5912fff, [reg!(R2)], [reg!(deref R1)]);
        // ldr r2, [r1, -r3]
        assert_arm!(
            0xe7112003,
            [reg!(R2)],
            [
                reg!(deref R1),
                Argument::OffsetReg(OffsetReg {
                    add: false,
                    post_indexed: false,
                    reg: Register::R3,
                })
            ]
        );
        // ldrd r2, r3, [r1, #0xff]
        assert_arm!(0xe1c12fdf, [reg!(R2), reg!(R3)], [reg!(deref R1)]);
        // ldrd r2, r3, [r1, -r3]
        assert_arm!(0xe10120d3, [reg!(R2), reg!(R3)], [reg!(deref R1), offsetreg!(sub R3)]);
        // lsl r1, r2, #0x3
        assert_arm!(0xe1a01182, [reg!(R1)], [reg!(R2)]);
        // lsl r1, r2, r3
        assert_arm!(0xe1a01312, [reg!(R1)], [reg!(R2), reg!(R3)]);
        // lsr r1, r2, #0x3
        assert_arm!(0xe1a011a2, [reg!(R1)], [reg!(R2)]);
        // lsr r1, r2, r3
        assert_arm!(0xe1a01332, [reg!(R1)], [reg!(R2), reg!(R3)]);
        // mcr p4, #1, r3, c2, c6, #5
        assert_arm!(0xee2234b6, [coreg!(C2), coreg!(C6)], [reg!(R3), coreg!(C6)]);
        // mcrr p3, #4, r2, r1, c5
        assert_arm!(0xec412345, [coreg!(C5)], [reg!(R2), reg!(R1)]);
        // mov r2, r3
        assert_arm!(0xe1a02003, [reg!(R2)], [reg!(R3)]);
        // mov r5, #0x230
        assert_arm!(0xe3a05e23, [reg!(R5)], []);
        // mrc p4, #1, r3, c2, c6, #5
        assert_arm!(0xee3234b6, [reg!(R3), coreg!(C6)], [coreg!(C2), coreg!(C6)]);
        // mrrc p3, #4, r2, r1, c5
        assert_arm!(0xec512345, [reg!(R2), reg!(R1)], [coreg!(C5)]);
        // mrs r7, cpsr
        assert_arm!(0xe10f7000, [reg!(R7)], [Argument::StatusReg(StatusReg::Cpsr)]);
        // msr spsr_fs, #0x42
        assert_arm!(
            0xe36cf042,
            [Argument::StatusMask(StatusMask {
                control: false,
                extension: false,
                flags: true,
                reg: StatusReg::Spsr,
                status: true,
            })],
            []
        );
        // msr spsr_sc, r1
        assert_arm!(
            0xe165f001,
            [Argument::StatusMask(StatusMask {
                control: true,
                extension: false,
                flags: false,
                reg: StatusReg::Spsr,
                status: true,
            })],
            [reg!(R1)]
        );
        // mul r1, r3, r2
        assert_arm!(0xe0010293, [reg!(R1)], [reg!(R3), reg!(R2)]);
        // mvnmi pc, r8, lsr r2
        assert_arm!(0x41e0f238, [reg!(Pc)], [reg!(R8), shiftreg!(Lsr, R2)]);
        // nopeq
        assert_arm!(0x0320f000, [], []);
        // pkhbt r2, r1, r3, lsl #0x11
        assert_arm!(0xe6812893, [reg!(R2)], [reg!(R1), reg!(R3)]);
        // pld [r1, #0xfff]
        assert_arm!(0xf5d1ffff, [], [reg!(deref R1)]);
        // pld [r1, -r3]
        assert_arm!(0xf751f003, [], [reg!(deref R1), offsetreg!(sub R3)]);
        // pop {r0, r2, r8, r10}
        assert_arm!(0xe8bd0505, [reglist!(R0, R2, R8, R10)], []);
        // popge {r5}
        assert_arm!(0xa49d5004, [reglist!(R5)], []);
        // push {r0, r2, r8, r10}
        assert_arm!(0xe92d0505, [], [reglist!(R0, R2, R8, R10)]);
        // pushge {r5}
        assert_arm!(0xa52d5004, [], [reglist!(R5)]);
        // qadd r2, r3, r1
        assert_arm!(0xe1012053, [reg!(R2)], [reg!(R3), reg!(R1)]);
        // rev r1, r2
        assert_arm!(0xe6bf1f32, [reg!(R1)], [reg!(R2)]);
        // rfeda r7
        assert_arm!(0xf8170a00, [], [reg!(R7)]);
        // rfeda r7!
        assert_arm!(0xf9370a00, [reg!(write R7)], [reg!(write R7)]);
        // ror r1, r2, #0x3
        assert_arm!(0xe1a011e2, [reg!(R1)], [reg!(R2)]);
        // ror r1, r2, r3
        assert_arm!(0xe1a01372, [reg!(R1)], [reg!(R2), reg!(R3)]);
        // smlabb r1, r4, r3, r2
        assert_arm!(0xe1012384, [reg!(R1)], [reg!(R4), reg!(R3), reg!(R2)]);
        // smlal r2, r1, r4, r3
        assert_arm!(0xe0e12394, [reg!(R2), reg!(R1)], [reg!(R2), reg!(R1), reg!(R4), reg!(R3)]);
        // ssat r1, #0x10, r2, lsl #0xa
        assert_arm!(0xe6af1512, [reg!(R1)], [reg!(R2)]);
        // stc p1, c2, [r3, #-0x1a4]
        assert_arm!(0xed032169, [coreg!(C2)], [reg!(deref R3)]);
        // stmda r1!, {r1, r3, r5, r7, r9, r11, sp, pc}
        assert_arm!(
            0xe821aaaa,
            [reg!(write R1)],
            [reg!(write R1), reglist!(R1, R3, R5, R7, R9, R11, Sp, Pc)]
        );
        // stmdble r3, {r2, r3, r6, r7, r10, r11, lr, pc}
        assert_arm!(0xd903cccc, [], [reg!(R3), reglist!(R2, R3, R6, R7, R10, R11, Lr, Pc)]);
        // str r2, [r1, #0xfff]
        assert_arm!(0xe5812fff, [], [reg!(R2), reg!(deref R1)]);
        // str r2, [r1, -r3]
        assert_arm!(0xe7012003, [], [reg!(R2), reg!(deref R1), offsetreg!(sub R3)]);
        // strd r2, r3, [r1, #0xff
        assert_arm!(0xe1c12fff, [], [reg!(R2), reg!(R3), reg!(deref R1)]);
        // strd r2, r3, [r1, -r3]
        assert_arm!(0xe10120f3, [], [reg!(R2), reg!(R3), reg!(deref R1), offsetreg!(sub R3)]);
        // swp r2, r3, [r1]
        assert_arm!(0xe1012093, [reg!(R2)], [reg!(R3), reg!(deref R1)]);
        // teq r1, r3
        assert_arm!(0xe1310003, [], [reg!(R1), reg!(R3)]);
        // teq r4, #0x230
        assert_arm!(0xe3340e23, [], [reg!(R4)]);
        // teqmi r5, r8, lsr r2
        assert_arm!(0x41350238, [], [reg!(R5), reg!(R8), shiftreg!(Lsr, R2)]);
    }

    #[test]
    pub fn test_thumb() {
        // adds r2, r1, #0x3
        assert_thumb!(0x1cca, [reg!(R2)], [reg!(R1)]);
        // adds r3, r2, r1
        assert_thumb!(0x1853, [reg!(R3)], [reg!(R2), reg!(R1)]);
        // add lr, lr, r11
        assert_thumb!(0x44de, [reg!(Lr)], [reg!(Lr), reg!(R11)]);
        // add r4, sp, #0x3fc
        assert_thumb!(0xacff, [reg!(R4)], [reg!(Sp)]);
        // asrs r3, r4, #0x20
        assert_thumb!(0x1023, [reg!(R3)], [reg!(R4)]);
        // asrs r7, r7, r2
        assert_thumb!(0x4117, [reg!(R7)], [reg!(R7), reg!(R2)]);
        // beq #0x88
        assert_thumb!(0xd042, [], []);
        // blx r10
        assert_thumb!(0x47d0, [], [reg!(R10)]);
        // cmp r1, #0x42
        assert_thumb!(0x2942, [], [reg!(R1)]);
        // cmp r7, r2
        assert_thumb!(0x4297, [], [reg!(R7), reg!(R2)]);
        // ldm r1!, {r0, r2, r4, r6}
        assert_thumb!(0xc955, [reg!(write R1)], [reg!(write R1), reglist!(R0, R2, R4, R6)]);
        // ldr r2, [r4, #0x40]
        assert_thumb!(0x6c22, [reg!(R2)], [reg!(deref R4)]);
        // ldr r2, [r4, r0]
        assert_thumb!(0x5822, [reg!(R2)], [reg!(deref R4), offsetreg!(R0)]);
        // ldr r7, [pc, #0x108]
        assert_thumb!(0x4f42, [reg!(R7)], [reg!(deref Pc)]);
        // ldr r7, [sp, #0x108]
        assert_thumb!(0x9f42, [reg!(R7)], [reg!(deref Sp)]);
        // movs r1, #0x63
        assert_thumb!(0x2163, [reg!(R1)], []);
        // movs r7, r2
        assert_thumb!(0x0017, [reg!(R7)], [reg!(R2)]);
        // pop {r0, r1, r2, r3, r4, r5, r6, r7, pc}
        assert_thumb!(0xbdff, [reglist!(R0, R1, R2, R3, R4, R5, R6, R7, Pc)], []);
        // push {r0, r1, r2, r3, r4, r5, r6, r7, lr}
        assert_thumb!(0xb5ff, [], [reglist!(R0, R1, R2, R3, R4, R5, R6, R7, Lr)]);
        // rev r2, r1
        assert_thumb!(0xba0a, [reg!(R2)], [reg!(R1)]);
        // stm r1!, {r0, r2, r4, r6}
        assert_thumb!(0xc155, [reg!(write R1)], [reg!(write R1), reglist!(R0, R2, R4, R6)]);
        // str r2, [r4, #0x40]
        assert_thumb!(0x6422, [], [reg!(R2), reg!(deref R4)]);
        // str r2, [r4, r0]
        assert_thumb!(0x5022, [], [reg!(R2), reg!(deref R4), offsetreg!(R0)]);
        // str r7, [sp, #0x108]
        assert_thumb!(0x9742, [], [reg!(R7), reg!(deref Sp)]);
        // sxtb r2, r1
        assert_thumb!(0xb24a, [reg!(R2)], [reg!(R1)]);
        // tst r7, r2
        assert_thumb!(0x4217, [], [reg!(R7), reg!(R2)]);
    }
}
