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
        _m: usize,
        a: [usize; n],
    }

    

    let mut map = std::collections::HashMap::new();
    for ai in a {
        *map.entry(ai).or_insert(0) += 1;
    }

    let mut sorted_map = map.into_iter()
        .sorted_by_key(|(_, v)| *v)
        .collect::<Vec<(usize, usize)>>();

    sorted_map.reverse();


    if sorted_map[0].1 > n / 2 {
        println!("{}", sorted_map[0].0);
    } else {
        println!("?");
    }
}
