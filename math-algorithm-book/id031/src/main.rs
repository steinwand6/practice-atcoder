use proconio;

fn main() {
    proconio::input! {
        n:usize,
        a:[usize;n]
    };
    println!("{}", taros_vacation(a));
}

fn taros_vacation(a: Vec<usize>) -> usize {
    let mut dp1 = vec![0; a.len()];
    let mut dp2 = vec![0; a.len()];

    if a.len() > 0 {
        dp1[0] = a[0];
    }
    if a.len() > 1 {
        dp1[1] = a[1];
        dp2[1] = a[0];
    }
    for i in 1..a.len() {
        dp1[i] = dp2[i - 1] + a[i];
        dp2[i] = dp1[i - 1].max(dp2[i - 1]);
    }
    dp1[a.len() - 1].max(dp2[a.len() - 1])
}

#[test]
fn sample1() {
    assert_eq!(taros_vacation(vec![2, 5, 3, 3, 1]), 8)
}
