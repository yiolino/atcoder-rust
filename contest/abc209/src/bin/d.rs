use std::mem::swap;
use proconio::*;

fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut graph = vec![vec![]; n];
    for _ in 0..n-1 {
        input! {a: marker::Usize1, b: marker::Usize1};
        graph[a].push(b);
        graph[b].push(a);
    }

    let mut lca = LCA::new(&graph);
    lca.init(&graph, 0);

    for _ in 0..q {
        input! {c: marker::Usize1, d: marker::Usize1};
        let l = lca.lca(c, d);
        let d = lca.dist[c] + lca.dist[d] - 2 * lca.dist[l];

        let ans = if d % 2 == 0 {
            "Town"
        } else {
            "Road"
        };

        println!("{}", ans);
    }
}



// Lowest Common Ancestor, 共通祖先を見つける 
// 参考：https://algo-logic.info/lca/
struct LCA {
    parent: Vec<Vec<usize>>,  // parent[k][u] := 2^k個上のuの親
    dist: Vec<usize>, // root からの距離
}

impl LCA {
    fn new(graph: &Vec<Vec<usize>>) -> Self {
        let n = graph.len();
        let mut k: usize = 1;
        while 1<<k < n {
            k += 1;
        }
        LCA { 
            parent: vec![vec![0; n]; k], 
            dist: vec![0; n],
        }
    }

    // 根からの距離と1つ先の頂点を求める
    fn dfs(&mut self, graph: &Vec<Vec<usize>>, v: usize, p: usize, d: usize) {
        self.parent[0][v] = p;
        self.dist[v] = d;
        for child in &graph[v] {
            if *child != p {
                self.dfs(graph, *child, v, d + 1);
            }
        }
    }

    // 準備
    fn init(&mut self, graph: &Vec<Vec<usize>>, root: usize) {
        self.dfs(graph, root, std::usize::MAX, 0);
        for k in 0..self.parent.len() - 1 {
            for j in 0..graph.len() {
                if self.parent[k][j] == std::usize::MAX {
                    self.parent[k + 1][j] = std::usize::MAX;
                } else {
                    self.parent[k + 1][j] = self.parent[k][self.parent[k][j]];
                }
            }
        }
    }

    // 共通祖先を求める
    fn lca(&self, mut u: usize, mut v: usize) -> usize {
        if self.dist[u] < self.dist[v] {
            swap(&mut u, &mut v); // uの方を深くする
        }

        let k = self.parent.len();
        // lcaまでの距離を同じにする
        for ki in 0..k {
            if (self.dist[u] - self.dist[v] >> ki) & 1 > 0 {
                u = self.parent[ki][u];
            }
        }
        // 二分探索でLCAを求める
        if u == v {
            return u;
        }

        for ki in (0..k).rev() {
            if self.parent[ki][u] != self.parent[ki][v] {
                u = self.parent[ki][u];
                v = self.parent[ki][v];
            }
        }

        self.parent[0][u]
    }
}