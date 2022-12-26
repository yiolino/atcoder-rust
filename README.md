# atcoder-rust

# Debug環境の構築
参考
https://blog.hamadu.net/2018/08/vscode-rust.html

# TIPS

# 教育的アルゴリズムリポジトリ
https://github.com/TheAlgorithms/Rust

## ランレングス圧縮
from https://atcoder.jp/contests/abc259/submissions/33076989
```
fn run_length_encoding<T: Eq>(a: Vec<T>) -> Vec<(T, usize)> {
    let mut a = a.into_iter().map(|a| (a, 1)).collect::<Vec<_>>();
    a.dedup_by(|a, b| {
        a.0 == b.0 && {
            b.1 += a.1;
            true
        }
    });
    a
}
```

## XORの性質
3 つ以上の整数の XOR の計算は自由な順序で行える
a ⊕ b ⊕ c = b ⊕ c ⊕ a = (a ⊕ b) ⊕ c = c ⊕ (a ⊕ b)
a ⊕ b ⊕ a = (a ⊕ a) ⊕ b = 0 ⊕ b = b
参考：abc171 

競技プログラミングにおけるXORのTips
https://qiita.com/kuuso1/items/778acaa7011d98a3ff3a

## 二分探索
upper_bound, lower_bound
https://docs.rs/superslice/1.0.0/superslice/trait.Ext.html

## 三分探索
https://kyopro.hateblo.jp/entry/2019/04/25/134128

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

    # keyが複数の場合はtupleをkeyとする。
    vec.sort_by_key(|(time, p)| (*time, *p));
