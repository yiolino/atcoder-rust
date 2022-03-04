use proconio::input;
use proconio::marker::Usize1;

const LOG : usize = 60;

fn main() {
    input! {
        n : usize,
        k : usize,
        a : [Usize1; n]
    }

    // doubling table
    // dub[i][j] = 町j の 2^i 個上の親の町
    let mut dub = vec![vec![0; n]; LOG + 1];
    dub[0] = a;  // 町iの転送先がaiであることから、iの親はaiとできる
                 // 2^0個上の親がaiであるということ

    // 行方向 ... 2^i 個上の親について考えていく
    for i in 0..LOG { 
        for j in 0..n {
            dub[i+1][j] = dub[i][dub[i][j]];
        }
    }

    // kを2進数展開すれば良い
    let mut ans = 0;
    for i in 0..LOG {
        if k & 1<<i == 0 { continue }
        ans = dub[i][ans];
    }

    println!("{}", ans + 1);
}

