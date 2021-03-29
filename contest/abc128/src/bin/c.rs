#[allow(unused_imports)]
use proconio::{input, fastout, marker::Chars};
#[allow(unused_imports)]
use std::collections::HashSet;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input!{
        N: usize,
        M: usize,
    }

    let mut switches = vec![]; // 各電球のスイッチをHashSetとして保持しておく。
    for _i in 0..M {
        input!{
            k: usize,
        }
        let mut set = HashSet::new();
        for _j in 0..k {
            input!{
                s: usize,
            }
            set.insert(s);
        }
        switches.push(set);
    }

    input!{
        P: [usize; M],
    }

    let mut counter = 0;
    for i in 0..1<<N {
        let mut count_on = vec![0; M];  // それぞれの電球のカウントを保持しておく。

        // onのスイッチの数を電球ごとに数える。
        for j in 0..N {
            for k in 0..M {
                if i >> j & 1 > 0 && switches[k].contains(&(j+1)) {
                    count_on[k] += 1;
                }
            }
        }

        // あまりが P と同じならcounterをインクリメント
        let mut flg = true;
        for l in 0..M {
            if count_on[l] % 2 != P[l] {
                flg = false;
                break;
            }
        }
        if flg {
            counter += 1;
        }
    }

    println!("{}", counter);
}
