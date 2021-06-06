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
        M: usize,
        AB: [(usize, usize); M],
    }

    let mut G = vec![vec![]; N + 1];

    for i in 0..M {
        let Ai = AB[i].0;
        let Bi = AB[i].1;

        G[Ai].push(Bi);
    }

    let mut Mat = vec![vec![false; N+1]; N+1];
    for i in 1..=N {
        Mat[i][i] = true;
    }


    for i in 1..=N {
        let mut seen = vec![false; N+1];

        dfs(&mut Mat, &G, &mut seen, i, i)
    }

    let mut ans = 0;

    for i in 1..=N {
        for j in 1..=N {
            if Mat[i][j] {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}



#[allow(non_snake_case)]
fn dfs(M: &mut Vec<Vec<bool>>, G: &Vec<Vec<usize>>, seen: &mut Vec<bool>, v: usize, consti: usize) -> () {
    seen[v] = true;

    for next_v in &G[v] {
        M[consti][*next_v] = true;
        if seen[*next_v] {
            continue;
        }

        dfs(M, G, seen, *next_v, consti)
    }
}