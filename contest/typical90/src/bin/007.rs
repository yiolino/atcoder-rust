#[allow(unused_imports)]
use proconio::{input, fastout, marker::Chars};
#[allow(unused_imports)]
use std::collections::{HashSet, HashMap, BTreeSet, VecDeque};
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use itertools::Itertools;

#[fastout]
fn main() {
    input!{
        n: usize,
        mut a: [usize; n],
        q: usize,
        b: [usize; q],
    }

    a.sort();

    for i in 0..q {
        let bi = b[i];

        // 二分探索
        let mut lower = 0;
        let mut upper = n - 1;

        while upper - lower > 1 {
            let mid = (lower + upper) / 2;
            if bi >= a[mid] {
                lower = mid;
            } else {
                upper = mid;
            }
        }

        let ans = min((a[upper] as i64 - bi as i64).abs(), (a[lower] as i64 - bi as i64).abs());
        println!("{}", ans);
    }
}
