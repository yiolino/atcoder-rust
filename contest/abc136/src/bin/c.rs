#[allow(unused_imports)]
use proconio::{input, fastout, marker::Chars};
#[allow(unused_imports)]
use std::collections::{HashSet, HashMap, BTreeSet};
#[allow(unused_imports)]
use std::cmp::{max, min};

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input!{
        N: usize,
        mut H: [i64; N],
    }

    let mut prev_h = 1_000_000_000;

    for h in H.iter_mut().rev() {
        if *h > prev_h {
            *h -= 1;
        }
        if *h > prev_h {
            println!("No");
            return;
        }
        prev_h = *h;
    }

    println!("Yes");
}
