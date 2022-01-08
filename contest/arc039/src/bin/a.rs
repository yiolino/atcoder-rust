#[allow(unused_imports)]
use proconio::{input, fastout, marker::Chars, marker::Usize1, marker::Bytes};
#[allow(unused_imports)]
use std::collections::{HashSet, HashMap, BTreeSet, VecDeque, BinaryHeap};
#[allow(unused_imports)]
use std::cmp::{max, min, Reverse};
#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use petgraph::unionfind::UnionFind;
#[allow(unused_imports)]
use superslice::Ext;
use std::char::from_digit;

fn main() {
    input!{
        a: Chars,
        b: Chars,
    }

    let mut ans = std::i32::MIN;

    for i in 1..=9 {
        let mut aa = a.clone();
        let mut bb = b.clone();
        aa[0] = from_digit(i, 10).unwrap();
        bb[0] = from_digit(i, 10).unwrap();

        ans = ans.max(a.iter().collect::<String>().parse::<i32>().unwrap()
                    - bb.iter().collect::<String>().parse::<i32>().unwrap());
        ans = ans.max(aa.iter().collect::<String>().parse::<i32>().unwrap()
                    - b.iter().collect::<String>().parse::<i32>().unwrap());
    }

    for i in 0..=9 {
        for j in 1..=2 {
            let mut aa = a.clone();
            let mut bb = b.clone();
            aa[j] = from_digit(i, 10).unwrap();
            bb[j] = from_digit(i, 10).unwrap();

            ans = ans.max(a.iter().collect::<String>().parse::<i32>().unwrap()
                        - bb.iter().collect::<String>().parse::<i32>().unwrap());
            ans = ans.max(aa.iter().collect::<String>().parse::<i32>().unwrap()
                        - b.iter().collect::<String>().parse::<i32>().unwrap());
        }
    }

    println!("{}", ans);
}
