use std::mem::swap;
use dsa_lib::io::Scanner;
use dsa_lib::algo::PermutationGenerator;

#[derive(Debug, Clone, Copy)]
struct Rectangle {
    side_1 : usize,
    side_2: usize 
}


impl Rectangle {
    pub fn area(&self) -> usize {
        self.side_1 * self.side_2
    }

    pub fn swap_sides(&mut self) {
        swap(&mut self.side_1, &mut self.side_2);
    }
}

fn solve(a_reader: &mut Scanner<impl std::io::BufRead>, a_writer: &mut impl std::io::Write) {

    let mut rectangles = vec![];
    for _ in 1..=3 {
        rectangles.push(Rectangle{
            side_1: a_reader.next(),
            side_2: a_reader.next()
        });
    }

    let total_area: usize = rectangles.iter().map(|rectangle| rectangle.area()).sum();
    let side = (total_area as f32).sqrt();
    if side != side.ceil() {
        writeln!(a_writer, "-1").unwrap();
        return;
    }

    let side = side as usize;

    let mut grid = vec![vec![' '; side]; side];
    let mut found = false;

    'outer: for perm in PermutationGenerator::new(3) {
        let base: Vec<Rectangle> = perm.iter().map(|&i| rectangles[i - 1]).collect();

        for mask in 0b000..=0b111 {
            let mut rects = base.clone();
            for j in 0..3 {
                if (mask >> j) & 1 == 1 {
                    rects[j].swap_sides();
                }
            }

            // Pattern 1: all stacked vertically
            if rects.iter().all(|r| r.side_1 == side)
                && rects.iter().map(|r| r.side_2).sum::<usize>() == side
            {
                let mut row = 0;
                for (idx, r) in rects.iter().enumerate() {
                    for i in 0..r.side_2 {
                        for j in 0..r.side_1 {
                            grid[row + i][j] = (b'@' + perm[idx] as u8) as char;
                        }
                    }
                    row += r.side_2;
                }
                found = true;
                break 'outer;
            }

            // Pattern 2: top rect spans full width
            if rects[0].side_1 == side && rects[0].side_2 < side {
                let rem = side - rects[0].side_2;
                if rects[1].side_2 == rem
                    && rects[2].side_2 == rem
                    && rects[1].side_1 + rects[2].side_1 == side
                {
                    // fill
                    for i in 0..rects[0].side_2 {
                        for j in 0..side {
                            grid[i][j] = (b'@' + perm[0] as u8) as char;
                        }
                    }
                    for i in 0..rem {
                        for j in 0..rects[1].side_1 {
                            grid[rects[0].side_2 + i][j] = (b'@' + perm[1] as u8) as char;
                        }
                        for j in rects[1].side_1..side {
                            grid[rects[0].side_2 + i][j] = (b'@' + perm[2] as u8) as char; 
                        }
                    }
                    found = true;
                    break 'outer;
                }
            }
        }
    }

    if !found {
        writeln!(a_writer, "-1").unwrap();
    } else {
        writeln!(a_writer, "{}", side).unwrap();
        for row in grid {
            writeln!(a_writer, "{}", row.iter().collect::<String>()).unwrap();
        }
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
    create_test!(test1, "5 1 2 5 5 2", "5
AAAAA
BBBBB
BBBBB
CCCCC
CCCCC
");
    create_test!(test2, "4 4 2 6 4 2", "6
BBBBBB
BBBBBB
AAAACC
AAAACC
AAAACC
AAAACC
");
}
