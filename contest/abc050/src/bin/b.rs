#[allow(unused_imports)]
use proconio::{input, fastout, marker::Chars};
#[allow(unused_imports)]
use std::collections::HashMap;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input!{
        N: i64, 
        vec_T: [i64; N],
        M: i64,
        mtx_PX: [[i64; 2]; M],
    }

    let mut score = 0;
    for t in &vec_T {
        score += t;
    }

    for px in &mtx_PX {
        let mut ans = score - &vec_T[(*&px[0] - 1) as usize];
        ans += &px[1];
        println!("{}", ans);
    }
}
