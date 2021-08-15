#[allow(unused_imports)]
use proconio::{input, fastout, marker::Chars};
#[allow(unused_imports)]
use std::collections::{HashSet, HashMap, BTreeSet, VecDeque, BinaryHeap};
#[allow(unused_imports)]
use std::cmp::{max, min, Reverse};
#[allow(unused_imports)]
use itertools::Itertools;

#[fastout]
fn main() {
    input!{
        n: usize,
        m: usize,
    }

    // UnionFind構造体を初期化
    let mut uf = UnionFind::new(n);

    for _ in 0..m {
        input!{
            a: usize,
            b: usize,
        }

        // 交差点aと交差点bは繋がっているので、併合する。
        uf.unite(a - 1, b - 1);
    }


    let mut cnt = 0;
    for i in 0..n {
        if uf.unite(0, i) {
            cnt += 1;
        }
    }


    println!("{}", cnt);
}



//---------- begin union_find by @nakamurus ----------
struct UnionFind {
    par: Vec<usize>,  // 各頂点の親頂点の番号を表す。
    siz: Vec<usize>,  // 各頂点の属する根付き木の頂点数を表す
}

#[allow(unused)]
impl UnionFind {
    fn new(n: usize) -> Self {
        // 初期値は自身が自身の根出る（各木の頂点数は1）
        UnionFind {
            par: (0..n).collect(),
            siz: vec![1; n],
        }
    }


    // 根を返す。経路圧縮による効率化の実装
    fn find_root(&mut self, x: usize) -> usize {
        if self.par[x] == x {
            return x;  // xが根の場合はxを返す。
        }
        self.par[x] = self.find_root(self.par[x]);
        self.par[x]
    }


    // xとyが同じ木に属するか判定
    fn is_same(&mut self, x: usize, y: usize) -> bool {
        self.find_root(x) == self.find_root(y)
    }


    // xの木とyの木を併合する. 関数の戻り値はboolであることに注意
    fn unite(&mut self, mut x: usize, mut y: usize) -> bool {
        // x, y それぞれ根まで移動する
        x = self.find_root(x);
        y = self.find_root(y);

        // 既に同じ木だった場合は何もしない
        if x == y  {
            return false;
        }

        // uion by size（y側の木の頂点数が小さくなるようにする）
        if self.siz[x] < self.siz[y] {
            std::mem::swap(&mut x, &mut y);
        }

        // yをxの子とする
        self.par[y] = x;
        self.siz[x] += self.siz[y];
        
        true
    }


    // 
    fn return_size(&mut self, x: usize) -> usize {
        let root = self.find_root(x);
        self.siz[root]
    }
    
}
//---------- end union_find ----------