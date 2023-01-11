use itertools::Itertools;
use proconio::input;

fn main() {
    input!{
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
    }

    let mut graph = vec![vec![]; 100000 + 10];
    for (a, b) in ab {
        graph[a].push(b);
        graph[b].push(a);
    }

    for i in 1..=n {
        print!("{}: {{", i);
        print!("{}", graph[i].iter().join(", "));
        println!("}}");
    }
}
