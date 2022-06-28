use proconio::input;

const M: i64 = 30;

fn main() {
    input!{
        h: [i64; 3],
        w: [i64; 3],
    }
    let mut a = vec![vec![0; 3]; 3];
    let mut ans = 0;

    // M進数の数を考える。
    let m4 = M*M*M*M;
    for s in 0..m4 {
        let mut tmp = s;
        a[0][0] = (tmp % M) + 1; tmp /= M;// 10進数の時を考えると分かりやすい
        a[0][1] = (tmp % M) + 1; tmp /= M;
        a[1][0] = (tmp % M) + 1; tmp /= M;
        a[1][1] = (tmp % M) + 1; tmp /= M;
        let mut ok = true;
        for i in 0..2 {
            a[i][2] = h[i] - a[i][0] - a[i][1];
            if a[i][2] < 1 {
                ok = false;
            }
        }
        for j in 0..3 {
            a[2][j] = w[j] - a[0][j] - a[1][j];
            if a[2][j] < 1 {
                ok = false;
            }
        }

        if a[2][0] + a[2][1] + a[2][2] != h[2] {
            ok = false;
        }

        if ok {
            ans += 1;
        }
    }    

    println!("{}", ans);
}
