#[allow(unused_imports)]
use proconio::{input, fastout, marker::Chars, marker::Usize1, marker::Bytes};
#[allow(unused_imports)]
use std::collections::{HashSet, HashMap, BTreeSet, VecDeque, BinaryHeap};
#[allow(unused_imports)]
use std::cmp::{max, min, Reverse};
#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use petgraph::unionfind::UnionFind;
#[allow(unused_imports)]
use superslice::Ext;

fn main() {
    input!{
        n: usize,
    }

    let mut time_vec = vec![];
    let mut graph = vec![vec![]; n];

    for i in 0..n {
        input!{
            t: i64,
            k: usize,
            a: [usize; k],
        }

        time_vec.push(t);
        for ai in a {
            let j = ai.wrapping_sub(1);
            graph[i].push(j);
        }
    }

    let mut dfs = DFS::new(time_vec, graph);

    dfs.dfs(n.wrapping_sub(1));

    println!("{}", dfs.time);

}


// 深さ優先探索で解く
struct DFS {
    time_vec: Vec<i64>,
    seen: Vec<i64>,
    graph: Vec<Vec<usize>>,
    time: i64,
}

impl DFS {
    fn new(time_vec: Vec<i64>, graph: Vec<Vec<usize>>) -> Self {
        let n = graph.len();

        DFS {
            time_vec: time_vec,
            seen: vec![-1; n],
            graph: graph,
            time: 0,
        }
    }

    fn dfs(&mut self, idx: usize) {
        if self.seen[idx] != -1 {
            return;
        }

        self.seen[idx] = 1;
        self.time += self.time_vec[idx];

        for i in 0..self.graph[idx].len() {
            self.dfs(self.graph[idx][i]);
        }
    }
}