///------------------------ DFS ------------------------
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




    
    // /// 二部グラフ判定（未完成なう）
    // /// けんちょん本 p.233を参照
    // fn is_bipartite＿graph(&mut self, graph: &Vec<Vec<usize>>, s: usize) {
    //     self.seen[s] = 0;  // 初期条件： 頂点sを初期頂点とする。
    //     self.que.push_back(s);  // sをtodoリストに入れる。

    //     // DFS開始（キューが空になるまで探索を行う）
    //     while !self.que.is_empty() {
    //         let tmp_v = self.que.pop_front();  // キューから先頭頂点を取り出す。
    //         let mut v = 0;

    //         if let Some(vv) = tmp_v {   
    //             v = vv;
    //         } else {
    //             panic!("que is empty. Something wrong!");
    //         }

    //         // vからたどれる頂点を全て調べる。
    //         for x in &graph[v] {
    //             // 既に発見済みの頂点は探索しない
    //             if self.seen[*x as usize] != -1 {
    //                 continue;
    //             }

    //             self.seen[*x as usize] = self.seen[v] + 1;  // 新たな頂点xについて、距離情報を更新（sから始まって、1ずつ足されていく）。
    //             self.que.push_back(*x);  // todoリストに入れる。
    //         }
    //     }
    // }

}

//------------------------ end DFS ------------------------
