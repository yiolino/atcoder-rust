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
        X: i64,
    }

    let sho = X / 100;
    let amari = X % 100;

    let mut ans = 0;
    if amari <= sho * 5 {
        ans = 1;
    }

    println!("{}", ans);
}
