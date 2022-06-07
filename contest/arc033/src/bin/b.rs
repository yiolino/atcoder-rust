use std::collections::HashMap;
use proconio::input;

fn main() {
    input!{
        na: usize,
        nb: usize,
        a: [usize; na],
        b: [usize; nb]
    }

    let mut map = HashMap::new();
    for ai in a {
        *map.entry(ai).or_insert(0) += 1;
    }

    let mut sa_c_sb = 0_usize;
    for bi in b {
        if map.contains_key(&bi) {
            sa_c_sb += 1;
        }
    }

    println!("{}", sa_c_sb as f64 / (na + nb - sa_c_sb) as f64);
}
