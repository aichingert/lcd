use crate::Solution;

impl Solution {
    pub fn maximum_beauty(mut items: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        let mut a = Vec::new();
        let mut m = Vec::<(i32, i32)>::new();
        let mut c = 0;
        items.sort_unstable();

        for i in items {
            c = c.max(i[1]);
            let l = m.len();

            if l > 0 && m[l - 1].0 == i[0] {
                m[l - 1].1 = m[l - 1].1.max(i[1]);
            } else {
                m.push((i[0], c));
            }
        }

        for q in queries {
            let res = m.binary_search_by(|p| p.0.cmp(&q));

            match res {
                Ok(p) => a.push(m[p].1),
                Err(p) => {
                    if p != 0 {
                        a.push(m[p - 1].1);
                    } else {
                        a.push(0);
                    }
                }
            }
        }

        a
    }
}
