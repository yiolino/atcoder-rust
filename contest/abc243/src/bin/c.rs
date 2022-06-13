use std::collections::BTreeMap;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
    }

    // 衝突が生じる条件を考える。
    // 1. y座標が同じ。
    // 2. 衝突「しない」条件を考えてやれば良い。

    let mut map = BTreeMap::new();
    for i in 0..n {
        input! {x: usize, y: usize};
        map.entry(y).or_insert_with(|| vec![]).push((i, x));
    }

    input! {s: Chars};

    let mut ans = "No";
    for (_y, v) in map {
        let mut l_max = 0;
        let mut r_min = std::usize::MAX;
        for (i, x) in v {
            match s[i] {
                'L' => l_max = l_max.max(x),
                'R' => r_min = r_min.min(x),
                _ => unreachable!(),
            }

            if l_max > r_min {
                ans = "Yes";
                break;
            }
        }
    }

    println!("{}", ans)
}
