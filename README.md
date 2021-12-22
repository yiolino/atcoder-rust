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
```
    shops.sort_by_key(|s| s.price);
```

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

## BTreeSet応用 昇順に並んだ配列から指定の値より1つ小さい値と1つ大きな値を取り出す。

```
// BTreeSetのある値よりも1つ小さな値、もしくは1つ大きな値を返す。
trait Neighbors<T> {
    fn before(&self, x: T) -> Option<&T>;
    fn after(&self, x: T) -> Option<&T>;
}

impl<T: Ord> Neighbors<T> for BTreeSet<T> {
    fn before(&self, x: T) -> Option<&T> {
        let mut bfr = self.range((std::ops::Bound::Unbounded, std::ops::Bound::Excluded(x)));

        bfr.next_back()
    }

    fn after(&self, x: T) -> Option<&T> {
        let mut aftr = self.range((std::ops::Bound::Excluded(x), std::ops::Bound::Unbounded));

        aftr.next()
    }
}
```


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
```
use std::cmp::Ordering;
/// Equivalent to std::lowerbound and std::upperbound in c++
/// 添字ではなく、境界としての添字を返すので注意！
pub trait BinarySearch<T> {
    fn lower_bound(&self, x: &T) -> usize;
    fn upper_bound(&self, x: &T) -> usize;
}

impl<T: Ord> BinarySearch<T> for [T] {
    fn lower_bound(&self, x: &T) -> usize {
        let mut low = 0;
        let mut high = self.len();

        while low != high {
            let mid = (low + high) / 2;
            match self[mid].cmp(x) {
                Ordering::Less => {
                    low = mid + 1;
                }
                Ordering::Equal | Ordering::Greater => {
                    high = mid;
                }
            }
        }
        low
    }

    fn upper_bound(&self, x: &T) -> usize {
        let mut low = 0;
        let mut high = self.len();

        while low != high {
            let mid = (low + high) / 2;
            match self[mid].cmp(x) {
                Ordering::Less | Ordering::Equal => {
                    low = mid + 1;
                }
                Ordering::Greater => {
                    high = mid;
                }
            }
        }
        low
    }
}
```


## 最小公倍数、最大公約数

- `num::integer::gcd(x, y)` を用いた方が良い（ゼロ除算など回避できる）

```
// ユークリッドの互助法による最大公約数
fn gcd(m:usize, n:usize) -> usize {
    // 再帰関数で実装する。
    // ベースケース
    if m % n == 0 {
        return n;
    }

    // 再帰呼び出し
    gcd(n, m % n)
}

// 最小公倍数
fn lcm(m:usize, n:usize) -> usize {
    let u = max(m, n);
    let l = min(m, n);

    u * l / gcd(u, l)
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
let popped_tail = d.pop_back().unwrap();
```


## Charcter の vector　を集めてStringにする
```
let ans = ans.into_iter().collect::<String>();
```

## usize の数字を集めてスペースでつなぎ、Stringにする。
```
let ans: String = tmp_vec
                                .into_iter()
                                .map(|x| x.to_string())
                                .join(" ");
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

## mod_pow, mod_inv
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

// 逆元の計算
fn mod_inf<T>()
```

## 2の累乗数の判定
```
// 2の累乗かどうかの判定
fn is_pow2<T>(x: T) -> bool
where
    T: num_traits::PrimInt,
{
    if x == T::zero() {
        return false
    } else {
        return (x & x - T::one()) == T::zero()
    }
}
```


## 素因数分解
```
// 素因数分解する関数
fn prime_factorize(mut n: i64) -> Vec<(i64, i64)> {
    let mut vec = vec![];

    let mut cnt = 2;
    while cnt * cnt <= n {
        if n % cnt != 0 {
            cnt += 1;
            continue;
        }

        let mut ex = 0; // 指数
        while n % cnt == 0 {
            ex += 1;
            n /= cnt;
        }

        vec.push((cnt, ex));

        cnt += 1;
    }   

    // 最後に残った数について
    if n != 1 {
        vec.push((n, 1));
    }
    
    vec
}
```


## 素数判定
```
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
```


## combination
```
use itertools::Itertools;

// nPr (n = 3, r = 2)
println!("Permutation.");
for perm in (0..3).permutations(2) {
    println!("{:?}", perm);
}

// nCr (n = 3, r = 2)
println!("\nCombination.");
for perm in (0..3).combinations(2) {
    println!("{:?}", perm);
}

// nHr (n = 3, r = 2)
println!("\nCombination with replacement.");
for perm in (0..3).combinations_with_replacement(2) {
    println!("{:?}", perm);
}
```


## 文字列の置換
```
    let s: &str = "foo foo foo";
    assert_eq!("faa faa faa", s.replace("o", "a"));
    assert_eq!("fee fee foo", s.replacen("o", "e", 4));
```

## Stringのスライスを行う
https://qiita.com/aflc/items/f2be832f9612064b12c6
```
"あいう".chars().collect::<Vec<char>>();
```


