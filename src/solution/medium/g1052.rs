use crate::Solution;

impl Solution {
    pub fn max_satisfied(customers: Vec<i32>, grumpy: Vec<i32>, minutes: i32) -> i32 {
        let (mut obv, mut ans, mut sum) = (0, 0, 0);

        for i in 0..customers.len() {
            if grumpy[i] == 0 {
                obv += customers[i];
            } else {
                sum += customers[i];
            }

            if i >= minutes as usize && grumpy[i - minutes as usize] == 1 {
                sum -= customers[i - minutes as usize];
            }

            ans = ans.max(sum);
        }

        obv + ans
    }
}
