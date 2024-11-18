use crate::Solution;

impl Solution {
    pub fn decrypt(code: Vec<i32>, k: i32) -> Vec<i32> {
        let mut a = Vec::new();

        for i in 0..code.len() {
            let mut s = 0;

            match k.cmp(&0) {
                std::cmp::Ordering::Less => {
                    for o in 1..-k + 1 {
                        s += code[(i as i32 - o).rem_euclid(code.len() as i32) as usize];
                    }
                }
                std::cmp::Ordering::Greater => {
                    for o in 1..k as usize + 1 {
                        s += code[(i + o) % code.len()];
                    }
                }
                _ => (),
            }

            a.push(s);
        }

        a
    }
}
