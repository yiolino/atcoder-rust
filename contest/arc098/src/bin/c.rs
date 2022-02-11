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
        s: Chars,
    }

    // [0, i) の区間にWが何個あるか、累積をメモしていく
    let mut num_w_vec = vec![];
    num_w_vec.push(0);
    for i in 0..n {
        if s[i] == 'W' {
            num_w_vec.push(num_w_vec[i] + 1);
        } else {
            num_w_vec.push(num_w_vec[i]);
        }
    }

    // [0, i) [i+1, n+1) の W の数と Eの数をカウントする。
    let mut ans = std::usize::MAX;
    for i in 0..n+1 {
        let mut to_switch = num_w_vec[i];
        to_switch += (n + 1 - i - 1) - (num_w_vec[n] - num_w_vec[i]);

        ans = ans.min(to_switch);
    }

    println!("{}", ans);
}
