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
        S: Chars,
    }

    let n = S.len();
    let mut max_len = 0;

    for i in 0..n {
        for j in i..n{
            let mut tmp_len = 0;
            for k in i..=j {
                if S[k] == 'A' || S[k] == 'C' || S[k] == 'G' || S[k] == 'T' {
                    tmp_len += 1;
                } else {
                    tmp_len = 0;
                    break;
                }
            }
            max_len = max_len.max(tmp_len);
        }
    }

    // for i in 0..1 << n+1 {
    //     let mut idx = 0;
    //     for j in 0..n+1 {
    //         let mut tmp_len = 0;
    //         if (i >> j) & 1 > 0 {
    //             for k in idx..j {
    //                 if S[k] == 'A' || S[k] == 'C' || S[k] == 'G' || S[k] == 'T' {
    //                     tmp_len += 1;
    //                 } else {
    //                     tmp_len = 0;
    //                     break;
    //                 }
    //             }
    //             max_len = max_len.max(tmp_len);
    //             idx = j;
    //         }

    //     } 
    // }

    println!("{}", max_len);
}
