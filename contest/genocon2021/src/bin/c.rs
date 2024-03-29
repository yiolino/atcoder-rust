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


fn main() {
    input!{
        m: usize,
    }

    let mut score = std::usize::MAX; 
    let mut ans_seq_vec= vec![vec!['X']; m];

    let mut seq_vec = vec![];  // m個の配列を格納しておくvector
    for _ in 0..m {
        input!(si: Chars);
        seq_vec.push(si);
    }


    for k in 0..m {
        let mut tmp_seq_vec = seq_vec.clone();

        for i in 0..m {
            if i == k {
                continue;
            }
            let tmp_seq = pairwise(&tmp_seq_vec[k], &seq_vec[i]);
            tmp_seq_vec[k] = tmp_seq.0;
            tmp_seq_vec[i] = tmp_seq.1;
        }

        for i in 0..m {
            if i == k {
                continue;
            }
            tmp_seq_vec[i] = pairwise_oneside(&tmp_seq_vec[k], &seq_vec[i]);
        }

        let tmp_score = calc_ct(&tmp_seq_vec);

        if tmp_score < score {
            score = tmp_score;
            ans_seq_vec = tmp_seq_vec.clone();
        }
    }


    for i in 0..m {
        println!("{}", ans_seq_vec[i].iter().collect::<String>());
    }
}



fn pairwise(s: &Vec<char>, t: &Vec<char>) -> (Vec<char>, Vec<char>) {
    let n = s.len();
    let m = t.len();

    let gap_penalty = -1;

    let mut h = vec![vec![0; m+1]; n+1];  // スコアの行列
    let mut l = vec![vec![0; m+1]; n+1]; // アライメントを出力するためのメモ行列

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
                0
            } else {
                -1
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

    (ans_s, ans_t)
}


// 片側の配列しか伸ばせないversionのpairwise
fn pairwise_oneside(s: &Vec<char>, t: &Vec<char>) -> Vec<char> {
    let n = s.len();
    let m = t.len();

    let gap_penalty = -1;  // C問題の場合はGAP同士は損にはならない

    let mut h = vec![vec![0; m+1]; n+1];  // スコアの行列
    let mut l = vec![vec![0; m+1]; n+1]; // アライメントを出力するためのメモ行列

    h[0][0] = 0;
    for j in 1..m+1 {
        h[0][j] = h[0][j-1] - 10000; // s1が伸びる方向には行かせない。
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
            score[0] = h[i][j-1] - 10000;  // s1が伸びる方向には行かせない。

            let tmp_s = if s[i-1] == t[j-1] {
                0
            } else {
                -1
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

    ans_t.reverse();

    ans_t
}


fn calc_ct(seq_vec: &Vec<Vec<char>>) -> usize {
    let n = seq_vec.len();
    let m = seq_vec[0].len();

    let mut ans = 0;
    for i in 0..m {
        let mut score = vec![0; 5];  // 各塩基が何回登場するかのカウント

        for j in 0..n {
            match seq_vec[j][i] {
                'A' => score[0] += 1,
                'T' => score[1] += 1,
                'G' => score[2] += 1,
                'C' => score[3] += 1,
                _ => score[4] += 1,
            };
        }

        score.sort();
        let s:usize = score.into_iter()
                            .enumerate()
                            .filter(|(i, _)| i != &4)
                            .map(|(_, s)| s)
                            .sum();
        ans += s;
    }

    ans
}