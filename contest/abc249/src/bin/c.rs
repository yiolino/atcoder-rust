use std::collections::HashMap;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        k: usize,
        s: [Chars; n],
    }

    let mut ans = 0;
    for bit in 0..(1<<n) {
        let mut map = HashMap::new();
        let mut tmp_ans = 0;

        for i in 0..n {
            if (bit >> i) & 1 == 1 {
                for c in &s[i] {
                    *map.entry(c).or_insert(0) += 1;
                }
            }
        }

        for (_k, v) in map {
            if v == k {
                tmp_ans += 1;
            }
        }

        ans = ans.max(tmp_ans);
    }

    println!("{}", ans);
}
