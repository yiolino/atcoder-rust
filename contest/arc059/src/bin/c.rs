use proconio::input;

fn main() {
    input!{
        n: usize,
        a: [i64; n],   
    }

    let mut ans = std::i64::MAX;
    for i in -100..=100 {
        let mut tmp = 0;
        for ai in &a {
            tmp += (i - ai) * (i - ai);
        }

        ans = ans.min(tmp);
    }

    println!("{}", ans);
}
