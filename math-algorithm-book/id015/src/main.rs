use proconio::input;

fn main() {
    input! {
        a:u32,
        b:u32,
    }
    println!("{}", greatest_common_divisor(a, b));
}

fn greatest_common_divisor(a: u32, b: u32) -> u32 {
    if b == 0 {
        return a;
    }
    return greatest_common_divisor(b, a % b);
}

#[cfg(test)]
mod tests {
    use crate::greatest_common_divisor;

    #[test]
    fn sample1() {
        assert_eq!(greatest_common_divisor(33, 88), 11)
    }
    #[test]
    fn sample2() {
        assert_eq!(greatest_common_divisor(123, 777), 3)
    }
    #[test]
    fn sample_book() {
        assert_eq!(greatest_common_divisor(372, 777), 3)
    }
}
