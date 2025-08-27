use dsa_lib::io::Scanner;

fn solve(a_reader: &mut Scanner<impl std::io::BufRead>, a_writer: &mut impl std::io::Write) {
    let num_cells = a_reader.next::<usize>();
    let target = a_reader.next::<usize>();

    let mut portals: Vec<usize> = Vec::with_capacity(num_cells - 1);
    (1..num_cells).for_each(|_| portals.push(a_reader.next()));

    let mut current = 1usize;
    while current != num_cells {
        current = portals[current-1] + current;

        if current == target {
            writeln!(a_writer, "YES").unwrap();
            return;
        }
    }

    writeln!(a_writer, "NO").unwrap();
}

fn main() {
    let mut my_reader = Scanner::new(std::io::stdin().lock());
    solve(&mut my_reader, &mut std::io::stdout().lock());
}

#[cfg(test)]
mod tests {
    use crate::solve;
    use dsa_lib::create_test;
    create_test!(test1, "8 4\n1 2 1 2 1 2 1", "YES\n");
}
