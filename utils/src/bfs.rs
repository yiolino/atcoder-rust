// 幅優先探索によって始点sから各頂点への最短路を求める。
use std::collections::VecDeque;

struct BFS {
    dist: Vec<i64>,  // 各頂点の訪問状態を格納するvector
    que: VecDeque<usize>, // todoリスト
}


//------------------------ begin BFS ------------------------
impl BFS {
    // n: 頂点数
    fn new(n: usize) -> Self {
        BFS {
            dist: vec![-1; n],  // 前兆点を未訪問に初期化
            que: VecDeque<usize>::new(),
        }
    }

    fn find_shortest (&mut self, graph: &Vec<Vec<usize>>, s: usize) -> Vec<i64> {
        self.dist[s] = 0;  // 初期条件： 頂点sを初期頂点とする。
        self.que.push_back(s);  // sをtodoリストに入れる。

        // BFS開始（キューが空になるまで探索を行う）
        while !self.que.is_empty() {
            let v = self.que.pop_front();  // キューから先頭頂点を取り出す。

            if let Some(vv) = v {
                v = vv;
            } else {
                println!("v is None. Something wrong!");
            }

            // vからたどれる頂点を全て調べる。
            for x in graph[v] {
                // 既に発見済みの頂点は探索しない
                if dist[x] != -1 {
                    continue;
                }

                dist[x] = dist[v] + 1;  // 新たな頂点xについて、距離情報を更新（sから始まって、1ずつ足されていく）。
                que.push_back(x);  // todoリストに入れる。
            }
        }

        dist
    }

}
//------------------------ end BFS ------------------------



// このモジュールはcargo testを実行した時のみコンパイルされる
#[cfg(test)]
mod tests {


    // #[test]の付いた関数はcargo testとしたときに実行される
    #[test]
    fn make_graph() -> Vec<Vec<usize>> {
        // 頂点数と辺数
        let n = 8; let m = 13;

        let input = [(4, 1), (4, 2), (4, 6), (1, 3), (1, 6), (2, 5), (2, 7), (6, 7), (3, 0), (3, 7), (7, 0), (0, 5)];
        let graph = vec![vec![]];

        for (v1, v2) in input {
            // 無向グラフ
            graph[v1].push(v2);
            graph[v2].push(v1);
        }
        let
        assert_eq!(graph, Vec);
    }

}


