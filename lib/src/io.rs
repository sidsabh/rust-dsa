use std::io::BufRead;
use std::str::FromStr;

pub struct Scanner<R> {
    the_reader: R,
    the_buffer: Vec<String>,
}

impl<R: BufRead> Scanner<R> {
    pub fn new(the_reader: R) -> Self {
        Scanner {
            the_reader,
            the_buffer: Vec::new(),
        }
    }

    pub fn next<T: FromStr>(&mut self) -> T
    where
        T::Err: std::fmt::Debug,
    {
        loop {
            if let Some(token) = self.the_buffer.pop() {
                return token.parse::<T>().unwrap();
            }
            let mut line = String::new();
            let n = self.the_reader.read_line(&mut line).unwrap();
            if n == 0 {
                panic!("EOF reached");
            }
            self.the_buffer = line
                .split_whitespace()
                .rev()
                .map(|s| s.to_string())
                .collect();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn read_vec() {
        let my_data = String::from("1 2 3\n");
        let mut my_reader= Scanner::new(Cursor::new(my_data));
        let my_read_vec : Vec<u32> = (0..2).map(|_| my_reader.next()).collect();

        assert_eq!(vec![1, 2, 3], my_read_vec);
    }

    #[test]
    fn read_singles() {
        let my_data = String::from("1\n2 3\n");
        let mut my_reader= Scanner::new(Cursor::new(my_data));

        let my_first = my_reader.next();
        let my_second = my_reader.next();
        let my_third = my_reader.next();

        assert_eq!(1, my_first);
        assert_eq!(2, my_second);
        assert_eq!(3, my_third);
    }
}
