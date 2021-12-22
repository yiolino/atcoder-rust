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

    let mut memo = vec![0; 100_000];

    for ai in a {
        memo[ai] += 1;
    }

    let mut ans = 0;
    for i in 0..100_000 {
        let tmp;
        if i == 0 {
            tmp = memo[i] + memo[i+1];
        } else if i == 100_000 - 1 {
            tmp = memo[i-1] + memo[i];
        } else {
            tmp = memo[i-1] + memo[i] + memo[i+1];
        }
        
        ans = ans.max(tmp);
    }

    println!("{}", ans);
}
