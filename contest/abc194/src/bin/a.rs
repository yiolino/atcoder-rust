#[allow(unused_imports)]
use proconio::{input, fastout, marker::Chars};
#[allow(unused_imports)]
use std::collections::{HashSet, HashMap, BTreeSet};

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input!{
        A: i64,
        B: i64,
    }

    let AB = A + B;

    let mut ans = 4;
    if AB >= 15 && B >= 8 {
        ans = 1;
    } else if AB >= 10 && B >= 3{
        ans = 2;
    } else if AB >= 3 {
        ans = 3;
    }

    println!("{}", ans);
}
