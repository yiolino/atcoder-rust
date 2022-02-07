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

const INF: usize = std::usize::MAX;

fn main() {
    input!{
        n: usize,
        m: usize,   
    }

    // 0 idx に注意
    let mut graph = vec![vec![]; n];
    for _ in 0..m {
        input!{a: usize, b: usize};
        graph[a - 1].push(b - 1);
        graph[b - 1].push(a - 1);
    }

    let mut bfs = BFS::new(n);
    bfs.bfs(0, &graph);

    if bfs.dist[n - 1] == 2 {
        println!("POSSIBLE")
    } else {
        println!("IMPOSSIBLE")
    }
}

struct BFS {
    dist: Vec<usize>,
    todo: VecDeque<usize>,
}

impl BFS {
    fn new(n: usize) -> Self {
        BFS {
            dist: vec![INF; n],
            todo: VecDeque::new()
        }
    }

    fn bfs(&mut self, s: usize, graph: &Vec<Vec<usize>>) {
        self.todo.push_back(s);
        self.dist[s] = 0;

        while !self.todo.is_empty() {
            let v = self.todo.pop_front().unwrap();
            for i in 0..graph[v].len() {
                let next_v = graph[v][i];
                if self.dist[next_v] == INF {
                    self.todo.push_back(next_v);
                    self.dist[next_v] = self.dist[v] + 1;
                }
            }
        }
    }
}