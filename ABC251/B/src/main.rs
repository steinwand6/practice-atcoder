use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        w: usize,
        a: [usize;n]
    };
    println!("{}", at_most_3(w, a));
}

fn at_most_3(w: usize, a: Vec<usize>) -> usize {
    let mut hs = HashSet::<usize>::new();
    let length = a.len();
    let mut i = 0;

    while i < length {
        if a[i] <= w {
            hs.insert(a[i]);
        }
        let mut j = i + 1;
        while j < length {
            if a[i] + a[j] <= w {
                hs.insert(a[i] + a[j]);
            }
            let mut k = j + 1;
            while k < length {
                if a[i] + a[j] + a[k] <= w {
                    hs.insert(a[i] + a[j] + a[k]);
                }
                k += 1;
            }
            j += 1;
        }
        i += 1;
    }
    hs.len()
}

#[test]
fn sample1() {
    assert_eq!(at_most_3(10, vec![1, 3]), 3)
}
#[test]
fn sample2() {
    assert_eq!(at_most_3(1, vec![2, 3]), 0)
}
#[test]
fn sample3() {
    assert_eq!(at_most_3(12, vec![3, 3, 3, 3]), 3)
}
#[test]
fn sample4() {
    assert_eq!(at_most_3(251, vec![202, 20, 5, 1, 4, 2, 100]), 48)
}
