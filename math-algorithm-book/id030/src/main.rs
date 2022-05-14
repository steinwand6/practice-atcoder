use proconio;

fn main() {
    proconio::input! {
        n:usize,
        w:usize,
        array: [(usize, usize);n]
    };
    let mut wa = vec![0; 0];
    for el in &array {
        wa.push(el.0)
    }
    let mut va = vec![0; 0];
    for el in &array {
        va.push(el.1)
    }
    println!("{}", knapsack(w, wa, va));
}

fn knapsack(limit: usize, w: Vec<usize>, v: Vec<usize>) -> usize {
    let mut dp = vec![vec![0; limit]; w.len()];
    for i in 0..limit {
        if i >= w[0] - 1 {
            dp[0][i] = v[0];
        }
    }
    let mut max = dp[0][limit - 1];
    for i in 1..w.len() {
        for j in 0..limit {
            if j <= w[i] - 1 {
                dp[i][j] = dp[i - 1][j];
            } else {
                dp[i][j] = (dp[i - 1][j]).max(dp[i - 1][j - w[i]] + v[i]);
            }
            max = max.max(dp[i][j]);
        }
    }
    max
}

#[test]
fn sample1() {
    assert_eq!(knapsack(8, vec![3, 4, 5], vec![30, 50, 60]), 90)
}
#[test]
fn sample2() {
    assert_eq!(
        knapsack(
            5,
            vec![1, 1, 1, 1, 1],
            vec![1000000000, 1000000000, 1000000000, 1000000000, 1000000000]
        ),
        5000000000
    )
}
#[test]
fn sample3() {
    assert_eq!(
        knapsack(15, vec![6, 5, 6, 6, 3, 7], vec![5, 6, 4, 6, 5, 2]),
        17
    )
}
#[test]
fn my_test1() {
    assert_eq!(knapsack(7, vec![7], vec![2]), 2)
}
