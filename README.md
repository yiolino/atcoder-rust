# atcoder-rust

# Debug環境の構築
参考
https://blog.hamadu.net/2018/08/vscode-rust.html

# TIPS

## 配列の最大値、最小値
`let mut v = vec![5, 6, 8, 4, 2, 7];`
https://stackoverflow.com/questions/58669865/how-to-get-the-minimum-value-of-a-vector-in-rust

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

## Vecのソート, 反転
vec.sort()
vec.reverse()

## HashMap 古い値に基づいて値を更新する。
https://doc.rust-jp.rs/book-ja/ch08-03-hash-maps.html#古い値に基づいて値を更新する
use std::collections::HashMap;

let text = "hello world wonderful world";

let mut map = HashMap::new();

for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
}

println!("{:?}", map);


## 平方根
i64にはsqrtメソッドはないので、f64にキャストします。
let s = (q as f64).sqrt();


## floatのmin, maxは std::cmp::min, maxを用いることができない。
https://doc.rust-lang.org/std/primitive.f64.html#method.max
let x = 1.0_f64;
let y = 2.0_f64;

assert_eq!(x.min(y), x);