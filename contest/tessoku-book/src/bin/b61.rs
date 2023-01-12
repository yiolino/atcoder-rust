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

    let mut max = 0;
    let mut ans = 0;
    for (i, node) in graph.into_iter().enumerate() {
        if node.len() >= max {
            max = node.len();
            ans = i;
        }
    }

    println!("{}", ans);
}
