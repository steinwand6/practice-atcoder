use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n:usize,
        a:[usize;n]
    };
    println!("{}", distinct_trio(a));
}

fn distinct_trio(a: Vec<usize>) -> usize {
    let mut hm = HashMap::<usize, usize>::new();
    for i in a {
        *hm.entry(i).or_insert(0) += 1
    }
    let mut ans = 0;
    for (k1, v1) in &hm {
        for (k2, v2) in &hm {
            if k1 == k2 {
                continue;
            }
            for (k3, v3) in &hm {
                if k1 == k3 || k2 == k3 {
                    continue;
                }
                ans += v1 * v2 * v3;
            }
        }
    }
    ans / 3 / 2
}

#[test]
fn sample1() {
    assert_eq!(distinct_trio(vec![3, 1, 4, 1]), 2)
}
#[test]
fn sample2() {
    assert_eq!(
        distinct_trio(vec![
            99999, 99998, 99997, 99996, 99995, 99994, 99993, 99992, 99991, 99990
        ]),
        120
    )
}
#[test]
fn sample3() {
    assert_eq!(
        distinct_trio(vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5, 8, 9, 7, 9]),
        355
    )
}
