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
        a: [usize; n],
    }

    let mut map = HashMap::new();

    for ai in a {
        match ai {
            1..=399 => *map.entry("grey").or_insert(0) += 1,
            400..=799 => *map.entry("brown").or_insert(0) += 1,
            800..=1199 => *map.entry("green").or_insert(0) += 1,
            1200..=1599 => *map.entry("skyblue").or_insert(0) += 1,
            1600..=1999 => *map.entry("blue").or_insert(0) += 1,
            2000..=2399 => *map.entry("yellow").or_insert(0) += 1,
            2400..=2799 => *map.entry("orange").or_insert(0) += 1,
            2800..=3199 => *map.entry("red").or_insert(0) += 1,
            _ => *map.entry("rainbow").or_insert(0) += 1,
        }
    }


    let mut min = 0;
    let mut max = 0;


    for (k, v) in map {
        if k != "rainbow" {
            min += 1;
            max += 1;
        } else {
            max += v;
        }
    }

    if min == 0 {
        min = 1;
    }

    println!("{} {}", min, max);
}
