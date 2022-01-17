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
        h: usize,
        w: usize,
        c: [Chars; h],
    }

    // BFSで解く

    let mut dist = vec![vec![-1; w]; h];
    dist[0][0] = 1;

    let mut que = VecDeque::new();
    que.push_back((0,0));

    while !que.is_empty() {
        let v = que.pop_front().unwrap();
        
        // 1つ下
        if v.0 + 1 < h && dist[v.0 + 1][v.1] == -1 && c[v.0 + 1][v.1] != '#' {
            que.push_back((v.0 + 1, v.1));
            dist[v.0 + 1][v.1] = dist[v.0][v.1] + 1;
        }

        // 1つ右
        if v.1 + 1 < w && dist[v.0][v.1 + 1] == -1 && c[v.0][v.1 + 1] != '#' {
            que.push_back((v.0, v.1 + 1));
            dist[v.0][v.1 + 1] = dist[v.0][v.1] + 1;
        }
    }

    let mut ans = 0;
    for d in dist {
        for dd in d {
            ans = ans.max(dd);
        }
    }

    println!("{}", ans);
}
