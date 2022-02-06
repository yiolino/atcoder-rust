#[allow(unused_imports)]
use proconio::{input, fastout, marker::Chars, marker::Usize1, marker::Bytes};
#[allow(unused_imports)]
use std::collections::*;
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
        x: usize,
        a: [[usize]; n]  // [usize] で受け取ると, 0idxが配列数となることに注意
    }

    // BTreeMapで解いてみる
    // 再起的かつdp的な考え方 xからaで割っていって、1に辿り着くかどうか

    let mut bmap = BTreeMap::new();
    bmap.insert(x, 1_usize);

    for a in a {
        let mut next = BTreeMap::new();
        for (x, w) in bmap {
            for a in &a {
                if x % *a == 0 {
                    *next.entry(x / *a).or_insert(0) += w;
                }
            }
        }

        bmap = next;
    }

    let ans = bmap.get(&1).map_or(0, |p| *p);

    println!("{}", ans);
}
