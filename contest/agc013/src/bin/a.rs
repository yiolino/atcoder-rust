use proconio::input;

fn main() {
    input! {
        n : usize,
        mut a : [usize; n]
    }

    // a の連続する重複を削除する
    a.dedup();
    let n = a.len();

    // 単調増加、あるいは単調減少が続く限り、indexを進める方針
    let mut ans = 0;
    let mut i = 0;

    while i < n {
        if i < n - 1 && a[i] < a[i + 1] {
            while i < n - 1 && a[i] < a[i + 1] {
                i += 1;
            }
        } else {
            while i < n - 1 && a[i] > a[i + 1] {
                i += 1;
            }
        }

        ans += 1;
        i += 1;
    }


    println!("{}", ans)
}
