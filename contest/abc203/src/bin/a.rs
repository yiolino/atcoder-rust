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
        mut abc: [i64; 3],
    }

    abc.sort();

    if abc[0] == abc[1] {
        println!("{}",abc[2]);
    } else if abc[1] == abc[2] {
        println!("{}", abc[0]);
    } else {
        println!("{}", 0);
    }
}
