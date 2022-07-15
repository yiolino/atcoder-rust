use proconio::input;

fn main() {
    input!{
        n: usize,
        x: usize,
        ab: [(usize, usize); n],
    }

    let mut ans = std::usize::MAX;
    let mut head = 0;
    
    for i in 0..n {
        head += ab[i].0;
        let tmp = (x - i) * ab[i].1 + head;
        ans = ans.min(tmp);

        head += ab[i].1;
    }

    println!("{}", ans);
}
