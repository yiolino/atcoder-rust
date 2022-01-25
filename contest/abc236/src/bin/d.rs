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

static INF:usize = std::usize::MAX;

fn main() {
    input!{
        n: usize,
    }

    let mut a = vec![vec![0; 2 * n]; 2 * n];
    for i in 0..(2 * n) {
        for j in i+1..(2 * n) {
            input!{aij: usize};
            a[i][j] = aij;
            // a[j][i] = aij;
        }
    }

    // 再帰（dfs）で解いていく
    let mut dfs = DFS::new(2 * n, a);

    dfs.dfs(0);


    println!("{}", dfs.max);
}


struct DFS {
    s: Vec<bool>,  // pairを組んだかどうか
    a: Vec<Vec<usize>>,
    max: usize,
}

impl DFS {
    fn new(n:usize, a: Vec<Vec<usize>>) -> Self {
        DFS {
            s: vec![false; n],
            a: a,
            max: 0,
        }
    }

    fn dfs(&mut self, aisho: usize) {
        // sの中でまだpairを組んでいない最も添字の若い人を取ってくる。
        let mut si = INF;
        for i in 0..self.s.len() {
            if !self.s[i] {
                si = i;
                break;
            }
        }
        // 再帰が最も深いのは全員がpairを組んでいた時
        if si == INF {
            self.max = self.max.max(aisho);
            return;
        }

        // si が残りのpairを組んでいない誰かとpairを組む場合について相性を計算
        self.s[si] = true;
        for i in 0..self.s.len() {
            if !self.s[i] && si != i {
                self.s[i] = true;

                self.dfs(aisho ^ self.a[si][i]);

                self.s[i] = false;
            }
        }
    }
}