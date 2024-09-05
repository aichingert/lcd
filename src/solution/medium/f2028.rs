use crate::Solution;

impl Solution {
    pub fn missing_rolls(rolls: Vec<i32>, mean: i32, n: i32) -> Vec<i32> {
        let s = rolls.iter().sum::<i32>();

        // s => sum of rolls
        // m => len of rolls
        // n => num of todos

        // (s + x) / (m + n) = mean
        // mean * (m + n) - s = x

        let mut x = mean * (rolls.len() as i32 + n) - s;

        if 6 * n < x || n > x {
            return vec![];
        }

        let mut a = Vec::new();

        for i in 0..n {
            if n - i == x {
                a.extend_from_slice(&vec![1; (n - i) as usize]);
                break;
            }

            if x - 6 >= n - i {
                a.push(6);
                x -= 6;
            } else {
                a.push(x - n + i + 1);
                x = n - i - 1;
            }
        }

        a
    }
}
