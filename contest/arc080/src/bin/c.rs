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

    let mut num_four_bai = 0;
    let mut num_odd = 0;
    let mut num_two_bai = 0;

    for a in a {
        if a % 4 == 0 {
            num_four_bai += 1;
        } else if a % 2 == 1 {
            num_odd += 1;
        } else {
            num_two_bai += 1;
        }
    }

    let mut ans = "No";

    if num_odd == 0 {
        ans = "Yes";
    } else {
        if num_four_bai > 0 {
            if num_odd + num_two_bai % 2 <= num_four_bai + 1 {
                ans = "Yes";
            }
        }
    }

    println!("{}", ans);
}
