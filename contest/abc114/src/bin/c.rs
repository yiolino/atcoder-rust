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
    }

    // dfsで解く
    // 空文字列に3,5,7を追加していく
    // この時、3,5,7を少なくとも1回はcontainしているか調べれば良い
    let mut dfs = DFS::new(n);

    dfs.dfs();

    println!("{}", dfs.ans);
}

struct DFS {
    s: Vec<char>,
    ans: usize,
    c_vec: Vec<char>,
    n: usize,
}

impl DFS {
    fn new(n: usize) -> Self {
        DFS { s: vec![], ans: 0 , c_vec: vec!['7', '5', '3'], n: n}
    }

    fn dfs(&mut self) {
        let res = self.s.iter().
                    collect::<String>()
                    .parse::<usize>();
        let s_int = match res {
            Ok(v) => v,
            _ => 0_usize,
        };

        // n を越えたらreturn
        if s_int > self.n {
            return;
        } else {
            if self.c_vec.iter().all(|x| self.s.contains(x)) {
                self.ans += 1;
            }
        }

        // 再帰全探索
        for i in 0..self.c_vec.len() {
            let c = self.c_vec[i];
            self.s.push(c);

            self.dfs();

            self.s.pop();
        }
        

        return;
    }


}