use crate::Solution;

impl Solution {
    pub fn xor_queries(arr: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut a = Vec::new();
        let mut ps = arr.clone();

        for i in 1..arr.len() {
            ps[i] ^= ps[i - 1];
        }

        for query in queries {
            let mut o = ps[query[1] as usize];

            if query[0] > 0 {
                o ^= ps[query[0] as usize - 1];
            }

            a.push(o);
        }

        a
    }
}
