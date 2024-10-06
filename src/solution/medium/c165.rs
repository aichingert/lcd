use crate::Solution;

impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
        let v1 = version1
            .split('.')
            .map(|n| n.parse::<u32>().unwrap())
            .collect::<Vec<_>>();
        let v2 = version2
            .split('.')
            .map(|n| n.parse::<u32>().unwrap())
            .collect::<Vec<_>>();
        let mut i = 0;

        while i < v1.len() || i < v2.len() {
            match v1.get(i).unwrap_or(&0).cmp(v2.get(i).unwrap_or(&0)) {
                std::cmp::Ordering::Greater => return 1,
                std::cmp::Ordering::Equal => (),
                std::cmp::Ordering::Less => return -1,
            }
            i += 1;
        }

        0
    }
}
