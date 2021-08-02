#[allow(unused_imports)]
use proconio::{input, fastout, marker::Chars};
#[allow(unused_imports)]
use std::collections::{HashSet, HashMap, BTreeSet, VecDeque, BinaryHeap};
#[allow(unused_imports)]
use std::cmp::{max, min, Reverse};
#[allow(unused_imports)]
use itertools::Itertools;

#[fastout]
fn main() {
    input!{
        q: usize,
    }

    let mut heap = BinaryHeap::new();
    let mut sum = 0;

    for _ in 0..q {
        input!{
            p: usize,
        }

        if p == 1 {
            input!{
                x: i64,
            }
            heap.push(Reverse(x - sum));
        } else if p == 2 {
            input!{
                x: i64,
            }
            sum += x;
        } else {
            let Reverse(m) = heap.pop().unwrap();
            println!("{}", m + sum);
        }
    }
}
