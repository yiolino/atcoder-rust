#[allow(unused_imports)]
use proconio::{input, fastout, marker::Chars};
#[allow(unused_imports)]
use std::collections::{HashSet, HashMap, BTreeSet, BTreeMap};

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input!{
        N: i64,
    }

    let mut map = HashMap::new();

    for _ in 0..N {
        input!{Si: String}
        *map.entry(Si).or_insert(0) += 1;
    }

    let (_, max) = &map.iter().max_by_key(|entry| entry.1).unwrap();
    let mut vec =  vec![];

    for (k, v) in &map {
        if &v == max {
            vec.push(k);
        }
    }

    vec.sort();

    for v in vec {
        println!("{}", v);
    }
}
