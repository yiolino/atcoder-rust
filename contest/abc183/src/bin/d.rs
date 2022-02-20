use proconio::*;


// imos法による解放
// @sansen氏による

fn main() {
    input! {
        n: usize,
        w: i64,
        ask: [(usize, usize, i64); n],
    }

    let m = 200_000;
    let mut imos = vec![0; m + 1];

    for (l, r, p) in ask {
        imos[l] += p;
        imos[r] -= p;
    }

    for i in 1..=m {
        imos[i] += imos[i - 1];
    }

    let ans = if imos.iter().all(|v| *v <= w) {
        "Yes"
    } else {
        "No"
    };

    println!("{}", ans);
}

