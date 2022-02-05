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
        m: usize,
        x: usize,
        c_a: [[usize; m + 1]; n],
    }

    // dfsの全探索で解いてみる
    let dfs = DFS::new(n, m, x, c_a);

    let res = dfs.dfs(0, vec![0; m + 1]);

    if res < std::usize::MAX {
        println!("{}", res);    
    } else {
        println!("-1");
    }
}


struct DFS {
    n: usize,
    m: usize,
    x: usize,
    c_a: Vec<Vec<usize>>,
}

impl DFS {
    fn new(n: usize, m: usize, x: usize, c_a: Vec<Vec<usize>>) -> Self {
        DFS {
            n: n,
            m: m,
            x: x,
            c_a: c_a,
        }
    }

    fn dfs(&self, idx: usize, asum: Vec<usize>) -> usize {
        let mut res = std::usize::MAX;
        if idx == self.n {
            if (1..self.m+1).all(|i| asum[i] >= self.x) {
                res = res.min(asum[0]);
            }
        } else {
            // 次を選ばない場合
            res = min(res, self.dfs(idx+1, asum.clone()));

            // 次を選ぶ場合
            let asum2 = asum.iter().zip(self.c_a[idx].iter())
                                    .map(|(x, y)| x + y)
                                    .collect::<Vec<usize>>();
            res = min(res, self.dfs(idx + 1, asum2))

        }

        res
    }
}