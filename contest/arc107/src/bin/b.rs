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
        n: i64,
        k: i64,
    }

    // x = a + b
    // y = c + d
    // と考えると、x - y = k で考えれば良い。
    // 1 <= a, b, c, d <= n　なので、2 <= x, y <= 2*n

    let mut ans = 0;
    let mut x = 2;
    while x <= 2 * n {
        let y = x - k;

        if 2 <= y && y <= 2 * n {
            let tmp1 = f(n, x);
            let tmp2 = f(n, y);

            ans += tmp1 * tmp2;
        }
        
        x += 1;
    }

    println!("{}", ans);
}


fn f(n: i64, k: i64) -> i64 {
    min(k - 1, 2 * n + 1 - k)
}