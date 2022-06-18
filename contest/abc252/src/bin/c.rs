use std::collections::HashMap;

use proconio::{input, marker::Chars};

fn main() {
    input!{
        n: usize,
        tmp_s: [Chars; n],
    }

    let mut s = vec![vec![]; n];
    for (i, tmp_si) in tmp_s.into_iter().enumerate() {
        for tmp_sii in tmp_si {
            s[i].push(tmp_sii as usize - 48);
        }
    }

    let mut ans = std::usize::MAX;
    for i in 0..10 {
        let mut map = HashMap::new();
        for si in s.iter() {
            let index = si.iter().position(|&r| r == i).unwrap();
            *map.entry(index).or_insert(0) += 1;
        }

        let mut tmp = 0;
        for (k, v) in map {
            if v == 0 {continue};
            tmp = tmp.max(k + 10 * (v - 1));
        }

        ans = ans.min(tmp);
    }

    println!("{}", ans);
}
