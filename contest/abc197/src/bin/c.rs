#[allow(unused_imports)]
use proconio::{input, fastout, marker::Chars};
#[allow(unused_imports)]
use std::collections::{HashSet, HashMap, BTreeSet};
#[allow(unused_imports)]
use std::cmp::{max, min};

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input!{
        N: usize,
        A: [usize; N],
    }

    let mut ans = std::usize::MAX;

    for i in 0..1 << N {
        let mut vec = vec![];

        let mut x = 0;
        for j in 0..N {
            x |= A[j];
            if i & 1 << j != 0 {
                vec.push(x);
                x = 0;
            }
        }
        vec.push(x);

        let x = vec.into_iter().fold(0, |x, y| x ^ y);
        ans = ans.min(x);
    }

    println!("{}", ans);
}
