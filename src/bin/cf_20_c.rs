use dsa_lib::io::Scanner;

#[derive(Clone, Copy)]
struct Edge {
    neighbor: usize,
    weight: u64,
}

#[derive(PartialEq, Eq, Ord)]
struct SearchNode {
    vertex: usize,
    distance_from_source: u64,
    prev: u32,
}

impl PartialOrd for SearchNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(
            self.distance_from_source
                .partial_cmp(&other.distance_from_source)
                .unwrap()
                .reverse(),
        )
    }
}

fn solve(a_reader: &mut Scanner<impl std::io::BufRead>, a_writer: &mut impl std::io::Write) {
    let n: usize = a_reader.next();
    let m: usize = a_reader.next();

    let mut neighbors = vec![vec![]; n];

    for _ in 0..m {
        let u = a_reader.next::<usize>() - 1;
        let v = a_reader.next::<usize>() - 1;
        let w = a_reader.next();
        neighbors[u].push(Edge {
            neighbor: v,
            weight: w,
        });
        neighbors[v].push(Edge {
            neighbor: u,
            weight: w,
        });
    }

    let mut prev = vec![u32::MAX; n];
    let mut dist = vec![u64::MAX; n];
    let mut visited = vec![false; n];
    let mut queue: std::collections::BinaryHeap<SearchNode> = std::collections::BinaryHeap::new();

    queue.push(SearchNode {
        vertex: 0,
        distance_from_source: 0,
        prev: u32::MAX,
    });
    dist[0] = 0;

    while !queue.is_empty() {
        let node = queue.pop().unwrap();

        if visited[node.vertex as usize] {
            continue;
        }

        visited[node.vertex as usize] = true;
        prev[node.vertex as usize] = node.prev;

        if node.vertex == n - 1 {
            break;
        }

        for edge in &neighbors[node.vertex as usize] {
            if visited[edge.neighbor as usize] {
                continue;
            }

            let potential_dist = node.distance_from_source + edge.weight;
            if potential_dist > dist[edge.neighbor as usize] {
                continue;
            }
            dist[edge.neighbor as usize] = potential_dist;

            queue.push(SearchNode {
                vertex: edge.neighbor,
                distance_from_source: node.distance_from_source + edge.weight,
                prev: node.vertex as u32,
            });
        }
    }

    if dist[n - 1] != u64::MAX {
        let mut path = vec![];

        let mut current: u32 = (n - 1) as u32;
        while current != u32::MAX {
            path.push(current);
            current = prev[current as usize];
        }

        let output: Vec<String> = path.iter().rev().map(|v| (v + 1).to_string()).collect();
        let path_output = output.join(" ");
        writeln!(a_writer, "{path_output}").unwrap();
    } else {
        writeln!(a_writer, "-1").unwrap();
    }
}

fn main() {
    let mut my_reader = Scanner::new(std::io::stdin().lock());
    solve(&mut my_reader, &mut std::io::stdout().lock());
}

#[cfg(test)]
mod tests {
    use crate::solve;
    use dsa_lib::create_test;
    create_test!(
        test1,
        "5 6
1 2 2
2 5 5
2 3 4
1 4 1
4 3 3
3 5 1",
        "1 4 3 5\n"
    );
    create_test!(
        test2,
        "5 6
1 2 2
2 5 5
2 3 4
1 4 1
4 3 3
3 5 1",
        "1 4 3 5\n"
    );
    create_test!(
        test3,
        "10 10
1 5 12
2 4 140
2 10 149
3 6 154
3 7 9
3 8 226
3 10 132
4 10 55
5 8 33
7 8 173",
        "1 5 8 7 3 10\n"
    );
}
