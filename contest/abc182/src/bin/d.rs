use proconio::input;

fn main() {
    input!{
        n: usize,
        a: [i64; n],
    }

    let mut cum = vec![0; n+1];
    for i in 1..n+1 {
        cum[i] = a[i - 1];
        cum[i] += cum[i - 1];
    }

    let mut move_max = vec![std::i64::MIN; n+1];
    for i in 0..n+1 {
        move_max[i] = cum[i].max(move_max[i]);
        if i > 0 {
            move_max[i] = move_max[i].max(move_max[i - 1]);
        }
    }


    let mut now= 0;  // 現在の座標
    let mut max = 0;  // 最大座標
    let mut pre_cum_move = 0;

    for (i, ai) in a.into_iter().enumerate() {
        max = max.max(now + move_max[i + 1]);
        now += ai + pre_cum_move;
        pre_cum_move += ai;
    }

    println!("{}", max);
}
