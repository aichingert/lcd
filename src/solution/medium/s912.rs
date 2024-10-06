use crate::Solution;

impl Solution {
    pub fn sort_array(nums: Vec<i32>) -> Vec<i32> {
        merge_sort(nums)
    }
}

fn merge_sort(mut arr: Vec<i32>) -> Vec<i32> {
    if arr.len() <= 1 {
        return arr;
    }

    let l = merge_sort(arr[..arr.len() / 2].to_vec());
    let r = merge_sort(arr[arr.len() / 2..].to_vec());
    let (mut lp, mut rp) = (0, 0);

    while lp < l.len() || rp < r.len() {
        if lp < l.len() && (rp >= r.len() || l[lp] < r[rp]) {
            arr[lp + rp] = l[lp];
            lp += 1;
        } else {
            arr[lp + rp] = r[rp];
            rp += 1;
        }
    }

    arr
}
