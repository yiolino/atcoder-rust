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

// Uniof Find で解く
fn main() {
    input!{
        n: usize,
        m: usize,
    }

    let mut uf = UnionFind::new(n);

    for _ in 0..m {
        input! {mut a: usize, mut b: usize};
        a -= 1;
        b -= 1;

        uf.union(a, b);
    }

    let set: HashSet<_> = uf.into_labeling()
                            .into_iter()
                            .collect();

    println!("{:?}", set.len() - 1);
}
