use proconio::{input, marker::Chars};

fn main() {
    input!{
        n: i64,
        q: usize,
        s: Chars,
    }

    // 先頭の文字を保存しておく
    let mut top_idx = 0;
    for _ in 0..q {
        input! {t: i64, x: i64};
        match t {
            1 => {
                top_idx = (top_idx - x + n) % n;
            },
            2 => {
                let idx = (top_idx + x - 1) % n;
                println!("{}", s[idx as usize])
            },
            _ => unreachable!(),
        }
    }
}
