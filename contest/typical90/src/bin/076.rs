#[allow(unused_imports)]
use proconio::{input, fastout, marker::Chars};
#[allow(unused_imports)]
use std::collections::{HashSet, HashMap, BTreeSet, VecDeque};
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use itertools::Itertools;

#[fastout]
fn main() {
    input!{
        n: usize,
        a: [usize; n]
    }

    let mut vec = vec![];
    let mut cnt = 0;
    vec.push(cnt);

    for i in 0..2*n {
        cnt += a[i % n];
        vec.push(cnt);
    }

    if vec[n] % 10 != 0 {
        println!("No");
        return;
    }


    for i in 0..n {
        let goal = vec[i] + vec[n]/10;
        let mut low = 0;
        let mut high = 2*n;

        while high - low > 1 {
            let mid = (low + high) / 2;
            if vec[mid]  <= goal {
                low = mid;
            } else {
                high = mid;
            }
        }

        if vec[low] == goal {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
