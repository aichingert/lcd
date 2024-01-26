use std::collections::HashSet;
use rand::prelude::IteratorRandom;

pub struct RandomizedSet {
    set: HashSet<i32>,
}

impl RandomizedSet {
    pub fn new() -> Self {
        RandomizedSet{set: HashSet::new()}
    }
    
    pub fn insert(&mut self, val: i32) -> bool {
        self.set.insert(val)
    }
    
    pub fn remove(&mut self, val: i32) -> bool {
        self.set.remove(&val)
    }
    
    pub fn get_random(&self) -> i32 {
        let mut rng = rand::thread_rng();
        *self.set.iter().choose(&mut rng).unwrap()
    }
}
