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
        A: i64,
        B: i64,
        W: i64,
    }

    let mut min = W * 1000 / B;
    let mut max = W * 1000 / A;

    let mut flg_min = false;
    let mut flg_max = false;

    for _ in 0..=(max - min) {
        for i in 0..=min {
            let tmp = A * 0 + B * (min - i);
            if tmp == W * 1000 {
                flg_min = true;
                break;
            }
        }

        for i in 0..=max {
            let tmp = A * max + B * (max - i);
            if tmp == W * 1000 {
                flg_max = true;
                break;
            }
        }

        min += 1;
        max -= 1;
    }

    if flg_min && flg_max {
        println!("{} {}", min, max);
    } else {
        println!("UNSATISFIABLE");
    }
}
