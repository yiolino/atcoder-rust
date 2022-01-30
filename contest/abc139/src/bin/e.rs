use proconio::*;

const INF:usize = std::usize::MAX;

fn main() {
    input!{
        n: usize,
    }

    // グラフの問題として捉える。
    // DAGでない場合(閉路がある場合)は -1. これはトポロジカルソート的発想で考えれば良い。
    // グラフの各頂点は選手のペアとして考える。これはペアidを表の[i, j] = [j, i] として
    // [i, i] = -1 とでもしておけばOK.

    // idテーブルの作成
    let mut id_table = vec![vec![INF; n]; n];
    let mut cnt = 0;
    for i in 0..n {
        for j in i+1..n {
            id_table[i][j] = cnt;
            id_table[j][i] = cnt;
            cnt += 1;
        }
    }

    let m = n * (n - 1) / 2;
    let mut graph = vec![vec![]; m];
    for i in 0..n {
        input! {a: [usize; n - 1]};
        for j in 1..n-1 {
            let id_pre = id_table[i][a[j - 1] - 1]; // 0 idx
            let id = id_table[i][a[j] - 1];
            graph[id_pre].push(id);
        }        
    }

    let mut dfs = DFS::new(graph);
    let mut ans = 0;
    
    for u in 0..m {
        dfs.dfs(u);
        ans = ans.max(dfs.path_len[u]);
    }
    

    if dfs.is_loop {
        println!("-1");
        return;
    }

    println!("{}", ans);
}


struct DFS {
    seen: Vec<bool>,
    graph: Vec<Vec<usize>>,
    is_rec_end: Vec<bool>,
    path_len: Vec<usize>,
    is_loop: bool,
}

impl DFS {
    fn new(graph: Vec<Vec<usize>>) -> Self {
        let n = graph.len();
        DFS {
            seen: vec![false; n],
            graph: graph,
            is_rec_end: vec![false; n],
            path_len: vec![0; n],  // uからスタートした時の最長経路
            is_loop: false,
        }
    }

    fn dfs(&mut self, u: usize) {
        if self.seen[u] {
            // 探索済みなのに、再帰から抜けていないなら、
            // ループしていることになる
            if !self.is_rec_end[u] {
                self.is_loop = true;
            }

            return; // 探索済みは再帰を終了
        }

        self.seen[u] = true;
        self.path_len[u] = 1; 

        for i in 0..self.graph[u].len() {
            let next_u = self.graph[u][i];
            self.dfs(next_u);

            // loopが見つかったら即終了
            if self.is_loop {return;}

            self.path_len[u] =  self.path_len[u].max(self.path_len[next_u] + 1);
        }

        self.is_rec_end[u] = true;  // uの再帰から帰ってきたフラグ
    }
}