use std::io;

// 動態規劃解法
fn fibonacci_dp(n: u64) -> u64 {
  if n == 0 {
      return 0;
  }

  let mut dp: Vec<u64> = vec![0, 1, 1];
  for _i in 2..=n {
      dp[2] = dp[0] + dp[1];
      dp[0] = dp[1];
      dp[1] = dp[2];
  }

  dp[2]
}

fn main() {
    println!("Enter the value of your Fibonacci sequence's n, and I will reply with the corresponding value.");
    println!("Exit with 'Ctrl + C'\n");


    loop {
        println!("Please input your Fibonacci sequence's n.");

        let mut fibonacci_n = String::new();

        io::stdin()
            .read_line(&mut fibonacci_n)
            .expect("Failed to read line");

        let fibonacci_n: u64 = match fibonacci_n.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You fibonacci value is {}!\n", fibonacci_dp(fibonacci_n));
    }
}
