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
        lr: [[usize; 4]; n],
    }

    // いもす法で解く
    // https://imoz.jp/algorithms/imos_method.html

    // 二次元平面の領域は最大で 1000 * 1000 なので いもす法に合わせて1001 * 1001 で作っておく
    let mut mat = vec![vec![0_i32; 1001]; 1001];

    // いもす法に則り、矩形内左上, 矩形外右下に+1, 矩形外右上、矩形外左下に-1を入れていく
    for lr_i in lr {
        let lx = lr_i[0];
        let ly = lr_i[1];
        let rx = lr_i[2];
        let ry = lr_i[3];

        mat[ly][lx] += 1;
        mat[ry][rx] += 1;
        mat[ly][rx] -= 1;
        mat[ry][lx] -= 1;
    }

    // 横方向の累積和
    for i in 0..1000 {
        let mut tmp_sum = 0;

        for j in 0..1000 {
            tmp_sum += mat[i][j];
            mat[i][j] = tmp_sum;
        }
    }

    // 縦方向の累積和
    for i in 0..1000 {
        let mut tmp_sum = 0;

        for j in 0..1000 {
            tmp_sum += mat[j][i];
            mat[j][i] = tmp_sum;
        }
    }

    let mut ans = vec![0; n+1];

    for i in 0..1000 {
        for j in 0..1000 {
            let idx = mat[i][j] as usize;
            ans[idx] += 1;
        }
    }

    for i in 1..=n {
        println!("{}", ans[i]);
    }
}
