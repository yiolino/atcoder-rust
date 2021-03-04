#[allow(unused_imports)]
use proconio::{input, fastout, marker::Chars};
#[allow(unused_imports)]
use std::collections::{HashSet, HashMap, BTreeSet};

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input!{
        N: i64,
    }

    if N < 10 {
        println!("{}", N);
        return;
    }
    
    let max: i64;
    let l = N.to_string().len() as i64;
    
    let mut n = N;
    let mut max_digit = -1;
    while n > 0 {
        n /= 10;
        if 1 <= n && n <= 9 {
            max_digit = n;
        }
    }

    if (max_digit + 1) * 10i64.pow((l - 1) as u32) - N == 1 {
        max = max_digit + 9 * (l - 1);
    } else {
        max = max_digit + 9 * (l - 1) - 1;
    }

    println!("{}", max);
}
