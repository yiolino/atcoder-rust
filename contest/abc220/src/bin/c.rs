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

#[fastout]
fn main() {
    input!{
        n: i64,
        a: [i64; n],
        x: i64,
    }

    let sum_a: i64 = a.iter().sum();

    let cnt  = x / sum_a;

    let mut ans = n * cnt;

    let xx = x % sum_a;
    // if xx == 0 {
    //     println!("{}", ans+1);
    //     return;
    // }

    let mut sum = 0;

    for ai in a.iter() {
        sum += ai;
        ans += 1;
        if sum > xx {
            println!("{}", ans);
            return;
        }
    }
}
