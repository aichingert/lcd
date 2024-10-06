use crate::Solution;

impl Solution {
    pub fn build_array(target: Vec<i32>, n: i32) -> Vec<String> {
        let mut ops = Vec::<String>::new();
        let mut ptr = 0;

        for i in 1..=n {
            if ptr >= target.len() {
                break;
            }

            ops.push(String::from("Push"));
            if target[ptr] > i {
                ops.push(String::from("Pop"));
            } else {
                ptr += 1;
            }
        }

        ops
    }
}
