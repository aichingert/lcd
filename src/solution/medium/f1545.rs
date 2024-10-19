use crate::Solution;

impl Solution {
    pub fn find_kth_bit(n: i32, k: i32) -> char {
        let mut c = vec![0u8];

        for _ in 1..n {
            let e = c.iter().rev().map(|&n| 1 - n).collect::<Vec<_>>();
            c.push(1);
            c.extend_from_slice(&e);
        }

        (c[k as usize - 1] + b'0') as char
    }
}
