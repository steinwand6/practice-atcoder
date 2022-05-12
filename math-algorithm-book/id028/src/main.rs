use proconio::input;
fn main() {
    input! {
        n: usize,
        h:[i64;n]
    };
    println!("{}", frog1(h));
}

fn frog1(h: Vec<i64>) -> i64 {
    let mut dp = Vec::<i64>::with_capacity(h.len());
    dp.push(0);
    dp.push((h[1] - h[0]).abs());
    for i in 2..h.len() {
        let jump1 = dp[i - 1] + (h[i - 1] - h[i]).abs();
        let jump2 = dp[i - 2] + (h[i - 2] - h[i]).abs();
        dp.push(jump1.min(jump2));
    }
    return dp[h.len() - 1];
}

#[cfg(test)]
mod tests {
    use crate::frog1;

    #[test]
    fn sample1() {
        assert_eq!(frog1(vec![10, 30, 40, 20]), 30);
    }
    #[test]
    fn sample2() {
        assert_eq!(frog1(vec![10, 10]), 00);
    }
    #[test]
    fn sample3() {
        assert_eq!(frog1(vec![30, 10, 60, 10, 60, 50]), 40);
    }
}
