#[allow(unused_imports)]
use proconio::{input, fastout, marker::Chars}; 
#[allow(unused_imports)]
use std::collections::HashMap;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input!{
        N: i64,
        W: [String; N],
    }

    let mut ans = "Yes";
    for i in 0..N - 1 {
        let word_1:Vec<char> = W[i as usize].chars().collect();
        let word_2:Vec<char> = W[(i + 1) as usize].chars().collect();
        let tail = word_1.last().unwrap();
        let head = &word_2[0];
        if tail != head {
            ans = "No";
            break;
        }
    }
    

    let mut map = HashMap::new();
    for w in &W {
        let count = map.entry(w).or_insert(0);
        *count += 1;
    }

    for (_k, v) in map {
        if v != 1 {
            ans = "No";
            break;
        }
    }

    println!("{}", ans);
}
