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
        let mut tmp_ans = 0;
        let mut tmp_max = 0;
        for i in 0..N {
            if bit >> i & 1 > 0 {
                if a[i] <= tmp_max {
                    tmp_ans += tmp_max - a[i] + 1;
                    tmp_max += 1;
                }
            }
            tmp_max = tmp_max.max(a[i]);
        }

        ans = ans.min(tmp_ans);
    }

    println!("{}", ans);
}
