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
        mut a: [i64; n],
    }

    a.sort();
    a.reverse();

    let sum_a: i64 = a.iter().sum();

    let mut ans = 0;
    let mut sum_ai = 0;
    for i in 0..n {
        ans += (n - i) as i64 * a[i] - sum_a + sum_ai;
        sum_ai += a[i];
    }

    println!("{}", ans);
}
