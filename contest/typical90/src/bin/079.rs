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
        h: usize,
        w: usize,
        mut a: [[i64; w]; h],
        b: [[i64; w]; h],
    }

    let mut ans = 0;

    for i in 0..h-1 {
        for j in 0..w-1 {
            let d = b[i][j] - a[i][j];
            a[i][j] += d;
            a[i][j+1] += d;
            a[i+1][j] += d;
            a[i+1][j+1] += d;
            ans += d.abs();
        }
    }

    if a == b {
        println!("Yes");
        println!("{}", ans)
    } else {
        println!("No");
    }
}
