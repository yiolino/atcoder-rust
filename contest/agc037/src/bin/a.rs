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
        S: Chars,
    }

    let mut prev = String::new();
    let mut buf = String::new();

    let mut ans = 0;

    for s in S {
        buf.push(s);
        if prev != buf {
            prev = buf;
            buf = String::new();
            ans += 1;
        }
    }

    println!("{}", ans);
}
