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
        H: i64,
        W: i64,
        field: [Chars; H],
    }

    // s と g の座標を特定する。
    let mut sh = -1;
    let mut sw = -1;
    let mut gh = -1;
    let mut gw = -1;
    for h in 0..H {
        for w in 0..W {
            if field[h as usize][w as usize] == 's' {
                sh = h; sw = w;
            }
            if field[h as usize][w as usize] == 'g' {
                gh = h; gw = w;
            }
        }
    }

    // 探索済かどうかのメモ
    let mut seen = vec![vec![false; W as usize]; H as usize];

    // 深さ優先探索
    dfs(sh, H, sw, W, &mut seen, &field);


    let ans: &str;
    if seen[gh as usize][gw as usize] {
        ans = "Yes";
    } else {
        ans = "No";
    }

    println!("{}", ans);
}


// 深さ優先探索
#[allow(non_snake_case)]
fn dfs(h: i64, H: i64, 
        w: i64, W: i64, 
        seen: &mut Vec<Vec<bool>>, field: &Vec<Vec<char>>) -> () {
    seen[h as usize][w as usize] = true;

    // 4方向への移動ベクトル；
    let dx = [1, 0, -1, 0];
    let dy = [0, 1, 0, -1];

    // 4方向を探索
    for dir in 0..4 {
        let nh = h + dx[dir];
        let nw = w + dy[dir];

        // 場外したり、移動先が壁の場合はスルー
        if nh < 0 || nh >= H || nw < 0 || nw >= W {
            continue;
        }
        if field[nh as usize][nw as usize] == '#' {
            continue;
        }

        // 移動先が探索済の場合
        if seen[nh as usize][nw as usize] {
            continue;
        }

        // 再帰的に探索
        dfs(nh, H, nw, W, seen, field)

    }
}