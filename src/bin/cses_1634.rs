use std::cmp::min;

use dsa_lib::io::Scanner;

fn solve(a_reader: &mut Scanner<impl std::io::BufRead>, a_writer: &mut impl std::io::Write) {
    let n  = a_reader.next::<usize>();
    let x  = a_reader.next::<usize>();

    let mut coins = Vec::with_capacity(n);
    (0..n).for_each(|_| coins.push(a_reader.next::<u32>()));    

    let mut dp = vec![u32::MAX; x+1];

    dp[0] = 0;

    for sum in 0..x {
        if dp[sum] == u32::MAX {
            continue;
        }
        for coin in coins.iter().map(|c| *c as usize){
            if sum + coin > x {
                continue;
            }
            dp[sum+coin] = min(dp[sum+coin], 1+dp[sum]);
        }
    }

    writeln!(a_writer, "{}", if dp[x] == u32::MAX {-1} else {dp[x] as i32}).unwrap();
}

fn main() {
    let mut my_reader = Scanner::new(std::io::stdin().lock());
    solve(&mut my_reader, &mut std::io::stdout().lock());
}

#[cfg(test)]
mod tests {
    use crate::solve;
    use dsa_lib::create_test;
    create_test!(test1, "3 11
1 5 7", "3\n");
}