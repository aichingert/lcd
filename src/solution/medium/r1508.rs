use crate::Solution;

const MOD: i32 = 1000000007;

impl Solution {
    pub fn range_sum(nums: Vec<i32>, n: i32, left: i32, right: i32) -> i32 {
        let n = n as usize;
        let mut arr = Vec::with_capacity(n * (n + 1) / 2);

        for (i, &e) in nums.iter().enumerate() {
            let mut sum = e;
            arr.push(sum);

            for &x in nums.iter().skip(i + 1) {
                sum += x;
                arr.push(sum);
            }
        }

        arr.sort_unstable();
        arr[left as usize - 1..right as usize].iter().fold(0, |xs, x| (xs + x) % MOD)
    }
}
