use crate::Solution;

impl Solution {
    pub fn relative_sort_array(mut arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
        arr1.sort_unstable();
        let mut ans = Vec::new();

        for a in arr2 {
            let len = arr1.len();
            arr1.retain(|&x| x != a);

            for _ in 0..(len - arr1.len()) {
                ans.push(a);
            }
        }

        ans.extend_from_slice(&arr1);

        ans
    }
}
