use crate::Solution;

impl Solution {
    pub fn count_triplets(arr: Vec<i32>) -> i32 {
        let mut ans = 0;

        for i in 0..arr.len() - 1 {
            let mut xor = arr[i];

            for (k, x) in arr.iter().enumerate().skip(i + 1) {
                xor ^= x;

                if xor == 0 {
                    ans += k - i;
                }
            }
        }

        ans as i32
    }
}
