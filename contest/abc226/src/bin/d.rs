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
        xy: [(i64, i64); n],
    }

    let mut set = HashSet::new();

    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }

            let mut x = xy[i].0 - xy[j].0;
            let mut y = xy[i].1 - xy[j].1;


            let gcd = num::integer::gcd(x, y);

            x /= gcd;
            y /= gcd;
            

            set.insert((x, y));

        }
    }

    

    println!("{}", set.len());
}
