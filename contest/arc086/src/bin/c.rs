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
        k: usize,
        a: [usize; n],
    }

    let mut map = HashMap::new();
    for ai in a {
        *map.entry(ai).or_insert(0) += 1;
    }

    let mut vec: Vec<_> = map.into_iter().collect();

    vec.sort_by_key(|v| v.1);

    let len = vec.len();

    let mut ans = 0;
    if len > k {
        for i in 0..len-k {
            ans += vec[i].1 ;
        }
    }

    println!("{}", ans);
}
