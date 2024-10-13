use crate::Solution;

use std::collections::HashMap;

impl Solution {
    pub fn smallest_range(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let mut m = Vec::new();

        for (i, arr) in nums.iter().enumerate() {
            for num in arr {
                m.push((*num, i));
            }
        }

        m.sort_unstable();
        let mut lhs = 0;
        let mut frq = HashMap::new();
        let mut rng = vec![0, i32::MAX];
        let mut cnt = 0;

        for (rmax, arr) in m.iter() {
            frq.entry(arr).and_modify(|c| *c += 1).or_insert(1);
            if frq[&arr] == 1 {
                cnt += 1;
            }

            while cnt == nums.len() {
                let rmin = m[lhs].0;

                if *rmax - rmin < rng[1] - rng[0] {
                    rng[1] = *rmax;
                    rng[0] = rmin;
                }

                *frq.get_mut(&m[lhs].1).unwrap() -= 1;
                if frq[&m[lhs].1] == 0 {
                    cnt -= 1;
                }
                lhs += 1;
            }
        }

        rng
    }
}
