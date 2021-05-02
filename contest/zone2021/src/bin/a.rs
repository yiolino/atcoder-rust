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
    }

    let mut counter = 0;
    for i in 0..S.len() - 3 {
        if S[i] == 'Z' {
            if S[i+1] == 'O' && S[i+2] == 'N' && S[i+3] == 'e' {
                counter += 1;
            }
        } 
        
    }

    println!("{}", counter);
}
