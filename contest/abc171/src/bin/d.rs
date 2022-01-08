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
        q: usize,
    }

    // aiの値が何個あるかカウント
    let mut vec = vec![0; 100_001];
    for ai in &a {
        vec[*ai] += 1;
    }

    // 差分を考える
    let mut ans: usize = vec.iter()
                            .enumerate()
                            .map(|(i, ai)| i * *ai)
                            .sum();

    for _ in 0..q {
        input!{b: usize, c: usize};

        ans -= b * vec[b];
        ans += vec[b] * c;

        vec[c] += vec[b];
        vec[b] = 0;

        println!("{}", ans);
    }
}
