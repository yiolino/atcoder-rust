use proconio::input;

fn main() {
    input!{
       n: usize, 
    }

    let mut a = vec![];
    let mut b = vec![];
    for _ in 0..n {
        input! {ai: i64, bi: i64};
        a.push(ai);
        b.push(bi);
    }

    let mut ans = 0;
    for i in (0..n).rev() {
        let amari = (a[i] + ans) % b[i];
        if amari != 0 {
            ans += b[i] - amari;
        }
    }

    println!("{}", ans);
}
