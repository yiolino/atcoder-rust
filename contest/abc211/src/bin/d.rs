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
    }

    let waru = 1_000_000_000 + 7;
    let mut graph = vec![vec![]; n];
    let mut dp = vec![0; n];
    let mut dist = vec![-1; n];
    let mut que = VecDeque::new();

    for _ in 0..m {
        input! {a: usize, b: usize};
        graph[a - 1].push(b - 1);
        graph[b - 1].push(a - 1);
    }

    dist[0] = 0;
    dp[0] = 1;

    que.push_back(0);

    while !que.is_empty() {
        let v = que.pop_front().unwrap();

        for vi in &graph[v] {
            if dist[*vi] == -1 {
                que.push_back(*vi);

                dist[*vi] = dist[v] + 1;
                dp[*vi] = dp[v];
                // dp[*vi] %= waru;
            } else if dist[*vi] == dist[v] + 1 {
                dp[*vi] += dp[v];
                dp[*vi] %= waru;
            }
        }

    }
    

    println!("{}", dp[n-1]);
}
