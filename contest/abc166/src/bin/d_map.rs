use std::collections::{HashSet, HashMap};
use proconio::input;

fn main() {
    input!{
        x: i64,
    }

    let mut map = HashMap::new();
    for i in -1000..1000 {
        map.insert(i * i * i * i * i, i);
    }

    for (a5, a) in &map {
        let b5 = a5 - x;
        match map.get(&b5) {
            Some(b) => {
                println!("{} {}", a, b);
                return;
            },
            None => (),
        }
    }
}
