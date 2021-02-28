#[allow(unused_imports)]
use proconio::{input, fastout, marker::Chars};
#[allow(unused_imports)]
use std::collections::HashMap;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input!{
        N: i64,
        APX: [[i64; 3]; N],
    }

    let mut vec = vec![];
    vec.push(-1);

    for i in 0..N {
        if APX[i as usize][2] - APX[i as usize][0] > 0 {
            vec.push(APX[i as usize][1]);
        }
    }

    vec.sort();

    if vec.len() == 1 {
        println!("{}", -1);
        return;
    } else {
        println!("{}", vec[1]);
    }
}
