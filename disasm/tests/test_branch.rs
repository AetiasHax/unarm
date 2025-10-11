#[cfg(test)]
mod tests {
    use unarm::{
        Options, parse_arm, parse_arm_with_discriminant, parse_thumb, parse_thumb_with_discriminant,
    };

    macro_rules! assert_ins {
        ($ins:ident, $disasm:literal, $options:ident) => {{
            let s = $ins.display(&$options).to_string();
            assert_eq!(s, $disasm)
        }};
        ($ins:ident, $disasm:literal, $options:ident, $msg:literal) => {{
            let s = $ins.display(&$options).to_string();
            assert_eq!(s, $disasm, $msg)
        }};
    }

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
        ($code:literal, $pc:literal, $disasm:literal) => {{
            let options = options!();
            let ins = parse_arm($code, $pc, &options);
            assert_ins!(ins, $disasm, options);
            let discriminant = ins.discriminant();
            let ins = parse_arm_with_discriminant($code, discriminant, $pc, &options);
            assert_ins!(ins, $disasm, options, "mismatched parse with discriminant")
        }};
    }

    macro_rules! assert_thumb {
        ($code:literal, $pc:literal, $disasm:literal) => {{
            let options = options!();
            let (ins, _size) = parse_thumb($code, $pc, &options);
            assert_ins!(ins, $disasm, options);
            let discriminant = ins.discriminant();
            let ins = parse_thumb_with_discriminant($code, discriminant, $pc, &options);
            assert_ins!(ins, $disasm, options, "mismatched parse with discriminant")
        }};
    }

    #[test]
    fn test_branch() {
        assert_arm!(0xea000000, 0x2000000, "b #0x2000008");
        assert_arm!(0x0a012345, 0x2000000, "beq #0x2048d1c");
        assert_arm!(0x1affffff, 0xfffffffc, "bne #0x0");
        assert_arm!(0x2afffffe, 0x1234, "bhs #0x1234");
        assert_arm!(0x3afffffd, 0x0, "blo #0xfffffffc");

        assert_thumb!(0xd042, 0x2000000, "beq #0x2000088");
        assert_thumb!(0xd942, 0x3000000, "bls #0x3000088");
        assert_thumb!(0xdc42, 0x4000000, "bgt #0x4000088");
        assert_thumb!(0xdbf3, 0x16, "blt #0x0");
        assert_thumb!(0xe5ee, 0x0, "b #0xfffffbe0");
    }
}
