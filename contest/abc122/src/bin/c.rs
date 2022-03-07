use proconio::{input, marker::{Chars, Usize1}};

fn main() {
    input!{
        n:usize,
        q: usize,
        s: Chars,
    }

    let mut cum = vec![0; n + 1];
    let mut mark_bet = vec![0; n + 1];  // AとCの間のチェック;
    for (i, w) in s.windows(2).enumerate() {
        if w[0] == 'A' && w[1] == 'C' {
            mark_bet[i + 1] = 1;
            cum[i + 2] = 1;
        }
    }

    for i in 1..cum.len() {
        cum[i] += cum[i - 1]
    }

    for _ in 0..q {
        input! {l: Usize1, r: Usize1};
        let mut ans: usize = cum[r + 1] - cum[l];
        if mark_bet[l] == 1 {
            ans -= 1;
        }

        println!("{}", ans)
    }
}
