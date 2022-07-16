use proconio::*;

#[fastout]
fn main() {
    input! {
        t: usize,
        ask: [u64; t],
    }
    for n in ask {
        let len = n.to_string().len();
        let mut ans = 0;

        for i in 2..=len {
            if len % i != 0 {
                continue;
            }

            let digits = 10u64.pow((len / i) as u32);
            let mut m = n;
            while m >= digits {
                m /= digits;
            }

            for mm in [m, m - 1].iter() {
                let mut val = 0;
                for _ in 0..i {
                    val = val * digits + mm;
                }
                if val <=n {
                    ans = ans.max(val);
                }

            }
        }

        if len > 2 {
            ans = ans.max(10u64.pow(len as u32 - 1) - 1);
        }

        println!("{}", ans);
    }
}

