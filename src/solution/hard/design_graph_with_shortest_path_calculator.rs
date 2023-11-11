use std::collections::{HashMap, VecDeque};

pub struct Graph {
    map: HashMap<i32, Vec<(i32, i32)>>,
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Graph {
    pub fn new(_n: i32, edges: Vec<Vec<i32>>) -> Self {
        let mut graph = Self { map: HashMap::new() };

        for i in 0..edges.len() {
            let edge = (edges[i][1], edges[i][2]);
            graph.map.entry(edges[i][0]).and_modify(|v| v.push(edge)).or_insert(vec![edge]);
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

            for i in 0..e.len() {
                let new_dst = if let Some(new_dst) = d.get(&e[i].0).and_then(|cur_dst| Some(if *cur_dst > (e[i].1 + dst) { e[i].1 + dst } else { *cur_dst })) {
                    new_dst
                } else {
                    e[i].1 + dst
                };

                d.insert(e[i].0, new_dst);

                if !(s.contains_key(&e[i].0) && *s.get(&e[i].0).unwrap() <= new_dst) {
                    s.insert(e[i].0, new_dst);
                    q.push_back((e[i].0, new_dst));
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
