#[allow(unused_imports)]
use proconio::{input, fastout, marker::Chars};
#[allow(unused_imports)]
use std::collections::{HashSet, HashMap, BTreeSet, VecDeque};
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use itertools::Itertools;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input!{
        N: i64,
    }

    for i in 0..1000001 {
        let cnd:i64 =  (i.to_string() + &i.to_string())
                    .parse()
                    .unwrap();
        if cnd > N {
            println!("{}", i - 1);
            return;
        }
    }
}
