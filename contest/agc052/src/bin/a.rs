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
        T: usize,
    }

    for _ in 0..T {
        input!{
            N: usize,
            _S1: String,
            _S2: String,
            _S3: String,
        }

        let ans_1 = (0..N).map(|_| '0').collect::<String>();
        let ans_2 = (0..N).map(|_| '1').collect::<String>();

        let ans = ans_1 + &ans_2 + "0";
        
        println!("{}", ans);
    }

}
