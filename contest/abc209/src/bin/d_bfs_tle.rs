#[allow(unused_imports)]
use proconio::{input, fastout, marker::Chars};
#[allow(unused_imports)]
use std::collections::{HashSet, HashMap, BTreeSet, VecDeque};
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use itertools::Itertools;


fn main() {
    input!{
        n: usize,
        q: usize,
    }

    let mut graph = vec![vec![]; n];
    for _ in 0..n-1 {
        input! {mut a: usize, mut b: usize};
        a -= 1;
        b -= 1;
        graph[a].push(b);
        graph[b].push(a);
    }

    for _ in 0..q {
        input! {mut c: usize, mut d: usize};
        c -= 1;
        d -= 1;

        let mut bfs = BFS::new(n);
        bfs.bfs(c, &graph);

        let dist = bfs.dist[d];

        
        let ans = if dist % 2 == 0 {
            "Town"
        } else {
            "Road"
        };
        println!("{}", ans)

    }
}


// bfs による最短路
struct BFS {
    dist: Vec<usize>,
}

impl BFS {
    fn new(n: usize) -> Self {
        BFS {
            dist: vec![std::usize::MAX; n],
            
        }
    }


    fn bfs(&mut self, s: usize, graph: &Vec<Vec<usize>>) {
        self.dist[s] = 0;
        let mut todo = VecDeque::new();
        todo.push_back(s);

        while !todo.is_empty() {
            let next_v = todo.pop_front().unwrap();

            for v in &graph[next_v] {
                if self.dist[*v] != std::usize::MAX {continue}

                self.dist[*v] = self.dist[next_v] + 1;
                todo.push_back(*v);
            }
        }
    }
}