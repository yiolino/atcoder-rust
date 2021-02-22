#[allow(unused_imports)]
use proconio::{input, fastout, marker::Chars};
#[allow(unused_imports)]
use std::collections::HashMap;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input!{
        A: i64,
        B: i64,
    }

    let mut ans = -1;
    for i in 13..=1009 {
        if i * 8 / 100 == A && i * 10 / 100 == B {
            ans = i;
            break;
        }
    } 

    println!("{}", ans);
}
