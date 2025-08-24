use dsa_lib::io::Scanner;

const MOD : u32 = (1e9 as u32) + 7;

fn solve(a_reader: &mut Scanner<impl std::io::BufRead>, a_writer: &mut impl std::io::Write) {
    let n  = a_reader.next::<usize>();
    let x  = a_reader.next::<usize>();

    let mut coins = Vec::with_capacity(n);
    (0..n).for_each(|_| coins.push(a_reader.next::<u32>()));    

    let mut dp = vec![0; x+1];

    dp[0] = 1;

    for sum in 0..x {
        if dp[sum] == 0 {
            continue;
        }
        for coin in coins.iter().map(|c| *c as usize){
            if sum + coin > x {
                continue;
            }
            dp[sum+coin] = (dp[sum+coin] + dp[sum]) % MOD;
        }
    }

    writeln!(a_writer, "{}", dp[x]).unwrap();


}

fn main() {
    let mut my_reader = Scanner::new(std::io::stdin().lock());
    solve(&mut my_reader, &mut std::io::stdout().lock());
}

#[cfg(test)]
mod tests {
    use crate::solve;
    use dsa_lib::create_test;
    create_test!(test1, "3 9
2 3 5
", "8\n");
}
