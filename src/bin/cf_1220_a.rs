use std::collections::HashMap;
use std::io;
use std::ops::Index;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;

    let _n: i32 = buffer.trim().parse().unwrap();

    buffer.clear();
    io::stdin().read_line(&mut buffer)?;

    let mut counts: HashMap<char, i32> = HashMap::new();
    counts.insert('z', 0);
    counts.insert('e', 0);
    counts.insert('r', 0);
    counts.insert('o', 0);
    counts.insert('n', 0);

    buffer.trim().chars().for_each(|c| {
        *counts.get_mut(&c).unwrap() += 1;
    });

    let num_ones = *counts
        .iter()
        .filter(|c| "one".contains(*c.0))
        .map(|c| c.1)
        .min()
        .unwrap();
    counts
        .iter_mut()
        .filter(|c| "one".contains(*c.0))
        .for_each(|c| *c.1 -= num_ones);
    let num_zeros = *counts
        .iter()
        .filter(|c| "zero".contains(*c.0))
        .map(|c| c.1)
        .min()
        .unwrap();

    for _ in 0..num_ones {
        print!("1 ");
    }

    for _ in 0..num_zeros {
        print!("0 ");
    }

    Ok(())
}

fn test() {
    let mut x = 5;
    let g = &mut x;
    call_fn(g);
    print!("{g}");

    let my_str = String::from("lamron");
    let x = my_str.index(0..1);
    print!("{x}");

    let my_vec = vec!["s"];
    let y = my_vec.index(0..1);
}

fn call_fn(v: &mut i32) {
    print!("{v}");
}
