use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [[usize;2];n],
    }
    let mut p = vec![0; n];
    let mut q = vec![0; n];
    for i in 0..n {
        p[i] = a[i][0];
        q[i] = a[i][1];
    }
    println!("{}", answer_exam_randomly(p, q));
}

fn answer_exam_randomly(p: Vec<usize>, q: Vec<usize>) -> f64 {
    let n = p.len();
    let mut result: f64 = 0.0;
    for i in 0..n {
        result += q[i] as f64 / p[i] as f64;
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::answer_exam_randomly;

    #[test]
    fn sample1() {
        assert_eq!(answer_exam_randomly(vec![2, 4], vec![50, 100]), 50.0)
    }
}
