#[rustfmt::skip]
mod generated;
mod disasm;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        disasm::{Ins, ParsedIns},
        generated::Opcode,
    };

    #[derive(Default, Clone, Copy)]
    struct OpcodeTest {
        op: Opcode,
        count: u32,
    }

    #[test]
    fn all_opcodes() {
        let mut opcode_counts = vec![OpcodeTest::default(); Opcode::count()].into_boxed_slice();
        opcode_counts
            .iter_mut()
            .enumerate()
            .for_each(|(i, op)| op.op = unsafe { std::mem::transmute::<u8, Opcode>(i.try_into().unwrap()) });
        for code in 0..=u32::MAX {
            let ins = Ins::new(code);
            if ins.op == Opcode::Illegal {
                continue;
            }
            opcode_counts[ins.op as usize].count += 1;
        }
        for opcode in opcode_counts.iter() {
            assert!(opcode.count > 0);
        }
    }

    #[test]
    fn all_instructions() {
        let mut mnemonic_counts: HashMap<&str, u32> = HashMap::with_capacity(15000);
        for code in 0..=u32::MAX {
            let ins = Ins::new(code);
            if ins.op == Opcode::Illegal {
                continue;
            }
            let parsed = ParsedIns::parse(ins);
            *mnemonic_counts.entry(parsed.mnemonic).or_insert(0) += 1;
        }
        println!("Mnemonics: {}", mnemonic_counts.len());
    }

    #[test]
    fn playground() {
        let ins = Ins::new(0xfa000000);
        assert_eq!(ins.op, Opcode::BlxI);
    }
}
