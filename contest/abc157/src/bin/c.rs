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

fn main() {
    input!{
        n: usize,
        m: usize,
    }

    let mut vec = vec!['a'; n];

    for _ in 0..m {
        input! {s: usize, c: char};

        if vec[s-1] == 'a' {
            vec[s-1] = c;
        } else if vec[s-1] != c {
            println!("-1");
            return;
        }
    }

    // 2桁以上かどうかで場合分け
    if n > 1 {
        if vec[0] == '0' {
            println!("-1");
            return;
        } else {
            for i in 0..n {
                if i == 0 && vec[i] == 'a' {
                    vec[0] = '1';
                } else {
                    if vec[i] == 'a' {
                        vec[i] = '0';
                    }
                }
            }

            let ans = vec.into_iter().collect::<String>();
            println!("{}", ans);
        }

    } else {
        if vec[0] == 'a' {
            vec[0] = '0';
        }

        println!("{}", vec[0]);
    }
}
