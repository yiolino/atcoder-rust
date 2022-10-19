use proconio::input;

fn main() {
    input!{
        n: usize,
        a: [usize; n],
        q: usize,
    }

    let mut cum = vec![0; n + 1];
    for (i, ai) in a.iter().enumerate() {
        if *ai == 1 {
            cum[i + 1] = cum[i] + 1;
        } else {
            cum[i + 1] = cum[i]
        }
    }

    for _ in 0..q {
        input! {l: usize, r: usize};
        let num_atari = cum[r] - cum[l - 1];
        let ans = if num_atari > r - l + 1 - num_atari {
            "win"
        } else if num_atari == r - l + 1 - num_atari {
            "draw"
        } else {
            "lose"
        };

        println!("{}", ans);
    }
}
