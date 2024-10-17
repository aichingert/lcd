use crate::Solution;

impl Solution {
    pub fn maximum_swap(num: i32) -> i32 {
        let mut c = num.to_string().bytes().collect::<Vec<_>>();

        for i in 0..c.len() - 1 {
            let (mut m, mut v) = (c[i], i);

            for (j, &b) in c.iter().enumerate().skip(i + 1) {
                if b > m || i != v && b >= m {
                    m = b;
                    v = j;
                }
            }

            if v != i {
                c[v] = c[i];
                c[i] = m;
                return c
                    .into_iter()
                    .map(|b| b as char)
                    .collect::<String>()
                    .parse::<i32>()
                    .unwrap();
            }
        }

        num
    }
}
