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
        s: [String; n],
    }

    let mut map = HashMap::new();


    for s in s {
        *map.entry(s).or_insert(0) += 1;
    }

    let vec = map.into_iter()
                            .sorted_by_key(|(_k, v)| Reverse(*v))
                            .collect::<Vec<(String, usize)>>();

    println!("{}", vec[0].0);
}
