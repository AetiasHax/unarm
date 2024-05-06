#[rustfmt::skip]
mod generated;
mod disasm;

#[cfg(test)]
mod tests {
    use crate::{disasm::Ins, generated::Opcode};

    #[derive(Default, Clone, Copy)]
    struct OpcodeTest {
        op: Opcode,
        count: u32,
    }

    #[test]
    fn all_instructions() {
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
    fn playground() {
        let ins = Ins::new(0xfa000000);
        assert_eq!(ins.op, Opcode::BlxI);
    }
}
