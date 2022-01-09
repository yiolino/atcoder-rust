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
        p: usize,
        t: [usize; n],
    }

    // t[i]人目の人がボタンを押すときに、t[i-1]人目の人のシャワーが止まる時刻を考える。
    let mut stop_time = 0;
    let mut ans = 0;

    for ti in t {
        let diff = ti + p - stop_time;
        if diff >= p {
            ans += p;
        } else {
            ans += diff;
        }

        stop_time = ti + p;
    }

    println!("{}", ans);
}
