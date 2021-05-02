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
        S: Chars,
    }

    let mut T = VecDeque::new();
    let mut reverse = false;

    for c in S {
        match c {
            'R' => reverse = !reverse,

            _ => {
                if reverse {
                    if let Some(&front) = T.front() {
                        if front == c {
                            T.pop_front();
                            continue;
                        }
                    }
                    T.push_front(c);

                } else {
                    if let Some(&last) = T.back() {
                        if last == c {
                            T.pop_back();
                            continue;
                        }
                    }
                    T.push_back(c);
                }
            }
        }
    }

    let iter = if reverse {
            T.into_iter().rev().collect::<Vec<_>>()
        } else {
            T.into_iter().collect::<Vec<_>>()
        };

    let ans = iter.into_iter().collect::<String>();

    print!("{}", ans);
}
