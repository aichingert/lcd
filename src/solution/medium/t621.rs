use crate::Solution;
use std::collections::HashMap;

// TODO: there is a better solution :/
impl Solution {
    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        let mut map = HashMap::new();
        for task in tasks {
            map.entry(task).and_modify(|n| *n += 1).or_insert(1);
        }

        let mut vec = map.into_values().collect::<Vec<_>>();
        vec.sort_unstable_by(|a,b| b.cmp(&a));
        let mut int = 0;

        while vec[0] != 0 {
            let mut cur = 0;

            for v in vec.iter_mut() {
                if *v == 0 || cur > n {
                    break;
                }

                *v -= 1;
                cur += 1;
            }

            vec.sort_unstable_by(|a,b| b.cmp(&a));

            if cur > n || vec[0] == 0 {
                int += cur;
            } else {
                int += n + 1;
            }
        }

        int
    }
}
