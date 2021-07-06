#[allow(unused_imports)]
use proconio::{input, fastout, marker::Chars};
#[allow(unused_imports)]
use std::collections::{HashSet, HashMap, BTreeSet, VecDeque};
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use itertools::Itertools;

// pi
const PI: f64 = std::f64::consts::PI;

#[fastout]
fn main() {
    input!{
        t: f64,  // 周回にかかる時間
        l: f64,  // 観覧車の高さ
        x: f64,  // 直大像のx座標
        y: f64,  // 直大像のy座標
        q: usize,  // クエリ個数
        e: [f64; q],  // e分後のE8君から見た直大象の俯角
    }

    for ei in e {
        // 観覧車の座標
        let _k_x = 0.0;
        let k_y = - l/2_f64 * (2_f64*PI * ei / t).sin();
        let k_z = l/2_f64 - (l/2_f64) * (2_f64*PI * ei / t).cos();

        // 直大像と観覧車の水平距離
        let a = (x*x + (y - k_y)*(y - k_y)).sqrt();
        let b = k_z;
        
        let ans = b.atan2(a) * 180_f64 / PI;  // ラジアンを返すので、度に戻す必要がある。

        println!("{}", ans)
    }

}
