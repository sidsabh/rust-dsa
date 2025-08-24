use dsa_lib::io::Scanner;
use std::ops::Neg;

#[derive(Clone, Copy)]
struct Edge {
    dest: usize,
    weight: i64,
}

fn solve(a_reader: &mut Scanner<impl std::io::BufRead>, a_writer: &mut impl std::io::Write) {
    let n = a_reader.next::<usize>();
    let m = a_reader.next::<usize>();

    let mut edges : Vec<Vec<Edge>> = vec![vec![]; n];
    let mut rev_edges : Vec<Vec<Edge>> = vec![vec![]; n];

    for _ in 0..m {
        let u = a_reader.next::<usize>() - 1;
        let v = a_reader.next::<usize>() - 1;
        let w = a_reader.next::<i64>();
        edges[u].push(Edge {
            dest: v,
            weight: w.neg(),
        });
        rev_edges[v].push(Edge {
            dest: u,
            weight: w.neg(),
        });
    }

    let mut dists : Vec<i64> = vec![i64::MAX; n];
    dists[0] = 0;

    for _ in 0..(n-1) {
        for src in 0..n {
            if dists[src] == i64::MAX {
                continue;
            }
            for edge in &edges[src] {
                let potential_dist = dists[src] + edge.weight;
                if potential_dist < dists[edge.dest] {
                    dists[edge.dest] = potential_dist;
                }
            }
        }
    }

    if dists[n-1] == i64::MAX {
        writeln!(a_writer, "-1").unwrap();
        return;
    }

    // find cycle vertices
    let mut cycle_vertices = vec![false; n];
    for src in 0..n {
        if dists[src] == i64::MAX {
            continue;
        }
        for edge in &edges[src] {
            let potential_dist = dists[src] + edge.weight;
            if potential_dist < dists[edge.dest] {
                dists[edge.dest] = potential_dist;
                cycle_vertices[edge.dest] = true;
            }
        }
    }


    // see if any of the cycles are reachable from n-1
    let mut visited= vec![false; n];
    let mut stack= vec![];
    stack.push(n-1);
    while !stack.is_empty() {
        let node = stack.pop().unwrap();
         
        if visited[node] {
            continue;
        }
        visited[node] = true;

        if cycle_vertices[node] {
            writeln!(a_writer, "-1").unwrap();
            return;
        }

        for edge in &rev_edges[node] {
            if !visited[edge.dest] {
                stack.push(edge.dest);
            }
        }
    }

    let max_score = dists[n-1].neg();
    writeln!(a_writer, "{max_score}").unwrap();

}

fn main() {
    let mut my_reader = Scanner::new(std::io::stdin().lock());
    solve(&mut my_reader, &mut std::io::stdout().lock());
}

#[cfg(test)]
mod tests {
    use crate::solve;
    use dsa_lib::create_test;
    create_test!(test1, "4 5
1 2 3
2 4 -1
1 3 -2
3 4 7
1 4 4",
"5\n");

    create_test!(test2, "5 4
1 2 1
2 3 1
3 4 1
4 5 1",
"4\n");

    create_test!(test3, "4 4
4 1 5
3 4 -1
2 3 -1
1 2 -1",
"-1\n");

    create_test!(test4, "4 4
1 2 1
2 3 1
3 2 1
1 4 1",
"1\n");

    create_test!(test5, "7 7
1 2 1
2 7 1
1 6 -1
6 5 -1
5 4 -1
4 3 -1
3 1 5",
"-1\n");
}