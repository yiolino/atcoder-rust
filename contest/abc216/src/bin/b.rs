#[allow(unused_imports)]
use proconio::{input, fastout, marker::Chars, marker::Usize1};
#[allow(unused_imports)]
use std::collections::{HashSet, HashMap, BTreeSet, VecDeque, BinaryHeap};
#[allow(unused_imports)]
use std::cmp::{max, min, Reverse};
#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use petgraph::unionfind::UnionFind;

#[fastout]
fn main() {
    input!{
        n: usize,
    }

    let mut map = std::collections::HashMap::new();

    for _ in 0..n {
        input!{s: String, t: String};

        let sep = "_".to_string();
        let v = s + &sep + &t;
        *map.entry(v).or_insert(0) += 1;
    }

    for (_, v) in map {
        if v >=2 {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
