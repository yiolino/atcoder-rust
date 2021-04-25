#[allow(unused_imports)]
use proconio::{input, fastout, marker::Chars};
#[allow(unused_imports)]
use std::collections::{HashSet, HashMap, BTreeSet};
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use itertools::Itertools;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input!{
        N: usize,
        mut S: Chars,
        Q: usize,
        TAB: [[usize; 3]; Q],
    }

    let mut A = vec![];
    let mut B = vec![];
    for j in 0..N {
        A.push(S[j]);
    }
    for j in N..2*N {
        B.push(S[j]);
    }


    for i in 0..Q {
        if TAB[i][0] == 2 {
            let tmp_B = A;
            let tmp_A = B;
            A = tmp_A;
            B = tmp_B;
        
        } else {
            let Ai = TAB[i][1] - 1;
            let Bi = TAB[i][2] - 1;

            if Bi < N {
                let tmp1 = A[Ai];
                let tmp2 = A[Bi];
                A[Ai] = tmp2;
                A[Bi] = tmp1;
            } else if Ai >= N {
                let tmp1 = B[Ai - N];
                let tmp2 = B[Bi - N];
                B[Ai - N] = tmp2;
                B[Bi - N] = tmp1;
            } else {
                let tmp1 = A[Ai];
                let tmp2 = B[Bi - N];
                A[Ai] = tmp2;
                B[Bi - N] = tmp1;
            }
        }
    }

    let mut nS = vec![];

    for i in 0..N {
        nS.push(A[i]);
    }
    for i in 0..N {
        nS.push(B[i]);
    }

    let ans:String = nS.iter().collect();

    println!("{}", ans);
}
