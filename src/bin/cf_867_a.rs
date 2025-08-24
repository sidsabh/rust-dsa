use dsa_lib::io::Scanner;

fn solve(mut a_reader: Scanner<impl std::io::BufRead>, a_writer: &mut impl std::io::Write) {
    let _my_num_days: u32 = a_reader.next();
    let my_location_string = a_reader.next::<String>();

    let mut my_left_seattle = 0;
    let mut my_left_sanfran = 0;

    for my_window in my_location_string.as_bytes().windows(2) {
        if my_window == b"SF" {
            my_left_seattle += 1;
        } else if my_window == b"FS" {
            my_left_sanfran += 1;
        }
    }

    writeln!(
        a_writer,
        "{}",
        if my_left_seattle > my_left_sanfran {
            "YES"
        } else {
            "NO"
        }
    )
    .expect("write failed");
}

fn main() {
    let my_reader = Scanner::new(std::io::stdin().lock());
    solve(my_reader, &mut std::io::stdout().lock());
}

#[cfg(test)]
mod tests {
    use crate::solve;
    use dsa_lib::create_test;
    create_test!(test1, "4\nFSSF", "NO\n");
    create_test!(test2, "2\nSF", "YES\n");
    create_test!(test3, "10\nSSFFSFFSFF", "YES\n");
    create_test!(test4, "10\nFFFFFFFFFF", "NO\n");
    create_test!(test5, "20\nSFSFFFFSSFFFFSSSSFSS", "NO\n");
}
