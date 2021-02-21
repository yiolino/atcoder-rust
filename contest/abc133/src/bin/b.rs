#[allow(unused_imports)]
use proconio::{input, fastout, marker::Chars};
#[allow(unused_imports)]
use std::collections::HashMap;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input!{
        N: i64,
        D: i64,
        X:[[i64; D]; N],
    }

    let mut count = 0;

    for i in 0..N {
        for j in (i + 1)..N {
            let dist = calc_square_dist(&X[i as usize], &X[j as usize]);
            if judge_squre_num(dist) {
                count += 1;
            }
        }
    }

    println!("{}", count);
}


#[allow(non_snake_case)]
fn calc_square_dist(p1: &Vec<i64>, p2: &Vec<i64>) -> i64 {
    let l = p1.len();

    let mut dist = 0;
    for i in 0..l {
        dist += (p1[i as usize] - p2[i as usize]).pow(2);
    }

    dist
}

#[allow(non_snake_case)]
fn judge_squre_num(N: i64) -> bool {
    let mut k: i64 = 1;
    let mut ans = false;
    let rootN = (N as f64).sqrt();
    while k as f64 <= rootN {
        if k.pow(2) == N {
            ans = true;
        }
        k += 1;
    }

    ans
}