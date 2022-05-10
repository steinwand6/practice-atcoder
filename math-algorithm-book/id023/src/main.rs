use proconio::input;

fn main() {
    input! {
        n: usize,
        b: [usize;n],
        r: [usize;n],
    }
    println!("{}", dice_expectation(b, r));
}

fn dice_expectation(b: Vec<usize>, r: Vec<usize>) -> f64 {
    let face = b.len();
    let expect1: f64 = b.iter().sum::<usize>() as f64 / face as f64;
    let expect2: f64 = r.iter().sum::<usize>() as f64 / face as f64;
    return expect1 + expect2;
}

#[cfg(test)]
mod tests {
    use crate::dice_expectation;

    #[test]
    fn sample1() {
        assert_eq!(
            dice_expectation(vec![1, 2, 3], vec![10, 20, 30]),
            22.00000000
        )
    }
}
