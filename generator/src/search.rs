use crate::isa::Opcode;

pub struct SearchTree {
    bitmask: u32,
    left_pattern: u32,
    left: Option<Box<SearchTree>>,
    right: Option<Box<SearchTree>>,
}

impl SearchTree {
    fn narrow_down(opcodes: &[Opcode]) -> Self {
        // Goal: Get this number as close to opcodes.len() / 2 as possible
        let mut best_narrow_count = 0;

        let mut best_bitmask = 0;
        let mut best_left_pattern = 0;

        for pos in 0..32 {
            let bitmask = 1 << pos;

            let (zeros, ones) = opcodes
                .iter()
                .filter(|op| (op.bitmask & bitmask) == bitmask)
                .fold((0, 0), |acc, op| {
                    if op.pattern & bitmask == 0 {
                        (acc.0 + 1, acc.1)
                    } else {
                        (acc.0, acc.1 + 1)
                    }
                });

            if zeros > best_narrow_count && zeros <= opcodes.len() / 2 {
                best_narrow_count = zeros;
                best_bitmask = bitmask;
                best_left_pattern = 0;
            }
            if ones > best_narrow_count && ones <= opcodes.len() / 2 {
                best_narrow_count = ones;
                best_bitmask = bitmask;
                best_left_pattern = bitmask;
            }
        }

        Self {
            bitmask: best_bitmask,
            left_pattern: best_left_pattern,
            left: None,
            right: None,
        }
    }

    pub fn filter(&self, opcodes: Vec<Opcode>) -> (Vec<Opcode>, Vec<Opcode>) {
        let mut left = vec![];
        let mut right = vec![];
        for opcode in opcodes {
            if (opcode.bitmask & self.bitmask) != 0 && (opcode.pattern & self.bitmask) == self.left_pattern {
                left.push(opcode);
            } else {
                right.push(opcode);
            }
        }
        (left, right)
    }

    pub fn optimize(opcodes: Vec<Opcode>) -> Self {
        let mut node = Self::narrow_down(&opcodes);
        let (left, right) = node.filter(opcodes);
        if left.len() > 1 {
            node.left = Some(Box::new(Self::optimize(left)));
        }
        if right.len() > 1 {
            node.right = Some(Box::new(Self::optimize(right)));
        }
        node
    }
}