## next_permutation, prev_permutation
順列の生成
https://github.com/bluss/permutohedron/blob/master/src/lexical.rs

追記 : [superslice](https://docs.rs/superslice/1.0.0/x86_64-apple-darwin/superslice/trait.Ext.html#tymethod.next_permutation) というクレートもあった。

```
// T が Vector の場合、昇順にsortしてある必要があるので注意
pub trait LexicalPermutation {
    /// Return `true` if the slice was permuted, `false` if it is already
    /// at the last ordered permutation.
    fn next_permutation(&mut self) -> bool;
    /// Return `true` if the slice was permuted, `false` if it is already
    /// at the first ordered permutation.
    fn prev_permutation(&mut self) -> bool;
}

impl<T> LexicalPermutation for [T] where T: Ord {
    /// Original author in Rust: Thomas Backman <serenity@exscape.org>
    fn next_permutation(&mut self) -> bool {
        // These cases only have 1 permutation each, so we can't do anything.
        if self.len() < 2 { return false; }

        // Step 1: Identify the longest, rightmost weakly decreasing part of the vector
        let mut i = self.len() - 1;
        while i > 0 && self[i-1] >= self[i] {
            i -= 1;
        }

        // If that is the entire vector, this is the last-ordered permutation.
        if i == 0 {
            return false;
        }

        // Step 2: Find the rightmost element larger than the pivot (i-1)
        let mut j = self.len() - 1;
        while j >= i && self[j] <= self[i-1]  {
            j -= 1;
        }

        // Step 3: Swap that element with the pivot
        self.swap(j, i-1);

        // Step 4: Reverse the (previously) weakly decreasing part
        self[i..].reverse();

        true
    }

    fn prev_permutation(&mut self) -> bool {
        // These cases only have 1 permutation each, so we can't do anything.
        if self.len() < 2 { return false; }

        // Step 1: Identify the longest, rightmost weakly increasing part of the vector
        let mut i = self.len() - 1;
        while i > 0 && self[i-1] <= self[i] {
            i -= 1;
        }

        // If that is the entire vector, this is the first-ordered permutation.
        if i == 0 {
            return false;
        }

        // Step 2: Reverse the weakly increasing part
        self[i..].reverse();

        // Step 3: Find the rightmost element equal to or bigger than the pivot (i-1)
        let mut j = self.len() - 1;
        while j >= i && self[j-1] < self[i-1]  {
            j -= 1;
        }

        // Step 4: Swap that element with the pivot
        self.swap(i-1, j);

        true
    }

}
```


## Union-Find
https://zenn.dev/nakamurus/articles/f398b7f4d7618ea5b7eb  

```
//---------- begin union_find by @nakamurus ----------
struct UnionFind {
    par: Vec<usize>,  // 各頂点の親頂点の番号を表す。
    siz: Vec<usize>,  // 各頂点の属する根付き木の頂点数を表す
}

#[allow(unused)]
impl UnionFind {
    fn new(n: usize) -> Self {
        // 初期値は自身が自身の根出る（各木の頂点数は1）
        UnionFind {
            par: (0..n).collect(),
            siz: vec![1; n],
        }
    }


    // 根を返す。経路圧縮による効率化の実装
    fn find_root(&mut self, x: usize) -> usize {
        if self.par[x] == x {
            return x;  // xが根の場合はxを返す。
        }
        self.par[x] = self.find_root(self.par[x]);
        self.par[x]
    }


    // xとyが同じ木に属するか判定
    fn is_same(&mut self, x: usize, y: usize) -> bool {
        self.find_root(x) == self.find_root(y)
    }


    // xの木とyの木を併合する. 関数の戻り値はboolであることに注意
    fn unite(&mut self, mut x: usize, mut y: usize) -> bool {
        // x, y それぞれ根まで移動する
        x = self.find_root(x);
        y = self.find_root(y);

        // 既に同じ木だった場合は何もしない
        if x == y  {
            return false;
        }

        // uion by size（y側の木の頂点数が小さくなるようにする）
        if self.siz[x] < self.siz[y] {
            std::mem::swap(&mut x, &mut y);
        }

        // yをxの子とする
        self.par[y] = x;
        self.siz[x] += self.siz[y];
        
        true
    }


    // 
    fn return_size(&mut self, x: usize) -> usize {
        let root = self.find_root(x);
        self.siz[root]
    }
    
}
//---------- end union_find ----------
```

## 2つの文字列の辞書順
```
    input!{
        s: Chars,
        t: Chars,
    }

    let ans = if s < t {
        "Yes"
    } else {
        "No"
    };

    println!("{}", ans);
```

## しゃくとり法
[このサイト](https://scrapbox.io/pocala-kyopro/しゃくとり法) がわかりやすかった。

## スペース含めた文字列を読み込み、スペースでスプリット
```
fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();

    let ans = s.split_ascii_whitespace().join(",");


    println!("{}", ans);
}
```

