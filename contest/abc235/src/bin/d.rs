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
        a: usize,
        n: usize,
    }

    // 探索する範囲はNの10倍までで良い
    let mut m = 1_usize;
    while m <= 10 * n {
        m *= 10;
    }

    let mut dist = vec![-1_i64; m];
    let mut que = VecDeque::new();

    dist[1] = 0;
    que.push_back(1);

    while !que.is_empty() {
        let v = que.pop_front().unwrap();

        let va = v * a;
        if va < m && dist[va] == -1 {
            dist[va] = dist[v] + 1;
            que.push_back(va);
        }


        if v > 10 && v % 10 != 0 {
            let mut v_chars = v.to_string().chars().collect::<Vec<char>>();
            let end = v_chars.pop().unwrap();
            v_chars.insert(0, end);

            let v_rotate = v_chars.iter()
                                    .collect::<String>()
                                    .parse::<usize>()
                                    .unwrap();

            if v_rotate < m && dist[v_rotate] == -1 {
                dist[v_rotate] = dist[v] + 1;
                que.push_back(v_rotate);
            }

        }
    }


    println!("{}", dist[n]);
}


// // BSF
// struct BFS {
//     dist: Vec<i64>,
//     todo: VecDeque<usize>,
//     graph: Vec<Vec<usize>>,
// }

// impl BFS {
//     fn new(graph: Vec<Vec<usize>>) -> Self {
//         let n = graph.len();
//         BFS {
//             dist: vec![-1; n],
//             todo: VecDeque::<usize>::new(),
//             graph: graph,
//         }
//     }

//     // start頂点からの距離（重みなし）を記録していく
//     fn calc_path_dist(&mut self, start: usize) {
//         self.dist[start] = 0;

//         // startをqueに追加
//         self.todo.push_back(start);

//         while !self.todo.is_empty() {
//             let v = self.todo.pop_front().unwrap();

//             for next_v in &self.graph[v] {
//                 if self.dist[*next_v] != -1 {
//                     continue;
//                 }

//                 self.dist[*next_v] = self.dist[v] + 1;
//                 self.todo.push_back(*next_v);
//             }
//         }
        
//     }
// }