use dsa_lib::io::Scanner;

fn solve(a_reader: &mut Scanner<impl std::io::BufRead>, a_writer: &mut impl std::io::Write) {

    let num_testcases = a_reader.next();

    for _ in 0..num_testcases {

        let num_neighbors = a_reader.next();
        let mut neighbors_grump: Vec<u64> = (0..num_neighbors).map(|_| a_reader.next()).collect();
        neighbors_grump.sort_by(|a, b| b.cmp(a));

        let mut emeralds = 0;

        neighbors_grump.chunks(2).into_iter().for_each(|chunk| {
            emeralds += chunk[0];
        });

        writeln!(a_writer, "{emeralds}").unwrap();
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
    create_test!(test1, 
"4
2
1 2
4
2 1 5 2
5
1000000000 1000000000 1000000000 1000000000 1000000000
6
3 1 4 1 5 9", 
"2
7
3000000000
14
");
}