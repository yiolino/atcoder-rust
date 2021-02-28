#[allow(unused_imports)]
use proconio::{input, fastout, marker::Chars};
#[allow(unused_imports)]
use std::collections::HashMap;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input!{
        n: usize,
        sp: [(String, usize); n],
    }

    let mut spi = sp
                .into_iter()
                .enumerate()
                .map(|(i, (s, p))| ((s, std::cmp::Reverse(p)), i))
                .collect::<Vec<_>>();
    
    spi.sort();

    for (_, i) in spi {
        println!("{}", i + 1);
    }
}
