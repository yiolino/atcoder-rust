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
        S: Chars,
        K: usize,
    }

    let mut ans = 1;
    for i in 0..K {
        let num = S[i] as i64 - 48;
        if num != 1 {
            ans = num;
            break;
        }
    }

    println!("{}", ans);
}
