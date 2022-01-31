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
        nml: [usize; 3],
        pqr: [usize; 3],
    }

    let mut ans = 0;
    for p in (0..3).permutations(3) {
        ans = ans.max((nml[0] / pqr[p[0]]) * (nml[1] / pqr[p[1]]) * (nml[2] / pqr[p[2]]));
    }

    println!("{}", ans);
}
