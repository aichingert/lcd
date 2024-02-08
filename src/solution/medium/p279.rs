use crate::Solution;

use std::collections::HashMap;

impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        least(&mut HashMap::new(), n)
    }
}

fn least(m: &mut HashMap<i32, i32>, n: i32) -> i32 {
    if let Some(res) = m.get(&n) {
        return *res;
    }

    let mut root = (n as f32).sqrt() as i32;

    let act = root * root;

    if n - act == 0 {
        return 1;
    }

    let mut res = least(m, n - act) + 1;
    m.insert(n, res);
    root -= 1;

    while root > 0 {
        res = res.min(least(m, n - root * root) + 1);
        m.insert(n, res);
        root -= 1;
    }

    res
}
