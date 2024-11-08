use crate::Solution;

impl Solution {
    pub fn get_maximum_xor(nums: Vec<i32>, maximum_bit: i32) -> Vec<i32> {
        let mut p = Vec::new();
        let mut a = Vec::new();
        let mut x = 0;

        for num in nums {
            x ^= num;
            p.push(x);
        }

        while let Some(r) = p.pop() {
            let mut k = 0;

            for s in 0..maximum_bit {
                if r | 1 << s > r {
                    k |= 1 << s;
                }
            }

            a.push(k);
        }

        a
    }
}
