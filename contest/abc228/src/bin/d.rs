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


#[fastout]
fn main() {
    input!{
        q: usize,
        ask: [(u8, usize); q],
    }

    let n = 1 << 20;
    let mut pos = (0..n).map(|k| k).collect::<Vec<_>>();
    let mut a = vec![-1; n];

    for (t, x) in ask {
        if t == 1 {
            let root = find(x%n, &mut pos);
            a[root] = x as i64;
            pos[root] = (root + 1) % n;

        } else {
            println!("{}", a[x%n]);
        }
    }
}


fn find(v: usize, pos: &mut Vec<usize>) -> usize {
    if v == pos[v] {
        return v;
    }

    let u = find(pos[v], pos);
    pos[v] = u;

    u
}   
