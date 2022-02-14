use proconio::*;
use std::collections::BTreeMap;


fn main() {
    input! {
        n: usize,
    }

    // メモ化再帰
    let mut map = BTreeMap::new();

    let ans = f(&mut map, n);


    println!("{}", ans)
}


fn f(memo: &mut BTreeMap<usize, usize>, n: usize) -> usize {
    // デフォルトパターン
    if let Some(&x) = memo.get(&n) {
        return x;
    }
    if n <= 4 {
        return n;
    }

    let a = f(memo, n / 2);
    let b = f(memo, (n + 1) / 2);
    let res = a * b % 998244353;
    memo.insert(n, res);
    
    res
}