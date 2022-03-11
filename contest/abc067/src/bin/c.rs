use proconio::input;

fn main() {
    input!{
        n: usize,
        a: [i64; n],
    }

    // 累積を考える。
    let mut cum = a.clone();
    cum.insert(0, 0);
    for i in 1..n+1 {
        cum[i] += cum[i - 1];
    }

    let mut min = std::i64::MAX;

    for i in 1..n {
        let x = cum[i];
        let y = cum[n] - x;

        min = min.min((x - y).abs());
    }


    println!("{}", min);
}
