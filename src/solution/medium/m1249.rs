use crate::Solution;

impl Solution {
    pub fn min_remove_to_make_valid(s: String) -> String {
        let mut ans = Vec::new();
        let mut cnt = 0;

        for c in s.chars() {
            match c {
                '(' => {
                    cnt += 1;
                    ans.push(c);
                }
                ')' => {
                    if cnt != 0 {
                        ans.push(c);
                        cnt -= 1;
                    }
                }
                _ => ans.push(c),
            }
        }

        let mut i = ans.len() as i32 - 1;

        while cnt != 0 && i >= 0 {
            if ans[i as usize] == '(' {
                cnt -= 1;
                ans.remove(i as usize);
            }
            i -= 1;
        }

        ans.into_iter().collect::<String>()
    }
}
