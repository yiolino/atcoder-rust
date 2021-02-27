#[allow(unused_imports)]
use proconio::{input, fastout, marker::Chars};
#[allow(unused_imports)]
use std::collections::HashMap;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input!{
        mut s: Chars,
    }

    let N = &s.len();
    let mut pos_A = 0;
    let mut pos_Z = 0;

    for (i, si) in (0i64..).zip(&s) {
        if *si == 'A' {
            pos_A = i as usize;
            break;
        }
    }

    s.reverse();

    for (i, si) in (0i64..).zip(&s) {
        if *si == 'Z' {
            pos_Z = i as usize;
            break;
        }
    }

    println!("{}", N - pos_A - pos_Z);
}
