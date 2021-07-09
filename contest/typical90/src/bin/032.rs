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
        a: [[usize; n]; n],
        m: usize,
        xy: [[usize; 2]; m],
    }

    let mut kenaku = vec![vec![false; 12]; 12];
    for i in 0..m {
        let xi = xy[i][0] - 1;
        let yi = xy[i][1] - 1;

        kenaku[xi][yi] = true;
        kenaku[yi][xi] = true;
    }


    let mut ans = std::usize::MAX;

    // nの順列を作るためのvec
    let mut vec = vec![];
    for i in 0..n {
        vec.push(i);
    }

    loop {
        let mut flag = true;
        let mut sum: usize = 0;

        for i in 0..n-1 {
            if kenaku[vec[i]][vec[i+1]] == true {
                flag = false;
            }
        }

        for i in 0..n {
            sum += a[vec[i]][i];
        }

        if flag {
            ans = ans.min(sum);
        }

        if !vec.next_permutation() {
            break;
        }
    }


    if ans == std::usize::MAX {
        println!("-1");
        return;
    }

    println!("{}", ans);
}


pub trait LexicalPermutation {
    /// Return `true` if the slice was permuted, `false` if it is already
    /// at the last ordered permutation.
    fn next_permutation(&mut self) -> bool;
    /// Return `true` if the slice was permuted, `false` if it is already
    /// at the first ordered permutation.
    fn prev_permutation(&mut self) -> bool;
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

    fn prev_permutation(&mut self) -> bool {
        // These cases only have 1 permutation each, so we can't do anything.
        if self.len() < 2 { return false; }

        // Step 1: Identify the longest, rightmost weakly increasing part of the vector
        let mut i = self.len() - 1;
        while i > 0 && self[i-1] <= self[i] {
            i -= 1;
        }

        // If that is the entire vector, this is the first-ordered permutation.
        if i == 0 {
            return false;
        }

        // Step 2: Reverse the weakly increasing part
        self[i..].reverse();

        // Step 3: Find the rightmost element equal to or bigger than the pivot (i-1)
        let mut j = self.len() - 1;
        while j >= i && self[j-1] < self[i-1]  {
            j -= 1;
        }

        // Step 4: Swap that element with the pivot
        self.swap(i-1, j);

        true
    }

}