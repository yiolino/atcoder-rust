#[allow(unused_imports)]
use proconio::{input, fastout, marker::Chars};
#[allow(unused_imports)]
use std::collections::{HashSet, HashMap, BTreeSet, VecDeque};
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use itertools::Itertools;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input!{
        N: i64,
        X: [i64; N],
    }

    // Xの要素ごとに素因数分解した物を格納
    let mut factors:Vec<Vec<i64>> = vec![];
    for x in &X {
        let mut f = vec![];  // 素数格納Vec
        let mut n = *x;  // nにコピー
        for i in 2..=*x {
            if i * i > n {
                break;  // root(n)までで十分
            } else if n % i != 0 {
                continue;  // 割り切れなければskip
            } else {
                f.push(i);
            }

            // 割り切れるまで回す
            while n % i == 0 {
                n /= i;
            }
        }

        // 最後に残ったのが1出なければpushする必要がある
        if n > 1 {
            f.push(n);
        }

        factors.push(f);
    }

    // 2から50以下の全ての素数を格納
    let mut primes:Vec<i64> = vec![];
    for i in 2..=50 {
        if is_prime(i) {
            primes.push(i);
        }
    }

    let mut ans = std::i64::MAX;

    // bit全探索でprimesの掛け合わせて互いに素かどうかを判定していく
    for i in 0..1<<primes.len() {
        let mut cnd = 1;
        for j in 0..primes.len() {
            if i>>j & 1 > 0 {
                cnd *= primes[j];
            }
        }

        let mut flg = true;
        for factor in &factors {
            let mut flg2 = false;
            for f in factor {
                if cnd % f == 0 {
                    flg2 = true;
                }
            }

            if !flg2 {
                flg = false;
                break;
            }
        }

        if flg {
            ans = ans.min(cnd);
        }
    }


    println!("{}", ans);
}


// 素数判定の関数
fn is_prime(n: i64) -> bool {
    if n == 1 {
        return false;
    }

    for j in 2..=n {
        if j * j > n {
            break;
        }

        if n % j == 0 {
            return false;
        }
    }

    true
}