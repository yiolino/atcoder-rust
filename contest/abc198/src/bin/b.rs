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
        mut vec: Chars,
    }

    vec.reverse();

    let mut point = -1;
    for i in 0..vec.len() {
        if vec[i] != '0' {
            point = i as i64;
            break;
        }
    }

    let mut vec_new = vec![];
    if point > 0 {
        for i in (point as usize)..vec.len() {
            vec_new.push(vec[i]);
        }
    } else {
        vec_new = vec;
    }

    let mut ans = "Yes";
    let n = vec_new.len();
    for i in 0..n {
        if vec_new[i] != vec_new[n - 1 - i] {
            ans = "No";
            break;
        }
    }

    println!("{}", ans);
}
