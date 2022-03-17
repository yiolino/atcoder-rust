use std::collections::HashMap;

use proconio::*;

fn main() {
    input!{
        n: usize,
        k: i64,
        a: [i64; n],
    }

    let mut cum = vec![0; n + 1];
    for i in 1..n+1 {
        cum[i] = cum[i - 1] + a[i - 1];
    }

    let mut map = HashMap::new();
    let mut ans = 0_usize;
    for i in 0..n+1 {
        ans += map.get(&cum[i]).unwrap_or(&0);
        *map.entry(cum[i] + k).or_insert(0) += 1;
    }


    println!("{}", ans);
}