```

## fload型のsort
```
sort_by(|a, b| a.partial_cmp(b).unwrap())
```

## 降順でのsort
https://stackoverflow.com/questions/60916194/how-to-sort-a-vector-in-descending-order-in-rust
```
use std::cmp::Reverse;
vec.sort_by_key(|w| Reverse(*w));
```

## 大文字、小文字の判断
c.is_uppercase(), c.is_lowercase()... c: char, return: bool

## Charsのiteration
```
for (i, c) in s.into_iter().enumerate() {
```

&Charsでiterationしたい場合 https://webbibouroku.com/Blog/Article/rust-iter-index
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

## HashMap value が vectorの場合
```
    let mut memo = HashMap::new();

    for (i, ai) in a.iter().enumerate() {
        memo.entry(*ai)
            .or_insert(vec![])
            .push(i as i32 + 1);
    }

    for _ in 0..q {
        input! {x: usize, k: usize};
        let ans = memo.get(&x)
                    .map_or(-1, |p| p.get(k - 1)
                                                            .cloned()
                                                            .unwrap_or(-1));

        println!("{}", ans);
    }
```

## BTreeSet
最大値と最小値を取り出す
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

## vecの要素のindexを取得
https://stackoverflow.com/questions/30558246/how-do-i-find-the-index-of-an-element-in-an-array-vector-or-slice
```
fn main() {
    let test = vec!["one", "two", "three"];
    let index = test.iter().position(|&r| r == "two").unwrap();
    println!("{}", index);
}
```


## combination
```
let vec:Vec<Vec<i64>> = (0..N).combinations(2).collect();
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


## Charcter の vectorを集めてStringにする
```
let ans = ans.into_iter().collect::<String>();
```

## usize の数字を集めてスペースでつなぎ、Stringにする。
```
// use itertools::Itertools; をインポートしておく
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

## Uniof-Find 連結成分
```
use petgraph::unionfind::UnionFind;

fn main() {
    input!{
        n: usize,
        m: usize,
    }

    let mut uf = UnionFind::new(n);

    for _ in 0..m {
        input! {mut a: usize, mut b: usize};
        a -= 1;
        b -= 1;

        uf.union(a, b);
    }

    let set: HashSet<_> = uf.into_labeling()
                            .into_iter()
                            .collect();

    println!("{:?}", set.len() - 1);
}
```

## 2つの文字列の辞書順
```
    input!{#[allow(unused_imports)]
use proconio::{input, fastout, marker::Chars, marker::Usize1, marker::Bytes};
#[allow(unused_imports)]
use std::collections::{HashSet, HashMap, BTreeSet, VecDeque, BinaryHeap};
#[allow(unused_imports)]
use std::cmp::{max, min, Reverse};
#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use petgraph::unionfind::UnionFind;
#[allow(unused_imports)]
use superslice::Ext;

fn main() {
    input!{
        mut s: Chars,
        a: usize,
        b: usize,
    }

    let aa = s[a-1];
    let bb = s[b-1];

    s[a-1] = bb;
    s[b-1] = aa;

    println!("{}", s.iter().join(""));
}

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
[このサイト](htt`ps://scrapbox.io/pocala-kyopro/しゃくとり法) がわかりやすかった。


## スペース含めた文字列を読み込み、スペースでスプリット
```
fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();

    let ans = s.split_ascii_whitespace().join(",");


    println!("{}", ans);
}
```


## 座標圧縮
```
fn main() {
    input!{ 
        _h: usize,
        _w: usize,
        p: [(u32, u32)],
    }

    // 座標圧縮で解く
    let (mut x, mut y): (Vec<_>, Vec<_>) = p.iter().cloned().unzip();

    x.sort();
    x.dedup(); // 連続した重複要素を除去

    y.sort();
    y.dedup();

    // superslice crate の Ext::lowerboundを使う
    for (a, b) in p {
        let a = x.lower_bound(&a) + 1;
        let b = y.lower_bound(&b) + 1;
        println!("{} {}", a, b);
    }
}
```

## rotate_right
https://doc.rust-lang.org/std/primitive.slice.html#method.rotate_right
```
let mut a = ['a', 'b', 'c', 'd', 'e', 'f'];
a.rotate_right(2);
assert_eq!(a, ['e', 'f', 'a', 'b', 'c', 'd']);
```


## 2進数変換
```
    while k > 0 {
        if k % 2 == 1 {
            ans.push(2);
        } else {
            ans.push(0);
        }
        k /= 2;
    }
```

## 優先度付きキュー
常に最大値 or 最小値を取り出せるデータ構造
```
// abc234 D
// 最小値を取り出す構造にするためには Reverse を用いる

    let mut heap = BinaryHeap::new();

    for i in 0..k {
        heap.push(Reverse(p[i]));
    }

    println!("{}", heap.peek().unwrap().0);

    for i in k..n {
        if heap.peek().unwrap().0 < p[i] {
            heap.pop();
            heap.push(Reverse(p[i]));
            println!("{}", heap.peek().unwrap().0);
        } else {
            println!("{}", heap.peek().unwrap().0);
        }
    }
}
```

## ゼロ埋めformat
https://zenn.dev/toga/books/rust-atcoder/viewer/13-format
2 進法，8 桁（0 埋め）
```
println!("{:08b}", 1_u8);
```

## bit反転
「!」をつけるとbit反転する
```
println!("{:08b} {:08b}", 1_u8, !1_u8);
// 00000001 11111110
```

## 2進数にしたときの桁数を求める
https://zenn.dev/anozon/articles/rust-bit-len
```
fn blen(v: i64) -> u32 {
    64 - v.leading_zeros()
}
```

## 2つの値を入れ替える swap スワップ
```
std::mem::swap(&mut a, &mut b);
vec.swap(i, j);  // Vec<_> バージョン
```


## Rustのcloneとclonedの違い
https://minerva.mamansoft.net/Notes/Rustのcloneとclonedの違い


## a / b の整数切り上げ
```
(a + (b - 1)) / b
```

---

## 閉路検出 トポロジカルソート
dfsで再帰から帰ってくるときのフラグを考える
past202107-open J問題

## いもす法
https://note.com/kirimin_chan/n/n7663e3bb8a05


## 正規表現
https://qiita.com/scivola/items/60141f262caa53983c3a
code-formula-2014-final/src/bin/c.rs

## グラフの2点間の距離
根付き木が構成出来るなら、根からの深さの和を考えればよい
https://yunix-kyopro.hatenablog.com/entry/2021/07/11/020240?_ga=2.121161536.9506465.1625937519-1301098457.1625937519#f-cd9f4786
https://blog.hamayanhamayan.com/entry/2021/07/11/154020

## ダブリング
https://www.slideshare.net/satanic2/ss-72500089
（例）abc167 d

## ダブリングによる木の最近共通祖先（LCA：Lowest Common Ancestor）を求めるアルゴリズム
https://algo-logic.info/lca/
abc209 d

## 木構造
木ならば、頂点 N、辺 N - 1
木は 2部グラフ

## 誤差
EPSを適切に設定して比較に使う（誤差対策でA < BはA < B - EPSとするテンプレがある）

## 整数の10進数表現と2進数表現の変換
```
let mut d = format!("{:b}", x);
let ans = format!("{:010b}", n); // 10桁左0詰め
i64::from_str_radix(&d, 2).unwrap()
```

<br> <br>

# rust 便利イテレータ集

## std::slice::windowsは、「前の値も含めて for ループを回したい」というユースケースで使うことができる
```
// https://qiita.com/hystcs/items/d33e77084277cdba8052

（例）
let v = vec![1, 2, 3];
for w in v.windows(2) {
    let (prev, next) = (w[0], w[1]);
    println!("{} {}", prev, next)
}
// 1 2
// 2 3

（例）
ans += t.windows(2).filter(|t| t[0] != t[1]).count(); // 前後で値が異なるときにカウント
```

## `fold` は状態を持ち， 各要素に対して関数を適用して状態を更新し，その状態を返す
```
let a = [1, 2, 3];
// the sum of all of the elements of the array
let sum = a.iter().fold(0, |acc, x| acc + x);

# 例えばこんな使い方
# abc109 c
use proconio::{input};
use num::integer::gcd;
 
fn main() {
    input!{n:u32,x:u64,mut xn:[u64;n]}
    xn.push(x);
    xn.sort();
    println!("{:?}",xn.windows(2).map(|a| a[1]-a[0]).fold(xn[1]-xn[0],|ans,v| gcd(ans,v)));
}
```

