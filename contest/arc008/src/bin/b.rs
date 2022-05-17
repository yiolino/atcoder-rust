use std::{collections::HashMap, hash::Hash};

use proconio::{input, marker::Chars};

fn main() {
    input!{
        _n: usize,
        _m: usize,
        name: Chars,
        kit: Chars,
    }

    let mut n_map = HashMap::new();
    let mut k_map = HashMap::new();
    for n in name {
        *n_map.entry(n).or_insert(0) += 1;
    }
    for k in kit {
        *k_map.entry(k).or_insert(0) += 1;
    }

    let mut ans = 0;
    for (c, n_n) in n_map {
        if let Some(k_n) = k_map.get(&c) {
            ans = ans.max((n_n +(k_n - 1))/ k_n);
            // (a + (b - 1)) / b
        } else {
            println!("-1");
            return;
        }
    }

    println!("{}", ans);
}
