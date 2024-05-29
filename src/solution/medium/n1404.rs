use crate::Solution;

impl Solution {
    pub fn num_steps(s: String) -> i32 {
        let cs = s.chars().collect::<Vec<_>>();
        let (mut ans, mut cry) = (0, false);

        for i in (1..cs.len()).rev() {
            if (cs[i] == '1' && !cry) || (cs[i] == '0' && cry) {
                ans += 2;
                cry = true;
            } else {
                ans += 1;
            }
        }

        ans + if cry { 1 } else { 0 }
    }
}
