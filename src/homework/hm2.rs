/*
    二、產生第 _n_ 個斐波那契數
*/

// 遞迴解法
fn fibonacci_recursive(n: i32) -> i32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2);
    }
}

// 動態規劃解法
fn fibonacci_dp(n: i32) -> i32 {
    if n == 0 {
        return 0;
    }

    let mut dp: Vec<i32> = vec![0, 1, 1];
    for _i in 2..=n {
        dp[2] = dp[0] + dp[1];
        dp[0] = dp[1];
        dp[1] = dp[2];
    }

    dp[2]
}

pub fn fibonacci() {
    println!("\n作業二：產生第 _n_ 個斐波那契數\n");

    for number in 1..=5 {
        println!(
            "斐波那契數第 {} 項為 {}",
            number,
            fibonacci_recursive(number)
        );
    }

    for number in 6..=10 {
        println!("斐波那契數第 {} 項為 {}", number, fibonacci_dp(number));
    }
}

fn main() {
    fibonacci();
}
