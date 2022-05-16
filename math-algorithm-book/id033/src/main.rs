struct Point {
    x: i64,
    y: i64,
}

struct Vector {
    x: f64,
    y: f64,
}

impl Vector {
    fn new(a: &Point, b: &Point) -> Self {
        Vector {
            x: (a.x - b.x) as f64,
            y: (a.y - b.y) as f64,
        }
    }
    fn inner_product(&self, another: &Vector) -> f64 {
        self.x * another.x + self.y * another.y
    }
    fn outer_product(&self, another: &Vector) -> f64 {
        (self.x * another.y - self.y * another.x).abs()
    }
    fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    println!("Hello, world!");
}

fn distance(a: Point, b: Point, c: Point) -> f64 {
    let bc = Vector::new(&b, &c);
    let ba = Vector::new(&b, &a);
    let ac = Vector::new(&a, &c);
    let distance = if bc.inner_product(&ba) < 0.0 {
        ba.magnitude()
    } else if bc.inner_product(&ba) > 0.0 {
        ac.magnitude()
    } else {
        bc.outer_product(&ba) / bc.magnitude()
    };
    dbg!(distance)
}

#[test]
fn magnitute_test() {
    let vctrA = Vector::new(&Point { x: 1, y: 1 }, &Point { x: 5, y: 4 });
    assert_eq!(vctrA.magnitude(), 5.0)
}

#[test]
fn sample1() {
    assert!(
        distance(
            Point { x: 0, y: 5 },
            Point { x: 1, y: 1 },
            Point { x: 3, y: 0 }
        ) - 4.123105625618
            < 0.000001
    )
}
#[test]
fn sample2() {
    assert!(
        distance(
            Point { x: -40, y: -30 },
            Point { x: -50, y: -10 },
            Point { x: -20, y: -20 }
        ) - 15.811388300842
            < 0.000001
    )
}
#[test]
fn sample3() {
    assert!(
        distance(
            Point {
                x: 1000000000,
                y: 1000000000
            },
            Point {
                x: -1000000000,
                y: -1000000000
            },
            Point {
                x: 0,
                y: -1000000000
            }
        ) - 2236067977.499789714813
            < 0.000001
    )
}
