use proconio::{input, marker::{Chars, Bytes}};

fn main() {
    input!{
        s: Bytes,
        t: Bytes,
    }

    let s = run_length_encoding(s);
    let t = run_length_encoding(t);
    let mut is_ok = s.len() == t.len();

    for (si, ti) in s.into_iter().zip(t) {
        is_ok &= si.0 == ti.0 && {si.1 == ti.1 || si.1 > 1 && si.1 <= ti.1};
    }

    let ans = if is_ok {
        "Yes"
    } else {
        "No"
    };

    println!("{}", ans);
}

// ランレングス圧縮
fn run_length_encoding<T: Eq>(a: Vec<T>) -> Vec<(T, usize)> {
    let mut a = a.into_iter().map(|x| (x, 1)).collect::<Vec<_>>();
    a.dedup_by(|a, b| {
        a.0 == b.0 && {
            b.1 += a.1;
            true
        }
    });

    a
}
