use crate::isa::Opcode;

pub struct SearchTree {
    pub bitmask: u32,
    pub left_pattern: u32,
    pub left: Option<Box<SearchTree>>,
    pub right: Option<Box<SearchTree>>,
}

impl SearchTree {
    fn narrow_down(opcodes: &[Opcode]) -> Option<Self> {
        // Goal: Get this number as close to opcodes.len() / 2 as possible
        let mut best_narrow_count = opcodes.len();
        let mut best_bitmask = 0;
        let mut best_left_pattern = 0;

        for pos in 0..32 {
            let bitmask = 1 << pos;

            let unks = opcodes.iter().filter(|op| (op.bitmask & bitmask) != bitmask).count();
            let (zeros, ones) = opcodes
                .iter()
                .filter(|op| (op.bitmask & bitmask) == bitmask)
                .fold((unks, unks), |acc, op| {
                    if op.pattern & bitmask == 0 {
                        (acc.0 + 1, acc.1)
                    } else {
                        (acc.0, acc.1 + 1)
                    }
                });

            if zeros > unks && zeros < best_narrow_count && zeros >= opcodes.len() / 2 {
                best_narrow_count = zeros;
                best_bitmask = bitmask;
                best_left_pattern = 0;
            }
            if ones > unks && ones < best_narrow_count && ones >= opcodes.len() / 2 {
                best_narrow_count = ones;
                best_bitmask = bitmask;
                best_left_pattern = bitmask;
            }
        }

        if best_narrow_count == opcodes.len() {
            // Impossible to narrow down
            None
        } else {
            Some(Self {
                bitmask: best_bitmask,
                left_pattern: best_left_pattern,
                left: None,
                right: None,
            })
        }
    }

    pub fn filter(&self, opcodes: &Vec<Opcode>) -> (Vec<Opcode>, Vec<Opcode>) {
        let mut left = vec![];
        let mut right = vec![];
        for opcode in opcodes {
            if (opcode.bitmask & self.bitmask) == 0 {
                left.push(opcode.clone());
                right.push(opcode.clone());
            } else if (opcode.pattern & self.bitmask) == self.left_pattern {
                left.push(opcode.clone());
            } else {
                right.push(opcode.clone());
            }
        }
        (left, right)
    }

    pub fn optimize(opcodes: &Vec<Opcode>, max_depth: u32) -> Option<Self> {
        if max_depth == 0 {
            return None;
        }

        let node = Self::narrow_down(opcodes);
        // println!("{{");
        if let Some(mut node) = node {
            // println!("\"size\":{}", opcodes.len());
            let (left, right) = node.filter(opcodes);
            if left.len() > 1 {
                // println!(",\"left\":");
                if let Some(left) = Self::optimize(&left, max_depth - 1) {
                    node.left = Some(Box::new(left));
                } else {
                    // println!("\"opcode {}\"", left[0].name);
                }
            }
            if right.len() > 1 {
                // println!(",\"right\":");
                if let Some(right) = Self::optimize(&right, max_depth - 1) {
                    node.right = Some(Box::new(right));
                } else {
                    // println!("\"opcode {}\"", right[0].name);
                }
            }
            // println!("}}");
            Some(node)
        } else {
            // println!("}}");
            None
        }
    }
}
