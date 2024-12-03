use crate::Solution;

impl Solution {
    pub fn add_spaces(s: String, spaces: Vec<i32>) -> String {
        let mut ans = String::new();
        let mut spc = 0;

        for (i, ch) in s.chars().enumerate() {
            if spc < spaces.len() && spaces[spc] as usize == i {
                ans.push(' ');
                spc += 1;
            }

            ans.push(ch);
        }

        ans
    }
}
