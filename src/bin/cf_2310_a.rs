use dsa_lib::io::Scanner;

fn solve(a_reader: &mut Scanner<impl std::io::BufRead>, a_writer: &mut impl std::io::Write) {
    let my_num_testcases: u32 = a_reader.next();

    for _ in 0..my_num_testcases {
        let my_set_size: u32 = a_reader.next();
        let my_multiset: Vec<u32> = (0..my_set_size).map(|_| a_reader.next()).collect();

        let mut my_score = 0;

        let my_num_zeros: u32 = my_multiset.iter().filter(|v| **v == 0).count() as u32;
        let my_mex = my_num_zeros;

        my_score += my_mex;
        my_score += my_multiset.iter().sum::<u32>();

        writeln!(a_writer, "{my_score}").expect("failed to write");
    }
}

fn main() {
    let mut my_reader = Scanner::new(std::io::stdin().lock());
    solve(&mut my_reader, &mut std::io::stdout().lock());
}

#[cfg(test)]
mod tests {
    use crate::solve;
    use std::io::Cursor;
    #[test]
    fn test() {
        let my_data = "2\n3\n0 1 1\n3\n1 2 3\n";
        let mut my_reader = crate::Scanner::new(Cursor::new(my_data));
        let mut my_writer: Vec<u8> = Vec::new();
        solve(&mut my_reader, &mut my_writer);

        let my_out = String::from_utf8(my_writer).unwrap();
        assert_eq!(my_out, "3\n6\n");
    }
}
