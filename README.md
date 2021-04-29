# atcoder-rust

# Debug環境の構築
参考
https://blog.hamadu.net/2018/08/vscode-rust.html

# TIPS

## 配列の最大値、最小値
https://stackoverflow.com/questions/58669865/how-to-get-the-minimum-value-of-a-vector-in-rust
```
let ans = dp[n - 1].iter().max().unwrap();
```

## sortしてindexを返す
https://qiita.com/osanshouo/items/71b0272cd5e156cbf5f2  

## 約数の列挙
https://algo-logic.info/divisor/


## pairでのsort
abc121-c

## 大文字、小文字の判断
c.is_uppercase(), c.is_lowercase()... c: char, return: bool

## Charsのiteration
for (i, c) in s.into_iter().enumerate() {

&Charsでiterationしたい場合　https://webbibouroku.com/Blog/Article/rust-iter-index
for (i, val) in (0_i32..).zip(a.iter()) {
    println!("{}: {}", i, val);
}

## Vecのソート, 反転
vec.sort()
vec.reverse()

## HashMap 古い値に基づいて値を更新する。
https://doc.rust-jp.rs/book-ja/ch08-03-hash-maps.html#古い値に基づいて値を更新する
https://qiita.com/hystcs/items/75183bcf38bf95cc2ce0

let mut map = std::collections::HashMap::new();
for c in "abcabc".chars() {
    *map.entry(c).or_insert(0) += 1;
}

## HashMap Keyがなければ挿入する
https://keens.github.io/blog/2020/05/23/rustnohashmaphaentrygabenri/
```
// 準備
let mut map = HashMap::<String, Vec<String>>::new();
let key = "Hoge".to_string();
let value = "Huga".to_string();

// Entry APIを使ったコード
map.entry(key).or_insert_with(|| vec![]).push(value);
```

println!("{:?}", map);  // {'c': 2, 'a': 2, 'b': 2}

## BTreeSet
https://maguro.dev/btree-maximum-value/



## 平方根
i64にはsqrtメソッドはないので、f64にキャストします。
let s = (q as f64).sqrt();


## floatのmin, maxは std::cmp::min, maxを用いることができない。
https://doc.rust-lang.org/std/primitive.f64.html#method.max
let x = 1.0_f64;
let y = 2.0_f64;

assert_eq!(x.min(y), x);

## vecの要素の比較
https://stackoverflow.com/questions/29504514/whats-the-best-way-to-compare-2-vectors-or-strings-element-by-element
fn main() {
    let a = "Hello";
    let b = "World";

    let matching = a.chars().zip(b.chars()).filter(|&(a, b)| a == b).count();
    println!("{}", matching);

    let a = [1, 2, 3, 4, 5];
    let b = [1, 1, 3, 3, 5];

    let matching = a.iter().zip(&b).filter(|&(a, b)| a == b).count();
    println!("{}", matching);
}


## combination
let vec:Vec<Vec<i64>> = (0..N).combinations(2).collect();


## 二分探索 upper_bound, lower_bound
https://github.com/hatoo/competitive-rust-snippets/blob/master/src/binary_search.rs

## 最小公倍数、最大公約数
```
// ユークリッドの互助法による最大公約数
#[allow(non_snake_case)]
fn GCD(m:i64, n:i64) -> i64 {
    // 再帰関数で実装する。
    // ベースケース
    if m % n == 0 {
        return n;
    }

    // 再帰呼び出し
    GCD(n, m % n)
}

// 最小公倍数
#[allow(non_snake_case)]
fn LCM(m:i64, n:i64) -> i64 {
    let u = max(m, n);
    let l = min(m, n);
    u * l / GCD(u, l)
}
````