#![allow(dead_code)]
pub struct Scanner {
    idx: usize,
    buf: Vec<String>,
}

impl Scanner {
    pub fn new<T: std::io::Read>(inf: &mut T) -> Scanner {
        Self {
            idx: 0,
            buf: {
                let mut s = String::new();
                inf.read_to_string(&mut s).expect("I/O error");
                s.split_whitespace().map(|x| x.to_owned()).collect()
            },
        }
    }

    pub fn read<T: std::str::FromStr>(&mut self) -> T
    where
        <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        if self.empty() {
            panic!("reached the end of input")
        }
        let ret = self.buf[self.idx].parse::<T>().expect("parse error");
        self.idx += 1;
        return ret;
    }

    pub fn empty(&self) -> bool {
        return self.idx >= self.buf.len();
    }
}

fn dfs(u: usize, t: &mut i32, g: &Vec<Vec<usize>>, d: &mut Vec<i32>, f: &mut Vec<i32>) {
    d[u] = *t;
    for v in &g[u] {
        if d[*v] > 0 { continue; }
        *t += 1;
        dfs(*v, t, g, d, f);
    }
    *t += 1;
    f[u] = *t;
}

fn main() {
    let mut sc = Scanner::new(&mut std::io::stdin());
    let n: usize = sc.read();
    let mut g = vec![Vec::new(); n];

    for _ in 0..n {
        let u: usize = sc.read::<usize>() - 1;
        let k: usize = sc.read();
        for _ in 0..k {
            let v: usize = sc.read::<usize>() - 1;
            g[u].push(v);
        }
    }

    let mut d = vec![0; n];
    let mut f = vec![0; n];
    let mut t = 1;
    for i in 0..n {
        if d[i] == 0 {
            dfs(i, &mut t, &g, &mut d, &mut f);
            t += 1;
        }
    }

    for i in 0..n {
        println!("{} {} {}", i + 1, d[i], f[i]);
    }
}
