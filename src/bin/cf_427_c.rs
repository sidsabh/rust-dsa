use std::collections::VecDeque;

use dsa_lib::io::Scanner;

const MOD : u64 = (1e9 as u64) + 7;

fn solve(a_reader: &mut Scanner<impl std::io::BufRead>, a_writer: &mut impl std::io::Write) {

    let num_junctions = a_reader.next::<usize>();
    let mut junctions_cost = Vec::with_capacity(num_junctions);
    (0..num_junctions).for_each(|_| junctions_cost.push(a_reader.next::<u32>()));

    let num_edges = a_reader.next::<usize>();
    let mut graph: Vec<Vec<u32>> = vec![vec![]; num_junctions];
    let mut transpose_graph: Vec<Vec<u32>> = vec![vec![]; num_junctions];
    for _ in 0..num_edges {
        let u = a_reader.next::<u32>() - 1;
        let v = a_reader.next::<u32>() - 1;
        graph[u as usize].push(v);
        transpose_graph[v as usize].push(u);
    }

    fn dfs(graph: &Vec<Vec<u32>>, stack: &mut VecDeque<(usize, bool)>, visited: &mut Vec<bool>, mut post_finish : impl FnMut(usize)) {
        while !stack.is_empty() {
            let (current, finished) = stack.pop_back().unwrap();
            if finished {
                post_finish(current);
                continue;
            }
            if visited[current] {
                continue;
            }
            visited[current] = true;

            stack.push_back((current, true));
            for neighbor in graph[current].iter().map(|x| *x as usize) {
                if !visited[neighbor] {
                    stack.push_back((neighbor, false));
                }
            }
        }
    }

    let mut visited = vec![false; num_junctions];
    let mut timestamp_stack= VecDeque::new();

    let mut stack = VecDeque::new();
    for node in 0..num_junctions {
        if visited[node] {
            continue;
        }
        stack.push_back((node, false));
        dfs(&graph, &mut stack, &mut visited, |current: usize| {
            timestamp_stack.push_back((usize::MAX, true));
            timestamp_stack.push_back((current, false));
        });
    }


    visited.fill(false);
    let mut components = vec![];
    let mut current_component = vec![];
    dfs(&transpose_graph, &mut timestamp_stack, &mut visited, |current: usize| {
        if current == usize::MAX { 
            if !current_component.is_empty(){
                components.push(current_component.clone());
                current_component.clear();
            }
        } else {
            current_component.push(current);
        }
    });

    let min_per_component : Vec<u64> =  components.iter().map(|component| component.iter().map(|j| junctions_cost[*j] as u64).min().unwrap()).collect();
    let min : u64 = min_per_component.iter().sum();
    write!(a_writer, "{min} ").unwrap();
    let mut count_min: u64= 1;
    for (idx, component) in components.iter().enumerate() {
        count_min = (count_min * component.iter().filter(|v| junctions_cost[**v] as u64 == min_per_component[idx]).count() as u64) % MOD;
    }
    writeln!(a_writer, "{count_min}").unwrap();
}

fn main() {
    let mut my_reader = Scanner::new(std::io::stdin().lock());
    solve(&mut my_reader, &mut std::io::stdout().lock());
}

#[cfg(test)]
mod tests {
    use crate::solve;
    use dsa_lib::create_test;
    create_test!(test1, "3
1 2 3
3
1 2
2 3
3 2", "3 1\n");
    create_test!(test2, "5
2 8 0 6 0
6
1 4
1 3
2 4
3 4
4 5
5 1",
"8 2\n");
    create_test!(test3, "10
1 3 2 2 1 3 1 4 10 10
12
1 2
2 3
3 1
3 4
4 5
5 6
5 7
6 4
7 3
8 9
9 10
10 9",
"15 6\n");

    create_test!(test4, "2
7 91
2
1 2
2 1",
"7 1
");
}
