#[allow(unused_imports)]
use proconio::{input, fastout, marker::Chars};
#[allow(unused_imports)]
use std::collections::{HashSet, HashMap, BTreeSet};
#[allow(unused_imports)]
use std::cmp::{max, min};

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input!{
        N: usize,
        mut a: [usize; 3 * N],
    }

    a.sort();

    let mut ans = 0;

    for i in (N..3*N).step_by(2) {
        ans += a[i];
    }

    println!("{}", ans);
}
