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

    let mut graph = vec![vec![]; n];

    for _ in 0..m {
        input! {mut a: usize, mut b: usize};
        // 0 idx
        a -= 1;
        b -= 1;

        graph[a].push(b);
        graph[b].push(a);
    }

    let mut dfs = DFS::new(n, graph);

    let mut ans = 0;
    // 訪問済みの印をつけていく
    for v in 0..n {
        if dfs.seen[v] != -1 {
            continue;
        }
        
        dfs.dfs(v);
        ans += 1;
    }

    // 連結成分の個数 - 1 が答え
    println!("{}", ans-1);
}





/// 幅優先探索によって始点sから各頂点への最短路を求める。
struct DFS {
    seen: Vec<i64>,  // 各頂点の訪問状態を格納するvector
    graph: Vec<Vec<usize>>,
}


//------------------------ DFS ------------------------
impl DFS {
    /// num_v: 頂点数
    /// seen: 訪問したかどうかのmemo. 距離情報としても使う場合がある
    fn new(num_v: usize, graph: Vec<Vec<usize>>) -> Self {
        DFS {
            seen: vec![-1; num_v],  // 全頂点を未訪問に初期化
            graph: graph,
        }
    }


    /// 連結成分のカウント
    /// けんちょんQiita参考
    fn dfs(&mut self, s: usize) {
        self.seen[s] = 1;  // sを訪問済みにする。

        for i in 0..self.graph[s].len() {
            let next_v = self.graph[s][i];
            
            if self.seen[next_v] != -1 {
                continue;
            }

            self.dfs(next_v);

        }

    }


}
