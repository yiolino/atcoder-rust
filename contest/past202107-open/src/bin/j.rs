#[allow(unused_imports)]
use proconio::{input, fastout, marker::Chars, marker::Usize1, marker::Bytes};
#[allow(unused_imports)]
use std::collections::{HashSet, HashMap, BTreeSet, VecDeque, BinaryHeap};
#[allow(unused_imports)]
use std::cmp::{max, min, Reverse};
use std::vec;
#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use petgraph::unionfind::UnionFind;
#[allow(unused_imports)]
use superslice::Ext;

fn main() {
    input!{
        n: usize,
        m: usize,
    }

    // 閉路検出の問題
    // トポロジカルソート的発想で解く
    let mut graph = vec![vec![]; n];
    for _ in 0..m {
        input! {u: usize, v: usize};
        graph[u - 1].push(v - 1);  // 0-index
    }

    let mut dfs = DFS::new(graph);
    
    for i in 0..n {
        // iからstartした時にloopがあるか検出
        dfs.dfs(i);

        if dfs.is_loop {
            println!("Yes");
            return;
        }
    }

    println!("No");
}

struct DFS {
    seen: Vec<bool>,
    is_rec_end: Vec<bool>,
    graph: Vec<Vec<usize>>,
    is_loop: bool,
}

impl DFS {
    fn new(graph: Vec<Vec<usize>>) -> Self {
        let n = graph.len();
        DFS {
            seen: vec![false; n],
            is_rec_end: vec![false; n],
            graph: graph,
            is_loop: false,
        }
    }

    fn dfs(&mut self, u: usize) {
        // 訪問済みならrec終了
        if self.seen[u] {
            // 訪問済みなら、再帰を抜けたフラグが立っているはず
            // もしfalseなら、それはループ構造がある
            if !self.is_rec_end[u] {
                self.is_loop = true;
            }
            return;
        }

        self.seen[u] = true;
        for i in 0..self.graph[u].len() {
            let u_child = self.graph[u][i];
            self.dfs(u_child);
        }

        // 再帰を抜けたらフラグ立てる
        self.is_rec_end[u] = true;
    }
}
