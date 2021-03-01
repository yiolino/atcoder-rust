#[allow(unused_imports)]
use proconio::{input, fastout, marker::Chars};
#[allow(unused_imports)]
use std::collections::{HashSet, HashMap, BTreeSet};

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input!{
        N: i64,
        _L: i64,
        mut S: [String; N],
    }

    S.sort();

    println!("{}", S.join(""));
}
