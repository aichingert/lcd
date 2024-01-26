use std::collections::{HashMap, VecDeque};

pub struct Graph {
    map: HashMap<i32, Vec<(i32, i32)>>,
}

impl Graph {
    pub fn new(_n: i32, edges: Vec<Vec<i32>>) -> Self {
        let mut graph = Self { map: HashMap::new() };

        for item in edges.iter() {
            let edge = (item[1], item[2]);
            graph.map.entry(item[0]).and_modify(|v| v.push(edge)).or_insert(vec![edge]);
        }

        graph
    }
    
    pub fn add_edge(&mut self, edge: Vec<i32>) {
        let ins = (edge[1], edge[2]);
        self.map.entry(edge[0]).and_modify(|v| v.push(ins)).or_insert(vec![ins]);
    }
    
    pub fn shortest_path(&self, node1: i32, node2: i32) -> i32 {
        if node1 == node2 {
            return 0;
        }

        let mut q = VecDeque::from([(node1, 0)]);
        let mut s: HashMap<i32, i32> = HashMap::new();
        let mut d: HashMap<i32, i32> = HashMap::new();

        while let Some((p, dst)) = q.pop_front() {
            let e = if let Some(e) = self.map.get(&p) {
                e
            } else{ continue; };

            for e in e.iter() {
                let new_dst = if let Some(new_dst) = d.get(&e.0).map(|cur_dst| if *cur_dst > (e.1 + dst) { e.1 + dst } else { *cur_dst }) {
                    new_dst
                } else {
                    e.1 + dst
                };

                d.insert(e.0, new_dst);

                if !(s.contains_key(&e.0) && *s.get(&e.0).unwrap() <= new_dst) {
                    s.insert(e.0, new_dst);
                    q.push_back((e.0, new_dst));
                }
            }
        }

        if let Some(dst) = d.get(&node2) {
            *dst
        } else{
            -1
        }
    }
}
