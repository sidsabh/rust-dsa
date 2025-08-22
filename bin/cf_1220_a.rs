use std::collections::HashMap;
use std::io::{self, Write};

fn main() -> io::Result<()>{
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;

    let _n : i32 = buffer.trim().parse().unwrap();

    io::stdin().read_line(&mut buffer)?;

    let mut counts: HashMap<char, i32> = HashMap::new();
    counts.insert('z', 0);
    counts.insert('e', 0);
    counts.insert('r', 0);
    counts.insert('o', 0);
    counts.insert('n', 0);

    buffer.trim().chars().for_each(|c| {
        if let Some(ctr) = counts.get_mut(&c) {
            *ctr += 1;
        }
    });

    let num_ones = *counts.iter().filter(|c| "one".contains(*c.0)).map(|c| c.1).min().unwrap();
    counts.iter_mut().filter(|c| "one".contains(*c.0)).for_each(|c|*c.1 -= num_ones);
    let num_zeros = *counts.iter().filter(|c| "zero".contains(*c.0)).map(|c| c.1).min().unwrap();

    let mut out = io::stdout();

    for _ in 0..num_ones {
        out.write("1 ".as_bytes())?;
    }
    
    for _ in 0..num_zeros{
        out.write("0 ".as_bytes())?;
    }

    Ok(())
}
