#[allow(unused_imports)]
use proconio::{input, fastout, marker::Chars};
#[allow(unused_imports)]
use std::collections::{HashSet, HashMap, BTreeSet};

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input!{
        N: usize,
        S: Chars,
    }

    let mut counter = 0;
    for i in 0..1000 {
        let mut pattern = vec![];
        pattern.push(i % 10);
        pattern.push((i / 10) % 10);
        pattern.push((i / 100) % 10);

        let mut idx = 0;
        for j in 0..N {
            if S[j] as i64 - 48 == pattern[idx] {
                idx += 1;
                if idx == 3 {
                    break;
                }
            }
        }

        if idx == 3 {
            counter += 1;
        }
    }

    println!("{}", counter);

    // let mut counter = 0;
    // let numbers = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    // // 全部で10^3の1000通りしかない。
    // // 1000通り全てに対して有り得るかどうかを確認すればよい。
    
    // for c1 in &numbers {
    //     for c2 in &numbers {
    //         for c3 in &numbers {
    //             let mut index = vec![-1, -1, -1];
    //             let mut tmp_idx = 0;
    //             for i in 0..=N-3 {
    //                 if *c1 == S[i] {
    //                     index[0] = i as i64;
    //                     tmp_idx = i;
    //                     break;
    //                 }
    //             }
    //             for i in tmp_idx+1..=N-2 {
    //                 if *c2 == S[i] {
    //                     index[1] = i as i64;
    //                     tmp_idx= i;
    //                     break;
    //                 }
    //             }
    //             for i in tmp_idx+1..=N-1 {
    //                 if *c3 == S[i] {
    //                     index[2] = i as i64;
    //                     break;
    //                 }
    //             }

    //             if index[0] < index[1] && index[1] < index[2] && index[0] >= 0 {
    //                 counter += 1;
    //             }

    //         }
    //     }
    // }


    // println!("{}", counter);
}
