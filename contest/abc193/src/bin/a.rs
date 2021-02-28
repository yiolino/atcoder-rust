#[allow(unused_imports)]
use proconio::{input, fastout, marker::Chars};
#[allow(unused_imports)]
use std::collections::HashMap;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input!{
        A: f64,
        B: f64,
    }

    let ans = 100.0 * (A-B) / A;

    println!("{}", ans);
}
