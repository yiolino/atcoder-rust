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

// オイラーツアー
// 行きがけ順と帰りがけ順を考える。
// https://drken1215.hatenablog.com/entry/2021/08/11/025100

const INF: usize = std::usize::MAX;

fn main() {
    input!{
        n: usize,
    }

    let mut graph = vec![vec![]; n];

    for _ in 0..n-1 {
        input! {mut a: usize, mut b: usize};
        a -= 1;
        b -= 1; 

        // 無向グラフ
        graph[a].push(b);
        graph[b].push(a);
    }

    // sort しておく
    for g in graph.iter_mut() {
        g.sort();
    }

    let mut dfs = DFS::new(graph);

    dfs.dfs(0, INF);

    let ans = dfs.path.iter().join(" ");

    println!("{}", ans);
}

struct DFS {
    graph: Vec<Vec<usize>>,
    path: Vec<usize>,
}

impl DFS {
    fn new(g: Vec<Vec<usize>>) -> Self {
        DFS { 
            graph: g, 
            path: Vec::new(),
        }
    }

    fn dfs(&mut self, v: usize, p:usize) {
        // 行きがけのpathを記録
        self.path.push(v+1);

        // 未訪問を探索していく
        for i in  0..self.graph[v].len() {
            let next_v = self.graph[v][i];
            
            // 親には帰らない
            if next_v == p {
                continue;
            }

            self.dfs(next_v, v);

            // 帰りがけのpathを記録
            self.path.push(v+1);
        }

        
    }


}
