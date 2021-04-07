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
        K: usize,
        a: [i64; N],
    }

    let mut ans = std::i64::MAX;

    // bit全探索: 1 のフラグが K 本立っている時以外はcontinue
    for bit in 0..1 << N {
        if (bit as u64).count_ones() != K as u32 {
            continue;
        }

        // フラグが立っている場所だけを取り出す
        let mut buildings = vec![];
        for i in 0..N {
            if bit >> i & 1 > 0 {
                buildings.push(a[i]);
            }
        }

        // 左から建物が見えるように、手前より低ければ高くしていく
        let mut tmp = 0;
        for i in 0..buildings.len()-1 {
            if buildings[i+1] <= buildings[i] {
                tmp += buildings[i] + 1 - buildings[i + 1];
                buildings[i+1] = buildings[i] + 1;
            }
        }
        ans = ans.min(tmp);
    }

    println!("{}", ans);
}
