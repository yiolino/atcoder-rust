#[allow(unused_imports)]
use proconio::{input, fastout, marker::Chars, marker::Usize1};
#[allow(unused_imports)]
use std::collections::{HashSet, HashMap, BTreeSet, VecDeque};
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use itertools::Itertools;

#[fastout]
fn main() {
    input!{
        h: usize,
        w: usize,
        q: usize,
    }

    let pos = |x: usize, y: usize| x * w + y; // h*w のマス目を1次元に展開した時の、(x, y)のposition
    let mut red = vec![vec![false; w]; h];  // 赤く塗られたマス目のboolean. falseで初期化.
    let mut uf = UnionFind::new(h * w);

    for _ in 0..q {
        input!(op: u8);

        if op == 1 {
            input!(mut x: usize, mut y: usize);
            x -= 1;
            y -= 1;
            red[x][y] = true;
            for &(dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)].iter() {
                let p = (x as i64 + dx) as usize;
                let q = (y as i64 + dy) as usize;
                if p < h && q < w && red[p][q] {
                    uf.unite(pos(x, y), pos(p, q));
                }
            }
        } else {
            input!(x: Usize1, y: Usize1, z: Usize1, w: Usize1);
            let ans = if red[x][y] && uf.is_same(pos(x, y), pos(z, w)) {
                "Yes"
            } else {
                "No"
            };

            println!("{}", ans);
        }
    }
}



//---------- begin union_find by @nakamurus ----------
struct UnionFind {
    par: Vec<usize>,  // 各頂点の親頂点の番号を表す
    siz: Vec<usize>,  // 各頂点の属する根付き木の頂点数を表す
}

#[allow(unused)]
impl UnionFind {
    fn new(n: usize) -> Self {
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


    // xの木とyの木を併合する
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