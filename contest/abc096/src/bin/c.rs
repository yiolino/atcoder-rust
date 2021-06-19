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
        s: [Chars; h],
    }


    for i in 0..h {
        for j in 0..w {
            // 文字が # の時に前後左右のいずれかが # なら塗り潰せる。
            let mut is_ok = false;
            if s[i][j] == '#' {
                // 左
                if j > 0 && s[i][j-1] == '#' {
                    is_ok = true;
                }
                // 右
                if j < w-1 && s[i][j+1] == '#' {
                    is_ok = true;
                }
                // 上
                if i > 0 && s[i-1][j] == '#' {
                    is_ok = true;
                }
                if i < h-1 && s[i+1][j] == '#' {
                    is_ok = true;
                }

                // # の周囲に # がなければ、終了
                if !is_ok {
                    println!("No");
                    return;
                }
            }
        }
    }

    println!("Yes");
}
