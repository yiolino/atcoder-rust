use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q :usize,
    }

    let mut graph = vec![vec![]; n];
    for _ in 0..n-1 {
        input! {a: Usize1, b: Usize1};
        graph[a].push(b);
        graph[b].push(a);
    }

    let mut dfs = DFS::new(&graph);

    dfs.color[0] = 0;
    dfs.dfs(&graph, 0);

    for _ in 0..q {
        input! {c: Usize1, d: Usize1};
        let ans = if dfs.color[c] == dfs.color[d] {
            "Town"
        } else {
            "Road"
        };
        println!("{}", ans)
    }
}


// 頂点 n, 辺 n - 1 なので木構造にできる
struct DFS {
    color: Vec<usize>,
}

impl DFS {
    fn new(graph: &Vec<Vec<usize>>) -> Self {
        let n = graph.len();
        DFS {
            color: vec![std::usize::MAX; n],
        }
    }

    fn dfs(&mut self, graph: &Vec<Vec<usize>>, v: usize) {
        let col = self.color[v];
        for chlid in &graph[v] {
            if self.color[*chlid] != std::usize::MAX {
                continue;
            }

            self.color[*chlid] = col ^ 1;
            self.dfs(&graph, *chlid);
        }
    }
}