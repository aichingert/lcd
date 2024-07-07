use crate::Solution;

// TODO: there is a math O(1) solution
impl Solution {
    pub fn num_water_bottles(mut num_bottles: i32, num_exchange: i32) -> i32 {
        let mut ans = num_bottles;

        while num_bottles / num_exchange > 0 {
            let diff = num_bottles / num_exchange;
            ans += diff;

            num_bottles = diff + num_bottles % num_exchange;
        }

        ans
    }
}
