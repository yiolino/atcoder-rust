#[allow(unused_imports)]
use proconio::{input, fastout, marker::Chars};
#[allow(unused_imports)]
use std::collections::{HashSet, HashMap, BTreeSet};
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use itertools::Itertools;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input!{
        N: usize,
        s: [String; N],
        M: usize,
        t: [String; M],
    }

    let mut map_s = std::collections::HashMap::new();
    let mut map_t = std::collections::HashMap::new();

    for si in s {
        *map_s.entry(si).or_insert(0) += 1;
    }

    for ti in t {
        *map_t.entry(ti).or_insert(0) += 1;
    }

    let s_vec: Vec<_> = map_s.iter().collect();

    let mut ans = 0;
    for (k, v) in s_vec {
        let v_t = map_t.get(k);
        let t  = match v_t {
            Some(i) => i,
            None => &0,
        };

        ans = ans.max(v - *t);

    }

    println!("{}", ans);
}
