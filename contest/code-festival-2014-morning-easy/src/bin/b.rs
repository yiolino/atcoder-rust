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
        mut n: usize,
    }

    // 0 indexにする
    n -= 1;

    // 20で割った余りと商を考えれば良い
    let sho = n / 20;
    let amari = n % 20;

    let ans: usize;
    
    if sho % 2 == 0 {
        ans = amari;
    } else {
        ans = 19 - amari;
    }

    println!("{}", ans + 1);
}
