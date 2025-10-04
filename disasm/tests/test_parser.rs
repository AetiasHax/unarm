#[cfg(test)]
mod tests {
    use unarm::{
        AddrLdrStr, AddrMiscLoad, Cond, Extensions, Ins, LdrStrOffset, MiscLoadOffset, Op2,
        Op2Shift, Options, ParseEndian, ParseMode, Parser, R9Use, Reg, RegList, ShiftImm, ShiftOp,
        Version,
    };

    #[test]
    fn test_arm() {
        #[rustfmt::skip]
        let code = &[
            0x48, 0x30, 0x90, 0xe5, // ldr r3, [r0, #0x48]
            0x01, 0x2c, 0x80, 0xe2, // add r2, r0, #0x100
            0x00, 0x30, 0x81, 0xe5, // str r3, [r1, #0x0]
            0x4c, 0x30, 0x90, 0xe5, // ldr r3, [r0, #0x4c]
            0x04, 0x30, 0x81, 0xe5, // str r3, [r1, #0x4]
            0x50, 0x00, 0x90, 0xe5, // ldr r0, [r0, #0x50]
            0x08, 0x00, 0x81, 0xe5, // str r0, [r1, #0x8]
            0xfe, 0x01, 0xd2, 0xe1, // ldrsh r0, [r2, #0x1e]
            0x04, 0x20, 0x91, 0xe5, // ldr r2, [r1, #0x4]
            0x00, 0x00, 0x82, 0xe0, // add r0, r2, r0
            0x04, 0x00, 0x81, 0xe5, // str r0, [r1, #0x4]
            0x1e, 0xff, 0x2f, 0xe1, // bx lr
        ];
        let mut parser = Parser::new(
            code,
            ParseMode::Arm,
            ParseEndian::Little,
            Options {
                version: Version::V6K,
                extensions: Extensions::all(),
                av: false,
                r9_use: R9Use::R9,
                sl: false,
                fp: false,
                ip: false,
                ual: true,
            },
        );
        // 0x00
        assert_eq!(
            parser.next(),
            Some(Ins::Ldr {
                cond: Cond::Al,
                rd: Reg::R3,
                addr: AddrLdrStr::Pre {
                    rn: Reg::R0,
                    offset: LdrStrOffset::Imm(0x48),
                    writeback: false
                }
            })
        );
        // 0x04
        assert_eq!(
            parser.next(),
            Some(Ins::Add {
                s: false,
                thumb: false,
                cond: Cond::Al,
                rd: Reg::R2,
                rn: Reg::R0,
                op2: Op2::Imm(0x100)
            })
        );
        // 0x08
        assert_eq!(
            parser.next(),
            Some(Ins::Str {
                cond: Cond::Al,
                rd: Reg::R3,
                addr: AddrLdrStr::Pre {
                    rn: Reg::R1,
                    offset: LdrStrOffset::Imm(0x0),
                    writeback: false
                }
            })
        );
        // 0x0c
        assert_eq!(
            parser.next(),
            Some(Ins::Ldr {
                cond: Cond::Al,
                rd: Reg::R3,
                addr: AddrLdrStr::Pre {
                    rn: Reg::R0,
                    offset: LdrStrOffset::Imm(0x4c),
                    writeback: false
                }
            })
        );
        // 0x10
        assert_eq!(
            parser.next(),
            Some(Ins::Str {
                cond: Cond::Al,
                rd: Reg::R3,
                addr: AddrLdrStr::Pre {
                    rn: Reg::R1,
                    offset: LdrStrOffset::Imm(0x4),
                    writeback: false
                }
            })
        );
        // 0x14
        assert_eq!(
            parser.next(),
            Some(Ins::Ldr {
                cond: Cond::Al,
                rd: Reg::R0,
                addr: AddrLdrStr::Pre {
                    rn: Reg::R0,
                    offset: LdrStrOffset::Imm(0x50),
                    writeback: false
                }
            })
        );
        // 0x18
        assert_eq!(
            parser.next(),
            Some(Ins::Str {
                cond: Cond::Al,
                rd: Reg::R0,
                addr: AddrLdrStr::Pre {
                    rn: Reg::R1,
                    offset: LdrStrOffset::Imm(0x8),
                    writeback: false
                }
            })
        );
        // 0x1c
        assert_eq!(
            parser.next(),
            Some(Ins::Ldrsh {
                cond: Cond::Al,
                rd: Reg::R0,
                addr: AddrMiscLoad::Pre {
                    rn: Reg::R2,
                    offset: MiscLoadOffset::Imm(0x1e),
                    writeback: false
                }
            })
        );
        // 0x20
        assert_eq!(
            parser.next(),
            Some(Ins::Ldr {
                cond: Cond::Al,
                rd: Reg::R2,
                addr: AddrLdrStr::Pre {
                    rn: Reg::R1,
                    offset: LdrStrOffset::Imm(0x4),
                    writeback: false
                }
            })
        );
        // 0x24
        assert_eq!(
            parser.next(),
            Some(Ins::Add {
                s: false,
                thumb: false,
                cond: Cond::Al,
                rd: Reg::R0,
                rn: Reg::R2,
                op2: Op2::ShiftImm(ShiftImm { rm: Reg::R0, shift_op: ShiftOp::Lsl, imm: 0x0 })
            })
        );
        // 0x28
        assert_eq!(
            parser.next(),
            Some(Ins::Str {
                cond: Cond::Al,
                rd: Reg::R0,
                addr: AddrLdrStr::Pre {
                    rn: Reg::R1,
                    offset: LdrStrOffset::Imm(0x4),
                    writeback: false
                }
            })
        );
        // 0x2c
        assert_eq!(parser.next(), Some(Ins::Bx { cond: Cond::Al, rm: Reg::Lr }));
        // 0x30
        assert_eq!(parser.next(), None);
    }

