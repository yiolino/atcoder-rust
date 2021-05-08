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
        mut N: i64,
        K: i64,
    }

    for _i in 0..K {
        if N % 200 == 0 {
            N /= 200;
        } else {
            let str_N = N.to_string();
            let str_200 = "200";
            N = (str_N + str_200).parse().unwrap();
        }
    }


    println!("{}", N);
}
