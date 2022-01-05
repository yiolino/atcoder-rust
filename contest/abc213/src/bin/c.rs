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
        _h: usize,
        _w: usize,
        p: [(u32, u32)],
    }

    // 座標圧縮で解く
    let (mut x, mut y): (Vec<_>, Vec<_>) = p.iter().cloned().unzip();

    x.sort();
    x.dedup(); // 連続した重複要素を除去

    y.sort();
    y.dedup();

    // superslice crate の Ext::lowerboundを使う
    for (a, b) in p {
        let a = x.lower_bound(&a) + 1;
        let b = y.lower_bound(&b) + 1;
        println!("{} {}", a, b);
    }
}

