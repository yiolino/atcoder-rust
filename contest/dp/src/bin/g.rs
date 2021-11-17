#[allow(unused_imports)]
use proconio::{input, fastout, marker::Chars, marker::Usize1, marker::Bytes};
#[allow(unused_imports)]
use std::collections::{HashSet, HashMap, BTreeSet, VecDeque, BinaryHeap};
#[allow(unused_imports)]
use std::cmp::{max, min, Reverse};
use std::usize;
#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use petgraph::unionfind::UnionFind;
#[allow(unused_imports)]
use superslice::Ext;

#[fastout]
fn main() {
    input!{
        n: usize,
        m: usize,
        xy: [(usize, usize); m],
    }

    // Graph
    let mut graph = vec![vec![]; n];
    for i in 0..m {
        let x = (xy[i].0 - 1) as usize;
        let y = (xy[i].1 - 1) as usize ;

        graph[x].push(y);
    }


    // メモ化再帰を使用する。
    // dp[i] := iから辿れる閉路の最長値
    let mut dpmemo = DpMemo::new(graph);

    let mut ans = 0;
    for i in 0..n {
        ans = ans.max(dpmemo.f(i));
    }

    println!("{}", ans);
}


struct DpMemo {
    seen: Vec<usize>,
    dp: Vec<usize>,
    graph: Vec<Vec<usize>>,
}


impl DpMemo {
    fn new(graph: Vec<Vec<usize>>) -> Self {
        let n = graph.len();
        DpMemo { seen: vec![0; n], dp: vec![0; n], graph: graph }
    }

    fn f(&mut self, idx: usize) -> usize {
        if self.seen[idx] != 0 {
            return self.dp[idx]
        }

        self.seen[idx] = 1;

        let mut f_ans = 0;
        let m = self.graph[idx].len();
        for i in 0..m {
            let j = self.graph[idx][i];
            f_ans = f_ans.max(self.f(j) + 1);
        }

        self.dp[idx] = f_ans;

        f_ans
    }
}