#[allow(unused_imports)]
use proconio::{input, fastout, marker::Chars};
#[allow(unused_imports)]
use std::collections::HashMap;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input!{
        S: Chars,
    }

    let mut map = HashMap::new();

    for c in ['N', 'W', 'S', 'E'].iter() {
        map.insert(c, 0);
    }

    for s in &S {
        *map.entry(s).or_default() += 1;
    }

    if *map.get(&'N').unwrap() == 0 {
        if *map.get(&'S').unwrap() != 0 {
            println!("No");
            return;
        }
    }

    if *map.get(&'W').unwrap() == 0 {
        if *map.get(&'E').unwrap() != 0 {
            println!("No");
            return;
        }
    }

    if *map.get(&'S').unwrap() == 0 {
        if *map.get(&'N').unwrap() != 0 {
            println!("No");
            return;
        }
    }

    if *map.get(&'E').unwrap() == 0 {
        if *map.get(&'W').unwrap() != 0 {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
