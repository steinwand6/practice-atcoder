use proconio;

fn main() {
    proconio::input! {
        n: usize
    };
    println!("{}", climb_stairs(n))
}

fn climb_stairs(n: usize) -> usize {
    let mut dp = Vec::<usize>::new();
    for i in 0..=n {
        if i == 0 || i == 1 {
            dp.push(1);
            continue;
        }
        dp.push(dp[i - 1] + dp[i - 2]);
    }
    dp[n]
}

#[test]
fn sample1() {
    assert_eq!(climb_stairs(4), 5)
}
#[test]
fn sample2() {
    assert_eq!(climb_stairs(45), 1836311903)
}
