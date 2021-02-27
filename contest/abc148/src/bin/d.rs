#[allow(unused_imports)]
use proconio::{input, fastout, marker::Chars};
#[allow(unused_imports)]
use std::collections::HashMap;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input!{
        N: i64,
        a: [i64; N],
    }

    let mut counter = 0;
    let mut num = 1;
    for ai in &a {
        if *ai == num {
            num += 1;
        } else {
            counter += 1;
        }
    }

    if num == 1 {
        counter = -1;
    }

    println!("{}", counter);
}
