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

    // aiが何個あるかカウント
    let mut vec = vec![0; n + 1];
    for ai in &a {
        vec[*ai] += 1;
    }

    let mut cnt = 0_usize;
    for v in &vec {
        if *v >= 2 {
            cnt += v * (v - 1) / 2;
        }
        
    }

    for ai in &a {
        let mut ans = cnt;
        if vec[*ai] == 2 {
            ans -= 1;
        } else if vec[*ai] > 2 {
            let v= vec[*ai];
            ans -= v * (v - 1) / 2;
            ans += (v - 1) * (v - 2) / 2;
        }

        println!("{}", ans)
    }
}
