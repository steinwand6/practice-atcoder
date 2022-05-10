use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize;n],
        b: [usize;n]
    };
    println!("{}", jiros_vacation(a, b));
}

fn jiros_vacation(a: Vec<usize>, b: Vec<usize>) -> f64 {
    let n = a.len();
    let mut result = 0.0;
    for i in 0..n {
        result += (a[i] as f64) * 1.0 / 3.0 + (b[i] as f64) * 2.0 / 3.0;
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::jiros_vacation;

    fn zatu_floor(f: f64) -> f64 {
        let tmp = (f * 1000.0) as i64;
        tmp as f64 / 1000.0
    }

    #[test]
    fn sample1() {
        assert_eq!(
            zatu_floor(jiros_vacation(vec![3, 1, 4, 1, 5], vec![9, 2, 6, 5, 3])),
            21.333f64
        )
    }
}
