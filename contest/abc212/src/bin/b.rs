#[allow(unused_imports)]
use proconio::{input, fastout, marker::Chars};
#[allow(unused_imports)]
use std::collections::{HashSet, HashMap, BTreeSet, VecDeque};
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use itertools::Itertools;

#[fastout]
fn main() {
    input!{
        x: Chars,
    }

    if x[0] == x[1] && x[1] == x[2] && x[2] == x[3] {
        println!("Weak");
        return;
    } else {
        let mut flg = true;
        for i in 0..3 {
            if x[i] as i32 - 48 != 9 {
                if x[i+1] as i32 - 48 != x[i] as i32 - 48 + 1 {
                    flg = false;
                }
            } else {
                if x[i+1] as i32 - 48 != 0 {
                    flg = false;
                }
            }
        }

        if flg {
            println!("Weak");
            return;
        }
    }

    println!("Strong");
}
