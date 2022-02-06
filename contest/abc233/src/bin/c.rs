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
        x: usize,   
    }

    let mut l = vec![];

    for _ in 0..n {
        input! {li: usize, a: [usize; li]};
        l.push(a);
    }

    let mut dfs = DFS::new(n, x);

    dfs.dfs(&l, 0, 1);

    println!("{}", dfs.ans)
}

struct DFS {
    ans: usize,
    n: usize,
    x: usize,
}

impl DFS {
    fn new(n: usize, x: usize) -> Self {
        DFS{ans: 0, n: n, x: x}
    }

    fn dfs(&mut self, l: &Vec<Vec<usize>>, i: usize, y: usize) {
        if i == self.n {
            if y == self.x {
                self.ans += 1;
            }
            return;
        }

        for j in 0..l[i].len() {
            if y > self.x / l[i][j] {
                continue;
            }
            self.dfs(l, i + 1, y * l[i][j])
        }
    }
}













    // DP で実装したが、どこかがあかん
    // // xの約数の箇所だけカウントする手法でないとメモリ、計算量共に間に合わない。
    // let mut yakusu = vec![];
    // for i in 1..x {
    //     if i * i > x {
    //         break;
    //     }
    //     if x % i == 0 {
    //         yakusu.push(i);
    //         yakusu.push(x / i);
    //     }
    // }
    // yakusu.sort(); // sortしておく

    // // idxのlookupを作っておく
    // let mut map = HashMap::new();
    // for (i, y) in yakusu.iter().enumerate() {
    //     map.insert(y, i);
    // }


    // let mut dp = vec![vec![0; yakusu.len()]; n + 1];

    // dp[0][yakusu.len() - 1] = 1;

    // for i in 0..n {
    //     for j in (0..yakusu.len()).rev() {
    //         for a in &l[i] {
    //             if dp[i][j] != 0 && yakusu[j] % a == 0 {
    //                 let idx = map.get(&(yakusu[j] / a)).unwrap();
    //                 dp[i+1][*idx] += dp[i][j];
    //             }
    //         }
            
    //     }
    // }

    // println!("{}", dp[n][0]);