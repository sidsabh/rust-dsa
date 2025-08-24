use std::collections::HashSet;

use dsa_lib::io::Scanner;

fn solve(a_reader: &mut Scanner<impl std::io::BufRead>, a_writer: &mut impl std::io::Write) {

    let num_testcases = a_reader.next();

    for _ in 0..num_testcases {

        let num_gears = a_reader.next();
        let gears: HashSet<u32> = (0..num_gears).map(|_| a_reader.next()).collect();
        let passed = gears.len() != num_gears;

        writeln!(a_writer, "{}", if passed {"YES"} else {"NO"}).unwrap();
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
    create_test!(test1, "5
2
5 5
4
6 3 6 9
2
2 3
7
30 10 12 10 10 9 18
5
2 4 8 16 32
", "YES
YES
NO
YES
NO\n");
}