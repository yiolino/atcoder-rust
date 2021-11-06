#[allow(unused_imports)]
use proconio::{input, fastout, marker::Chars};
#[allow(unused_imports)]
use std::collections::{HashSet, HashMap, BTreeSet, VecDeque};
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use itertools::Itertools;

const MOD: usize = 1000000007;

#[fastout]
fn main() {
    input!{
        n: usize,
        c: [char; n],
    }

    let mut graph = vec![vec![]; n];

    for _ in 0..n-1 {
        input!(a: usize, b: usize);
        graph[a.wrapping_sub(1)].push(b.wrapping_sub(1));
        graph[b.wrapping_sub(1)].push(a.wrapping_sub(1));
    }

    let mut dpdfs = DpDfs::new(graph, c);

    dpdfs.dfs(0, 9999999);


    println!("{}", dpdfs.dp[0][2]);
}


struct DpDfs {
    dp: Vec<Vec<usize>>,
    graph: Vec<Vec<usize>>,
    c: Vec<char>,
}

impl DpDfs {
    fn new(graph: Vec<Vec<usize>>, c: Vec<char>) -> Self {
        let n = graph.len();

        DpDfs {
            dp: vec![vec![0; 3]; n],
            graph: graph,
            c: c
        }
    }

    /// pos ... 該当頂点
    /// pare ... 親頂点
    fn dfs(&mut self, pos: usize, pare: usize) {
        let mut val1 = 1;
        let mut val2 = 1;

        for i in 0..self.graph[pos].len() {
            let child =self.graph[pos][i];
            if child == pare {
                continue;
            }
            self.dfs(child, pos);

            if self.c[pos] == 'a' {
                val1 *= self.dp[child][0] + self.dp[child][2];
                val2 *= self.dp[child][0] + self.dp[child][1] + 2*self.dp[child][2];

            } else if self.c[pos] == 'b' {
                val1 *= self.dp[child][1] + self.dp[child][2];
                val2 *= self.dp[child][0] + self.dp[child][1] + 2*self.dp[child][2];
            }

            val1 %= MOD;
            val2 %= MOD;
        }
        
        if self.c[pos] == 'a' {
            self.dp[pos][0] = val1;
            self.dp[pos][2] = (val2 - val1 + MOD) % MOD;
        } else if self.c[pos] == 'b' {
            self.dp[pos][1] = val1;
            self.dp[pos][2] = (val2 - val1 + MOD) % MOD;
        }
    }
}