use crate::Solution;

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        // NOT THE BEST: there is a solution using O(1) space without considering the output array
        // [ 0 | 0*1 | 0*1*2 | 0*1*2*3 ]
        // [ 3 | 3*2 | 3*2*1 | 3*2*1*0 ]

        // f[f-1] f[f-2] * b[b-3] [f-3] * b[b-2] b[-1]

        let len = nums.len();
        let mut f = vec![nums[0]; len];
        let mut b = vec![nums[len - 1]; len];

        for i in 1..len {
            f[i] = f[i - 1] * nums[i];
            b[i] = b[i - 1] * nums[len - i - 1];
        }

        let mut res = vec![b[len - 2]];

        for i in 0..len - 2 {
            res.push(f[i] * b[len - i - 3])
        }
        
        res.push(f[len - 2]);
        res
    }
}
