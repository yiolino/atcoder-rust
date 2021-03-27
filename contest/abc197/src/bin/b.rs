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
        H: usize,
        W: usize,
        X: usize,
        Y: usize,
        S: [Chars; H],
    }

    let mut counter = 0;
    let mut tmp = 0;

    for i in 0..W {
        if i < Y {
            if S[X-1][i] == '.' {
                tmp += 1;
            } else {
                tmp = 0;
            }
        } else {
            if S[X-1][i] == '.' {
                counter += 1;
            } else {
                break;
            }
        }
    }
    counter += tmp;

    let mut tmp = 0;

    for i in 0..H {
        if i < X {
            if S[i][Y - 1] == '.' {
                tmp += 1;
            } else {
                tmp = 0;
            }
        } else {
            if S[i][Y - 1] == '.' {
                counter += 1;
            } else {
                break;
            }
        }
    }

    counter += tmp;

    println!("{}", counter - 1);
}
