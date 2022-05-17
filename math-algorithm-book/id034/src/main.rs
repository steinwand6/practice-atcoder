use std::f64::INFINITY;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [(i64, i64);n]
    };
    println!("{}", nearest_point(a));
}

struct Point {
    x: i64,
    y: i64,
}

struct Vector {
    x: f64,
    y: f64,
}

impl Vector {
    fn new(p1: &Point, p2: &Point) -> Self {
        Vector {
            x: (p1.x - p2.x) as f64,
            y: (p1.y - p2.y) as f64,
        }
    }

    fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn nearest_point(a: Vec<(i64, i64)>) -> f64 {
    let mut ans = INFINITY;
    let size = a.len();
    for i in 0..size - 1 {
        for j in i + 1..size {
            let tmp = Vector::new(
                &Point {
                    x: a[i].0,
                    y: a[i].1,
                },
                &Point {
                    x: a[j].0,
                    y: a[j].1,
                },
            );
            ans = if ans > tmp.magnitude() {
                tmp.magnitude()
            } else {
                ans
            };
        }
    }
    ans
}

#[test]
fn sample1() {
    assert!(
        (nearest_point(vec![(0, 1), (2, 0), (2, 3), (3, 1)]) - 1.4142135623730950488016887242)
            .abs()
            < 0.000001
    )
}

#[test]
fn magnitute_test() {
    let vctr_a = Vector::new(&Point { x: 1, y: 1 }, &Point { x: 5, y: 4 });
    assert_eq!(vctr_a.magnitude(), 5.0)
}
