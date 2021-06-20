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
        h: usize,
        w: usize,
        a: [[usize; w]; h],
    }

    let mut memo_row_sum = vec![]; // 各行の総和のメモ
    let mut memo_col_sum = vec![]; // 各列の総和のメモ

    // 行方向の総和
    for i in 0..h {
        let tmp: usize = a[i].iter().sum();
        memo_row_sum.push(tmp);
    }

    // 列方向の総和
    for i in 0..w {
        let mut tmp = 0;
        for j in 0..h {
            tmp += a[j][i];
        }
        memo_col_sum.push(tmp);
    }

    for i in 0..h {
        let mut tmp_vec = vec![];
        for j in 0..w {
            let tmp_ans = memo_row_sum[i] + memo_col_sum[j] - a[i][j];
            tmp_vec.push(tmp_ans);
        }

        let ans: String = tmp_vec
                                .into_iter()
                                .map(|x| x.to_string())
                                .join(" ");

        println!("{}", ans);
    }
}
