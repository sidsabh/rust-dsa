use std::cmp::min;
use dsa_lib::io::Scanner;

fn solve(a_reader: &mut Scanner<impl std::io::BufRead>, a_writer: &mut impl std::io::Write) {

    let num_testcases = a_reader.next();

    for _ in 0..num_testcases {
        let num_mobs = a_reader.next::<usize>();
        let mut health: Vec<u64> = Vec::with_capacity(num_mobs+1);
        health.push(0);
        (1..=num_mobs).for_each(|idx| health.insert(idx, a_reader.next()));

        let mut dp = vec![0u64;num_mobs+1];

        // let dp[i] represent the cost of clearing all mobs up till mob i
        dp[0] = 0;
        dp[1] = health[1]; // have to clear it directly

        for i in 2..=num_mobs {
            let fall_damage = i as u64 - 1;
            dp[i] = min(
                health[i-1] + dp[i-2] + if health[i] > fall_damage {health[i] - fall_damage} else {0}, // max fall damage
                dp[i-1] + health[i] - 1 // one fall damage
            );
        }

        writeln!(a_writer, "{}", dp[num_mobs]).unwrap();
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
    create_test!(test1, 
"5
5
3 1 4 1 2
4
1 1 1 1
6
1 2 1 3 5 2
6
3 1 1 3 2 1
3
1000000000 1000000000 1000000000
",
"7
1
7
5
2999999998
");
}