#[allow(unused_imports)]
use proconio::{input, fastout, marker::Chars};
#[allow(unused_imports)]
use std::collections::{HashSet, HashMap, BTreeSet, VecDeque};
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use itertools::Itertools;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input!{
        N: usize,
        C: [[i64; N]; N],
    }

    // Aの最小値は0で固定する
    // 条件を満たす列が存在する場合を考える。
    // C[i][1]列に限定して考えてみると、min(C[i][1]) = 0 + B[1]となる。
    // よって、B[1]が求まる。続いてA[1]が決められる。
    // これを繰り返していけば良い。

    let mut A = vec![];
    let mut B = vec![];

    for j in 0..N {
        let bj = (0..N).map(|i| C[i][j]).min().unwrap(); 
        B.push(bj);
        A.push(C[j][j] - bj);
    }

    for i in 0..N {
        for j in 0..N {
            if C[i][j] != A[i] + B[j] {
                println!("No");
                return;
            }
        }
    }

    println!("Yes");

    for i in 0..N {
        if i == N - 1 {
            println!("{}", A[i]);
        } else {
            print!("{}", A[i]);
            print!(" ");
        }
    }

    for i in 0..N {
        if i == N - 1 {
            println!("{}", B[i]);
        } else {
            print!("{}", B[i]);
            print!(" ");
        }
    }
}
