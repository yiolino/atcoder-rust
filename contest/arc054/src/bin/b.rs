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
        P: f64,
    }

    // 三分探索で求める。
    let mut lower:f64 = 0.0;
    let mut upper:f64 = 10f64.powf(18f64) + 1.0;

    // 更新後の探索範囲に注意 https://kyopro.hateblo.jp/entry/2019/04/25/134128
    while upper - lower > 1.0 / 1_000_000_000.0 {
        let t1 = (2.0 * lower + upper) / 3.0;
        let t2 = (lower + 2.0 * upper) / 3.0;

        if fx(t1, P) <= fx(t2, P) {
            upper = t2;
        } else {
            lower = t1;   
        }
    }

    let ans: f64;
    if lower >= 0.0 {
        ans = fx(lower, P);
    } else {
        ans = P;
    }

    println!("{}", ans);
}


fn fx(x: f64, p: f64) -> f64 {
    x + p / 2.0f64.powf(x / 1.5)
}

// // 素直に導関数 = 0 として解を求める方法
// #[fastout]
// #[allow(non_snake_case)]
// fn main() {
//     input!{
//         P: f64,
//     }

//     let log_e_2 = 2.0_f64.ln();
//     let log_2_pe2:f64 = (P * log_e_2).log2();

//     let x:f64 = 1.5_f64 * (log_2_pe2 - 1.5_f64.log2());
    
//     let ans:f64;
//     if x < 0.0 {
//         ans = P;
//     } else {
//         ans = x + 2.0_f64.powf(- x / 1.5) * P;
//     }

//     println!("{}", ans);
// }