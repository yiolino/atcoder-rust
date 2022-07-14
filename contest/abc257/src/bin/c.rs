use proconio::{input, marker::Chars};
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        s: Chars,
        mut w: [usize; n],
    }

    let mut adult = vec![];
    let mut child = vec![];

    for (i, si) in s.iter().enumerate() {
        if *si == '0' {
            child.push(w[i]);
        } else {
            adult.push(w[i]);
        }
    }

    w.sort();
    w.dedup();
    child.sort();
    adult.sort();

    w.insert(0, 0);
    w.push(1000_000_000 + 1);

    let mut ans = 0;
    for wi in w {
        let c_judeged_num = child.lower_bound(&wi);
        let a_judeged_num = adult.len() - adult.lower_bound(&wi);

        ans = ans.max(c_judeged_num + a_judeged_num);
    }

    println!("{}", ans);
}
