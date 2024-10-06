use crate::Solution;

impl Solution {
    pub fn sort_jumbled(mapping: Vec<i32>, nums: Vec<i32>) -> Vec<i32> {
        let mut ans = Vec::new();

        for &p in nums.iter() {
            let mut n = p;
            let mut len = 1;
            let mut res = 0;

            if n == 0 {
                ans.push((mapping[0], p));
                continue;
            }

            while n > 0 {
                let cur = mapping[n as usize % 10] * len;
                res += cur;

                n /= 10;
                len *= 10;
            }

            ans.push((res, p));
        }
        ans.sort_unstable_by_key(|a| a.0);
        ans.into_iter().map(|(_, n)| n).collect()
    }
}
