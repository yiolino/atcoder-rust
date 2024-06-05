use std::collections::VecDeque;
use itertools::Itertools;
use proconio::input;

fn main() {
    input!{
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
    }

    let mut graph = vec![vec![]; n + 1];
    for (a, b) in ab {
        graph[a].push(b);
        graph[b].push(a);
    }

    let mut seen = vec![false; n + 1];
    let mut stack = VecDeque::new();

    dfs(1, &graph, &mut seen, &mut stack);

    let ans = stack.iter().join(" ");
    println!("{}", ans);
}

fn dfs(node: usize, graph: &Vec<Vec<usize>>, seen: &mut Vec<bool>, stack: &mut VecDeque<usize>) {
    let goal = graph.len() + 1;
    if node == goal {
        return;
    }

    seen[node] = true;
    stack.push_back(node);
    for next in &graph[node] {
        if !seen[*next] {
            dfs(*next, graph, seen, stack);
        }
    }

    stack.pop_back();
}
