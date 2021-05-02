#[allow(unused_imports)]
use proconio::{input, fastout, marker::Chars};
#[allow(unused_imports)]
use std::collections::{HashSet, HashMap, BTreeSet};
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use itertools::Itertools;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input!{
        N: usize,
        D: f64,
        H: f64,
        dh: [(f64, f64); N],
    }

    let mut ans = 0.0;

    for i in 0..N {
        let alpha = (H - dh[i].1) * dh[i].0 / (D - dh[i].0);
        if dh[i].1 - alpha > ans {
            ans = dh[i].1 - alpha;
        }
    }

    println!("{}", ans);
}
