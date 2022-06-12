use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars,
    }

    let n = s.len();
    let mut ans = vec![0; n];

    // RRR..RLLL の並びを考えた時に R->L の切れ目を考えてやると良い。
    // RRRの子供は R->L のどちらかに集約される。 10^100は偶数回なので
    // indexの偶奇で考えてやると良い。
    // Lについては全てを逆からみて考えてやると等価
    for _ in 0..2 {
        let mut cnt = 0_usize;
        for (i, si) in s.iter().enumerate() {
            match si {
                'L' => {
                    ans[i - 1] += (cnt + (2 - 1)) / 2;
                    ans[i] += cnt / 2;
                    cnt = 0;
                }
                'R' => cnt += 1,
                _ => unreachable!(),
            }
        }

        s.reverse();
        ans.reverse();
        for si in s.iter_mut() {
            match si {
                'L' => *si = 'R',
                'R' => *si = 'L',
                _ => unreachable!(),
            }
        }
    }

    println!("{}", ans.iter().join(" "));
}
