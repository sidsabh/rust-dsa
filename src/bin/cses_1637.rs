use std::cmp::min;
use dsa_lib::io::Scanner;

fn solve(a_reader: &mut Scanner<impl std::io::BufRead>, a_writer: &mut impl std::io::Write) {
    let n  = a_reader.next::<usize>();

    let mut dp = vec![u32::MAX; n+1];

    dp[0] = 0;

    for num in 1..=n {
        let mut val = num;
        while val > 0 {
            let rem = val % 10;
            val /= 10;
            if rem != 0 {
                dp[num] = min(dp[num], dp[num-rem]+1);
            }
        }
    }
    
    writeln!(a_writer, "{}", dp[n]).unwrap();
}

fn main() {
    let mut my_reader = Scanner::new(std::io::stdin().lock());
    solve(&mut my_reader, &mut std::io::stdout().lock());
}

#[cfg(test)]
mod tests {
    use crate::solve;
    use dsa_lib::create_test;
    create_test!(test1, "27", "5\n");
}
