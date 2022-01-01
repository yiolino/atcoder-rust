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
        mut  vec: [usize; 5],
    }

    let mut ans = vec![];

    for perm in (0..5).combinations(3) {
        let mut tmp = 0;
        for p in perm {
            tmp += vec[p];
        }
        ans.push(tmp);
    }

    ans.sort();
    ans.reverse();

    println!("{}", ans[2]);
}
