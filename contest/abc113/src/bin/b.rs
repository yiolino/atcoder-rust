#[allow(unused_imports)]
use proconio::{input, fastout, marker::Chars};
#[allow(unused_imports)]
use std::collections::HashMap;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input!{
        N:i64,
        T:f64,
        A:f64,
        H:[f64; N],
    }

    let mut min = 10000.0;
    let mut p:usize = 0;

    for (i, h) in H.iter().enumerate() {
        let temp = T - h * 0.006;
        if (A - temp).abs() < (A - min).abs() {
            min = temp;
            p = i + 1;
        }
    }

    println!("{}", p);
}
