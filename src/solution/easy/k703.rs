use std::collections::VecDeque;

pub struct KthLargest {
    nth: VecDeque<i32>,
    len: usize,
}

impl KthLargest {
    pub fn new(k: i32, mut nums: Vec<i32>) -> Self {
        nums.sort_unstable();
        let skip = (nums.len() as i32 - k).max(0) as usize;

        Self {
            nth: VecDeque::from_iter(nums.into_iter().skip(skip)),
            len: k as usize,
        }
    }

    pub fn add(&mut self, val: i32) -> i32 {
        if self.nth.len() == self.len && val <= self.nth[0] {
            return self.nth[0];
        }

        if self.nth.len() == self.len {
            self.nth.pop_front();
        }

        match self.nth.binary_search(&val) {
            Ok(i) => self.nth.insert(i, val),
            Err(i) => self.nth.insert(i, val),
        }

        self.nth[0]
    }
}
