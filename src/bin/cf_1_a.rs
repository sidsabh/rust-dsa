use dsa_lib::io::Scanner;

fn solve(a_reader: &mut Scanner<impl std::io::BufRead>, a_writer: &mut impl std::io::Write) {
    let m: u64 = a_reader.next();
    let n: u64 = a_reader.next();
    let a: u64 = a_reader.next();

    let tiles_m = (m + a - 1) / a;
    let tiles_n = (n + a - 1) / a;

    let res = tiles_m * tiles_n;

    writeln!(a_writer, "{res}").expect("write failed");
}

fn main() {
    let mut my_reader = Scanner::new(std::io::stdin().lock());
    solve(&mut my_reader, &mut std::io::stdout().lock());
}

#[cfg(test)]
mod tests {
    use crate::solve;
    use dsa_lib::create_test;
    create_test!(test1, "6 6 4", "4\n");
    create_test!(test2, "1 1 1", "1\n");

    create_test!(test3, "1 1 5", "1\n");
    create_test!(test4, "5 1 5", "1\n");
    create_test!(test5, "5 1 1", "5\n");
    create_test!(test6, "6 1 5", "2\n");
    create_test!(test7, "1000000000 1000000000 1", "1000000000000000000\n");
}
