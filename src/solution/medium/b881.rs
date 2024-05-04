use crate::Solution;

impl Solution {
    pub fn num_rescue_boats(mut people: Vec<i32>, limit: i32) -> i32 {
        people.sort_unstable();
        let mut ans = 0;
        let (mut i, mut j) = (0, people.len() as i32 - 1);

        while i <= j {
            if people[i as usize] + people[j as usize] <= limit {
                i += 1;
                j -= 1;
            } else {
                j -= 1;
            }

            ans += 1;
        }

        ans
    }
}
