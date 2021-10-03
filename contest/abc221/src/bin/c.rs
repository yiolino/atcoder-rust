#[allow(unused_imports)]
use proconio::{input, fastout, marker::Chars, marker::Usize1, marker::Bytes};
#[allow(unused_imports)]
use std::collections::{HashSet, HashMap, BTreeSet, VecDeque, BinaryHeap};
#[allow(unused_imports)]
use std::cmp::{max, min, Reverse};
#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use petgraph::unionfind::UnionFind;

#[fastout]
fn main() {
    input!{
        n: Bytes,
    }

    let mut vec = vec![];

    for ni in n {
        vec.push(ni - 48);
    }

    vec.sort();

    let mut ans = 0;

    loop {
        for i in 1..vec.len() {
            let mut a = vec![];
            let mut b = vec![];

            for j in 0..i {
                a.push(vec[j]);
            }
            for j in i..vec.len() {
                b.push(vec[j]);
            }

            if a[0] == 0 || b[0] == 0 {
                continue;
            }
    
            let aa: usize = a.into_iter()
                            .map(|x| (x+48) as char)
                            .collect::<String>()
                            .parse().unwrap();
    
            let bb: usize = b.into_iter()
                            .map(|x| (x+48) as char)
                            .collect::<String>()
                            .parse().unwrap();
    
            ans = ans.max(aa * bb);
        }



        if !vec.next_permutation() {
            break;
        }
    }

    println!("{}", ans);
}


pub trait LexicalPermutation {
    /// Return `true` if the slice was permuted, `false` if it is already
    /// at the last ordered permutation.
    fn next_permutation(&mut self) -> bool;
}

// next_permuation, prev_permutation
impl<T> LexicalPermutation for [T] where T: Ord {
    /// Original author in Rust: Thomas Backman <serenity@exscape.org>
    fn next_permutation(&mut self) -> bool {
        // These cases only have 1 permutation each, so we can't do anything.
        if self.len() < 2 { return false; }

        // Step 1: Identify the longest, rightmost weakly decreasing part of the vector
        let mut i = self.len() - 1;
        while i > 0 && self[i-1] >= self[i] {
            i -= 1;
        }

        // If that is the entire vector, this is the last-ordered permutation.
        if i == 0 {
            return false;
        }

        // Step 2: Find the rightmost element larger than the pivot (i-1)
        let mut j = self.len() - 1;
        while j >= i && self[j] <= self[i-1]  {
            j -= 1;
        }

        // Step 3: Swap that element with the pivot
        self.swap(j, i-1);

        // Step 4: Reverse the (previously) weakly decreasing part
        self[i..].reverse();

        true
    }
}