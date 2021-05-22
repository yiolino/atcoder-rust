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
        mut S: Chars,
    }

    S.reverse();

    for s in S {
        if s == '0' {
            print!("{}", s);
        } else if s == '1' {
            print!("{}", s);
        } else if s == '6' {
            print!("9");
        } else if s == '8' {
            print!("{}", s);
        } else if s == '9' {
            print!("6");
        }
    }

    println!("");
}
