#[allow(unused_imports)]
use proconio::{input, fastout, marker::Chars};
#[allow(unused_imports)]
use std::collections::{HashSet, HashMap, BTreeSet, VecDeque};
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use itertools::Itertools;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input!{
        N: usize,
        mut a: [usize; N],
    }

    a.insert(0, 0);

    let mut ans = -1;
    let mut idx_now = 1;
    for i in 1..=N {
        idx_now = a[idx_now];
        if idx_now == 2 {
            ans = i as i64;
            break;
        }
    }

    println!("{}", ans);
}
