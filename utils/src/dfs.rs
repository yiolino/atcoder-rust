use core::panic;
// 幅優先探索によって始点sから各頂点への最短路を求める。

struct DFS {
    seen: Vec<i64>,  // 各頂点の訪問状態を格納するvector
}


//------------------------ begin DFS ------------------------
impl DFS {
    /// num_v: 頂点数
    fn new(num_v: usize) -> Self {
        DFS {
            dist: vec![-1; num_v],  // 全頂点を未訪問に初期化
        }
    }

    /// けんちょん本 p.233を参照
    /// 与えられたグラフが二部グラフかどうかの判定
    fn is_bipartite＿graph (&mut self, graph: &Vec<Vec<usize>>, s: usize) {
        self.dist[s] = 0;  // 初期条件： 頂点sを初期頂点とする。
        self.que.push_back(s);  // sをtodoリストに入れる。

        // BFS開始（キューが空になるまで探索を行う）
        while !self.que.is_empty() {
            let tmp_v = self.que.pop_front();  // キューから先頭頂点を取り出す。
            let mut v = 0;

            if let Some(vv) = tmp_v {   
                v = vv;
            } else {
                panic!("que is empty. Something wrong!");
            }

            // vからたどれる頂点を全て調べる。
            for x in &graph[v] {
                // 既に発見済みの頂点は探索しない
                if self.dist[*x as usize] != -1 {
                    continue;
                }

                self.dist[*x as usize] = self.dist[v] + 1;  // 新たな頂点xについて、距離情報を更新（sから始まって、1ずつ足されていく）。
                self.que.push_back(*x);  // todoリストに入れる。
            }
        }
    }

}
//------------------------ end DFS ------------------------
