#[allow(unused_imports)]
use proconio::{input, fastout, marker::Chars};
#[allow(unused_imports)]
use std::collections::{HashSet, HashMap, BTreeSet, VecDeque};
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use itertools::Itertools;

#[fastout]
fn main() {
    input!{
         n: usize,
         ab: [(usize, usize); n - 1],
    }

    let mut graph = vec![vec![]; n];

    for (ai, bi) in ab {
        graph[ai-1].push(bi-1);
        graph[bi-1].push(ai-1);
    }

    let mut dfs = DFS::new(n);

    
    for i in 0..n {
        if dfs.seen[i] != -1 {
            continue;
        }
        dfs.is_bipartite_graph(&graph, i, 0);
    }

    let sum: i64 = dfs.seen.iter().map(|x| *x).sum();

    let key = if n as i64 - sum > n as i64 / 2 {
        0
    } else {
        1
    };

    let mut vec = vec![];

    for (i, v) in dfs.seen.iter().enumerate() {
        if *v as i32 == key {
            vec.push(i+1);
        }

        if vec.len() >= n / 2 {
            break;
        }
    }

    let ans: String = vec
                    .into_iter()
                    .map(|x| x.to_string())
                    .join(" ");

    println!("{}", ans);
}



struct DFS {
    seen: Vec<i64>,  // 各頂点の訪問状態を格納するvector
}


//------------------------ begin DFS ------------------------
impl DFS {
    /// n: 頂点数
    fn new(n: usize) -> Self {
        DFS {
            seen: vec![-1; n],  // 全頂点を未訪問に初期化
        }
    }

    /// けんちょん本 p.233を参照
    /// 与えられたグラフが二部グラフかどうかの判定。seen配列において、0, 1を2つの色とする。
    fn is_bipartite_graph (&mut self, graph: &Vec<Vec<usize>>, s: usize, cur: i64) -> bool {
        self.seen[s] = cur;  // 初期条件： 頂点sを初期頂点とする。

        // DFS開始 再帰関数を用いる
        for next_v in &graph[s] {
            // 隣接頂点が既に色確定していた場合
            if self.seen[*next_v] != -1 {
                // 同じ色が隣接した場合は二部グラフではない
                if self.seen[*next_v] == cur {
                    return false;
                }

                // 色が確定していた場合は探索しない
                continue;
            }

            // 隣接頂点の色を変えて、再帰的に探索
            // false が返ってきたら false を返す
            if !self.is_bipartite_graph(&graph, *next_v, 1 - cur) {
                return false;
            }
        }

        true
    }

}