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
        n: usize,
        m: usize,
        a: [usize; n],
        mut b: [usize; m],
    }

    b.sort();

    let mut ans = 1_000_000_000_000;

    for i in 0..n {

        let cnd_idx = b.lower_bound(&a[i]);

        if cnd_idx < m  {
            let cnd = (a[i] as i64 - b[cnd_idx] as i64).abs();
            ans = ans.min(cnd);
        }

        if cnd_idx as i64 - 1 >= 0 {
            let cnd = (a[i] as i64 - b[cnd_idx - 1] as i64).abs();
            ans = ans.min(cnd);
        }
    }

    println!("{}", ans);
}


use std::cmp::Ordering;
/// Equivalent to std::lowerbound and std::upperbound in c++
pub trait BinarySearch<T> {
    fn lower_bound(&self, x: &T) -> usize;
}

impl<T: Ord> BinarySearch<T> for [T] {
    fn lower_bound(&self, x: &T) -> usize {
        let mut low = 0;
        let mut high = self.len();

        while low != high {
            let mid = (low + high) / 2;
            match self[mid].cmp(x) {
                Ordering::Less => {
                    low = mid + 1;
                }
                Ordering::Equal | Ordering::Greater => {
                    high = mid;
                }
            }
        }
        low
    }
}


// lower_boundを使用しない方法
// #[fastout]
// fn main() {
//     input!{
//         n: usize,
//         m: usize,
//         a: [usize; n],
//         mut b: [usize; m],
//     }

//     b.sort();

//     let mut ans = 1_000_000_000_000;

//     for i in 0..n {
//         let mut lower = 0;
//         let mut upper = m - 1;

//         while upper - lower > 1 {
//             let mid = (upper + lower) / 2;
//             if b[mid] <= a[i] {
//                 lower = mid;
//             } else {
//                 upper = mid;
//             }
//         }

//         ans = ans.min( (a[i] as i64 - b[lower] as i64).abs() );

//         if lower + 1 < m {
//             ans = ans.min( (a[i] as i64 - b[lower+1] as i64).abs() );
//         }

//     }

//     println!("{}", ans);
// }
