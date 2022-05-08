use proconio;

fn main() {
    proconio::input! {
        n: u32,
        array: [u64;n],
    };
    let ans = greatest_common_divisor_n(array);
    println!("{}", ans);
}

fn greatest_common_divisor_n(array: Vec<u64>) -> u64 {
    let mut ans = 0;
    for num in array {
        ans = greatest_common_divisor(num, ans);
    }
    ans
}

fn greatest_common_divisor(a: u64, b: u64) -> u64 {
    match b {
        0 => return a,
        _ => greatest_common_divisor(b, a % b),
    }
}

#[cfg(test)]
mod tests {
    use crate::greatest_common_divisor_n;

    #[test]
    fn sample1() {
        assert_eq!(greatest_common_divisor_n(vec![12, 18, 24]), 6)
    }
    #[test]
    fn sample2() {
        assert_eq!(greatest_common_divisor_n(vec![24, 40, 60, 80, 90, 120]), 2)
    }
}