    #[test]
    fn test_thumb() {
        #[rustfmt::skip]
        let code = &[
            0x30, 0xb4, // push {r4, r5}
            0x4a, 0x22, // movs r2, #0x4a
            0x92, 0x00, // lsls, r2, r2, #0x2
            0x82, 0x18, // adds r2, r0, r2
            0x48, 0x09, // lsrs r0, r1, #0x5
            0x80, 0x00, // lsls r0, r0, #0x2
            0x1f, 0x23, // movs r3, #0x1f
            0x01, 0x24, // movs r4, #0x1
            0x19, 0x40, // ands r1, r1, r3
            0x23, 0x1c, // adds r3, r4, #0x0
            0x15, 0x58, // ldr r5, [r2, r0]
            0x8b, 0x40, // lsls r3, r3, r1
            0x29, 0x1c, // adds r1, r5, #0x0
            0x19, 0x43, // orrs r1, r1, r3
            0x11, 0x50, // str r1, [r2, r0]
            0x30, 0xbc, // pop {r4, r5}
            0x70, 0x47, // bx lr
        ];
        let mut parser = Parser::new(
            code,
            ParseMode::Thumb,
            ParseEndian::Little,
            Options {
                version: Version::V6K,
                extensions: Extensions::all(),
                av: false,
                r9_use: R9Use::R9,
                sl: false,
                fp: false,
                ip: false,
                ual: true,
            },
        );
        // 0x00
        assert_eq!(
            parser.next(),
            Some(Ins::Push { cond: Cond::Al, regs: RegList::parse(0b0000_0000_0011_0000) })
        );
        // 0x02
        assert_eq!(
            parser.next(),
            Some(Ins::Mov {
                s: true,
                thumb: true,
                cond: Cond::Al,
                rd: Reg::R2,
                op2: Op2::Imm(0x4a)
            })
        );
        // 0x04
        assert_eq!(
            parser.next(),
            Some(Ins::Lsl {
                s: true,
                thumb: true,
                cond: Cond::Al,
                rd: Reg::R2,
                rn: Reg::R2,
                op2: Op2Shift::Imm(0x2)
            })
        );
        // 0x06
        assert_eq!(
            parser.next(),
            Some(Ins::Add {
                s: true,
                thumb: true,
                cond: Cond::Al,
                rd: Reg::R2,
                rn: Reg::R0,
                op2: Op2::ShiftImm(ShiftImm { rm: Reg::R2, shift_op: ShiftOp::Lsl, imm: 0x0 })
            })
        );
        // 0x08
        assert_eq!(
            parser.next(),
            Some(Ins::Lsr {
                s: true,
                thumb: true,
                cond: Cond::Al,
                rd: Reg::R0,
                rn: Reg::R1,
                op2: Op2Shift::Imm(0x5)
            })
        );
        // 0x0a
        assert_eq!(
            parser.next(),
            Some(Ins::Lsl {
                s: true,
                thumb: true,
                cond: Cond::Al,
                rd: Reg::R0,
                rn: Reg::R0,
                op2: Op2Shift::Imm(0x2)
            })
        );
        // 0x0c
        assert_eq!(
            parser.next(),
            Some(Ins::Mov {
                s: true,
                thumb: true,
                cond: Cond::Al,
                rd: Reg::R3,
                op2: Op2::Imm(0x1f)
            })
        );
        // 0x0e
        assert_eq!(
            parser.next(),
            Some(Ins::Mov {
                s: true,
                thumb: true,
                cond: Cond::Al,
                rd: Reg::R4,
                op2: Op2::Imm(0x1)
            })
        );
        // 0x10
        assert_eq!(
            parser.next(),
            Some(Ins::And {
                s: true,
                thumb: true,
                cond: Cond::Al,
                rd: Reg::R1,
                rn: Reg::R1,
                op2: Op2::ShiftImm(ShiftImm { rm: Reg::R3, shift_op: ShiftOp::Lsl, imm: 0x0 })
            })
        );
        // 0x12
        assert_eq!(
            parser.next(),
            Some(Ins::Add {
                s: true,
                thumb: true,
                cond: Cond::Al,
                rd: Reg::R3,
                rn: Reg::R4,
                op2: Op2::Imm(0x0)
            })
        );
        // 0x14
        assert_eq!(
            parser.next(),
            Some(Ins::Ldr {
                cond: Cond::Al,
                rd: Reg::R5,
                addr: AddrLdrStr::Pre {
                    rn: Reg::R2,
                    offset: LdrStrOffset::Reg {
                        subtract: false,
                        rm: Reg::R0,
                        shift_op: ShiftOp::Lsl,
                        imm: 0x0
                    },
                    writeback: false
                }
            })
        );
        // 0x16
        assert_eq!(
            parser.next(),
            Some(Ins::Lsl {
                s: true,
                thumb: true,
                cond: Cond::Al,
                rd: Reg::R3,
                rn: Reg::R3,
                op2: Op2Shift::Reg(Reg::R1)
            })
        );
        // 0x18
        assert_eq!(
            parser.next(),
            Some(Ins::Add {
                s: true,
                thumb: true,
                cond: Cond::Al,
                rd: Reg::R1,
                rn: Reg::R5,
                op2: Op2::Imm(0x0)
            })
        );
        // 0x1a
        assert_eq!(
            parser.next(),
            Some(Ins::Orr {
                s: true,
                thumb: true,
                cond: Cond::Al,
                rd: Reg::R1,
                rn: Reg::R1,
                op2: Op2::ShiftImm(ShiftImm { rm: Reg::R3, shift_op: ShiftOp::Lsl, imm: 0x0 })
            })
        );
        // 0x1c
        assert_eq!(
            parser.next(),
            Some(Ins::Str {
                cond: Cond::Al,
                rd: Reg::R1,
                addr: AddrLdrStr::Pre {
                    rn: Reg::R2,
                    offset: LdrStrOffset::Reg {
                        subtract: false,
                        rm: Reg::R0,
                        shift_op: ShiftOp::Lsl,
                        imm: 0x0
                    },
                    writeback: false
                }
            })
        );
        // 0x1e
        assert_eq!(
            parser.next(),
            Some(Ins::Pop { cond: Cond::Al, regs: RegList::parse(0b0000_0000_0011_0000) })
        );
        // 0x20
        assert_eq!(parser.next(), Some(Ins::Bx { cond: Cond::Al, rm: Reg::Lr }));
        // 0x22
        assert_eq!(parser.next(), None);
    }

    #[test]
    fn test_data() {
        #[rustfmt::skip]
        let code = &[
            0x01, 0x02, 0x03, 0x04, // .word 0x04030201
            0x05, 0x06,             // .hword 0x0605
            0x07,                   // .byte 0x07
        ];
        let mut parser = Parser::new(
            code,
            ParseMode::Data,
            ParseEndian::Little,
            Options {
                version: Version::V6K,
                extensions: Extensions::all(),
                av: false,
                r9_use: R9Use::R9,
                sl: false,
                fp: false,
                ip: false,
                ual: true,
            },
        );
        // 0x00
        assert_eq!(parser.next(), Some(Ins::Word(0x0403_0201)));
        // 0x04
        assert_eq!(parser.next(), Some(Ins::HalfWord(0x0605)));
        // 0x06
        assert_eq!(parser.next(), Some(Ins::Byte(0x07)));
        // 0x07
        assert_eq!(parser.next(), None);
    }
}
