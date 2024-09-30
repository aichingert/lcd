pub struct CustomStack {
    cap: usize,
    stack: Vec<i32>,
}

impl CustomStack {
    pub fn new(max_size: i32) -> Self {
        Self {
            cap: max_size as usize,
            stack: Vec::with_capacity(max_size as usize),
        }
    }
    
    pub fn push(&mut self, x: i32) {
        if self.cap <= self.stack.len() {
            return;
        }

        self.stack.push(x);
    }
    
    pub fn pop(&mut self) -> i32 {
        let Some(ret) = self.stack.pop() else { return -1 };
        ret
    }
    
    pub fn increment(&mut self, k: i32, val: i32) {
        self.stack.iter_mut().take(k as usize).for_each(|n| *n += val);
    }
}
