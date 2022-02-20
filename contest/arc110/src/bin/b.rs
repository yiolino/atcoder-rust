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
        t: Chars,
    }

    if n == 1 {
        if t.to_vec() == vec!['0'] {
            println!("{}", 10_000_000_000i64);
            return;
        } else {
            println!("{}", 10_000_000_000i64 * 2);
            return;
        }
    }

    let m = n / 3 + 10;

    let mut vec = vec![];
    for _ in 0..m {
        vec.push('1');
        vec.push('1');
        vec.push('0');
    }

    let mut idx = std::usize::MAX;  // [1,1,0] のどの位置から始まるか
    for i in 0..3 {
        let a = &vec[i..(i+n)].to_vec();
        if a == &t {
            idx = i;
            break;
        }
    }

    if idx == std::usize::MAX {
        println!("0");
        return;
    }

    let mut len = (idx + n) / 3;
    if (idx + n) % 3 == 0 {
        len -= 1;
    }

    let ans = 10_000_000_000 - len;


    println!("{}", ans);
}
