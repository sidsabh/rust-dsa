use std::io::{stdout, BufRead, Write};
use dsa_lib::io::Scanner;

fn solve<R: BufRead, W: Write>(mut a_the_reader: Scanner<R>, a_writer: &mut W) {
    let my_num_testcases: u32 = a_the_reader.next();

    for _ in 0..my_num_testcases {
        let my_set_size : u32 = a_the_reader.next();
        let my_multiset : Vec<u32> = (0..my_set_size).map(|_| a_the_reader.next()).collect();
        
        let mut my_score = 0;

        let my_num_zeros: u32  = my_multiset.iter().filter(|v| **v == 0).count() as u32;
        let my_mex = my_num_zeros;

        my_score += my_mex;
        my_score += my_multiset.iter().sum::<u32>();

        writeln!(a_writer,"{my_score}").expect("failed to write");
    }
}

fn main() {
    let my_the_reader = Scanner::new(std::io::stdin().lock());
    solve(my_the_reader, &mut stdout().lock());
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use crate::solve;
    #[test]
    fn test() {
        let my_data = "2\n3\n0 1 1\n3\n1 2 3\n";
        let my_the_reader= crate::Scanner::new(Cursor::new(my_data));
        let mut my_writer: Vec<u8> = Vec::new(); 
        solve(my_the_reader, &mut my_writer);

        let out = String::from_utf8(my_writer).unwrap();
        assert_eq!(out, "3\n6\n");

    }

}