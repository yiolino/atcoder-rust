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
        n: i64,
        a: i64,
        b: i64,
        c: i64,
    }

    let mut ans = 9999;

    for i in 0..=9999 {
        for j in 0..=9999-i {
            let v = n - i * a - j * b;
            let r = i + j + v / c;
            if v % c != 0 || v < 0 || r > 9999 {
                continue;
            }
            ans = ans.min(r);
        }
    }

    println!("{}", ans);
}
