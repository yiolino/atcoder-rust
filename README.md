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
```
for (i, c) in s.into_iter().enumerate() {
```

&Charsでiterationしたい場合　https://webbibouroku.com/Blog/Article/rust-iter-index
```
for (i, val) in (0_i32..).zip(a.iter()) {
    println!("{}: {}", i, val);
}
```

## Vecのソート, 反転
```
vec.sort()
vec.reverse()
```

## HashMap 古い値に基づいて値を更新する。
https://doc.rust-jp.rs/book-ja/ch08-03-hash-maps.html#古い値に基づいて値を更新する
https://qiita.com/hystcs/items/75183bcf38bf95cc2ce0
```
let mut map = std::collections::HashMap::new();
for c in "abcabc".chars() {
    *map.entry(c).or_insert(0) += 1;
}
```

## HashMap Keyがなければ挿入する
https://keens.github.io/blog/2020/05/23/rustnohashmaphaentrygabenri/
```
// 準備
let mut map = HashMap::<String, Vec<String>>::new();
let key = "Hoge".to_string();
let value = "Huga".to_string();

// Entry APIを使ったコード
map.entry(key).or_insert_with(|| vec![]).push(value);

println!("{:?}", map);  // {'c': 2, 'a': 2, 'b': 2}
```

## BTreeSet
https://maguro.dev/btree-maximum-value/



## 平方根
i64にはsqrtメソッドはないので、f64にキャストします。
```
let s = (q as f64).sqrt();
```

## floatのmin, maxは std::cmp::min, maxを用いることができない。
https://doc.rust-lang.org/std/primitive.f64.html#method.max
```
let x = 1.0_f64;
let y = 2.0_f64;

assert_eq!(x.min(y), x);
```

## vecの要素の比較
https://stackoverflow.com/questions/29504514/whats-the-best-way-to-compare-2-vectors-or-strings-element-by-element
```
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
```

## combination
```
let vec:Vec<Vec<i64>> = (0..N).combinations(2).collect();
```

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
```


## deque 
https://qiita.com/wotsushi/items/4a6797f52080453a0440#deque

```
let mut d = std::collections::VecDeque::new();

# dequeの先頭に要素を追加する
#（例）deque d の先頭に4を追加する
d.push_front(4);

# deque d の末尾に2を追加する
d.push_back(2);

# deque d の先頭要素を削除し、削除した要素をpopped_headに束縛する。
let popped_head = d.pop_front().unwrap();

# deque d の末尾要素を削除し、削除した要素をpopped_tail に束縛する
```
let popped_tail = d.pop_back().unwrap();
```


## Charcter の vector　を集めてStringにする
```
let ans = ans.into_iter().collect::<String>();
```

## パスカルの三角形
```
    // パスカルの三角形をテーブルとして作る
    let mut comb = vec![vec![0; 61]; 61];  // 可変配列の確保
    comb[0][0] = 1_i64;
    for i in 0..60 {
        for j in 0..i+1 {
            comb[i+1][j] += comb[i][j];
            comb[i+1][j+1] += comb[i][j];
        }
    }
```

## mod_pow
https://qiita.com/drken/items/3b4fdf0a78e7a138cd9a#4-累乗-an
```
// mod_powの実装。型がprimitiveなら何でも取れるようにする。
// a: 底, n: 指数, m: mod
fn mod_pow<T>(mut a: T, mut n: T, m: T) -> T 
where
    T: num_traits::PrimInt,
{   
    let mut res = T::one();
    while n > T::zero() {
        if n & T::one() == T::one() {
            res = res * a % m;
        }
        a = a * a % m;
        n = n >> 1;
    }

    res
}
```