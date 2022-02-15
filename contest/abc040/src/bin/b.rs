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
        n: i64,
    }

    if n == 1 {
        println!("0");
        return;
    }

    let mut ans = std::i64::MAX;

    for i in 1..n {
        let y = n / i;
        let amari = n - (n / i * i );

        ans = ans.min((y - i).abs() + amari);
    }


    println!("{}", ans);
}
