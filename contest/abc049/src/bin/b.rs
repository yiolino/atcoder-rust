#[allow(unused_imports)]
use proconio::{input, fastout, marker::Chars};
#[allow(unused_imports)]
use std::collections::HashMap;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input!{
        H: i64,
        W: i64,
        C: [Chars; H],
    }

    for i in 0..H {
        for j in 0..W-1 {
            print!{"{}", &C[i as usize][j as usize]};
        }
        println!{"{}", &C[i as usize][(W-1) as usize]};
        for j in 0..W-1 {
            print!{"{}", &C[i as usize][j as usize]};
        }
        println!{"{}", &C[i as usize][(W-1) as usize]};
    }
}
