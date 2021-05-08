#[allow(unused_imports)]
use proconio::{input, fastout, marker::Chars};
#[allow(unused_imports)]
use std::collections::{HashSet, HashMap, BTreeSet, VecDeque};
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use itertools::Itertools;

#[allow(non_snake_case)]
fn main() {
    input!{
        mut S: Chars,
    }

    let c_0 = S.iter()
                .filter(|&&x| x == '0')
                .collect::<Vec<_>>()
                .len();
    
    let c_1 = S.iter()
        .filter(|&&x| x == '1')
        .collect::<Vec<_>>()
        .len();

    let cnt = min(c_0, c_1) * 2;

    println!("{}", cnt)
}
