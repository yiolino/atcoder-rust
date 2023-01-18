use proconio::{input, marker::Usize1};

fn main() {
    input!{
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
    }

    let mut graph = vec![vec![]; n];
    for (a, b) in ab {
        graph[a].push(b);
        graph[b].push(a);
    }

    let mut seen = vec![false; n];
    dfs(&graph, 0, &mut seen);

    if seen.iter().all(|x| *x) {
        println!("The graph is connected.")
    } else {
        println!("The graph is not connected.")
    }
}

fn dfs(graph: &Vec<Vec<usize>>, start: usize, seen: &mut Vec<bool>) {
    if !seen[start] {
        seen[start] = true;
        for next in &graph[start] {
            dfs(graph, *next, seen);
        }
    }
}