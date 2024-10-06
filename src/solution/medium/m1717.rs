use crate::Solution;

impl Solution {
    pub fn maximum_gain(s: String, x: i32, y: i32) -> i32 {
        let mut chs = s.chars().collect::<Vec<_>>();
        let mut ans = 0;
        let lookup = [('a', 'b', x), ('b', 'a', y)];
        let best = if x > y { 0 } else { 1 };

        let mut st = Vec::new();

        for &ch in chs.iter() {
            if ch == lookup[best].1 && !st.is_empty() && st[st.len() - 1] == lookup[best].0 {
                ans += lookup[best].2;
                st.pop();
                continue;
            }

            st.push(ch);
        }

        chs.clear();

        for ch in st {
            if ch == lookup[1 - best].1
                && !chs.is_empty()
                && chs[chs.len() - 1] == lookup[1 - best].0
            {
                ans += lookup[1 - best].2;
                chs.pop();
                continue;
            }

            chs.push(ch);
        }

        ans
    }
}
