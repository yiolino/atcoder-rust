use std::collections::HashMap;
use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut x_key_map = HashMap::new();
    let mut pos_key_map = HashMap::new();
    for i in 1..=n {
        x_key_map.insert(i, i);
        pos_key_map.insert(i, i);
    }

    for _ in 0..q {
        input! {x_left: usize};
        let pos_left = *x_key_map.get(&x_left).unwrap();
        let x_adj = if pos_left == n {
            *pos_key_map.get(&(pos_left - 1)).unwrap()
        } else {
            *pos_key_map.get(&(pos_left + 1)).unwrap()
        };
        let pos_adj = *x_key_map.get(&x_adj).unwrap();

        x_key_map.insert(x_left, pos_adj);
        x_key_map.insert(x_adj, pos_left);
        pos_key_map.insert(pos_left, x_adj);
        pos_key_map.insert(pos_adj, x_left);
    }

    let ans = pos_key_map
        .iter()
        .sorted_by_key(|x| x.0)
        .map(|x| *x.1)
        .collect::<Vec<usize>>()
        .iter()
        .join(" ");

    println!("{}", ans);
}
