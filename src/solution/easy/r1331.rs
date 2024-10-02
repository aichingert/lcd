use crate::Solution;

impl Solution {
    pub fn array_rank_transform(arr: Vec<i32>) -> Vec<i32> {
        let mut tfm = arr.into_iter().enumerate().collect::<Vec<_>>();
        tfm.sort_unstable_by_key(|(_, v)| *v);
        let mut ans = vec![0; tfm.len()];

        let mut cnt = 1;
        let mut prv = None::<i32>;

        for (i, v) in tfm {
            if prv.is_some() && prv.unwrap() != v {
                cnt += 1;
            }

            ans[i] = cnt;
            prv = Some(v);
        }
        
        ans
    }
}
