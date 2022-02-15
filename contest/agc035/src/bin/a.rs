use std::{collections::HashMap};

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    let mut map = HashMap::new();
    for a in a {
        *map.entry(a).or_insert(0) += 1;
    }

    let ans = match map.len() {
        0 => unreachable!(),
        1 => *map.keys().next().unwrap() == 0,
        2 => {
            let k1 = map.keys().min().unwrap();
            *k1 == 0 && map[k1] * 3 == n
        },
        3 => {
            let keys = map.keys().copied().collect::<Vec<usize>>();
            keys.iter().fold(0, |acc, x| acc ^ x) == 0 && keys.iter().all(|k| map[k] * 3 == n)
        },
        _ => false,
    };

    println!("{}", if ans {"Yes"} else {"No"});
}

