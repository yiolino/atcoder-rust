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
        a: [i64; N],
    }

    // 全ての要素が奇数になったら終わり　-> 発想を変えて、2の何乗を因数に持つかを考える。
    let mut ans = 0;

    for i in 0..N {
        let mut ai = a[i];
        while ai % 2 == 0 {
            ai /= 2;
            ans += 1;
        }
    }

    println!("{}", ans);
}