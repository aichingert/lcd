use crate::Solution;

impl Solution {
    pub fn find_different_binary_string(nums: Vec<String>) -> String {
        let chs = nums.iter().map(|s| s.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
        let mut i = 0usize;

        chs.iter().map(|ch| { i+=1; if ch[i - 1] == '0' { '1' } else { '0' }}).collect::<String>()
    }
}
