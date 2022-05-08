use core::fmt;
use proconio;

struct NumVec(Vec<u64>);

impl fmt::Display for NumVec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut str = String::new();
        for num in &self.0[0..self.0.len() - 1] {
            str.push_str(&num.to_string());
            str.push_str(" ");
        }
        str.push_str(&self.0[self.0.len() - 1].to_string());
        write!(f, "{}", str)
    }
}

fn main() {
    proconio::input! {
        num: u64
    };
    let results = NumVec(factorization(num));
    println!("{}", results);
}

fn factorization(mut num: u64) -> Vec<u64> {
    let mut results = Vec::<u64>::new();
    let mut i: u64 = 2;

    while num >= i * i {
        while num % i == 0 {
            num /= i;
            results.push(i);
        }
        i += 1;
    }
    if num > 1 {
        results.push(num);
    }
    results
}

#[cfg(test)]
mod tests {
    use crate::factorization;

    #[test]
    fn sample1() {
        assert_eq!(factorization(10), vec![2, 5])
    }
    #[test]
    fn sample2() {
        assert_eq!(factorization(36), vec![2, 2, 3, 3])
    }
}
