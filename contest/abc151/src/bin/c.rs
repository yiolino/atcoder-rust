#[allow(unused_imports)]
use proconio::{input, fastout, marker::Chars};
#[allow(unused_imports)]
use std::collections::{HashSet, HashMap, BTreeSet};
#[allow(unused_imports)]
use std::cmp::{max, min};

#[allow(non_snake_case)]
fn main() {
    input!{
        _N: usize,
        M: usize,
    }

    let mut map: HashMap<usize, Vec<String>> = HashMap::new();

    for _i in 0..M {
        input!{
            key: usize,
            value: String,
        }
        map.entry(key).or_insert_with(|| vec![]).push(value);
    }

    let mut AC = 0;
    let mut WA = 0;

    for (_k, v) in map {
        let mut tmpWA = 0;  // 全問WAの時はカウントされないので注意
        for res in v {
            if res == "AC" {
                AC += 1;
                WA += tmpWA;
                break;
            } else {
                tmpWA += 1;
            }
        }
    }

    println!("{} {}", AC, WA);
}
