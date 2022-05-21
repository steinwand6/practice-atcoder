use proconio::input;

fn main() {
    input! {
        n:usize,
        k:usize,
        a: [usize;n],
        b: [usize;k]
    };
    println!(
        "{}",
        if takahashis_failure(a, b) {
            "Yes"
        } else {
            "No"
        }
    )
}

fn takahashis_failure(a: Vec<usize>, b: Vec<usize>) -> bool {
    let mut max_oishisa = 0;
    for i in &a {
        max_oishisa = max_oishisa.max(*i);
    }
    for i in a.iter().enumerate() {
        if *i.1 == max_oishisa {
            if b.contains(&(i.0 + 1)) {
                return true;
            }
        }
    }
    false
}

#[test]
fn sample1() {
    assert_eq!(
        takahashis_failure(vec![6, 8, 10, 7, 10], vec![2, 3, 4]),
        true
    );
}

#[test]
fn sample2() {
    assert_eq!(
        takahashis_failure(vec![100, 100, 100, 1, 1], vec![5, 4]),
        false
    );
}

#[test]
fn sample3() {
    assert_eq!(takahashis_failure(vec![100, 1], vec![2]), false);
}
#[test]
fn my_test1() {
    assert_eq!(takahashis_failure(vec![100], vec![1]), true);
}
