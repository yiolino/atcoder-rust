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

println!("{:?}", map);  // {'c': 2, 'a': 2, 'b': 2}



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
