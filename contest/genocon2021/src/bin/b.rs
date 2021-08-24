#[allow(unused_imports)]
use proconio::{input, fastout, marker::Chars, marker::Usize1};
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
        s: Chars,
        t: Chars,
    }

    let n = s.len();
    let m = t.len();

    let gap_penalty = -5;

    let mut h = vec![vec![0; m+1]; n+1];
    let mut l = vec![vec![0; m+1]; n+1];

    h[0][0] = 0;
    for j in 1..m+1 {
        h[0][j] = h[0][j-1] + gap_penalty;
        l[0][j] = 0;
    }

    for i in 1..n+1 {
        h[i][0] = h[i-1][0] + gap_penalty;
        l[i][0] = 2;
    }

    // Horizontal:0, Diagonal:1, Vertical:2
    let mut score = vec![0; 3];

    for i in 1..n+1 {
        for j in 1..m+1 {
            score[0] = h[i][j-1] + gap_penalty;

            let tmp_s = if s[i-1] == t[j-1] {
                1
            } else {
                -3
            };

            score[1] = h[i-1][j-1] + tmp_s;
            score[2] = h[i-1][j] + gap_penalty;

            h[i][j] = *score.iter().max().unwrap();
            l[i][j] = score.iter()
                            .enumerate()
                            .max_by(|(_, value0), (_, value1)| value0.cmp(value1)).map(|(idx, _)| idx)
                            .unwrap();
        }
    }


    let mut ans_s = vec![];
    let mut ans_t = vec![];
    let mut i = n;
    let mut j = m;

    while i != 0 || j != 0 {
        if l[i][j] == 0 {
            ans_s.push('-');
            ans_t.push(t[j-1]);
            j -= 1;

        } else if l[i][j] == 1 {
            ans_s.push(s[i-1]);
            ans_t.push(t[j-1]);
            i -= 1;
            j -= 1;

        } else {
            ans_s.push(s[i-1]);
            ans_t.push('-');
            i -= 1;
        }
    }

    ans_s.reverse();
    ans_t.reverse();

    println!("{}", ans_s.into_iter().collect::<String>());
    println!("{}", ans_t.into_iter().collect::<String>());
}


